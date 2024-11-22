{
  description = "A Rust package flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cliflux";
          version = "v1.4.4";
          src = ./.;
          cargoHash = "sha256-TtK1hN1RNusVWxWVTity+N1cpfauhfeQpkqAOI1fIco=";
        };
      }
    );
}
