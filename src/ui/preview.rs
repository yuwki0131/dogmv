use crate::markdown::{create_html, load_markdown, render_markdown};
use log::{error, info};
use std::path::Path;
use webkit6::prelude::*;
use webkit6::WebView;

/// Displays a Markdown file in the WebView
pub fn display_markdown(webview: &WebView, file_path: &Path) {
    match load_markdown(file_path) {
        Ok(markdown) => {
            let html_body = render_markdown(&markdown);

            // Get base directory for relative paths
            let base_dir = file_path
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("");

            let full_html = create_html(&html_body, base_dir);
            webview.load_html(&full_html, None);
            info!("Markdown displayed successfully");
        }
        Err(e) => {
            error!("Failed to load markdown file '{}': {}", file_path.display(), e);
            let error_html = create_error_html(
                "Failed to Load File",
                &format!("Could not read file: {}\n\nError: {}", file_path.display(), e)
            );
            webview.load_html(&error_html, None);
        }
    }
}

/// Displays the welcome message when no file is selected
pub fn display_welcome_message(webview: &WebView) {
    info!("Displaying welcome message");
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
            text-align: center;
            margin-top: 50px;
            color: #24292e;
        }
        h1 {
            font-size: 2.5em;
            font-weight: 600;
            margin-bottom: 20px;
        }
        .subtitle {
            color: #666;
            font-size: 1.2em;
            margin-bottom: 50px;
        }
        .shortcuts {
            margin-top: 50px;
            text-align: left;
            display: inline-block;
        }
        .shortcuts h3 {
            font-size: 1.5em;
            margin-bottom: 20px;
        }
        .shortcuts ul {
            list-style: none;
            padding-left: 0;
            font-size: 1.1em;
        }
        .shortcuts li {
            margin-bottom: 10px;
        }
        kbd {
            background-color: #f6f8fa;
            border: 1px solid #d1d5da;
            border-radius: 3px;
            padding: 3px 8px;
            font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <h1>dogmv - Markdown Viewer</h1>
    <p class="subtitle">← 左側のツリーからMarkdownファイルを選択してください</p>

    <div class="shortcuts">
        <h3>キーボードショートカット:</h3>
        <ul>
            <li><kbd>Ctrl+O</kbd> : ファイルを開く</li>
            <li><kbd>Ctrl+R</kbd> : リロード</li>
            <li><kbd>Ctrl+Q</kbd> : 終了</li>
        </ul>
    </div>
</body>
</html>"#;
    webview.load_html(html, None);
}

/// Creates an error HTML page with styled error message
pub fn create_error_html(title: &str, message: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
            padding: 40px;
            max-width: 800px;
            margin: 0 auto;
            color: #24292e;
        }}
        h1 {{
            color: #d73a49;
            font-size: 2em;
            margin-bottom: 20px;
        }}
        .error-message {{
            background-color: #fff5f5;
            border: 1px solid #feb2b2;
            border-radius: 6px;
            padding: 16px;
            margin-bottom: 20px;
            white-space: pre-wrap;
            font-family: monospace;
        }}
        .hint {{
            color: #6a737d;
            font-size: 0.9em;
        }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <div class="error-message">{}</div>
    <p class="hint">Try using Ctrl+O to open a different file, or check that the file exists and is readable.</p>
</body>
</html>"#,
        title, message
    )
}
