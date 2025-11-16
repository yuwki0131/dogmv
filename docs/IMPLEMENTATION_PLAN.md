# 実装計画 (Implementation Plan)

## プロジェクト: dogmv - Markdown Viewer

**作成日**: 2025-10-20
**対象**: 初期実装（v0.1.0）

---

## 1. プロジェクト概要

NixOS/Hyprland環境で動作するGUIベースのMarkdownビューアを実装する。

**主要機能**:
- Markdownファイルの読み込み・表示
- シンタックスハイライト付きコードブロック
- ファイル変更の自動検知・リロード
- 基本的なキーボードショートカット

**技術スタック**: ADR.md参照

---

## 2. 開発フェーズ

### フェーズ1: プロジェクト基盤構築
**目標**: ビルド環境とプロジェクト構造の確立

#### タスク1-1: Rustプロジェクト初期化
- [ ] `cargo init`でプロジェクト作成
- [ ] `Cargo.toml`の基本設定
- [ ] ディレクトリ構造の作成

**成果物**:
```
dogmv/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

**時間見積**: 30分

---

#### タスク1-2: 依存クレートの追加
- [ ] gtk4-rsの追加
- [ ] webkit2gtk-rsの追加
- [ ] comrakの追加
- [ ] syntectの追加
- [ ] notifyの追加
- [ ] logとenv_loggerの追加

**Cargo.toml例**:
```toml
[dependencies]
gtk4 = "0.9"
webkit2gtk = { version = "2.0", features = ["v2_40"] }
comrak = "0.28"
syntect = "5.2"
notify = "6.1"
log = "0.4"
env_logger = "0.11"
```

**時間見積**: 1時間（依存解決、コンパイル確認）

---

#### タスク1-3: Nixパッケージング設定
- [ ] `flake.nix`の作成
- [ ] `shell.nix`の作成（開発環境）
- [ ] craneを使ったビルド設定
- [ ] システムライブラリ依存の明記

**flake.nix構成**:
```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
      in {
        packages.default = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = with pkgs; [
            gtk4
            webkitgtk_6_0
            pkg-config
          ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            gtk4
            webkitgtk_6_0
            pkg-config
          ];
        };
      }
    );
}
```

**時間見積**: 2時間

---

### フェーズ2: 基本的なGUIアプリケーション
**目標**: GTK4ウィンドウの表示とWebView統合

#### タスク2-1: GTK4アプリケーション初期化
- [ ] `gtk::Application`の作成
- [ ] メインウィンドウの作成
- [ ] ウィンドウサイズ設定（デフォルト1024x768）
- [ ] アプリケーション終了処理

**実装例** (`src/main.rs`):
```rust
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

const APP_ID: &str = "com.github.dogmv";

