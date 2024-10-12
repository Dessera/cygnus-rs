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
          toolchain = fenix.packages.${system}.stable.withComponents [
            "cargo"
            "rustc"
            "rust-src"
            "rust-analyzer"
          ];
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        in
        {
          packages.default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;

            # CARGO_BUILD_TARGET = systemRef.${system};
          };

          devShells.default = craneLib.devShell {
            packages = with pkgs; [
              nil
              nixfmt-rfc-style
            ];
          };
        };
    };
}
