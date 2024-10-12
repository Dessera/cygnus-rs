{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-parts,
      fenix,
      ...
    }@inputs:
    let
      osModule = import ./nix/modules/nixos.nix { cygnus-packages = self.packages; };
    in
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      flake = {
        nixosModules = {
          default = osModule;
        };
      };
      perSystem =
        {
          config,
          pkgs,
          system,
          ...
        }:
        let
          craneLibDefault = (crane.mkLib pkgs).overrideToolchain (
            import ./nix/toolchain { inherit fenix system; }
          );
          craneLibWindows = (crane.mkLib pkgs).overrideToolchain (
            import ./nix/toolchain/windows.nix { inherit fenix system; }
          );
        in
        {
          packages.default = craneLibDefault.buildPackage {
            src = craneLibDefault.cleanCargoSource ./.;

            strictDeps = true;
          };
          packages.windows = craneLibWindows.buildPackage {
            src = craneLibWindows.cleanCargoSource ./.;

            strictDeps = true;
            doCheck = false;

            CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";

            # fixes issues related to libring
            TARGET_CC = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";

            #fixes issues related to openssl
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

            depsBuildBuild = with pkgs; [
              pkgsCross.mingwW64.stdenv.cc
              pkgsCross.mingwW64.windows.pthreads
            ];
          };

          devShells.default = craneLibDefault.devShell {
            packages = with pkgs; [
              nil
              nixfmt-rfc-style
            ];
          };
        };
    };
}
