{
  description = "PostgreSQL data connector";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.flake-utils.follows = "flake-utils";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay, advisory-db }:
    flake-utils.lib.eachDefaultSystem (localSystem:
      let
        pkgs = import nixpkgs {
          system = localSystem;
          overlays = [ rust-overlay.overlays.default ];
        };

        # Edit ./nix/sqlserver-agent.nix to adjust library and buildtime
        # dependencies or other build configuration for sqlserver-agent
        crateExpression = import ./nix/sqlserver-agent.nix;
        cargoBuild = import ./nix/cargo-build.nix;

        # Build for the architecture and OS that is running the build
        sqlserver-agent = cargoBuild {
          inherit crateExpression nixpkgs crane rust-overlay localSystem;
        };

        inherit (sqlserver-agent) cargoArtifacts rustToolchain craneLib buildArgs;

        sqlserver-agent-x86_64-linux = cargoBuild {
          inherit crateExpression nixpkgs crane rust-overlay localSystem;
          crossSystem = "x86_64-linux";
        };

        sqlserver-agent-aarch64-linux = cargoBuild {
          inherit crateExpression nixpkgs crane rust-overlay localSystem;
          crossSystem = "aarch64-linux";
        };
      in
      {
        checks = {
          # Build the crate as part of `nix flake check`
          inherit sqlserver-agent;

          crate-clippy = craneLib.cargoClippy (buildArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          crate-nextest = craneLib.cargoNextest (buildArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });

          crate-audit = craneLib.cargoAudit {
            inherit advisory-db;
            inherit (sqlserver-agent) src;
          };
        };

        packages = {
          default = sqlserver-agent;
          sqlserver-agent-x86_64-linux = sqlserver-agent-x86_64-linux;
          sqlserver-agent-aarch64-linux = sqlserver-agent-aarch64-linux;

          docker = pkgs.callPackage ./nix/docker.nix { inherit sqlserver-agent; };

          dockerDev = pkgs.callPackage ./nix/docker.nix {
            inherit sqlserver-agent;
            tag = "dev";
          };

          docker-x86_64-linux = pkgs.callPackage ./nix/docker.nix {
            sqlserver-agent = sqlserver-agent-x86_64-linux;
            architecture = "amd64";
          };

          docker-aarch64-linux = pkgs.callPackage ./nix/docker.nix {
            sqlserver-agent = sqlserver-agent-aarch64-linux;
            architecture = "arm64";
          };

          publish-docker-image = pkgs.writeShellApplication {
            name = "publish-docker-image";
            runtimeInputs = with pkgs; [ coreutils skopeo ];
            text = builtins.readFile ./ci/deploy.sh;
          };
        };

        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks.${localSystem};
          nativeBuildInputs = [
            # runtime
            pkgs.protobuf

            # development
            pkgs.cargo-edit
            pkgs.cargo-flamegraph
            pkgs.cargo-insta
            pkgs.cargo-machete
            pkgs.cargo-watch
            pkgs.just
            pkgs.k6
            pkgs.pkg-config
            pkgs.rnix-lsp
            rustToolchain
          ];
        };
      });
}
