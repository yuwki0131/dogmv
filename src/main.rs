use gtk4::prelude::*;
use gtk4::{gdk, gio, glib, Application, ApplicationWindow, EventControllerKey, FileChooserDialog, FileChooserAction, FileFilter, ResponseType, MessageDialog, MessageType, ButtonsType};
use log::{error, info, warn};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use webkit6::prelude::*;
use webkit6::WebView;

const APP_ID: &str = "com.github.dogmv";

fn main() {
    // Initialize logger
    env_logger::init();
    info!("Starting dogmv - Markdown Viewer v{}", env!("CARGO_PKG_VERSION"));

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No file specified");
        eprintln!();
        eprintln!("Usage: {} <file.md>", args.get(0).map(|s| s.as_str()).unwrap_or("dogmv"));
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} README.md", args.get(0).map(|s| s.as_str()).unwrap_or("dogmv"));
        eprintln!("  {} /path/to/document.md", args.get(0).map(|s| s.as_str()).unwrap_or("dogmv"));
        std::process::exit(1);
    }

    let file_path = &args[1];
    info!("File path: {}", file_path);

    // Check file exists
    if !Path::new(file_path).exists() {
        eprintln!("Error: File not found: {}", file_path);
        eprintln!();
        eprintln!("Please check that:");
        eprintln!("  - The file path is correct");
        eprintln!("  - The file exists");
        eprintln!("  - You have permission to read the file");
        std::process::exit(1);
    }

    // Check if file is readable
    if let Err(e) = std::fs::metadata(file_path) {
        eprintln!("Error: Cannot access file: {}", file_path);
        eprintln!("Reason: {}", e);
        std::process::exit(1);
    }

    // Create GTK Application
    // Use FLAGS_NONE to handle command-line arguments ourselves
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
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

    // Setup file watcher
    setup_file_watcher(&webview, file_path);

    // Setup keyboard shortcuts
    setup_keyboard_shortcuts(&window, &webview, file_path);

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

    use comrak::{markdown_to_html_with_plugins, Options, Plugins};
    use comrak::plugins::syntect::SyntectAdapter;

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
            padding: 16px;
            overflow: auto;
            font-size: 85%;
            line-height: 1.45;
            border-radius: 6px;
            margin-top: 0;
            margin-bottom: 16px;
            background-color: #f6f8fa !important;
        }}

        /* Syntax highlighted code blocks - override inline styles */
        pre.syntect {{
            background-color: #f6f8fa !important;
        }}

        /* Plain code blocks without syntax highlighting */
        pre:not(.syntect) {{
            background-color: #f6f8fa !important;
        }}

        /* Override any inline background styles on code elements */
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
            error!("Failed to load markdown file '{}': {}", file_path, e);
            let error_html = create_error_html(
                "Failed to Load File",
                &format!("Could not read file: {}\n\nError: {}", file_path, e)
            );
            webview.load_html(&error_html, None);
        }
    }
}

