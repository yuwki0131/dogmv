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

### 🚧 開発中
- シンタックスハイライト (syntect統合)
- ファイル監視・自動リロード (notify使用)
- キーボードショートカット (Ctrl+O/R/Q)

## 次のステップ

- [ ] フェーズ4: シンタックスハイライト
- [ ] フェーズ5: ファイル監視・自動リロード
- [ ] フェーズ6: キーボードショートカット
- [ ] フェーズ7: エラーハンドリング強化
- [ ] フェーズ8: ユニットテスト

詳細は`IMPLEMENTATION_PLAN.md`を参照してください。
