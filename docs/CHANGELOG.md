# Changelog

All notable changes to dogmv will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-19

### Added

#### Core Features
- **Markdown Viewer** - Display Markdown files with GitHub-style rendering
- **GitHub Flavored Markdown** - Full GFM support including:
  - Tables with alignment
  - Task lists with checkboxes
  - Strikethrough text (~~deleted~~)
  - Automatic URL linking
  - Fenced code blocks
- **Syntax Highlighting** - Support for 200+ programming languages using syntect
  - InspiredGitHub color theme
  - Automatic language detection
  - Inline CSS styling for consistent rendering
- **Auto-reload** - Automatically refresh when file is modified
  - File system watcher using inotify (Linux)
  - 500ms polling interval
  - Background thread monitoring
- **Image Support** - Display images with relative and absolute paths
  - Relative path resolution using `<base>` tag
  - Proper handling of image directories

#### User Interface
- **GTK4 Application Window** - Native Wayland support
  - 1024x768 default window size
  - Clean, minimalist interface
  - WebView-based rendering
- **Keyboard Shortcuts**:
  - `Ctrl+Q` - Quit application
  - `Ctrl+R` - Reload current file
  - `Ctrl+O` - Open file dialog
- **File Dialog** - Native GTK file chooser
  - Markdown file filter (*.md, *.markdown)
  - Cancel and Open buttons
  - Remembers last directory

#### Error Handling
- **Enhanced Error Messages** - Detailed, user-friendly error reporting
  - File not found errors with helpful hints
  - Permission errors with suggestions
  - Styled error pages in WebView
- **Input Validation** - Comprehensive CLI argument validation
  - Required file path check
  - File existence verification
  - File readability check
- **Error Display** - Multiple error presentation methods
  - Terminal error messages with formatting
  - HTML error pages in WebView
  - MessageDialog support (for future use)

#### Development
- **Nix Support** - Complete Nix/NixOS integration
  - `flake.nix` for reproducible builds
  - `shell.nix` for development environment
  - Proper GSettings schema configuration
- **Comprehensive Documentation**:
  - `USER_MANUAL.md` - End-user guide
  - `DEVELOPER.md` - Developer documentation
  - `IMPLEMENTATION_PLAN.md` - Development roadmap
  - `ADR.md` - Architecture decisions
  - `QA.md` - Technical Q&A
  - `log.md` - Development log
- **Testing** - 6 unit tests covering:
  - Markdown rendering
  - GFM features
  - Syntax highlighting
  - HTML generation
  - CSS injection

### Fixed

#### Keyboard Shortcuts
- **WebView Event Capture** - Fixed keyboard shortcuts not working
  - EventControllerKey now attached to WebView instead of Window
  - Proper key detection using `key.to_unicode()`
  - Application quit using `app.quit()` instead of `window.close()`

#### File Chooser
- **GSettings Schema Error** - Resolved crash when opening file dialog
  - Added `gsettings-desktop-schemas` to Nix dependencies
  - Configured `XDG_DATA_DIRS` environment variable
  - Changed from `FileChooserNative` to `FileChooserDialog`

### Technical Details

#### Dependencies
- gtk4 = "0.10" - GTK4 Rust bindings
- webkit6 = "0.5" - WebKitGTK 6.0 bindings
- comrak = "0.24" - CommonMark/GFM parser
- syntect = "5.2" - Syntax highlighting
- notify = "6.1" - File system watcher
- log = "0.4" - Logging facade
- env_logger = "0.11" - Logger implementation

#### Architecture Highlights
- **WebView Integration** - Uses WebKitGTK 6.0 for HTML rendering
- **File Watching Pattern** - Arc<Mutex<bool>> + glib::timeout_add_local
  - Workaround for WebView not implementing Send trait
  - Background thread sets flag, main thread polls
- **Markdown Pipeline** - comrak + syntect integration
  - SyntectAdapter for code block highlighting
  - GitHub Flavored Markdown options enabled
  - Custom CSS for GitHub-style appearance

#### Environment
- **Target Platform**: NixOS with Hyprland (Wayland)
- **Rust Version**: 1.90.0 (via rustup)
- **Edition**: 2021

### Known Issues

1. **Read-only Viewer** - No editing capabilities (by design)
2. **Single File Limitation** - Can only view one file at a time
3. **No Navigation** - No built-in file browser
4. **Task Lists** - Checkboxes are display-only, not interactive

### Future Enhancements

Planned for future releases (see IMPLEMENTATION_PLAN.md):
- Dark mode / Light mode toggle
- Custom CSS theme support
- Export to PDF/HTML
- Multi-file tabs
- Search functionality
- Table of contents generation
- Plugin system
- Custom keybindings

## Development Timeline

- **Phase 1** (Complete): Project foundation - Rust + Nix setup
- **Phase 2** (Complete): Basic GUI - GTK4 + WebView
- **Phase 3** (Complete): Markdown rendering - comrak + GFM
- **Phase 4** (Complete): Syntax highlighting - syntect integration
- **Phase 5** (Complete): Auto-reload - notify file watcher
- **Phase 6** (Complete): Keyboard shortcuts - Ctrl+Q/R/O
- **Phase 7** (Complete): Error handling - Enhanced messages and dialogs
- **Phase 8** (Complete): Documentation - USER_MANUAL.md, DEVELOPER.md
- **Phase 9** (Complete): Distribution - Nix package, release notes

## Credits

- **Author**: dogmv development team
- **Built with**: Rust, GTK4, WebKitGTK, comrak, syntect
- **Platform**: NixOS, Hyprland, Wayland
- **License**: See LICENSE file

## Links

- Repository: <repository-url>
- Issues: <repository-url>/issues
- Documentation: See USER_MANUAL.md and DEVELOPER.md

---

## Version History

- **0.1.0** (2025-10-19) - Initial release
