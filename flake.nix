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
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          # xorg.libX11
          # xorg.libXcursor
          # xorg.libXi
          # xorg.libXrandr
        ];
      in
      {
        # For `nix build` and `nix run`
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;
          pname = "chip8";

          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper

            libxkbcommon
            libGL
            wayland
            cmake
            fontconfig
          ];
          postInstall = ''
            wrapProgram "$out/bin/chip8" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';
        };

        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        # For `nix develop`
        devShell = with pkgs; mkShell rec {
          buildInputs = [ 
            pkg-config
            cargo 
            rustc 
            rustfmt 
            pre-commit 
            rustPackages.clippy
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          # LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          LD_LIBRARY_PATH = libPath;
        };
      });
}
