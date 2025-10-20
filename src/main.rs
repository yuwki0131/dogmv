mod file_system;
mod markdown;
mod models;

use file_system::parse_arguments;
use gtk4::prelude::*;
use gtk4::{gdk, gio, glib, Application, ApplicationWindow, EventControllerKey, FileChooserDialog, FileChooserAction, FileFilter, ResponseType, HeaderBar, Paned, Orientation, ScrolledWindow, Box as GtkBox, Button, Label, ListView, SignalListItemFactory, SingleSelection, TreeListModel, TreeListRow};
use log::{error, info, warn};
use markdown::{create_html, load_markdown, render_markdown};
use models::FileItem;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::fs;
use webkit6::prelude::*;
use webkit6::WebView;

const APP_ID: &str = "com.github.dogmv";

// アプリケーション状態を保持する構造体
#[derive(Clone)]
struct AppState {
    current_file: Arc<Mutex<Option<PathBuf>>>,
    root_dir: Arc<Mutex<Option<PathBuf>>>,
    webview: WebView,
    tree_scroll: ScrolledWindow,
    toggle_button: Button,
    paned: Paned,
}

fn main() {
    // Initialize logger
    env_logger::init();
    info!("Starting dogmv - Markdown Viewer v{}", env!("CARGO_PKG_VERSION"));

    // Create GTK Application
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();

    app.connect_activate(move |app| {
        build_ui(app);
    });

    app.run_with_args(&Vec::<String>::new());
}

fn build_ui(app: &Application) {
    info!("Building UI");

    // Setup CSS for toggle button (remove border on hover)
    setup_toggle_button_css();

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    let (initial_file, root_dir) = parse_arguments(&args);

    // Create HeaderBar (CSD)
    let header_bar = HeaderBar::new();
    header_bar.set_show_title_buttons(true);
    header_bar.set_title_widget(Some(&Label::new(Some("dogmv - Markdown Viewer"))));

    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1024)
        .default_height(768)
        .build();

    window.set_titlebar(Some(&header_bar));

    // Create WebView for preview
    info!("Creating WebView");
    let webview = WebView::new();

    // Create sidebar toggle button (initially showing close icon since sidebar is visible)
    let toggle_button = Button::from_icon_name("pan-start-symbolic");
    toggle_button.set_tooltip_text(Some("サイドバー閉じる"));
    toggle_button.add_css_class("flat"); // Remove button border
    toggle_button.add_css_class("flat-toggle"); // Remove border on hover/active

    // Create tree view (initially visible)
    let (tree_scroll, selection_model) = create_tree_view(&root_dir);

    // Create toggle button box (right-aligned)
    let toggle_box = GtkBox::new(Orientation::Horizontal, 0);
    toggle_box.set_halign(gtk4::Align::End);
    toggle_box.append(&toggle_button);

    // Create sidebar box
    let sidebar_box = GtkBox::new(Orientation::Vertical, 0);
    sidebar_box.append(&toggle_box);
    sidebar_box.append(&tree_scroll);

    // Create Paned layout
    let paned = Paned::new(Orientation::Horizontal);
    paned.set_start_child(Some(&sidebar_box));
    paned.set_end_child(Some(&webview));
    paned.set_position(250); // Initial width: 250px

    // Setup app state
    let app_state = AppState {
        current_file: Arc::new(Mutex::new(initial_file.clone())),
        root_dir: Arc::new(Mutex::new(Some(root_dir.clone()))),
        webview: webview.clone(),
        tree_scroll: tree_scroll.clone(),
        toggle_button: toggle_button.clone(),
        paned: paned.clone(),
    };

    // Setup toggle button click handler
    setup_toggle_button(&app_state);

    // Setup file selection handler
    setup_file_selection_handler(&selection_model, &app_state);

    // Display initial content
    if let Some(ref file_path) = initial_file {
        display_markdown(&webview, file_path);
        setup_file_watcher(&webview, file_path);
    } else {
        display_welcome_message(&webview);
    }

    // Setup keyboard shortcuts
    setup_keyboard_shortcuts(&window, &app_state);

    // Add layout to window
    window.set_child(Some(&paned));

    info!("Presenting window");
    window.present();
}

