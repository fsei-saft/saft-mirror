{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-mozilla.url = "github:mozilla/nixpkgs-mozilla";
  };

  outputs = { self, nixpkgs, flake-utils, nixpkgs-mozilla, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ nixpkgs-mozilla.overlays.rust ];
      };
    in {
      devShell = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          (rustChannelOf { channel = "nightly"; date = "2024-02-08"; }).rust
          (rustChannelOf { channel = "nightly"; date = "2024-02-08"; }).rust-src
          typst
        ];
      };

      packages.default = pkgs.mkDerivation {};
    }
  );
}
