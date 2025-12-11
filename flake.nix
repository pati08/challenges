{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , crane
    ,
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
        ];
      };
      rustPlatform = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" "rustfmt" ];
        targets = [ "x86_64-unknown-linux-gnu" ];
      };
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustPlatform
          bacon
          swi-prolog
          llvmPackages.libclang
          llvmPackages.clang
          pkg-config
          zlib
          z3
        ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      };
    });
}
