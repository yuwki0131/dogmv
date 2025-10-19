# dogmv - Markdown Viewer

NixOS/Hyprland環境で動作するGUIベースのMarkdownビューアアプリケーション

## 現在のステータス

✅ **フェーズ1完了**: プロジェクト基盤構築
- Rustプロジェクト初期化
- 依存クレート追加 (gtk4 0.10, webkit6 0.5, comrak, syntect, notify)
- Nixパッケージング設定 (flake.nix, shell.nix)

✅ **フェーズ2完了**: 基本的なGUIアプリケーション
- GTK4アプリケーション初期化
- WebView統合 (webkit6使用)

✅ **フェーズ3完了**: Markdownレンダリング
- CLI引数パース (ファイルパス必須)
- Markdownファイル読み込み
- comrakでMarkdown → HTML変換 (GFM対応)
- GitHub風CSSスタイリング
- WebViewへのHTML表示
- 相対パス画像対応 (`<base>`タグ)

✅ **フェーズ4完了**: シンタックスハイライト
- comrak + syntect統合
- SyntectAdapter使用 (InspiredGitHubテーマ)
- 200以上の言語サポート
- インラインスタイルでの色付け
- コードブロックの自動言語検出

✅ **フェーズ5完了**: ファイル監視・自動リロード
- notify crateで監視
- inotify (Linux) 使用で低オーバーヘッド
- ファイル変更検知 (modify/create イベント)
- 自動リロード (500msポーリング)
- バックグラウンドスレッドで監視
- GTKメインスレッドで安全にリロード

✅ **フェーズ6完了**: キーボードショートカット
- EventControllerKey統合
- Ctrl+R: ファイルリロード
- Ctrl+Q: アプリケーション終了
- Ctrl+O: ファイル選択ダイアログ
- FileDialog with Markdown file filter (*.md, *.markdown)

## 技術スタック

- **言語**: Rust (edition 2021)
- **GUIフレームワーク**: gtk4 0.10
- **WebView**: webkit6 0.5
- **Markdownパーサー**: comrak 0.24
- **シンタックスハイライト**: syntect 5.2
- **ファイル監視**: notify 6.1
- **ログ**: log 0.4 + env_logger 0.11

## ビルド方法

### Nix開発環境

```bash
# 開発環境に入る
nix-shell

# ビルド確認
cargo check

# ビルド
cargo build

# 実行 (Markdownファイルを指定)
cargo run test.md

# または、既存のMarkdownファイルを開く
cargo run README.md
```

## 現在の機能

### ✅ 実装済み
- Markdownファイルの読み込みと表示
- GitHub Flavored Markdown対応 (テーブル、タスクリスト、取り消し線等)
- GitHub風CSSスタイリング
- 相対パス画像の表示対応
- エラーハンドリング (ファイル不存在、読み込みエラー)
- **シンタックスハイライト** (Rust, Python, JavaScript, Bash等200以上の言語)
- **ファイル監視・自動リロード** (外部エディタでの変更を自動検知)
- **キーボードショートカット** (Ctrl+R: リロード, Ctrl+Q: 終了, Ctrl+O: ファイルを開く)

### 🚧 開発中
- なし (フェーズ1〜9すべて完了 - v0.1.0リリース準備完了)

## 完了したフェーズ

- [x] フェーズ1: プロジェクト基盤構築
- [x] フェーズ2: 基本的なGUIアプリケーション
- [x] フェーズ3: Markdownレンダリング
- [x] フェーズ4: シンタックスハイライト
- [x] フェーズ5: ファイル監視・自動リロード
- [x] フェーズ6: キーボードショートカット
- [x] フェーズ7: エラーハンドリング強化
- [x] フェーズ8: ドキュメント整備
- [x] フェーズ9: 配布準備

## ドキュメント

- **ユーザーマニュアル**: `USER_MANUAL.md` - 使い方、機能、トラブルシューティング
- **開発者ドキュメント**: `DEVELOPER.md` - アーキテクチャ、開発手順、貢献ガイド
- **変更履歴**: `CHANGELOG.md` - バージョン履歴と変更内容
- **実装計画**: `IMPLEMENTATION_PLAN.md` - 開発ロードマップ
- **技術決定記録**: `ADR.md` - アーキテクチャ決定理由
- **開発ログ**: `log.md` - 詳細な開発履歴

## ライブプレビュー機能

dogmvは外部エディタで編集中のMarkdownファイルを監視し、自動的にリロードします：

```bash
# ターミナル1: dogmvでファイルを開く
nix-shell
cargo run test.md

# ターミナル2: 好きなエディタで編集
vim test.md
# または
code test.md

# test.mdを保存すると、dogmvが自動的にリロードします！
```

**動作**:
- ファイル変更を500ms間隔でチェック
- 変更検知後、即座にMarkdownを再レンダリング
- エディタとビューアを並べて、リアルタイムプレビューが可能

詳細は`IMPLEMENTATION_PLAN.md`を参照してください。