fn main() {
    env_logger::init();

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("dogmv - Markdown Viewer")
        .default_width(1024)
        .default_height(768)
        .build();

    window.present();
}
```

**検証**:
- `cargo run`でウィンドウが表示されること
- Waylandで動作すること（`GDK_BACKEND=wayland`）

**時間見積**: 2時間

---

#### タスク2-2: WebView統合
- [ ] `WebView`ウィジェットの追加
- [ ] HTMLコンテンツの読み込みテスト
- [ ] ウィンドウへの配置

**実装例**:
```rust
use webkit2gtk::WebView;

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("dogmv - Markdown Viewer")
        .default_width(1024)
        .default_height(768)
        .build();

    let webview = WebView::new();
    webview.load_html("<h1>Hello, dogmv!</h1>", None);

    window.set_child(Some(&webview));
    window.present();
}
```

**検証**:
- WebViewに"Hello, dogmv!"が表示されること

**時間見積**: 1時間

---

### フェーズ3: Markdownレンダリング
**目標**: Markdownファイルを読み込んでHTMLに変換・表示

#### タスク3-1: CLI引数パース
- [ ] `std::env::args()`でファイルパス取得
- [ ] 引数チェック（なければエラー）
- [ ] ファイル存在確認

**実装例**:
```rust
use std::env;
use std::fs;
use log::{error, info};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error!("Usage: dogmv <file.md>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    info!("Opening file: {}", file_path);

    if !std::path::Path::new(file_path).exists() {
        error!("File not found: {}", file_path);
        std::process::exit(1);
    }

    // GTK起動処理へ
}
```

**時間見積**: 1時間

---

#### タスク3-2: Markdownファイル読み込み
- [ ] ファイル読み込み（`fs::read_to_string`）
- [ ] UTF-8エラーハンドリング
- [ ] エラーログ出力

**実装例**:
```rust
fn load_markdown(path: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}
```

**時間見積**: 30分

---

#### タスク3-3: Markdown → HTML変換（comrak）
- [ ] comrakでMarkdown → HTML変換
- [ ] GFMオプション有効化（テーブル、タスクリスト等）
- [ ] HTML出力テスト

**実装例**:
```rust
use comrak::{markdown_to_html, Options};

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;

    markdown_to_html(markdown, &options)
}
```

**時間見積**: 1時間

---

#### タスク3-4: HTMLテンプレート作成
- [ ] CSSスタイルの作成（GitHub風）
- [ ] `<base>`タグで画像パス解決
- [ ] HTMLテンプレート関数

**実装例**:
```rust
fn create_html(body: &str, base_path: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <base href="file://{}/">
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
            line-height: 1.6;
            padding: 20px;
            max-width: 900px;
            margin: 0 auto;
        }}
        code {{
            background-color: #f6f8fa;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-family: monospace;
        }}
        pre {{
            background-color: #f6f8fa;
            padding: 16px;
            border-radius: 6px;
            overflow: auto;
        }}
        table {{
            border-collapse: collapse;
            width: 100%;
        }}
        th, td {{
            border: 1px solid #ddd;
            padding: 8px;
            text-align: left;
        }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        base_path, body
    )
}
```

**時間見積**: 2時間

---

#### タスク3-5: WebViewにHTML読み込み
- [ ] `webview.load_html()`でHTML表示
- [ ] ファイルパスから親ディレクトリ取得（`<base>`用）
- [ ] 表示確認

**実装例**:
```rust
use std::path::Path;

fn display_markdown(webview: &WebView, file_path: &str) {
    let markdown = load_markdown(file_path).expect("Failed to load file");
    let html_body = render_markdown(&markdown);

    let base_dir = Path::new(file_path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("");

    let full_html = create_html(&html_body, base_dir);
    webview.load_html(&full_html, None);
}
```

**検証**:
- Markdownファイルが正しく表示されること
- 画像（相対パス）が表示されること

**時間見積**: 1時間

---

### フェーズ4: シンタックスハイライト
**目標**: コードブロックにシンタックスハイライトを適用

#### タスク4-1: syntect統合
- [ ] syntectの初期化
- [ ] シンタックスセット、テーマセットの読み込み
- [ ] コードブロック検出とハイライト処理

**実装例**:
```rust
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::html::highlighted_html_for_string;

fn highlight_code(code: &str, lang: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ss.find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let theme = &ts.themes["InspiredGitHub"];

    highlighted_html_for_string(code, &ss, syntax, theme)
        .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", code))
}
```

**時間見積**: 3時間

---

#### タスク4-2: comrakとsyntectの統合
- [ ] comrakのカスタムレンダラー実装
- [ ] コードブロックをsyntectで処理
- [ ] HTML出力に統合

**注**: comrakの`plugins`機能またはカスタムレンダラーを使用

**時間見積**: 4時間（調査含む）

---

### フェーズ5: ファイル監視・自動リロード
**目標**: ファイル変更を検知して自動的にリロード

#### タスク5-1: notify統合
- [ ] `notify::Watcher`の作成
- [ ] ファイル変更イベントのハンドリング
- [ ] GTK4のメインループへの通知

**実装例**:
```rust
use notify::{Watcher, RecursiveMode, Result};
use std::sync::mpsc::channel;

fn watch_file(path: &str, callback: impl Fn() + Send + 'static) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        tx.send(res).unwrap();
    })?;

    watcher.watch(Path::new(path), RecursiveMode::NonRecursive)?;

    std::thread::spawn(move || {
        for res in rx {
            match res {
                Ok(_) => callback(),
                Err(e) => error!("Watch error: {:?}", e),
            }
        }
    });

    Ok(())
}
```

**時間見積**: 3時間

---

#### タスク5-2: リロード処理実装
- [ ] ファイル再読み込み
- [ ] HTML再生成
- [ ] WebView更新

**実装例**:
```rust
use glib::clone;

