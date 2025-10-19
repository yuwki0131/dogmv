{
  description = "dogmv - Markdown Viewer for NixOS/Hyprland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        # Build dependencies
        buildInputs = with pkgs; [
          gtk4
          webkitgtk_6_0
          glib
          cairo
          pango
          gdk-pixbuf
          atk
          gsettings-desktop-schemas
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          wrapGAppsHook4
        ];

        # Source filtering
        src = craneLib.cleanCargoSource ./.;

        # Common arguments for crane
        commonArgs = {
          inherit src buildInputs nativeBuildInputs;
          strictDeps = true;
        };

        # Build the dependencies
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the application
        dogmv = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

      in {
        packages = {
          default = dogmv;
          inherit dogmv;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = dogmv;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ dogmv ];

          buildInputs = buildInputs ++ (with pkgs; [
            # Development tools
            rust-analyzer
            rustfmt
            clippy
          ]);

          nativeBuildInputs = nativeBuildInputs;

          # Set environment variables for development
          shellHook = ''
            export RUST_LOG=dogmv=debug
            export GDK_BACKEND=wayland
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk4}/share/gsettings-schemas/${pkgs.gtk4.name}:$XDG_DATA_DIRS"
            echo "dogmv development environment loaded"
            echo "Run 'cargo run <file.md>' to start the application"
          '';
        };
      }
    );
}
