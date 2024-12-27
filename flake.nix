{
  description = "openMulle using flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            alsa-lib
            udev
            #NOTE Add more deps
            vulkan-loader
            xorg.libX11
            xorg.libXrandr
            xorg.libXcursor
            xorg.libXi
            git
            llvmPackages.bintools
            clang
            pkg-config
            wasm-bindgen-cli
            wasm-pack
            libxkbcommon
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" "rustfmt" "clippy" ];
              targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
            }))
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              lib.makeLibraryPath [ udev alsa-lib vulkan-loader xorg.libX11 libxkbcommon ]
            }"
          '';
        };
      }
    );
}
