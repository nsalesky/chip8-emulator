{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        # For `nix build` and `nix run`
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;

          nativeBuildInputs = with pkgs; [
            pkg-config
            SDL2
          ];
        };

        # For `nix develop`
        devShell = with pkgs; mkShell {
          buildInputs = [ 
            pkg-config
            cargo 
            rustc 
            rustfmt 
            pre-commit 
            rustPackages.clippy

            SDL2
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
