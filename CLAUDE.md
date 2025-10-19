# dogmv - GUI Markdown Viewer for NixOS/Hyprland

## プロジェクト概要
NixOS/Hyprland環境で動作するGUIベースのMarkdownビューアアプリケーション

## 目的
- Wayland/Hyprland環境でネイティブに動作するMarkdownビューア
- 高速でシンプルなUI
- コードブロックのシンタックスハイライト対応
- プレビュー機能

## 技術スタック（検討中）
### 候補言語
- Rust
- Go
- Python

### 候補GUIフレームワーク
- GTK4 (Wayland対応良好)
- Qt6 (Wayland対応)
- egui (Rust向けイミディエイトモードGUI)
- Iced (Rust向け宣言的GUI)
- Slint (クロスプラットフォーム)

### Markdownレンダリング
- pulldown-cmark (Rust)
- comrak (Rust, CommonMark準拠)
- python-markdown2 (Python)
- goldmark (Go)

## 主要機能（予定）
- [ ] Markdownファイルの読み込み
- [ ] リアルタイムプレビュー
- [ ] シンタックスハイライト
- [ ] テーブル、リスト、コードブロックのレンダリング
- [ ] 画像表示
- [ ] リンクのクリック対応
- [ ] ダークモード/ライトモード切り替え
- [ ] ファイル監視（自動リロード）

## 開発環境
- OS: NixOS
- Window Manager: Hyprland (Wayland)
- Display Server: Wayland

## ビルド方法（未定）
NixパッケージまたはCargoビルドを予定

## ライセンス
未定
