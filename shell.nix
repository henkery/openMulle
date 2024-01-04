let
  # Pinned nixpkgs, deterministic. Last updated: 2/12/21.
  #pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/a58a0b5098f0c2a389ee70eb69422a052982d990.tar.gz")) {};

  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball ("channel:nixpkgs-unstable")) { };
in pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.clang
    pkgs.lld # To use lld linker
  ];
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.pre-commit
    pkgs.rustPackages.clippy
    pkgs.alsa-lib
    pkgs.udev
    #NOTE Add more deps
    pkgs.vulkan-loader
    pkgs.xorg.libX11
    pkgs.xorg.libXrandr
    pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.git
  ];
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
      pkgs.lib.makeLibraryPath [ pkgs.udev pkgs.alsaLib pkgs.vulkan-loader ]
    }"'';
  RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
}
