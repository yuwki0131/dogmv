# GSettings FileChooser Crash - Solution Summary

## 問題の経緯と解決までの過程

### 初期問題
Ctrl+O を押してファイル選択ダイアログを開くと、アプリケーションがクラッシュする。

```
(dogmv:XXXXX): GLib-GIO-ERROR **: Settings schema 'org.gtk.gtk4.Settings.FileChooser' is not installed
Trace/breakpoint trap (core dumped)
```

---

## 試行錯誤の過程

### 試行 1: FileChooserNativeへの置き換え
**実施内容**: `FileChooserDialog` → `FileChooserNative` に変更

**結果**: ❌ 失敗（クラッシュは継続）

**理由**: FileChooserNativeも内部的にGSettingsにアクセスしようとする

---

### 試行 2: main()内で環境変数設定
**実施内容**:
```rust
fn main() {
    std::env::set_var("GSETTINGS_BACKEND", "memory");
    // ...
}
```

**結果**: ❌ 失敗（クラッシュは継続）

**理由**: GTKの初期化がmain()の前または非常に早い段階で行われるため、環境変数の設定が間に合わない

**検証**:
```bash
# 外部から環境変数を設定すると動作することを確認
GSETTINGS_BACKEND=memory ./target/debug/dogmv README.md
# → 正常動作 ✅
```

---

### 試行 3: ctorクレートを使用した早期初期化
**実施内容**:
```rust
use ctor::ctor;

#[ctor]
fn init_environment() {
    std::env::set_var("GSETTINGS_BACKEND", "memory");
}
```

**結果**: ✅ 成功！

**理由**:
- `#[ctor]`属性により、`main()`よりも前に環境変数が設定される
- GTKの初期化時に正しい環境変数が参照される
- GSettingsがメモリバックエンドを使用し、スキーマファイルへのアクセスを回避

---

## 最終的な解決策

### 実装内容

#### 1. Cargo.tomlにctorクレートを追加
```toml
[dependencies]
ctor = "0.2"
```

#### 2. main.rsで早期初期化
```rust
use ctor::ctor;

#[ctor]
fn init_environment() {
    // This runs before main(), ensuring the env var is set before GTK initializes
    std::env::set_var("GSETTINGS_BACKEND", "memory");
}
```

#### 3. FileChooserNativeの使用（副次的な改善）
```rust
let dialog = FileChooserNative::new(
    Some("Open File"),
    Some(window),
    FileChooserAction::Open,
    Some("_Open"),
    Some("_Cancel"),
);
```

---

## 技術的な詳細

### なぜ #[ctor] が必要だったのか？

1. **GTKの初期化タイミング**
   - GTKは静的リンクされたライブラリの初期化処理として、環境変数を読み取る
   - この処理はRustの`main()`関数が呼ばれる前に実行される可能性がある

2. **GSettingsの動作**
   - GSettingsは環境変数 `GSETTINGS_BACKEND` を読み取って、どのバックエンドを使用するか決定する
   - この読み取りはGLib/GTKの初期化時に一度だけ行われる
   - 一度初期化された後は変更できない

3. **ctorの動作**
   - `#[ctor]` 属性は、リンカーの `.init_array` セクションを使用
   - このセクションのコードは、`main()`よりも前、かつ動的ライブラリの初期化と同じタイミングで実行される
   - そのため、GTKの初期化前に確実に環境変数を設定できる

### プログラムの実行順序

```
1. [OS] プロセス起動
2. [リンカー] .init_array セクションの実行
   → #[ctor] init_environment() ← ここで GSETTINGS_BACKEND=memory 設定
3. [GTK] 動的ライブラリの初期化
   → GSettings初期化（環境変数を読み取る）
4. [Rust] main() 関数開始
5. [GTK] Application::run()
```

---

## 検証結果

### Dev build
```bash
$ cargo build
$ ./target/debug/dogmv README.md
# Ctrl+O を押す
# → クラッシュせず、ファイル選択ダイアログが正常に表示 ✅
```

### Release build
```bash
$ cargo build --release
$ ./target/release/dogmv README.md
# Ctrl+O を押す
# → クラッシュせず、ファイル選択ダイアログが正常に表示 ✅
```

---

## トレードオフ

### メリット
- ✅ GSettingsスキーマがなくてもアプリケーションが動作
- ✅ NixOS などの特殊な環境でも安定動作
- ✅ クラッシュの完全回避

### デメリット
- ⚠️ FileChooserの設定が保存されない
  - 最後に開いたディレクトリ
  - 表示モード（リストビュー/アイコンビュー）
  - ソート順序
- ⚠️ `ctor` クレートへの依存追加

### 判断
ユーザー体験として、**アプリケーションのクラッシュ回避**が**設定の永続化**よりも重要であると判断。

---

## 今後の改善案

1. **設定ファイルの独自実装**
   - GSettingsを使わず、独自の設定ファイル（TOML/JSON）で最後のディレクトリを保存

2. **環境検出**
   - GSettingsスキーマの存在を検出し、利用可能な場合のみ使用

3. **NixOS対応**
   - Nixパッケージングでスキーマを正しくインストール

---

## 関連する警告

以下の警告は、この修正とは無関係で、機能に影響しません：

```
Gdk-WARNING: Failed to read portal settings
MESA-INTEL: warning: cannot initialize blitter engine
```

これらはWayland/Hyprland環境とIntelグラフィックスドライバーの既知の警告です。
