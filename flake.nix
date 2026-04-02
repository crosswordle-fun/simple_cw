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
        toolchain = fenix.packages.${system}.stable.minimalToolchain;
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
            fenix.packages.${system}.stable.rust-analyzer
          ];
        };
      });
}
