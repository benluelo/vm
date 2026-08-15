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
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };
  outputs =
    inputs@{
      nixpkgs,
      rust-overlay,
      zig-overlay,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      imports = [ inputs.treefmt-nix.flakeModule ];

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
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
            src = crane.lib.cleanCargoSource ./.;
            doCheck = false;
            cargoBuildCommand = "cargo build --release -Ftracing-off";
            meta.mainProgram = "vm";
          };
          buildObject =
            mirFile:
            pkgs.stdenv.mkDerivation {
              name = "${baseNameOf mirFile}.o";
              src = mirFile;
              dontUnpack = true;
              buildInputs = [ build-rust ];
              buildPhase = ''
                vm build ${mirFile} -o a.out
              '';
              installPhase = ''
                mv ./a.out "$out"
              '';
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
              installPhase = ''
                mv ./zig-out "$out"
              '';
              meta.mainProgram = "vm";
            };
            build-c = pkgs.gcc16Stdenv.mkDerivation {
              pname = "vm-c";
              version = "0.0.0";
              src = ./c;
              buildInputs = [ pkgs.gcc16Stdenv.cc.libc.static ];
              buildPhase = ''
                gcc --version
                gcc -flto -Ofast -static -g vm.c
                # clang -flto -O3 vm.c
              '';
              dontStrip = true;
              installPhase = ''
                mkdir "$out"
                mkdir "$out/bin"
                mv ./a.out "$out/bin/vm"
              '';
              meta.mainProgram = "vm";
            };
          };
          apps =
            builtins.mapAttrs
              (name: value: {
                type = "app";
                program = value;
              })
              {
                run-c = pkgs.writeShellApplication {
                  name = "run-c";
                  text = ''
                    time ${pkgs.lib.getExe self'.packages.build-c} ${buildObject ./tests/sha3-256.mir} ${./random.bin}
                  '';
                };
                run-zig = pkgs.writeShellApplication {
                  name = "run-zig";
                  text = ''
                    time ${pkgs.lib.getExe self'.packages.build-zig} ${buildObject ./tests/sha3-256.mir} ${./random.bin}
                  '';
                };
                run-rust = pkgs.writeShellApplication {
                  name = "run-rust";
                  text = ''
                    time ${pkgs.lib.getExe self'.packages.build-rust} run --obj ${buildObject ./tests/sha3-256.mir} --input-file ${./random.bin}
                  '';
                };
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
            default = pkgs.mkShell.override { stdenv = pkgs.gcc16Stdenv; } {
              buildInputs = [
                self'.packages.rust-nightly
              ]
              ++ [ pkgs.zigpkgs.master ]
              ++ (with pkgs; [
                jq
                moreutils
                nixd
                nil
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
            nativeBuildInputs = [
              pkgs.gcc16Stdenv.cc.libc.static
              config.treefmt.build.wrapper
            ]
            ++ pkgs.lib.attrsets.attrValues config.treefmt.build.programs;
          };

          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              rustfmt.enable = true;
              nixfmt.enable = true;
            };
            settings = {
              rustfmt = { };
            };
          };
        };
    };
}
