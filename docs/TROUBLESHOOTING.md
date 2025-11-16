# Troubleshooting Guide

## GSettings Schema Error (FileChooser crash)

### Problem

When pressing Ctrl+O to open a file, the application crashes with:

```
(dogmv:XXXXX): GLib-GIO-ERROR **: Settings schema 'org.gtk.gtk4.Settings.FileChooser' is not installed
Trace/breakpoint trap (core dumped)
```

### Root Cause

GTK4's FileChooser components try to access GSettings schema `org.gtk.gtk4.Settings.FileChooser` to store/retrieve user preferences (last directory, view mode, etc.). In some environments (especially NixOS), this schema may not be properly installed or accessible.

### Solutions Implemented

#### 1. Use FileChooserNative instead of FileChooserDialog

FileChooserNative uses the platform's native file picker, which doesn't rely on GSettings.

```rust
let dialog = FileChooserNative::new(
    Some("Open File"),
    Some(window),
    FileChooserAction::Open,
    Some("_Open"),
    Some("_Cancel"),
);
```

#### 2. Disable GSettings Backend (Early Initialization)

Set `GSETTINGS_BACKEND=memory` **before** GTK initialization to force GSettings to use an in-memory backend instead of trying to access system schemas.

This is done using the `ctor` crate, which runs the initialization function before `main()`:

```rust
use ctor::ctor;

#[ctor]
fn init_environment() {
    // This runs before main(), ensuring the env var is set before GTK initializes
    std::env::set_var("GSETTINGS_BACKEND", "memory");
}
```

**Why #[ctor] is necessary:**
- GTK reads environment variables during its initialization
- Setting the variable in `main()` is too late
- The `#[ctor]` attribute ensures the variable is set before any GTK code runs

#### 3. Use the Wrapper Script (Alternative)

If you encounter issues, use the provided wrapper script:

```bash
./run.sh README.md
```

The wrapper ensures the environment variable is set before launching the application.

### Verification

To verify the fix is working:

1. Build the application:
   ```bash
   cargo build --release
   ```

2. Run the application:
   ```bash
   cargo run README.md
   ```

3. Press **Ctrl+O** to open the file dialog

4. Expected behavior:
   - No crash
   - Native file picker dialog appears
   - You can select files and they display correctly

### Related Issues

- Portal settings warning: `Failed to read portal settings`
  - This is a harmless warning in Wayland/Hyprland environments
  - The application functions correctly despite this warning

- MESA warnings: `cannot initialize blitter engine`
  - These are Intel graphics driver warnings
  - They don't affect application functionality
  - Can be safely ignored

### Additional Notes

The GSettings backend setting only affects dogmv and doesn't interfere with other GTK applications on the system.