fn setup_file_watch(webview: &WebView, file_path: String) {
    let webview = webview.clone();

    watch_file(&file_path, move || {
        glib::idle_add_local(clone!(@weak webview, @strong file_path => @default-return glib::ControlFlow::Break, move || {
            info!("File changed, reloading...");
            display_markdown(&webview, &file_path);
            glib::ControlFlow::Break
        }));
    }).expect("Failed to watch file");
}
```

**時間見積**: 2時間

---

### フェーズ6: キーボードショートカット
**目標**: Ctrl+O/R/Qの実装

#### タスク6-1: キーイベントハンドリング
- [ ] GTK4の`EventControllerKey`使用
- [ ] Ctrl+O: ファイルを開く（`FileChooserDialog`）
- [ ] Ctrl+R: リロード
- [ ] Ctrl+Q: 終了

**実装例**:
```rust
use gtk4::{EventControllerKey, gdk};

fn setup_shortcuts(window: &ApplicationWindow, webview: &WebView, file_path: &str) {
    let controller = EventControllerKey::new();

    controller.connect_key_pressed(clone!(@weak window, @weak webview, @strong file_path => @default-return glib::Propagation::Proceed, move |_, key, _, modifier| {
        if modifier.contains(gdk::ModifierType::CONTROL_MASK) {
            match key {
                gdk::Key::o => {
                    // ファイルを開く
                    open_file_dialog(&window, &webview);
                    glib::Propagation::Stop
                },
                gdk::Key::r => {
                    // リロード
                    display_markdown(&webview, &file_path);
                    glib::Propagation::Stop
                },
                gdk::Key::q => {
                    // 終了
                    window.close();
                    glib::Propagation::Stop
                },
                _ => glib::Propagation::Proceed,
            }
        } else {
            glib::Propagation::Proceed
        }
    }));

    window.add_controller(controller);
}
```

**時間見積**: 3時間

---

#### タスク6-2: ファイルダイアログ実装
- [ ] `FileChooserDialog`の作成
- [ ] Markdownフィルター設定
- [ ] ファイル選択後のロード処理

**実装例**:
```rust
use gtk4::FileChooserDialog;

fn open_file_dialog(window: &ApplicationWindow, webview: &WebView) {
    let dialog = FileChooserDialog::new(
        Some("Open Markdown File"),
        Some(window),
        gtk4::FileChooserAction::Open,
        &[("Open", gtk4::ResponseType::Accept), ("Cancel", gtk4::ResponseType::Cancel)]
    );

    dialog.connect_response(clone!(@weak webview => move |dialog, response| {
        if response == gtk4::ResponseType::Accept {
            if let Some(file) = dialog.file() {
                if let Some(path) = file.path() {
                    display_markdown(&webview, path.to_str().unwrap());
                }
            }
        }
        dialog.close();
    }));

    dialog.show();
}
```

**時間見積**: 2時間

---

### フェーズ7: エラーハンドリング・ログ強化
**目標**: 堅牢なエラー処理とログ出力

#### タスク7-1: エラーハンドリング実装
- [ ] ファイル読み込みエラー処理
- [ ] UTF-8エラー処理
- [ ] 権限エラー処理
- [ ] エラーログ出力

**時間見積**: 2時間

---

#### タスク7-2: ログ出力強化
- [ ] 各処理にログ追加（info, debug）
- [ ] エラー時の詳細ログ
- [ ] `RUST_LOG`環境変数での制御確認

**時間見積**: 1時間

---

### フェーズ8: テスト・ドキュメント
**目標**: ユニットテストとドキュメント整備

#### タスク8-1: ユニットテスト作成
- [ ] Markdownパース機能のテスト
- [ ] HTML生成機能のテスト
- [ ] ファイルパス解決のテスト

**実装例**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_markdown() {
        let md = "# Hello\n\nThis is a **test**.";
        let html = render_markdown(md);
        assert!(html.contains("<h1>"));
        assert!(html.contains("<strong>"));
    }

    #[test]
    fn test_create_html() {
        let body = "<p>Test</p>";
        let html = create_html(body, "/tmp");
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<base href="));
    }
}
```

**時間見積**: 3時間

---

