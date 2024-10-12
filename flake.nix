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
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      flake = {
        nixosModules = {
          default = import ./nix/modules/nixos.nix { cygnus-packages = self.packages; };
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
          # toolchain =
          #   with fenix.packages.${system};
          #   combine [
          #     minimal.rustc
          #     minimal.cargo
          #     targets.x86_64-pc-windows-gnu.stable.rust-std
          #     targets.x86_64-unknown-linux-gnu.stable.rust-std
          #   ];
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