/// Create tree view for file browser
/// Returns: (ScrolledWindow, SingleSelection)
fn create_tree_view(root_dir: &Path) -> (ScrolledWindow, SingleSelection) {
    let scroll = ScrolledWindow::new();
    scroll.set_vexpand(true);
    scroll.set_hexpand(true);

    // Load root directory items
    let root_items = load_directory_items(root_dir);

    // Create root model
    let root_model = gio::ListStore::new::<FileItem>();
    for item in root_items {
        root_model.append(&item);
    }

    // Create TreeListModel with expand function
    let tree_model = TreeListModel::new(
        root_model.clone(),
        false, // passthrough
        false,  // don't autoexpand
        |item| {
            let file_item = item.downcast_ref::<FileItem>()?;
            if file_item.is_dir() && !file_item.is_symlink() {
                let path = file_item.path_buf();
                let children = load_directory_items(&path);
                if children.is_empty() {
                    return None;
                }
                let child_model = gio::ListStore::new::<FileItem>();
                for child in children {
                    child_model.append(&child);
                }
                Some(child_model.upcast())
            } else {
                None
            }
        },
    );

    // Create selection model
    let selection_model = SingleSelection::new(Some(tree_model.clone()));

    // Create ListView
    let list_view = ListView::new(Some(selection_model.clone()), None::<SignalListItemFactory>);

    // Create factory for rendering list items
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let row = GtkBox::new(Orientation::Horizontal, 6);
        row.set_margin_start(6);
        row.set_margin_end(6);
        row.set_margin_top(3);
        row.set_margin_bottom(3);

        let icon = Label::new(None);
        let label = Label::new(None);
        label.set_xalign(0.0);
        label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

        row.append(&icon);
        row.append(&label);

        list_item.set_child(Some(&row));
    });

    factory.connect_bind(move |_, list_item| {
        let tree_list_row = list_item
            .item()
            .and_downcast::<TreeListRow>()
            .expect("Item must be TreeListRow");

        let file_item = tree_list_row
            .item()
            .and_downcast::<FileItem>()
            .expect("TreeListRow item must be FileItem");

        let row_widget = list_item.child().and_downcast::<GtkBox>().unwrap();
        let icon_label = row_widget.first_child().unwrap().downcast::<Label>().unwrap();
        let name_label = icon_label.next_sibling().unwrap().downcast::<Label>().unwrap();

        // Set indentation based on depth
        let depth = tree_list_row.depth();
        row_widget.set_margin_start((depth * 16 + 6) as i32);

        // Set icon based on file type
        let icon_text = if file_item.is_symlink() {
            "🔗" // Symlink
        } else if file_item.is_dir() {
            if tree_list_row.is_expanded() {
                "📂" // Open folder
            } else {
                "📁" // Closed folder
            }
        } else {
            "📄" // File
        };
        icon_label.set_text(icon_text);

        // Set file name with tooltip
        let name = file_item.name();
        name_label.set_text(&name);
        name_label.set_tooltip_text(Some(&name));

        // Add (symlink) suffix for symlinks
        if file_item.is_symlink() {
            name_label.set_text(&format!("{} (symlink)", name));
        }
    });

    list_view.set_factory(Some(&factory));

    scroll.set_child(Some(&list_view));
    (scroll, selection_model)
}

/// Load directory items and sort them (directories first, then alphabetically)
fn load_directory_items(dir_path: &Path) -> Vec<FileItem> {
    let mut items = Vec::new();

    let read_dir = match fs::read_dir(dir_path) {
        Ok(rd) => rd,
        Err(e) => {
            warn!("Failed to read directory {}: {}", dir_path.display(), e);
            return items;
        }
    };

    for entry in read_dir.flatten() {
        let path = entry.path();

        // Skip hidden files (starting with .)
        if let Some(name) = path.file_name() {
            if name.to_string_lossy().starts_with('.') {
                continue;
            }
        }

        if let Some(item) = FileItem::new(&path) {
            items.push(item);
        }
    }

    // Sort: directories first, then alphabetically by name
    items.sort_by(|a, b| {
        match (a.is_dir(), b.is_dir()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name().to_lowercase().cmp(&b.name().to_lowercase()),
        }
    });

    items
}

/// Setup file selection handler for tree view
fn setup_file_selection_handler(
    selection_model: &SingleSelection,
    state: &AppState,
) {
    let webview = state.webview.clone();
    let current_file = state.current_file.clone();

    selection_model.connect_selected_item_notify(move |model| {
        if let Some(selected_item) = model.selected_item() {
            if let Some(tree_list_row) = selected_item.downcast_ref::<TreeListRow>() {
                if let Some(file_item) = tree_list_row.item().and_downcast::<FileItem>() {
                    let path = file_item.path_buf();

                    // Only open files, not directories or symlinks
                    if !file_item.is_dir() && !file_item.is_symlink() {
                        info!("File selected: {}", path.display());

                        // Update current file in app state
                        if let Ok(mut current) = current_file.lock() {
                            *current = Some(path.clone());
                        }

                        // Display markdown
                        display_markdown(&webview, &path);
                    } else if file_item.is_dir() && !file_item.is_symlink() {
                        // Toggle directory expansion
                        tree_list_row.set_expanded(!tree_list_row.is_expanded());
                    }
                }
            }
        }
    });
}