#### タスク8-2: ドキュメント整備
- [ ] README.md更新（使い方、ビルド方法）
- [ ] CLAUDE.md更新
- [ ] コメント追加

**時間見積**: 2時間

---

### フェーズ9: ビルド・動作確認
**目標**: NixOS環境でのビルド・実行確認

#### タスク9-1: Nixビルド確認
- [ ] `nix build`でビルド成功
- [ ] `nix develop`で開発環境起動
- [ ] 依存関係の確認

**時間見積**: 2時間

---

#### タスク9-2: 統合テスト
- [ ] 実際のMarkdownファイルで動作確認
- [ ] 画像表示確認
- [ ] コードハイライト確認
- [ ] ファイル監視確認
- [ ] キーボードショートカット確認

**時間見積**: 3時間

---

## 3. プロジェクト構成（実装済み）

```
dogmv/
├── src/
│   ├── main.rs              # アプリケーション起動、UI構築
│   ├── error.rs             # エラー定義
│   ├── file_system/         # CLI引数パース
│   │   ├── mod.rs
│   │   └── cli.rs
│   ├── markdown/            # Markdownレンダリング
│   │   ├── mod.rs
│   │   └── renderer.rs
│   ├── models/              # データモデル
│   │   ├── mod.rs
│   │   └── file_item.rs
│   └── ui/                  # UI関連
│       ├── mod.rs
│       ├── preview.rs       # プレビュー表示
│       ├── sidebar.rs       # サイドバートグル
│       └── tree_view.rs     # ツリービュー
├── docs/                    # ドキュメント
├── Cargo.toml
├── Cargo.lock
├── flake.nix
├── shell.nix
├── README.md
├── CLAUDE.md
└── test.md
```

---

## 4. マイルストーン

| マイルストーン | 内容 | 期限目安 |
|--------------|------|---------|
| M1: プロジェクト基盤 | フェーズ1完了 | 1日目 |
| M2: 基本GUI | フェーズ2完了 | 2日目 |
| M3: Markdown表示 | フェーズ3完了 | 3-4日目 |
| M4: ハイライト | フェーズ4完了 | 5-6日目 |
| M5: ファイル監視 | フェーズ5完了 | 7日目 |
| M6: キーボード対応 | フェーズ6完了 | 8日目 |
| M7: エラー処理 | フェーズ7完了 | 9日目 |
| M8: テスト | フェーズ8完了 | 10日目 |
| M9: リリース準備 | フェーズ9完了 | 11-12日目 |

**総時間見積**: 約40-50時間（実装のみ）

---

## 5. リスク管理

| リスク | 影響 | 対策 | ステータス |
|--------|------|------|----------|
| webkit6のAPI変更 | 高 | バージョン固定、ドキュメント確認 | ✅ 解決済み |
| syntect統合の複雑性 | 中 | 初期は簡易実装、段階的改善 | ✅ 解決済み |
| notify動作不安定 | 中 | エラーハンドリング強化 | ✅ 解決済み |
| NixOSビルドエラー | 高 | 早期にNix環境でテスト | ✅ 解決済み |
| Wayland固有の問題 | 中 | X11フォールバック検討（後回し） | ✅ Wayland安定動作 |

---

## 6. 実装完了状況

✅ **全フェーズ完了** (v0.1.0リリース準備完了)

1. ✅ **フェーズ1-9完了**: すべての主要機能が実装済み
2. ✅ **段階的実装**: 各フェーズを順番に完了
3. ✅ **継続的テスト**: 各フェーズ終了時に動作確認済み
4. ✅ **ドキュメント更新**: ドキュメント整備完了

### 今後の拡張候補
- ダークモード/ライトモード切り替え
- カスタムCSSテーマ
- PDF/HTMLエクスポート
- 検索機能
- 目次自動生成

---

## 付録: 参考リンク

- [gtk4-rs Documentation](https://gtk-rs.org/gtk4-rs/)
- [webkit6-rs Documentation](https://docs.rs/webkit6/)
- [comrak Documentation](https://docs.rs/comrak/)
- [syntect Documentation](https://docs.rs/syntect/)
- [notify Documentation](https://docs.rs/notify/)
- [Nix Flakes](https://nixos.wiki/wiki/Flakes)
- [crane](https://github.com/ipetkov/crane)
