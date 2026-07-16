{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
        };
        runtimeDeps = with pkgs; [
          libx11
          libxkbcommon
          libxi
          libGL
        ];
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            nativeBuildInputs = [
              pkg-config
              rustToolchain
            ];
            buildInputs = runtimeDeps;
            LD_LIBRARY_PATH = lib.makeLibraryPath runtimeDeps;
            shellHook = ''
              export CARGO_HOME="$PWD/.cargo-local"
              export PATH="$PWD/.cargo-local/bin:$PATH"
            '';
          };
      }
    );
}
