# dogmv Refactoring Plan

**作成日**: 2025-10-20
**目的**: コードの保守性と拡張性を向上させるためのリファクタリング計画

## 現状分析

### 現在の問題点

1. **単一ファイルへの集中** (src/main.rs: ~1059行)
   - すべての機能が1ファイルに実装されている
   - 責務の分離が不十分
   - コードの見通しが悪い
   - 並行開発が困難

2. **外部リソースの埋め込み**
   - CSSがRustコード内にハードコーディングされている
   - HTMLテンプレートが文字列リテラルとして存在
   - スタイル変更のたびに再コンパイルが必要

3. **モジュール構造の欠如**
   - `mod file_item`以外のモジュール分割なし
   - 機能ごとの名前空間が存在しない
   - テストが散在している

4. **状態管理の複雑さ**
   - `AppState`が複数のウィジェット参照を保持
   - `Arc<Mutex<>>`が多用されている
   - スレッド間通信の見通しが悪い

5. **エラーハンドリングの一貫性不足**
   - `Result`型の使い方が統一されていない
   - エラー型が標準ライブラリのみ
   - カスタムエラー型が未定義

## リファクタリングの目標

### 短期目標（Phase 1）
- [x] モジュール分割の設計
- [ ] 外部リソースファイルの分離
- [ ] 基本的な責務分離の実装

### 中期目標（Phase 2）
- [ ] カスタムエラー型の導入
- [ ] 状態管理の改善
- [ ] テストの再編成

### 長期目標（Phase 3）
- [ ] プラグインアーキテクチャの検討
- [ ] 設定ファイルの導入
- [ ] パフォーマンス最適化

## Phase 1: モジュール分割と外部リソース化

### 1.1 ディレクトリ構造の再設計

```
dogmv/
├── src/
│   ├── main.rs                  # アプリケーションエントリーポイント (~50行)
│   ├── app.rs                   # Application構築とライフサイクル
│   ├── ui/
│   │   ├── mod.rs               # UI module root
│   │   ├── window.rs            # ApplicationWindow, HeaderBar
│   │   ├── tree_view.rs         # ディレクトリツリー関連
│   │   ├── preview.rs           # WebViewプレビュー
│   │   ├── sidebar.rs           # サイドバートグル
│   │   └── dialogs.rs           # FileChooserDialog等
│   ├── markdown/
│   │   ├── mod.rs               # Markdown module root
│   │   ├── parser.rs            # comrak wrapper
│   │   ├── renderer.rs          # HTML生成
│   │   └── highlighter.rs       # syntect wrapper
│   ├── models/
│   │   ├── mod.rs               # Models module root
│   │   ├── app_state.rs         # AppState定義
│   │   └── file_item.rs         # FileItem GObject
│   ├── file_system/
│   │   ├── mod.rs               # File system module root
│   │   ├── watcher.rs           # notify wrapper
│   │   ├── loader.rs            # ディレクトリローディング
│   │   └── cli.rs               # CLI引数パース
│   ├── styles/
│   │   └── mod.rs               # CSS管理
│   └── error.rs                 # カスタムエラー型
├── resources/
│   ├── styles/
│   │   ├── markdown.css         # Markdown用CSS
│   │   ├── toggle_button.css    # トグルボタン用CSS
│   │   └── error.css            # エラー表示用CSS
│   └── templates/
│       ├── markdown.html        # Markdownラッパーテンプレート
│       ├── welcome.html         # ウェルカムメッセージ
│       └── error.html           # エラーページテンプレート
├── tests/
│   ├── integration/
│   │   └── basic_workflow.rs    # 統合テスト
│   └── unit/
│       ├── markdown_tests.rs    # Markdownレンダリングテスト
│       └── file_system_tests.rs # ファイルシステムテスト
└── Cargo.toml
```

### 1.2 外部リソースファイルの分離

#### resources/styles/markdown.css
```css
/* GitHubスタイルのMarkdown CSS */
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    line-height: 1.6;
    padding: 20px;
    max-width: 900px;
    margin: 0 auto;
    color: #24292e;
}
/* ... 以下略 ... */
```

