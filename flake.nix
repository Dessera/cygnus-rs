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
          craneLibCross = (crane.mkLib pkgs).overrideToolchain (
            import ./nix/toolchain/cross.nix { inherit fenix system; }
          );
        in
        {
          packages.default = craneLibDefault.buildPackage {
            src = craneLibDefault.cleanCargoSource ./.;
            strictDeps = true;
          };
          packages.windows = pkgs.callPackage ./nix/packages/x86_64-windows-gnu.nix {
            craneLib = craneLibCross;
            craneSrc = ./.;
            # pkgsCross = pkg;
            # openssl = pkgs.openssl;
          };
          packages.aarch64-linux-gnu =
            let
              crossPkgs = import nixpkgs {
                crossSystem = "aarch64-linux";
                localSystem = system;
              };
            in
            crossPkgs.callPackage ./nix/packages/aarch64-linux-gnu.nix {
              stdenv = crossPkgs.stdenv;
              craneLib = craneLibCross;
              craneSrc = ./.;
            };
          packages.x86_64-linux-musl = pkgs.callPackage ./nix/packages/x86_64-linux-musl.nix {
            craneLib = craneLibDefault;
            craneSrc = ./.;
          };

          devShells.default = craneLibDefault.devShell {
            packages = with pkgs; [
              nixd
              nixfmt-rfc-style
            ];
          };
        };
    };
}
