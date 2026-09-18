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
            lib = ((inputs.crane.mkLib pkgs).overrideToolchain (_: self'.packages.rust-nightly)).overrideScope (
              final: prev: {
                stdenvSelector = p: p.clangStdenv;
              }
            );
          };

          nist-vectors = pkgs.fetchzip {
            url = "https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/sha3/sha-3bytetestvectors.zip";
            stripRoot = false;
            hash = "sha256-nWNYO4H2piqf6CW7NJfqc4+DHzByYoNbbjGE3QeO4uc=";
          };
          blake3-vectors = builtins.fetchurl {
            name = "test_vectors.json";
            url = "https://raw.githubusercontent.com/BLAKE3-team/BLAKE3/refs/heads/master/test_vectors/test_vectors.json";
            sha256 = "sha256:097n6bdn9l67jnjqsr6gg2pg7acr3bf7rbrjwvbfcxycmjl1xffw";
          };
          build = crane.lib.buildPackage {
            src =
              let
                unfilteredRoot = ./.;
              in
              pkgs.lib.fileset.toSource {
                root = unfilteredRoot;
                fileset = pkgs.lib.fileset.unions [
                  (crane.lib.fileset.commonCargoSources unfilteredRoot)
                  ./c
                  ./zig
                ];
              };
            doCheck = false;
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.rustPlatform.bindgenHook
            ];
            buildInputs = [
              pkgs.llvmPackages_latest.libclang.lib
              pkgs.llvmPackages_latest.libllvm
              pkgs.llvmPackages_latest.lld
              pkgs.llvmPackages_latest.bintools
              pkgs.clangStdenv.cc.libc
              pkgs.zigpkgs.master
            ];
            LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";
            preBuild = ''
              # zig needs a $HOME dir for caching (non-configurable)
              export ZIG_GLOBAL_CACHE_DIR=.
              zig version
            '';
            cargoBuildCommand = "cargo build --release -Ftracing-off";
            meta.mainProgram = "vm";
          };
          buildObject =
            mirFile:
            pkgs.clangStdenv.mkDerivation {
              name = "${baseNameOf mirFile}.o";
              src = mirFile;
              dontUnpack = true;
              buildInputs = [ build ];
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
            default = build;
            build-rust = build;
          };
          apps =
            builtins.mapAttrs
              (name: value: {
                type = "app";
                program = value;
              })
              {
                run =
                  let
                    cmd = "${pkgs.lib.getExe self'.packages.build-rust} run --obj ${buildObject ./tests/sha3-256.mir} --input-file ${./random.bin}";
                  in
                  pkgs.writeShellApplication {
                    name = "run";
                    text = ''
                      echo running rust
                      time ${cmd} -i rust
                      echo

                      echo running rust-tail-call
                      time ${cmd} -i rust-tail-call
                      echo

                      echo running zig
                      time ${cmd} -i zig
                      echo

                      echo running c-computed-goto
                      time ${cmd} -i c-computed-goto
                      echo
                    '';
                  };
                fetch-nist-vectors = pkgs.writeShellApplication {
                  name = "fetch-nist-vectors";
                  text = ''
                    rm -r .nist-vectors/ 2>/dev/null || echo ""
                    mkdir -p .nist-vectors
                    cp -r --no-preserve=mode ${nist-vectors}/* .nist-vectors
                  '';
                };
                fetch-blake3-vectors = pkgs.writeShellApplication {
                  name = "fetch-blake3-vectors";
                  text = ''
                    rm -r .blake3-vectors/ 2>/dev/null || echo ""
                    mkdir -p .blake3-vectors
                    cp -r --no-preserve=mode ${blake3-vectors} .blake3-vectors/test_vectors.json
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
            default = pkgs.mkShellNoCC.override { stdenv = pkgs.clangStdenv; } {
              # inputsFrom = [ build-rust ];
              buildInputs = [
                (dbg pkgs.llvmPackages_latest.libclang.lib)
                (dbg pkgs.llvmPackages_latest.libllvm)
                pkgs.llvmPackages_latest.lld
                pkgs.llvmPackages_latest.bintools
                pkgs.clangStdenv.cc.libc
                self'.packages.rust-nightly
              ]
              ++ [ pkgs.zigpkgs.master ]
              ++ (with pkgs; [
                # (dbg overrideCC)
                cargo-fuzz
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
                (dbg clang-tools)
                # llvmPackages_latest.libllvm
                # llvmPackages_latest.libcxx
                # llvmPackages_latest.clang
              ]);
              LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";
              CUSTOM_LIBFUZZER_PATH = "${pkgs.llvmPackages_latest.compiler-rt}/lib/linux/libclang_rt.fuzzer-aarch64.a";
              nativeBuildInputs = [
                pkgs.pkg-config
                pkgs.rustPlatform.bindgenHook
                # pkgs.gcc16Stdenv.cc.libc.static
                config.treefmt.build.wrapper
              ]
              ++ pkgs.lib.attrsets.attrValues config.treefmt.build.programs;
            };
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
