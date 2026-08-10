{
  description = "Rust flake template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    zig-overlay = {
      url = "github:mitchellh/zig-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
  };
  outputs = inputs@{ nixpkgs, rust-overlay, zig-overlay, flake-parts, ... }:
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
          build-rust = crane.lib.buildPackage {
              src = ./.;
              doCheck = false;
              cargoBuildCommand = "cargo build --release";
            };
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
              zig-overlay.overlays.default
            ];
          };

          packages = {
            rust-nightly = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            default = build-rust;
            inherit build-rust;
            build-zig = pkgs.stdenv.mkDerivation {
              pname = "vm-zig";
              version = "0.0.0";
              src = ./zig;
              buildInputs = [ pkgs.zigpkgs.master ];
              buildPhase = ''
                # zig needs a $HOME dir for caching (non-configurable)
                export HOME=.
                zig build --release=fast
              '';
              installPhase =''
                mv ./zig-out "$out"
              '';
            };
            build-c = pkgs.stdenv.mkDerivation {
              pname = "vm-c";
              version = "0.0.0";
              src = ./c;
              buildInputs = [];
              buildPhase = ''
                gcc -flto -Ofast vm.c
                # clang -flto -O3 vm.c
              '';
              installPhase =''
                mkdir "$out"
                mkdir "$out/bin"
                mv ./a.out "$out/bin/vm"
              '';
            };
          };
          apps = builtins.mapAttrs (name: value: {type = "app"; program = value;}) {
            run-c = pkgs.writeShellApplication {name = "run-c"; text = ''
              ${pkgs.getExe self'.packages.build-c} ${./tests/sha3-256.o} ${./random.bin}
            '';};
            run-zig = pkgs.writeShellApplication {name = "run-zig"; text = ''
              ${pkgs.getExe self'.packages.build-zig} ${./tests/sha3-256.o} ${./random.bin}
            '';};
            run-rust = pkgs.writeShellApplication {name = "run-rust"; text = ''
              ${pkgs.getExe self'.packages.build-rust} ${./tests/sha3-256.o} ${./random.bin}
            '';};
            fetch-nist-vectors = pkgs.writeShellApplication {
              name = "fetch-nist-vectors";
              text = ''
                rm -r .nist-vectors/ || echo ""
                mkdir -p .nist-vectors
                cp -r --no-preserve=mode ${nist-vectors}/* .nist-vectors
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
                ++ [ pkgs.zigpkgs.master ]
                ++ (with pkgs; [
                jq
                moreutils
                nixd
                tree-sitter
                nodejs
                typescript-language-server
                graphviz
                samply
                cargo-pgo
                cargo-criterion
                hexyl
                zig
                zls
                # libclang
                clang-tools
                # llvmPackages_latest.libllvm
                # llvmPackages_latest.libcxx
                # llvmPackages_latest.clang
              ]);
            };
          };
        };
    };
}
