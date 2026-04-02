{
  description = "Minimal Rust development shell for simple_cw";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { fenix, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages.${system};
        toolchain = fenixPkgs.combine [
          fenixPkgs.stable.minimalToolchain
          fenixPkgs.stable.rust-src
          fenixPkgs.stable.rustfmt
        ];
        nativeLibraries = with pkgs; [
          alsa-lib
          libGL
          libxkbcommon
          wayland
        ];
      in {
        devShells.default = pkgs.mkShell {
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath nativeLibraries}"
            export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
          '';
          packages = [
            toolchain
            fenixPkgs.stable.rust-analyzer
            pkgs.pkg-config
          ] ++ nativeLibraries;
        };
      });
}
