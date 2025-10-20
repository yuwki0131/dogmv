# dogmv Developer Documentation

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Project Structure](#project-structure)
3. [Development Setup](#development-setup)
4. [Building and Testing](#building-and-testing)
5. [Code Organization](#code-organization)
6. [Key Components](#key-components)
7. [Adding New Features](#adding-new-features)
8. [Debugging](#debugging)
9. [Contributing](#contributing)

## Architecture Overview

dogmv is built using:

- **Rust** (edition 2021) - Safe, fast systems programming language
- **GTK4** - Modern UI toolkit with Wayland support
- **WebKitGTK 6.0** - Web rendering engine for displaying HTML
- **comrak** - CommonMark/GFM Markdown parser
- **syntect** - Syntax highlighting library
- **notify** - File system watcher for auto-reload

### Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                        dogmv CLI                                 │
│  (File/Directory argument parsing, validation)                   │
└──────────────────────┬───────────────────────────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────────────────────────┐
│                    GTK4 Application                              │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │           ApplicationWindow (1024x768)                     │ │
│  │  ┌──────────────────────────────────────────────────────┐ │ │
│  │  │             HeaderBar (CSD)                          │ │ │
│  │  │  - Title: "dogmv - Markdown Viewer"                  │ │ │
│  │  └──────────────────────────────────────────────────────┘ │ │
│  │  ┌──────────────────────────────────────────────────────┐ │ │
│  │  │               Paned (Horizontal Split)               │ │ │
│  │  │  ┌──────────────┬───────────────────────────────┐   │ │ │
│  │  │  │  Sidebar     │      WebView (webkit6)        │   │ │ │
│  │  │  │  (250px)     │      (Preview Pane)           │   │ │ │
│  │  │  │              │                               │   │ │ │
│  │  │  │ ┌──────────┐ │  - Rendered HTML              │   │ │ │
│  │  │  │ │ Toggle ◀ │ │  - Syntax highlighted code    │   │ │ │
│  │  │  │ └──────────┘ │  - GitHub-style CSS           │   │ │ │
│  │  │  │              │  - Images, tables, links      │   │ │ │
│  │  │  │ TreeView:    │                               │   │ │ │
│  │  │  │ ┌─📁 src    │                               │   │ │ │
│  │  │  │ ├─📄 main.rs│                               │   │ │ │
│  │  │  │ └─📄 README │                               │   │ │ │
│  │  │  │              │                               │   │ │ │
│  │  │  └──────────────┴───────────────────────────────┘   │ │ │
│  │  └──────────────────────────────────────────────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
┌──────────────┐ ┌─────────────┐ ┌──────────────┐
│   Markdown   │ │  Directory  │ │ File Watcher │
│  Rendering   │ │  Tree Model │ │   (notify)   │
│              │ │             │ │              │
│  comrak +    │ │ TreeList    │ │  - inotify   │
│  syntect     │ │ Model +     │ │  - 500ms     │
│              │ │ ListView    │ │    polling   │
└──────────────┘ └─────────────┘ └──────────────┘
```

## Project Structure

```
dogmv/
├── src/
│   └── main.rs              # Main application code (~1059 lines)
├── Cargo.toml               # Rust dependencies
├── Cargo.lock               # Locked dependency versions
├── flake.nix                # Nix flake for packaging
├── shell.nix                # Development environment
├── README.md                # Project overview
├── USER_MANUAL.md           # End-user documentation
├── DEVELOPER.md             # This file
├── IMPLEMENTATION_PLAN.md   # Development roadmap
├── ADR.md                   # Architecture Decision Records
├── QA.md                    # Technical Q&A (directory tree specs)
├── CHANGELOG.md             # Version history
├── log.md                   # Development log
├── CLAUDE.md                # Project instructions for AI
├── test.md                  # Test markdown file
└── img/
    └── screenshot.png       # Application screenshot
```

## Development Setup

### Prerequisites

- **NixOS** or Nix package manager
- **Hyprland** (optional, for Wayland testing)
- Basic understanding of Rust, GTK4, and Markdown

### Setting up the development environment

```bash
# Clone the repository
git clone <repository-url>
cd dogmv

# Enter development environment
nix-shell

# This will:
# - Install Rust via rustup (stable channel)
# - Install GTK4, WebKitGTK 6.0, and dependencies
# - Set up environment variables
```

### Environment Variables

The `shell.nix` sets up:

- `RUST_LOG=dogmv=debug` - Debug logging enabled
- `GDK_BACKEND=wayland` - Force Wayland backend
- `XDG_DATA_DIRS` - GSettings schema paths for GTK4

## Building and Testing

### Build Commands

```bash
# Development build (with debug symbols)
cargo build

# Release build (optimized)
cargo build --release

# Check for compilation errors (faster than full build)
cargo check

# Run clippy linter
cargo clippy

# Format code
cargo fmt
```

### Running

```bash
# Run in development mode
cargo run -- README.md

# Run with debug logging
RUST_LOG=debug cargo run -- README.md

# Run release build
./target/release/dogmv README.md
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_render_markdown

# Run tests and show all output
RUST_LOG=debug cargo test -- --nocapture
```

### Current Test Coverage

6 unit tests covering:
- `test_render_markdown()` - Basic Markdown rendering
- `test_render_markdown_gfm()` - GitHub Flavored Markdown features
- `test_syntax_highlighting()` - Code block syntax highlighting
- `test_code_block_without_language()` - Fallback for unlabeled code
- `test_create_html()` - HTML wrapper generation
- `test_create_html_includes_css()` - CSS injection verification

## Code Organization

### Main Components (src/main.rs ~1059 lines)

**Current Status**: All code in single file - requires refactoring for maintainability

#### 1. Data Models
- **AppState** (struct) - Application state container
  - `current_file`: Currently opened file path
  - `root_dir`: Root directory for tree view
  - `webview`: WebView widget reference
  - `tree_scroll`: Tree view scroll container
  - `toggle_button`: Sidebar toggle button
  - `paned`: Paned layout widget

- **FileItem** (GObject subclass) - Tree view item model
  - `path`: File/directory path
  - `name`: Display name
  - `is_dir`: Directory flag
  - `is_symlink`: Symlink flag

#### 2. UI Construction
- **main()** - GTK application initialization
- **build_ui()** - Main UI assembly
  - CSS setup for toggle button
  - HeaderBar with CSD
  - Paned layout creation
  - Sidebar with tree view
  - WebView for preview
  - Keyboard shortcuts setup

#### 3. Directory Tree
- **create_tree_view()** - TreeListModel + ListView setup
- **load_directory_items()** - Directory scanning and sorting
- **setup_file_selection_handler()** - File click handling
- **FileItem GObject implementation** - Properties macro based

#### 4. Sidebar Toggle
- **setup_toggle_button_css()** - Flat button CSS
- **setup_toggle_button()** - Toggle behavior with width preservation

#### 5. Markdown Rendering Pipeline
- **load_markdown()** - File I/O
- **render_markdown()** - comrak + syntect integration
- **create_html()** - HTML wrapper with GitHub-style CSS
- **display_markdown()** - WebView loading
- **display_welcome_message()** - Initial screen

#### 6. CLI Argument Parsing
- **parse_arguments()** - File/directory argument handling
  - Single file: Opens file, uses parent directory as root
  - Directory: Uses as root, shows welcome screen
  - No arguments: Uses current directory

#### 7. File Watching
- **setup_file_watcher()** - notify-based auto-reload
  - Background thread with inotify
  - 500ms polling in GTK main loop
  - Arc<Mutex<bool>> for thread-safe state

#### 8. Keyboard Shortcuts
- **setup_keyboard_shortcuts()** - EventControllerKey
  - Ctrl+Q: Quit
  - Ctrl+R: Reload
  - Ctrl+O: Open file dialog

#### 9. File Dialog
- **open_file_dialog()** - FileChooserDialog
  - Markdown filter (*.md, *.markdown)
  - Updates tree view and preview

#### 10. Error Handling
- **create_error_html()** - Styled error pages
- Display errors in WebView instead of dialogs

## Key Components

### Markdown Rendering Pipeline

```rust
fn render_markdown(markdown: &str) -> String {
    use comrak::{markdown_to_html_with_plugins, Options, Plugins};
    use comrak::plugins::syntect::SyntectAdapter;

    let mut options = Options::default();
    options.extension.strikethrough = true;  // GFM
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;

    let adapter = SyntectAdapter::new(Some("InspiredGitHub"));
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(markdown, &options, &plugins)
}
```

### File Watcher Pattern

**Challenge**: WebView doesn't implement `Send`, so we can't pass it to background threads.

**Solution**: Arc<Mutex<bool>> flag pattern

```rust
let file_changed = Arc::new(Mutex::new(false));
let file_changed_clone = Arc::clone(&file_changed);

// Background thread: Set flag on file change
std::thread::spawn(move || {
    // ... notify watcher setup ...
    *file_changed_clone.lock().unwrap() = true;
});

// Main GTK thread: Poll flag every 500ms
glib::timeout_add_local(Duration::from_millis(500), move || {
    let mut changed = file_changed.lock().unwrap();
    if *changed {
        display_markdown(&webview, &file_path);
        *changed = false;
    }
    glib::ControlFlow::Continue
});
```

### Keyboard Event Handling

**Important**: EventControllerKey must be attached to WebView, not Window!

```rust
// ❌ Wrong - events won't be received
window.add_controller(controller);

// ✅ Correct - WebView captures all keyboard input
webview.add_controller(controller);
```

## Adding New Features

### Adding a New Keyboard Shortcut

1. Edit `setup_keyboard_shortcuts()` function:

```rust
fn setup_keyboard_shortcuts(window: &ApplicationWindow, webview: &WebView, file_path: &str) {
    // ... existing code ...

    controller.connect_key_pressed(move |_, key, _keycode, modifier| {
        if !modifier.contains(gdk::ModifierType::CONTROL_MASK) {
            return glib::Propagation::Proceed;
        }

        if let Some(ch) = key.to_unicode() {
            match ch {
                // ... existing shortcuts ...

                'n' | 'N' => {
                    // Ctrl+N: Your new feature
                    info!("Ctrl+N pressed - Doing something");
                    your_new_function();
                    return glib::Propagation::Stop;
                }

                _ => {}
            }
        }

        glib::Propagation::Proceed
    });
}
```

2. Implement the feature function
3. Update USER_MANUAL.md
4. Add tests if applicable

### Adding a New CSS Theme

Edit `create_html()` function to modify the `<style>` block:

```rust
fn create_html(body: &str, base_path: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <base href="file://{}/">
    <style>
        /* Your custom CSS here */
        body {{
            background-color: #1e1e1e;  /* Dark theme */
            color: #d4d4d4;
        }}
        /* ... */
    </style>
</head>
<body>{}</body>
</html>"#, base_path, body)
}
```

### Adding New Markdown Extensions

Modify `render_markdown()` to enable comrak options:

```rust
let mut options = Options::default();
options.extension.footnotes = true;           // Enable footnotes
options.extension.description_lists = true;   // Enable definition lists
options.extension.front_matter_delimiter = Some("---".to_string());
```

See [comrak documentation](https://docs.rs/comrak/) for all options.

## Debugging

### Enable Debug Logging

```bash
# Debug level for dogmv only
RUST_LOG=dogmv=debug cargo run -- README.md

# Trace level (very verbose)
RUST_LOG=dogmv=trace cargo run -- README.md

# All libraries
RUST_LOG=trace cargo run -- README.md
```

### Common Debug Scenarios

**Keyboard shortcuts not working:**
- Check if controller is attached to WebView, not Window
- Verify `key.to_unicode()` returns expected character
- Check modifier mask: `gdk::ModifierType::CONTROL_MASK`

**File watcher not triggering:**
- Verify inotify events are being received (check logs)
- Ensure file path is absolute
- Check polling interval (500ms)

**Markdown not rendering:**
- Print HTML output: `println!("{}", full_html);`
- Check comrak options
- Verify syntect theme exists

**GSettings errors:**
- Ensure `gsettings-desktop-schemas` in `shell.nix`
- Check `XDG_DATA_DIRS` is set correctly
- Run `gsettings list-schemas | grep FileChooser`

### Using GDB

```bash
# Build with debug symbols
cargo build

# Run with GDB
gdb --args target/debug/dogmv README.md

# Inside GDB:
(gdb) run
(gdb) break main
(gdb) continue
```

## Contributing

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy and fix warnings: `cargo clippy`
- Add tests for new features
- Update documentation (USER_MANUAL.md, DEVELOPER.md)

### Commit Messages

Follow conventional commits:

```
feat: Add dark mode support
fix: Resolve keyboard shortcut conflict with Hyprland
docs: Update user manual with new features
refactor: Simplify file watcher logic
test: Add tests for error handling
```

### Pull Request Process

1. Create feature branch: `git checkout -b feature/my-feature`
2. Make changes and test: `cargo test`
3. Update documentation
4. Commit with descriptive messages
5. Push and create PR

### Architecture Decisions

Major architecture changes should be documented in `ADR.md` following the existing format:

```markdown
### ADR-XX: [Decision Title]

**Date**: YYYY-MM-DD
**Status**: Proposed/Accepted/Rejected

**Context**: ...
**Decision**: ...
**Consequences**: ...
```

## Performance Considerations

### File Watching

- Uses inotify (Linux) - very low overhead
- 500ms polling interval balances responsiveness vs CPU usage
- Single background thread for all file watching

### Rendering

- comrak is fast but can be slow for very large files (>1MB)
- syntect caches syntax definitions
- WebView uses GPU acceleration for rendering

### Memory

- Entire file is loaded into memory
- HTML is regenerated on each reload
- No caching of rendered output (trade-off for simplicity)

## Known Issues

1. **WebView Send trait**: WebView doesn't implement Send, requires Arc<Mutex> pattern
2. **GSettings dependency**: Requires proper NixOS environment setup
3. **Single file limitation**: No multi-tab or split view support
4. **Read-only**: No editing capabilities

## Future Enhancements

See `IMPLEMENTATION_PLAN.md` for planned features:

- Dark mode toggle
- Custom CSS themes
- Export to PDF/HTML
- Multi-file support
- Search functionality
- Table of contents generation

## Resources

- [GTK4 Rust Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- [comrak documentation](https://docs.rs/comrak/)
- [syntect documentation](https://docs.rs/syntect/)
- [notify documentation](https://docs.rs/notify/)
- [WebKitGTK API](https://webkitgtk.org/)

## License

See LICENSE file for details.
