{
  description = "twt";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};

    custom-nixpkgs = {
      url = "github:msfjarvis/custom-nixpkgs";
      inputs = {
        fenix.follows = "fenix";
        nixpkgs.follows = "nixpkgs";
      };
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    flake-utils = {url = "github:numtide/flake-utils";};

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    custom-nixpkgs,
    fenix,
    crane,
    flake-utils,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      rustStable = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc=";
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
      commonArgs = {
        src = craneLib.cleanCargoSource ./.;
        buildInputs = [];
        nativeBuildInputs = [];
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      };
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {doCheck = false;});

      twt = craneLib.buildPackage (commonArgs // {doCheck = false;});
      twt-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts;
        });
      twt-fmt = craneLib.cargoFmt (commonArgs // {});
      twt-audit = craneLib.cargoAudit (commonArgs // {inherit advisory-db;});
      twt-nextest = craneLib.cargoNextest (commonArgs
        // {
          inherit cargoArtifacts;
          partitions = 1;
          partitionType = "count";
        });
    in {
      checks = {
        inherit twt twt-audit twt-clippy twt-fmt twt-nextest;
      };

      packages.default = twt;

      apps.default = flake-utils.lib.mkApp {drv = twt;};

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        nativeBuildInputs = with pkgs; [
          cargo-nextest
          cargo-release
          rustStable
          custom-nixpkgs.packages.${system}.oranda
        ];

        CARGO_REGISTRIES_CRATES_IO_PROTOCOL = "sparse";
      };
    });
}
