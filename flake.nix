{
  inputs = {
    nixpkgs.url = "nixpkgs";
    utils.url = "github:numtide/flake-utils";

    rustoverlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "utils";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rustoverlay, utils, crane }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rustoverlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        toolchain = pkgs.rust-bin.stable."1.72.0".default;

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        commonArgs = { inherit src; };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        text2url = craneLib.buildPackage commonArgs // {
          inherit cargoArtifacts;
        };

      in {
        checks = { inherit text2url; };

        devShells.default = craneLib.devShell commonArgs // {
          name = "devshell";
          checks = self.checks.${system};
        };
        apps.default = utils.lib.mkApp { drv = text2url; };
        packages.default = text2url;
      });
}
