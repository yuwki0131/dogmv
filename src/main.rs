use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use log::{error, info};
use std::env;
use std::path::Path;
use webkit6::prelude::*;
use webkit6::WebView;

const APP_ID: &str = "com.github.dogmv";

fn main() {
    // Initialize logger
    env_logger::init();
    info!("Starting dogmv - Markdown Viewer");

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error!("Usage: dogmv <file.md>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    info!("File path: {}", file_path);

    // Check file exists
    if !Path::new(file_path).exists() {
        error!("File not found: {}", file_path);
        std::process::exit(1);
    }

    // Create GTK Application
    // Use FLAGS_NONE to handle command-line arguments ourselves
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gtk4::gio::ApplicationFlags::FLAGS_NONE)
        .build();

    let file_path_clone = file_path.clone();
    app.connect_activate(move |app| {
        build_ui(app, &file_path_clone);
    });

    // Run the application without passing command-line arguments to GTK
    // Pass empty args to prevent GTK from trying to handle our file argument
    app.run_with_args(&Vec::<String>::new());
}

fn build_ui(app: &Application, file_path: &str) {
    info!("Building UI");

    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("dogmv - Markdown Viewer")
        .default_width(1024)
        .default_height(768)
        .build();

    // Create WebView
    info!("Creating WebView");
    let webview = WebView::new();

    // Load and display markdown
    display_markdown(&webview, file_path);

    // Add WebView to window
    window.set_child(Some(&webview));

    info!("Presenting window");
    window.present();
}

fn load_markdown(path: &str) -> Result<String, std::io::Error> {
    info!("Loading markdown file: {}", path);
    let content = std::fs::read_to_string(path)?;
    info!("Loaded {} bytes", content.len());
    Ok(content)
}

fn render_markdown(markdown: &str) -> String {
    info!("Rendering markdown ({} chars)", markdown.len());

    use comrak::{markdown_to_html, Options};

    let mut options = Options::default();
    // Enable GitHub Flavored Markdown extensions
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;

    markdown_to_html(markdown, &options)
}

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
            background-color: #f6f8fa;
            padding: 16px;
            overflow: auto;
            font-size: 85%;
            line-height: 1.45;
            border-radius: 6px;
            margin-top: 0;
            margin-bottom: 16px;
        }}

        pre code {{
            background-color: transparent;
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

        /* Task list */
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

fn display_markdown(webview: &WebView, file_path: &str) {
    match load_markdown(file_path) {
        Ok(markdown) => {
            let html_body = render_markdown(&markdown);

            // Get base directory for relative paths
            let base_dir = Path::new(file_path)
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("");

            let full_html = create_html(&html_body, base_dir);
            webview.load_html(&full_html, None);
            info!("Markdown displayed successfully");
        }
        Err(e) => {
            error!("Failed to load markdown file: {}", e);
            let error_html = format!(
                r#"<html><body><h1>Error</h1><p>Failed to load file: {}</p></body></html>"#,
                e
            );
            webview.load_html(&error_html, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_markdown() {
        let md = "# Hello\n\nThis is a **test**.";
        let html = render_markdown(md);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello"));
        assert!(html.contains("<strong>"));
        assert!(html.contains("test"));
    }

    #[test]
    fn test_render_markdown_gfm() {
        // Test table
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = render_markdown(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<td>"));

        // Test strikethrough
        let md2 = "~~strikethrough~~";
        let html2 = render_markdown(md2);
        assert!(html2.contains("<del>") || html2.contains("strikethrough"));
    }

    #[test]
    fn test_create_html() {
        let body = "<p>Test</p>";
        let base = "/tmp";
        let html = create_html(body, base);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<base href="));
        assert!(html.contains(base));
        assert!(html.contains(body));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn test_create_html_includes_css() {
        let body = "<h1>Title</h1>";
        let html = create_html(body, "/tmp");

        // Check for GitHub-style CSS
        assert!(html.contains("font-family"));
        assert!(html.contains("line-height"));
        assert!(html.contains("border-bottom"));
    }
}
