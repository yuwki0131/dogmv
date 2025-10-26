mod error;
mod file_system;
mod markdown;
mod models;
mod ui;

use ctor::ctor;
use file_system::parse_arguments;
use ui::{
    create_tree_view, display_markdown, display_welcome_message, setup_file_selection_handler,
    setup_toggle_button, setup_toggle_button_css,
};
use gtk4::prelude::*;
use gtk4::{gdk, gio, glib, Application, ApplicationWindow, EventControllerKey, FileChooserNative, FileChooserAction, FileFilter, ResponseType, HeaderBar, Paned, Orientation, Box as GtkBox, Button, Label};
use log::{error, info, warn};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use webkit6::WebView;

const APP_ID: &str = "com.github.dogmv";

// Initialize environment variables before main() is called
// This is necessary to prevent GSettings schema errors in GTK4's FileChooser
#[ctor]
fn init_environment() {
    // Disable GSettings to avoid schema errors in FileChooser
    // This prevents GTK from trying to access FileChooser settings via GSettings
    // Must be set before GTK initialization, so we use #[ctor] to run before main()
    std::env::set_var("GSETTINGS_BACKEND", "memory");
}

// アプリケーション状態を保持する構造体
#[derive(Clone)]
struct AppState {
    current_file: Arc<Mutex<Option<PathBuf>>>,
    root_dir: Arc<Mutex<Option<PathBuf>>>,
    webview: WebView,
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
    };

    // Setup toggle button click handler
    setup_toggle_button(&toggle_button, &tree_scroll, &paned);

    // Setup file selection handler
    setup_file_selection_handler(&selection_model, &webview, app_state.current_file.clone());

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
        let (event_tx, event_rx) = mpsc::channel::<std::result::Result<Event, notify::Error>>();

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

    let dialog = FileChooserNative::new(
        Some("Open File"),
        Some(window),
        FileChooserAction::Open,
        Some("_Open"),
        Some("_Cancel"),
    );

    // Set file filter for all files (allow both markdown and source code)
    let filter = FileFilter::new();
    filter.add_pattern("*.md");
    filter.add_pattern("*.markdown");
    filter.add_pattern("*.rs");
    filter.add_pattern("*.py");
    filter.add_pattern("*.js");
    filter.add_pattern("*.ts");
    filter.add_pattern("*.c");
    filter.add_pattern("*.cpp");
    filter.add_pattern("*.h");
    filter.add_pattern("*.java");
    filter.add_pattern("*.go");
    filter.add_pattern("*.sh");
    filter.add_pattern("*.txt");
    filter.set_name(Some("All supported files"));
    dialog.add_filter(&filter);

    // Add "All files" filter
    let all_filter = FileFilter::new();
    all_filter.add_pattern("*");
    all_filter.set_name(Some("All files"));
    dialog.add_filter(&all_filter);

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
    });

    dialog.show();
}

// All tests have been moved to respective modules:
// - Markdown tests: src/markdown/renderer.rs
// - CLI tests: src/file_system/cli.rs
