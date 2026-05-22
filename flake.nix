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

          nist-vectors = pkgs.fetchzip {
            url = "https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/sha3/sha-3bytetestvectors.zip";
            stripRoot = false;
            hash = "sha256-nWNYO4H2piqf6CW7NJfqc4+DHzByYoNbbjGE3QeO4uc=";
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
            fetch-nist-vectors = pkgs.writeShellApplication {
              name = "fetch-nist-vectors";
              text = ''
                cp -r ${nist-vectors} .nist-vectors
              '';
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
