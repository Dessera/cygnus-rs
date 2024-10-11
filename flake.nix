{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      flake-parts,
      rust-overlay,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      flake = {
        nixosModules = {
          default =
            {
              config,
              lib,
              pkgs,
              ...
            }:
            let
              cfg = config.modules.services.cygnus-rs;
              system = pkgs.system;
              cygnus-rs = self.packages.${system}.cygnus-rs;
              inherit (lib)
                mkOption
                mkEnableOption
                mkIf
                types
                ;
            in
            {
              options.modules.services.cygnus-rs = {
                enable = mkEnableOption "Enable JLU Network Auth Service";
                userFile = mkOption {
                  type = types.str;
                  description = "Path to the user file";
                };
              };

              config = mkIf cfg.enable {
                environment.systemPackages = [
                  cygnus-rs
                ];

                systemd.services.cygnus-rs = {
                  description = "JLU Network Auth Service";
                  enable = true;
                  after = [ "network.target" ];
                  wantedBy = [ "multi-user.target" ];
                  serviceConfig = {
                    Type = "simple";
                    ExecStart = "${cygnus-rs}/bin/cygnus -f ${cfg.userFile}";
                    Restart = "always";
                    RestartSec = 5;
                  };
                };
              };
            };
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
          rustTools = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
          };
          craneLib = (crane.mkLib pkgs).overrideToolchain (_: rustTools);
        in
        {
          packages.default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;

            # Add extra inputs here or any other derivation settings
            # doCheck = true;
            # buildInputs = [];
            # nativeBuildInputs = [];
          };

          devShells.default = craneLib.devShell {
            packages = with pkgs; [
              nil
              nixfmt-rfc-style
            ];
          };

          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
        };
    };
}
