{
  description = "Development environment for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src
            ]
          );
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        let
          libs = [
            pkgs.alsa-lib
            pkgs.openssl
            pkgs.systemd # For libudev
            pkgs.wayland # For wayland support
            pkgs.libxkbcommon # Wayland backend crashes without this.
            pkgs.libGL # Graphics API
          ];
          libPath = pkgs.lib.makeLibraryPath libs;
        in {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              pkg-config
              cargo-deny
              cargo-edit
              cargo-watch
              rust-analyzer
            ] ++ libs;

            env = {
              # Required by rust-analyzer.
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
              # Required to make `cargo run` work.
              LD_LIBRARY_PATH = libPath;
              
            };
          };
        }
      );
    };
}