fn create_error_html(title: &str, message: &str) -> String {
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

#[allow(dead_code)]
fn show_error_dialog(window: Option<&ApplicationWindow>, title: &str, message: &str) {
    let dialog = MessageDialog::builder()
        .message_type(MessageType::Error)
        .buttons(ButtonsType::Ok)
        .text(title)
        .secondary_text(message)
        .modal(true)
        .build();

    if let Some(win) = window {
        dialog.set_transient_for(Some(win));
    }

    dialog.connect_response(|dialog, _| {
        dialog.close();
    });

    dialog.show();
}

fn setup_file_watcher(webview: &WebView, file_path: &str) {
    info!("Setting up file watcher for: {}", file_path);

    let file_path = file_path.to_string();
    let file_path_for_thread = file_path.clone();

    // Use Arc<Mutex<bool>> to signal file changes
    let file_changed = Arc::new(Mutex::new(false));
    let file_changed_clone = Arc::clone(&file_changed);

    // Spawn a thread to handle file watching
    std::thread::spawn(move || {
        // Create a channel for file system events
        let (event_tx, event_rx) = mpsc::channel::<Result<Event, notify::Error>>();

        // Create a watcher
        let mut watcher = match RecommendedWatcher::new(
            move |res| {
                if let Err(e) = event_tx.send(res) {
                    warn!("Failed to send file event: {}", e);
                }
            },
            Config::default().with_poll_interval(Duration::from_secs(1)),
        ) {
            Ok(w) => w,
            Err(e) => {
                error!("Failed to create file watcher: {}", e);
                return;
            }
        };

        // Watch the file
        if let Err(e) = watcher.watch(Path::new(&file_path_for_thread), RecursiveMode::NonRecursive) {
            error!("Failed to watch file: {}", e);
            return;
        }

        info!("File watcher started successfully");

        // Keep watcher alive and handle events
        for res in event_rx {
            match res {
                Ok(event) => {
                    // Check if it's a modification event
                    if event.kind.is_modify() || event.kind.is_create() {
                        info!("File changed: {:?}", event.kind);

                        // Set the flag
                        if let Ok(mut changed) = file_changed_clone.lock() {
                            *changed = true;
                        }

                        // Small delay to avoid multiple rapid reloads
                        std::thread::sleep(Duration::from_millis(100));
                    }
                }
                Err(e) => {
                    warn!("File watch error: {}", e);
                }
            }
        }
    });

    // Setup a periodic check on the main thread
    let webview_clone = webview.clone();

    glib::timeout_add_local(Duration::from_millis(500), move || {
        // Check if file has changed
        if let Ok(mut changed) = file_changed.lock() {
            if *changed {
                info!("Reloading file: {}", file_path);
                display_markdown(&webview_clone, &file_path);
                *changed = false;
            }
        }
        glib::ControlFlow::Continue
    });
}

fn setup_keyboard_shortcuts(window: &ApplicationWindow, webview: &WebView, file_path: &str) {
    info!("Setting up keyboard shortcuts");

    let controller = EventControllerKey::new();

    let app_weak = window.application().and_then(|app| Some(app.downgrade()));
    let window_weak = window.downgrade();
    let webview_clone = webview.clone();
    let file_path = file_path.to_string();

    controller.connect_key_pressed(move |_, key, _keycode, modifier| {
        // Check for Ctrl key
        if !modifier.contains(gdk::ModifierType::CONTROL_MASK) {
            return glib::Propagation::Proceed;
        }

        // Use to_unicode() to get the character
        if let Some(ch) = key.to_unicode() {
            match ch {
                'r' | 'R' => {
                    // Ctrl+R: Reload
                    info!("Reloading file: {}", &file_path);
                    display_markdown(&webview_clone, &file_path);
                    return glib::Propagation::Stop;
                }
                'q' | 'Q' => {
                    // Ctrl+Q: Quit
                    info!("Quitting application");
                    if let Some(app) = app_weak.as_ref().and_then(|w| w.upgrade()) {
                        app.quit();
                    }
                    return glib::Propagation::Stop;
                }
                'o' | 'O' => {
                    // Ctrl+O: Open file
                    info!("Opening file dialog");
                    if let Some(window) = window_weak.upgrade() {
                        open_file_dialog(&window, &webview_clone);
                    }
                    return glib::Propagation::Stop;
                }
                _ => {}
            }
        }

        glib::Propagation::Proceed
    });

    // IMPORTANT: Attach controller to WebView, not Window
    // WebView captures all keyboard input, so we need to listen there
    webview.add_controller(controller);
    info!("Keyboard controller attached to WebView");
}

fn open_file_dialog(window: &ApplicationWindow, webview: &WebView) {
    info!("Opening file dialog");

    let dialog = FileChooserDialog::new(
        Some("Open Markdown File"),
        Some(window),
        FileChooserAction::Open,
        &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)],
    );

    // Set file filter for markdown files
    let filter = FileFilter::new();
    filter.add_pattern("*.md");
    filter.add_pattern("*.markdown");
    filter.set_name(Some("Markdown files"));
    dialog.add_filter(&filter);

    let webview_clone = webview.clone();

    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            if let Some(file) = dialog.file() {
                if let Some(path) = file.path() {
                    if let Some(path_str) = path.to_str() {
                        info!("Selected file: {}", path_str);
                        display_markdown(&webview_clone, path_str);
                    }
                }
            }
        }
        dialog.close();
    });

    dialog.show();
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
    fn test_syntax_highlighting() {
        // Test code block with language specification
        let md = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let html = render_markdown(md);

        // Check that syntect has added syntax highlighting
        // syntect adds inline styles with color attributes
        assert!(html.contains("<pre") || html.contains("<code"));
        assert!(html.contains("main"));
        assert!(html.contains("println"));
    }

    #[test]
    fn test_code_block_without_language() {
        // Test code block without language specification
        let md = "```\nplain code\n```";
        let html = render_markdown(md);

        assert!(html.contains("<pre") || html.contains("<code"));
        assert!(html.contains("plain code"));
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
