{
  description = "casper-rust-sdk";

  nixConfig = {
    extra-substituters = [
      "https://crane.cachix.org"
      "https://nix-community.cachix.org"
      "https://casper-cache.marijan.pro"
      "https://cspr.cachix.org"
    ];
    extra-trusted-public-keys = [
      "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "casper-cache.marijan.pro:XIDjpzFQTEuWbnRu47IqSOy6IqyZlunVGvukNROL850="
      "cspr.cachix.org-1:vEZlmbOsmTXkmEi4DSdqNVyq25VPNpmSm6qCs4IuTgE="
    ];
  };

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ flake-parts, treefmt-nix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      imports = [
        treefmt-nix.flakeModule
      ];
      perSystem = { self', inputs', pkgs, lib, ... }:
        let
          rustToolchain = inputs'.fenix.packages.stable.toolchain;
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

          casperSDKAttrs = {
            pname = "casper-rust-sdk";

            src = lib.fileset.toSource {
              root = ./.;
              fileset = lib.fileset.unions [
                ./Cargo.toml
                ./Cargo.lock
                ./src
                ./tests
              ];
            };

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = with pkgs; [
              openssl.dev
            ] ++ lib.optionals stdenv.isDarwin [
              libiconv
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

            # the coverage report will run the tests
            doCheck = false;
          };
        in
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ self'.packages.casper-rust-sdk ];
          };

          packages = {
            casper-rust-sdk-deps = craneLib.buildDepsOnly casperSDKAttrs;

            casper-rust-sdk-docs = craneLib.cargoDoc (casperSDKAttrs // {
              pname = "casper-rust-sdk-docs";
              cargoArtifacts = self'.packages.casper-rust-sdk-deps;
            });

            casper-rust-sdk = craneLib.buildPackage (casperSDKAttrs // {
              cargoArtifacts = self'.packages.casper-rust-sdk-deps;
            });

            default = self'.packages.casper-rust-sdk;
          };

          checks = {
            lint = craneLib.cargoClippy (casperSDKAttrs // {
              cargoArtifacts = self'.packages.casper-rust-sdk-deps;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

            coverage-report = craneLib.cargoTarpaulin (casperSDKAttrs // {
              pname = "casper-rust-sdk-coverage-report";
              cargoArtifacts = self'.packages.casper-rust-sdk-deps;
              # Default values from https://crane.dev/API.html?highlight=tarpau#cranelibcargotarpaulin
              # --avoid-cfg-tarpaulin fixes nom/bitvec issue https://github.com/xd009642/tarpaulin/issues/756#issuecomment-838769320
              cargoTarpaulinExtraArgs = "--skip-clean --out xml --output-dir $out --avoid-cfg-tarpaulin";
            });
          };

          treefmt = {
            projectRootFile = ".git/config";
            programs.nixpkgs-fmt.enable = true;
            programs.rustfmt.enable = true;
            programs.rustfmt.package = craneLib.rustfmt;
            settings.formatter = { };
          };
        };
      flake = {
        herculesCI.ciSystems = [ "x86_64-linux" ];
      };
    };
}
