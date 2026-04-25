{
  description = "Rust flake template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
  };
  outputs = inputs@{ nixpkgs, rust-overlay, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          dbg =
            value:
            builtins.trace (
              if value ? type && value.type == "derivation" then
                "derivation: ${value}"
              else
                pkgs.lib.generators.toPretty { } value
            ) value;

          crane = {
            lib = (inputs.crane.mkLib pkgs).overrideToolchain (_: self'.packages.rust-nightly);
          };
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
            ];
          };

          packages = {
            rust-nightly = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            default = crane.lib.buildPackage {
              src = ./.;
              doCheck = false;
              cargoBuildCommand = "cargo build --release";
            };
          };
          checks = {
            default = crane.lib.cargoTest {
              strictDeps = true;
              src = ./.;
              cargoArtifacts = crane.lib.buildDepsOnly {
                strictDeps = true;
                src = ./.;
              };
            };
          };
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ self'.packages.rust-nightly ]
                ++ (with pkgs; [
                jq
                moreutils
                nixd
                tree-sitter
                nodejs
                typescript-language-server
                graphviz
              ]);
            };
          };
        };
    };
}
