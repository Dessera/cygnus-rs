{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustTools = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain (_: rustTools);
      in
      rec {
        packages.default = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;

          # Add extra inputs here or any other derivation settings
          # doCheck = true;
          # buildInputs = [];
          # nativeBuildInputs = [];
        };

        nixosModules = {
          default = { config, lib, ... }:
            let
              cfg = config.modules.services.cygnus;
              inherit (lib) mkOption mkEnableOption mkIf types;
            in
            {
              options.modules.services.cygnus = {
                enable = mkEnableOption "Enable JLU Network Auth Service";
                userFile = mkOption {
                  type = types.str;
                  description = "Path to the user file";
                };
              };

              config = mkIf cfg.enable {
                environment.systemPackages = [
                  self.packages.default
                ];

                systemd.services.cygnus-auth = {
                  description = "JLU Network Auth Service";
                  enable = true;
                  after = [ "network.target" ];
                  wantedBy = [ "multi-user.target" ];
                  serviceConfig = {
                    Type = "simple";
                    ExecStart = "${self.packages.default}/bin/cygnus -f ${cfg.userFile}";
                    Restart = "always";
                    RestartSec = 5;
                  };
                };
              };
            };
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [ nil nixpkgs-fmt ];
        };
      });
}
