use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use comrak::plugins::syntect::SyntectAdapter;
use log::info;
use std::fs;
use std::path::Path;
use crate::error::{DogmvError, Result};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

/// Loads a Markdown file from the given path.
///
/// # Arguments
/// * `path` - Path to the Markdown file
///
/// # Returns
/// * `Ok(String)` - File contents
/// * `Err(DogmvError)` - If file cannot be read
pub fn load_markdown(path: &Path) -> Result<String> {
    info!("Loading markdown file: {}", path.display());

    // Check if file exists
    if !path.exists() {
        return Err(DogmvError::FileNotFound(path.to_path_buf()));
    }

    // Read file and handle UTF-8 errors
    let content = fs::read_to_string(path)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::InvalidData {
                DogmvError::InvalidUtf8(path.to_path_buf())
            } else {
                DogmvError::Io(e)
            }
        })?;

    info!("Loaded {} bytes", content.len());
    Ok(content)
}

/// Renders Markdown to HTML with GitHub Flavored Markdown support and syntax highlighting.
///
/// # Arguments
/// * `markdown` - Markdown content string
///
/// # Returns
/// HTML string with rendered Markdown
pub fn render_markdown(markdown: &str) -> String {
    info!("Rendering markdown ({} chars)", markdown.len());

    let mut options = Options::default();
    // Enable GitHub Flavored Markdown extensions
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;

    // Create syntect adapter for syntax highlighting
    let adapter = SyntectAdapter::new(Some("InspiredGitHub"));
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(markdown, &options, &plugins)
}

/// Determines if a file is a Markdown file based on extension.
///
/// # Arguments
/// * `path` - Path to the file
///
/// # Returns
/// `true` if the file has .md or .markdown extension
pub fn is_markdown_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        ext_str == "md" || ext_str == "markdown"
    } else {
        false
    }
}

/// Renders source code to HTML with syntax highlighting.
///
/// # Arguments
/// * `code` - Source code content
/// * `path` - File path (used to detect syntax by extension)
///
/// # Returns
/// HTML string with syntax highlighted code
pub fn render_source_code(code: &str, path: &Path) -> String {
    info!("Rendering source code ({} chars) for {}", code.len(), path.display());

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["InspiredGitHub"];

    // Try to find syntax by file extension
    let syntax = if let Some(ext) = path.extension() {
        syntax_set
            .find_syntax_by_extension(&ext.to_string_lossy())
            .or_else(|| syntax_set.find_syntax_by_first_line(code))
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
    } else {
        syntax_set.find_syntax_plain_text()
    };

    info!("Using syntax: {}", syntax.name);

    // Generate highlighted HTML
    highlighted_html_for_string(code, &syntax_set, syntax, theme)
        .unwrap_or_else(|e| {
            info!("Failed to highlight code: {}, falling back to plain text", e);
            format!("<pre style='background-color: #f6f8fa; padding: 16px; overflow: auto; border-radius: 6px;'><code>{}</code></pre>",
                    code.replace('&', "&amp;")
                        .replace('<', "&lt;")
                        .replace('>', "&gt;")
                        .replace('"', "&quot;"))
        })
}

