# dogmv - GUI Markdown Viewer for NixOS/Hyprland

## プロジェクト概要
NixOS/Hyprland環境で動作するGUIベースのMarkdownビューアアプリケーション

## 目的
- Wayland/Hyprland環境でネイティブに動作するMarkdownビューア
- 高速でシンプルなUI
- コードブロックのシンタックスハイライト対応
- サイドバー付きファイルブラウザ
- ソースコードビューア機能

## 技術スタック（実装済み）
### 言語
- **Rust** (edition 2021)

### GUIフレームワーク
- **GTK4** 0.10 (Wayland対応)
- **WebKit6** 0.5 (プレビュー表示用)

### Markdownレンダリング
- **comrak** 0.24 (CommonMark/GFM準拠)
- **syntect** 5.2 (シンタックスハイライト)

### その他ライブラリ
- **notify** 6.1 (ファイル監視)
- **log** 0.4 + **env_logger** 0.11 (ログ出力)
- **thiserror** 1.0 (エラーハンドリング)
- **ctor** 0.2 (初期化処理)

## 主要機能（実装済み）
- [x] Markdownファイルの読み込み
- [x] リアルタイムプレビュー（ファイル監視・自動リロード）
- [x] シンタックスハイライト（200以上の言語対応）
- [x] テーブル、リスト、コードブロックのレンダリング
- [x] 画像表示（相対パス対応）
- [x] サイドバー付きファイルツリービュー
- [x] ソースコード表示機能（.rs, .py, .js等）
- [x] キーボードショートカット（Ctrl+O/R/Q）
- [x] ファイル監視（自動リロード、500ms間隔）

## プロジェクト構成

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
│   ├── USER_MANUAL.md       # ユーザーマニュアル
│   ├── DEVELOPER.md         # 開発者ドキュメント
│   ├── IMPLEMENTATION_PLAN.md # 実装計画
│   ├── ADR.md               # アーキテクチャ決定記録
│   ├── CHANGELOG.md         # 変更履歴
│   └── ...
├── examples/                # サンプルファイル
│   ├── test.md              # Markdownサンプル
│   └── test_highlight.*     # ソースコードサンプル
├── Cargo.toml
├── flake.nix
├── shell.nix
├── README.md
└── CLAUDE.md (このファイル)
```

## 開発環境
- OS: NixOS
- Window Manager: Hyprland (Wayland)
- Display Server: Wayland
- Package Manager: Nix (with flakes)

## ビルド方法

### Nix開発環境を使用
```bash
# 開発環境に入る
nix-shell

# ビルド
cargo build

# 実行
cargo run <file.md>
```

### リリースビルド
```bash
nix-shell --command "cargo build --release"
./target/release/dogmv <file.md>
```

## ドキュメント
詳細なドキュメントは`docs/`ディレクトリを参照：
- **ユーザーマニュアル**: `docs/USER_MANUAL.md`
- **開発者ドキュメント**: `docs/DEVELOPER.md`
- **変更履歴**: `docs/CHANGELOG.md`

## ライセンス
未定
