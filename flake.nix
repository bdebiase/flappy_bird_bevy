{
  description = "A development environment for Rust with fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShell = pkgs.mkShell {
	  nativeBuildInputs = [
	    pkgs.pkg-config
	    pkgs.clang
	    pkgs.lld
	    pkgs.mold
	  ];

          buildInputs = with pkgs; [
	    cargo
	    rustc
	    rustfmt
	    pre-commit
	    rustPackages.clippy
	    alsa-lib
	    udev
	    vulkan-loader
	    xorg.libX11
	    xorg.libXrandr
	    xorg.libXcursor
	    xorg.libXi
	  ];

	  shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
	      pkgs.lib.makeLibraryPath [
                #pkgs.alsaLib
                #pkgs.udev
                pkgs.vulkan-loader
              ]}"
          '';
	  RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    );
}