/// Setup CSS for toggle button to remove borders completely
fn setup_toggle_button_css() {
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        r#"
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
        .flat-toggle:active {
            border: none;
            background: rgba(255, 255, 255, 0.2);
            box-shadow: none;
        }
        "#
    );

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display"),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Setup toggle button for sidebar visibility
fn setup_toggle_button(state: &AppState) {
    let tree_scroll = state.tree_scroll.clone();
    let toggle_button = state.toggle_button.clone();
    let paned = state.paned.clone();

    // Store the original width when sidebar is open
    let original_width = Arc::new(Mutex::new(250)); // Default initial width

    toggle_button.connect_clicked(move |btn| {
        let is_visible = tree_scroll.is_visible();

        if is_visible {
            // Closing sidebar - store current width and minimize
            let current_pos = paned.position();
            if let Ok(mut width) = original_width.lock() {
                *width = current_pos;
            }

            // Set to minimum width (just enough for toggle button, approximately 40-50px)
            paned.set_position(40);
            tree_scroll.set_visible(false);

            btn.set_icon_name("pan-end-symbolic");
            btn.set_tooltip_text(Some("サイドバー展開"));
        } else {
            // Opening sidebar - restore original width
            if let Ok(width) = original_width.lock() {
                paned.set_position(*width);
            }
            tree_scroll.set_visible(true);

            btn.set_icon_name("pan-start-symbolic");
            btn.set_tooltip_text(Some("サイドバー閉じる"));
        }
    });
}

fn display_markdown(webview: &WebView, file_path: &Path) {
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

fn display_welcome_message(webview: &WebView) {
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

fn setup_file_watcher(webview: &WebView, file_path: &Path) {
    info!("Setting up file watcher for: {}", file_path.display());

    let file_path = file_path.to_path_buf();
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
        if let Err(e) = watcher.watch(&file_path_for_thread, RecursiveMode::NonRecursive) {
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
                info!("Reloading file: {}", file_path.display());
                display_markdown(&webview_clone, &file_path);
                *changed = false;
            }
        }
        glib::ControlFlow::Continue
    });
}

fn setup_keyboard_shortcuts(window: &ApplicationWindow, state: &AppState) {
    info!("Setting up keyboard shortcuts");

    let controller = EventControllerKey::new();

    let app_weak = window.application().and_then(|app| Some(app.downgrade()));
    let window_weak = window.downgrade();
    let state_clone = state.clone();

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
                    if let Ok(current_file) = state_clone.current_file.lock() {
                        if let Some(ref file_path) = *current_file {
                            info!("Reloading file: {}", file_path.display());
                            display_markdown(&state_clone.webview, file_path);
                        }
                    }
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
                        open_file_dialog(&window, &state_clone);
                    }
                    return glib::Propagation::Stop;
                }
                _ => {}
            }
        }

        glib::Propagation::Proceed
    });

    // Attach controller to WebView
    state.webview.add_controller(controller);
    info!("Keyboard controller attached to WebView");
}

fn open_file_dialog(window: &ApplicationWindow, state: &AppState) {
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

    let state_clone = state.clone();

    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            if let Some(file) = dialog.file() {
                if let Some(path) = file.path() {
                    info!("Selected file: {}", path.display());

                    // Update current file
                    if let Ok(mut current_file) = state_clone.current_file.lock() {
                        *current_file = Some(path.clone());
                    }

                    // Update root directory to parent of selected file
                    if let Some(parent) = path.parent() {
                        if let Ok(mut root_dir) = state_clone.root_dir.lock() {
                            *root_dir = Some(parent.to_path_buf());
                        }
                        // TODO: Update tree view to show new root directory
                    }

                    display_markdown(&state_clone.webview, &path);
                }
            }
        }
        dialog.close();
    });

    dialog.show();
}

// All tests have been moved to respective modules:
// - Markdown tests: src/markdown/renderer.rs
// - CLI tests: src/file_system/cli.rs
