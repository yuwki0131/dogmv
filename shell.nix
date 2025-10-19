{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain - use rustup for latest stable
    rustup

    # GTK4 and WebKitGTK dependencies
    gtk4
    gtk3
    webkitgtk_6_0
    glib
    cairo
    pango
    gdk-pixbuf
    atk
    gobject-introspection

    # Build tools
    pkg-config
    wrapGAppsHook4
  ];

  shellHook = ''
    # Ensure rustup is initialized and latest stable is installed
    if [ ! -d "$HOME/.rustup" ]; then
      rustup-init -y --default-toolchain stable
    fi
    export PATH="$HOME/.cargo/bin:$PATH"

    # Update to latest stable if needed
    rustup default stable 2>/dev/null || true
    rustup update stable 2>/dev/null || true

    export RUST_LOG=dogmv=debug
    export GDK_BACKEND=wayland
    echo "dogmv development environment loaded (shell.nix)"
    echo "Rust version: $(rustc --version)"
    echo "Run 'cargo run <file.md>' to start the application"
  '';
}
