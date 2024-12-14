{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      crane,
      flake-parts,
      fenix,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      flake = {
        nixosModules = import ./nix/nixos.nix self.packages;
      };

      perSystem =
        { pkgs, ... }@pinputs:
        let
          rs = import ./nix/rust.nix { inherit pkgs crane fenix; };
        in
        {
          packages = import ./nix/packages.nix pinputs rs;
          devShells = import ./nix/devshells.nix pinputs rs;
        };
    };
}
