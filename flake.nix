{
  description = "twt";

  inputs = {
    nixpkgs = { url = "github:NixOS/nixpkgs/nixpkgs-unstable"; };

    flake-utils = { url = "github:numtide/flake-utils"; };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    { self, nixpkgs, crane, flake-utils, advisory-db, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustStable =
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
        src = ./.;
        cargoArtifacts = craneLib.buildDepsOnly { inherit src buildInputs; };
        buildInputs = [ ];

        twt = craneLib.buildPackage {
          inherit src;
          doCheck = false;
        };
        twt-clippy = craneLib.cargoClippy {
          inherit cargoArtifacts src buildInputs;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };
        twt-fmt = craneLib.cargoFmt { inherit src; };
        twt-audit = craneLib.cargoAudit { inherit src advisory-db; };
        twt-nextest = craneLib.cargoNextest {
          inherit cargoArtifacts src buildInputs;
          partitions = 1;
          partitionType = "count";
        };
      in {
        checks = {
          # TODO: Switch to cargo-audit and re-enable this
          # twt-audit
          inherit twt twt-clippy twt-fmt twt-nextest;
        };

        packages.default = twt;

        apps.default = flake-utils.lib.mkApp { drv = twt; };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          nativeBuildInputs = with pkgs; [
            cargo-nextest
            cargo-release
            rustStable
          ];
        };
      });
}
