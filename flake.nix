{
  description = "A development environment for Rust with fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
      in
      {
        devShell = pkgs.mkShell {
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
              pkgs.alsaLib
              pkgs.udev
              pkgs.vulkan-loader
            ]}"
          '';

          buildInputs = with pkgs; [
            (
              with fenix.packages.${system};
              combine (
                with default; [
                  cargo
                  clippy-preview
                  latest.rust-src
                  rust-analyzer
                  rust-std
                  rustc
                  rustfmt-preview
                ]
              )
            )
            cargo-edit
            cargo-watch
            lld
            clang
            pkg-config
            udev
            alsaLib
            lutris
	          xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            vulkan-tools
            vulkan-headers
            vulkan-loader
            vulkan-validation-layers
          ];
        };
      }
    );
}
