{
  description = "A character creation tool that is compatible with the Cypher System";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-analyzer" "rust-src" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        htmlFilter = path: _type: builtins.match ".*html$" path != null;

        srcFilter = path: type: (htmlFilter path type)
          || (craneLib.filterCargoSources path type);

        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = srcFilter;
        };

        commonArgs = {
          inherit src;

          # uncomment if the project is a workspace
          pname = "cypher_character_creator";
          version = "0.1.0";
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        cypher_character_creator = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      rec {
        checks = {
          inherit cypher_character_creator;

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
          });

          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          fmt = craneLib.cargoFmt (commonArgs // {
            inherit src;
          });
        };

        packages.cypher_character_creator = cypher_character_creator;
        packages.default = packages.cypher_character_creator;

        # uncomment if there is a binary to be run
        # apps.cypher_character_creator = flake-utils.lib.mkApp {
        #   drv = packages.cypher_character_creator;
        #   name = "cypher_character_creator";
        # };
        # apps.default = apps.cypher_character_creator;

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks.${system};

          packages = with pkgs; [
            rustToolchain
            cargo-edit
            cargo-msrv
            cargo-outdated

            # GitHub tooling
            gh

            # Nix tooling
            nixpkgs-fmt
          ];
        };
      });
}
