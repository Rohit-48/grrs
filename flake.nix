{
  description = "Rust dev shell with rust-src for rust-analyzer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust-toolchain = pkgs.symlinkJoin {
          name = "rust-toolchain";
          paths = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rustPlatform.rustLibSrc
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            clippy
            rustfmt
            rust-analyzer
            rust-toolchain
          ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "IntelliJ toolchain: $(dirname "$(command -v rustc)")"
            echo "Stdlib source: $RUST_SRC_PATH"
          '';
        };
      });
}