#### resources/styles/toggle_button.css
```css
.flat-toggle {
    border: none;
    background: none;
    box-shadow: none;
    padding: 4px;
}
.flat-toggle:hover {
    border: none;
    background: rgba(255, 255, 255, 0.1);
    box-shadow: none;
}
/* ... */
```

#### resources/templates/markdown.html
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <base href="{{base_path}}">
    <style>{{css_content}}</style>
</head>
<body>
{{body_content}}
</body>
</html>
```

### 1.3 リソース埋め込み方法

**Cargo.toml**に`include_dir`または`rust-embed`を追加：

```toml
[dependencies]
rust-embed = "8.0"
```

**src/styles/mod.rs**:
```rust
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/styles/"]
pub struct Styles;

pub fn get_markdown_css() -> String {
    let css = Styles::get("markdown.css")
        .expect("markdown.css not found");
    String::from_utf8_lossy(css.data.as_ref()).to_string()
}
```

### 1.4 モジュール分割の優先順位

#### 高優先度（Phase 1-A）
1. **src/models/file_item.rs**: FileItem GObjectの分離
2. **src/markdown/renderer.rs**: Markdownレンダリングの分離
3. **resources/styles/*.css**: CSS外部化

#### 中優先度（Phase 1-B）
4. **src/ui/tree_view.rs**: ディレクトリツリー関連
5. **src/file_system/cli.rs**: CLI引数パース
6. **src/error.rs**: カスタムエラー型

#### 低優先度（Phase 1-C）
7. **src/ui/sidebar.rs**: サイドバートグル
8. **src/file_system/watcher.rs**: ファイル監視
9. **テストの再編成**

## Phase 2: カスタムエラー型と状態管理

### 2.1 カスタムエラー型の導入

**src/error.rs**:
```rust
use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum DogmvError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Invalid UTF-8 in file: {0}")]
    InvalidUtf8(PathBuf),

    #[error("Not a markdown file: {0}")]
    NotMarkdownFile(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Rendering error: {0}")]
    RenderingError(String),
}

pub type Result<T> = std::result::Result<T, DogmvError>;
```

**Cargo.toml**:
```toml
[dependencies]
thiserror = "1.0"
```

### 2.2 状態管理の改善

現在の`AppState`を分割：

**src/models/app_state.rs**:
```rust
#[derive(Clone)]
pub struct AppState {
    pub ui: UiState,
    pub document: DocumentState,
}

#[derive(Clone)]
pub struct UiState {
    pub webview: WebView,
    pub tree_scroll: ScrolledWindow,
    pub toggle_button: Button,
    pub paned: Paned,
}

#[derive(Clone)]
pub struct DocumentState {
    pub current_file: Arc<Mutex<Option<PathBuf>>>,
    pub root_dir: Arc<Mutex<Option<PathBuf>>>,
}
```

## Phase 3: 拡張性向上

### 3.1 設定ファイルの導入

**~/.config/dogmv/config.toml**:
```toml
[ui]
window_width = 1024
window_height = 768
sidebar_width = 250

[theme]
markdown_style = "github"  # github, gitlab, custom
syntax_theme = "InspiredGitHub"

[behavior]
auto_reload = true
show_hidden_files = false
```

### 3.2 テーマシステム

```
resources/
└── themes/
    ├── github.css
    ├── gitlab.css
    └── custom.css