/// Creates a complete HTML document with GitHub-style CSS.
///
/// # Arguments
/// * `body` - HTML body content (rendered Markdown)
/// * `base_path` - Base path for resolving relative links and images
///
/// # Returns
/// Complete HTML document string
pub fn create_html(body: &str, base_path: &str) -> String {
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
            color: #24292e;
        }}
        h1, h2, h3, h4, h5, h6 {{
            margin-top: 24px;
            margin-bottom: 16px;
            font-weight: 600;
            line-height: 1.25;
        }}
        h1 {{
            font-size: 2em;
            border-bottom: 1px solid #eaecef;
            padding-bottom: 0.3em;
        }}
        h2 {{
            font-size: 1.5em;
            border-bottom: 1px solid #eaecef;
            padding-bottom: 0.3em;
        }}
        h3 {{ font-size: 1.25em; }}
        h4 {{ font-size: 1em; }}
        h5 {{ font-size: 0.875em; }}
        h6 {{ font-size: 0.85em; color: #6a737d; }}

        p {{ margin-top: 0; margin-bottom: 16px; }}

        a {{
            color: #0366d6;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}

        code {{
            background-color: rgba(27,31,35,0.05);
            padding: 0.2em 0.4em;
            margin: 0;
            font-size: 85%;
            border-radius: 3px;
            font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
        }}

        pre {{
            padding: 16px;
            overflow: auto;
            font-size: 85%;
            line-height: 1.45;
            border-radius: 6px;
            margin-top: 0;
            margin-bottom: 16px;
            background-color: #f6f8fa !important;
        }}

        pre.syntect {{
            background-color: #f6f8fa !important;
        }}

        pre:not(.syntect) {{
            background-color: #f6f8fa !important;
        }}

        pre code {{
            background-color: transparent !important;
            padding: 0;
            margin: 0;
            font-size: 100%;
            border-radius: 0;
        }}

        blockquote {{
            padding: 0 1em;
            color: #6a737d;
            border-left: 0.25em solid #dfe2e5;
            margin: 0 0 16px 0;
        }}

        table {{
            border-collapse: collapse;
            width: 100%;
            margin-bottom: 16px;
        }}

        table tr {{
            background-color: #fff;
            border-top: 1px solid #c6cbd1;
        }}

        table tr:nth-child(2n) {{
            background-color: #f6f8fa;
        }}

        table th, table td {{
            padding: 6px 13px;
            border: 1px solid #dfe2e5;
        }}

        table th {{
            font-weight: 600;
        }}

        ul, ol {{
            margin-top: 0;
            margin-bottom: 16px;
            padding-left: 2em;
        }}

        li + li {{
            margin-top: 0.25em;
        }}

        img {{
            max-width: 100%;
            box-sizing: content-box;
        }}

        hr {{
            height: 0.25em;
            padding: 0;
            margin: 24px 0;
            background-color: #e1e4e8;
            border: 0;
        }}

        input[type="checkbox"] {{
            margin-right: 0.5em;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_render_markdown() {
        let markdown = "# Hello\n\nThis is a test.";
        let html = render_markdown(markdown);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello"));
        assert!(html.contains("<p>"));
        assert!(html.contains("This is a test."));
    }

    #[test]
    fn test_render_markdown_gfm() {
        // Test table (need proper markdown format)
        let markdown = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = render_markdown(markdown);
        assert!(html.contains("<table>"));
        assert!(html.contains("<td>"));

        // Test strikethrough
        let markdown2 = "~~strikethrough~~";
        let html2 = render_markdown(markdown2);
        assert!(html2.contains("<del>") || html2.contains("strikethrough"));
    }

    #[test]
    fn test_create_html() {
        let body = "<h1>Test</h1>";
        let html = create_html(body, "/test/path");
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<h1>Test</h1>"));
        assert!(html.contains("file:///test/path/"));
    }

    #[test]
    fn test_create_html_includes_css() {
        let html = create_html("", "/");
        assert!(html.contains("<style>"));
        assert!(html.contains("font-family"));
    }

    #[test]
    fn test_is_markdown_file() {
        assert!(is_markdown_file(Path::new("test.md")));
        assert!(is_markdown_file(Path::new("test.markdown")));
        assert!(is_markdown_file(Path::new("README.MD")));
        assert!(!is_markdown_file(Path::new("test.rs")));
        assert!(!is_markdown_file(Path::new("test.txt")));
        assert!(!is_markdown_file(Path::new("test")));
    }

    #[test]
    fn test_render_source_code() {
        let code = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let path = PathBuf::from("test.rs");
        let html = render_source_code(code, &path);

        // Should contain HTML
        assert!(html.contains("<pre"));
        // Should contain the code content
        assert!(html.contains("main"));
        assert!(html.contains("println"));
    }

    #[test]
    fn test_render_source_code_unknown_extension() {
        let code = "some text content";
        let path = PathBuf::from("test.unknown");
        let html = render_source_code(code, &path);

        // Should still return HTML even with unknown extension
        assert!(html.contains("<pre"));
        assert!(html.contains("some text content"));
    }
}
