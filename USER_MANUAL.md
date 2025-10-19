# dogmv User Manual

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Basic Usage](#basic-usage)
4. [Keyboard Shortcuts](#keyboard-shortcuts)
5. [Features](#features)
6. [Troubleshooting](#troubleshooting)

## Introduction

**dogmv** is a lightweight, fast Markdown viewer designed specifically for NixOS and Hyprland (Wayland) environments. It provides a clean, GitHub-style rendering of Markdown files with syntax highlighting for code blocks.

### Key Features

- **GitHub Flavored Markdown** support (tables, task lists, strikethrough, etc.)
- **Syntax highlighting** for 200+ programming languages
- **Auto-reload** - automatically refreshes when files are modified
- **Live preview** - perfect for editing Markdown files in your favorite editor
- **Keyboard shortcuts** for quick navigation
- **Native Wayland** support for Hyprland

## Installation

### Using Nix Shell (Development)

```bash
cd /path/to/dogmv
nix-shell
cargo build --release
```

### Using Nix Flakes (Recommended)

```bash
# Build the package
nix build

# Run directly
nix run . -- README.md
```

### From Source

```bash
# Prerequisites: Rust toolchain, GTK4, WebKitGTK 6.0

# Clone and build
git clone <repository-url>
cd dogmv
cargo build --release

# Binary will be in target/release/dogmv
```

## Basic Usage

### Opening a File

```bash
# Open a Markdown file
dogmv README.md

# Open with full path
dogmv /path/to/document.md

# With logging enabled
RUST_LOG=info dogmv README.md
```

### Command-Line Options

```
dogmv <file.md>

Arguments:
  <file.md>    Path to the Markdown file to view

Examples:
  dogmv README.md
  dogmv /path/to/document.md
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| **Ctrl+Q** | Quit the application |
| **Ctrl+R** | Reload the current file |
| **Ctrl+O** | Open a different file (shows file chooser dialog) |
| **Space** | Scroll down |
| **Shift+Space** | Scroll up |

## Features

### GitHub Flavored Markdown

dogmv supports all GitHub Flavored Markdown features:

- **Tables** - Full table support with alignment
- **Task lists** - Interactive checkboxes (display only)
- **Strikethrough** - `~~deleted text~~`
- **Autolinks** - Automatic URL linking
- **Fenced code blocks** - With language specification

### Syntax Highlighting

Code blocks are automatically highlighted based on the specified language:

\`\`\`rust
fn main() {
    println!("Hello, world!");
}
\`\`\`

Supported languages include:
- Rust, Python, JavaScript, TypeScript, Go, C, C++, Java
- Shell/Bash, PowerShell, SQL
- HTML, CSS, JSON, YAML, TOML, XML
- Markdown, LaTeX, and 200+ more

### Auto-Reload

dogmv automatically watches the opened file for changes. When you save the file in your editor, the viewer will refresh automatically within ~500ms.

**Perfect for live preview workflow:**

```bash
# Terminal 1: Open the viewer
dogmv document.md

# Terminal 2: Edit with your favorite editor
vim document.md
# or
code document.md
# or
nvim document.md

# Changes are reflected automatically in dogmv!
```

### Image Support

Relative and absolute image paths are supported:

```markdown
![Local image](./images/screenshot.png)
![Absolute path](/path/to/image.jpg)
```

## Troubleshooting

### Application doesn't start

**Error: "File not found"**
- Check that the file path is correct
- Verify the file exists: `ls -l <file.md>`
- Ensure you have read permissions: `chmod +r <file.md>`

**Error: "Settings schema not installed"**
- Make sure you're running in a nix-shell environment
- Run `exit` and then `nix-shell` again to reload environment

### Display issues

**Markdown not rendering correctly**
- Ensure the file is valid Markdown
- Try reloading with Ctrl+R
- Check the terminal for error messages

**Images not showing**
- Verify image paths are correct
- For relative paths, ensure they're relative to the Markdown file location
- Check file permissions on image files

### Performance issues

**Slow rendering**
- Large files (>1MB) may take a moment to render
- Syntax highlighting can be slow for very large code blocks
- Consider splitting large documents into smaller files

**High CPU usage**
- Disable auto-reload if not needed (currently always enabled)
- Check if there are other processes modifying the file frequently

### Keyboard shortcuts not working

**Ctrl+Q, Ctrl+O, or Ctrl+R not responding**
- Ensure the dogmv window has focus
- Try clicking on the window first
- Check if Hyprland has conflicting keybindings

### Known Limitations

1. **Read-only** - dogmv is a viewer only, it cannot edit Markdown files
2. **No navigation** - No built-in file browser or directory navigation
3. **Single file** - Can only view one file at a time (use Ctrl+O to switch)
4. **Task lists** - Checkboxes are display-only, not interactive

## Getting Help

- **Issues**: Report bugs at `<repository-url>/issues`
- **Documentation**: See README.md and IMPLEMENTATION_PLAN.md
- **Logs**: Run with `RUST_LOG=debug dogmv <file.md>` for detailed logs

## Tips & Tricks

### Use with your editor

Set up a split-screen workflow:

1. Open dogmv in one window/workspace
2. Open your editor in another
3. Edit and save - changes appear instantly!

### Quick file switching

Use Ctrl+O to quickly switch between Markdown files without closing the application.

### Preview before commit

Use dogmv to preview README files and documentation before committing to Git:

```bash
dogmv README.md
# Make edits in editor
# Preview in dogmv
# Commit when satisfied
```

## Version

Current version: 0.1.0

## License

See LICENSE file for details.