```

## 実装計画

### Week 1: Phase 1-A
- [ ] Day 1-2: FileItem分離 (`src/models/file_item.rs`)
- [ ] Day 3-4: Markdownレンダリング分離 (`src/markdown/`)
- [ ] Day 5: CSS外部化 (`resources/styles/`)

### Week 2: Phase 1-B
- [ ] Day 1-2: ディレクトリツリー分離 (`src/ui/tree_view.rs`)
- [ ] Day 3: CLI引数パース分離 (`src/file_system/cli.rs`)
- [ ] Day 4-5: カスタムエラー型導入 (`src/error.rs`)

### Week 3: Phase 1-C + Phase 2
- [ ] Day 1-2: サイドバー・ファイル監視分離
- [ ] Day 3-4: 状態管理改善
- [ ] Day 5: テスト再編成

### Week 4: Phase 3（Optional）
- [ ] Day 1-2: 設定ファイルシステム
- [ ] Day 3-4: テーマシステム
- [ ] Day 5: ドキュメント更新

## リファクタリングの原則

### 安全性
1. **各ステップでビルド・テストを確認**
   ```bash
   cargo build && cargo test && cargo run README.md
   ```

2. **Git commitを細かく**
   - 1つの機能分離 = 1 commit
   - コミットメッセージに目的を明記

3. **後方互換性の維持**
   - 既存の動作を変更しない
   - UIの変更なし（リファクタリングのみ）

### コーディング規約

#### モジュール公開の原則
```rust
// モジュールは必要最小限のみpub
pub(crate) fn internal_function() { }  // クレート内のみ
pub fn public_api() { }                 // 外部公開
```

#### ドキュメント
```rust
/// Loads a Markdown file from the given path.
///
/// # Arguments
/// * `path` - Path to the Markdown file
///
/// # Returns
/// * `Ok(String)` - File contents
/// * `Err(DogmvError)` - If file cannot be read
///
/// # Example
/// ```
/// let content = load_markdown(Path::new("README.md"))?;
/// ```
pub fn load_markdown(path: &Path) -> Result<String> {
    // ...
}
```

#### テスト
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_markdown_success() {
        // ...
    }

    #[test]
    fn test_load_markdown_not_found() {
        // ...
    }
}
```

## リスク管理

### 想定されるリスク

| リスク | 影響度 | 対策 |
|--------|--------|------|
| GTK4バインディングの変更が必要 | 中 | 小さな変更から開始、段階的移行 |
| GObject subclassingの移動が困難 | 高 | 専用モジュールに慎重に分離 |
| リソース埋め込みのビルドエラー | 中 | rust-embedの事前検証 |
| テストの破綻 | 低 | 各ステップでテスト実行 |
| パフォーマンス劣化 | 低 | リリースビルドで計測 |

### ロールバック計画

各Phase完了時にGit tagを作成：
```bash
git tag phase-1a-complete
git tag phase-1b-complete
# ...
```

問題が発生した場合：
```bash
git checkout phase-1a-complete
```

## 成功指標

### Phase 1完了時
- [ ] src/main.rsが300行以下
- [ ] モジュールが少なくとも5つ以上
- [ ] CSSがすべて外部ファイル化
- [ ] すべてのテストがパス
- [ ] 既存の機能が動作

### Phase 2完了時
- [ ] カスタムエラー型が全面採用
- [ ] `unwrap()`の使用が激減
- [ ] 状態管理が明確化
- [ ] テストカバレッジ70%以上

### Phase 3完了時
- [ ] 設定ファイルシステムが動作
- [ ] 複数テーマの切り替えが可能
- [ ] ドキュメントが最新

## 付録: マイグレーション例

### Before (src/main.rs)
```rust
fn render_markdown(markdown: &str) -> String {
    use comrak::{markdown_to_html_with_plugins, Options, Plugins};
    use comrak::plugins::syntect::SyntectAdapter;

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;

    let adapter = SyntectAdapter::new(Some("InspiredGitHub"));
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(markdown, &options, &plugins)
}
```

### After (src/markdown/renderer.rs)
```rust
use super::highlighter::SyntaxHighlighter;
use crate::error::Result;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};

pub struct MarkdownRenderer {
    options: Options,
    highlighter: SyntaxHighlighter,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        let mut options = Options::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;

        Self {
            options,
            highlighter: SyntaxHighlighter::new("InspiredGitHub"),
        }
    }

    pub fn render(&self, markdown: &str) -> Result<String> {
        let mut plugins = Plugins::default();
        plugins.render.codefence_syntax_highlighter = Some(self.highlighter.adapter());

        Ok(markdown_to_html_with_plugins(markdown, &self.options, &plugins))
    }
}
```

## まとめ

このリファクタリング計画により、dogmvは以下のような改善が期待できます：

1. **保守性の向上**: モジュール分割により、各機能の責務が明確化
2. **拡張性の向上**: 新機能追加が容易になる
3. **テスト容易性**: ユニットテストが書きやすくなる
4. **開発効率**: 並行開発が可能になる
5. **可読性**: コードの見通しが良くなる

段階的な実装により、リスクを最小化しながら確実に品質を向上させます。
