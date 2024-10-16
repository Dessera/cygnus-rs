{ cygnus-packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.modules.services.cygnus-rs;
  cygnus-rs = cygnus-packages.${pkgs.system}.default;
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
      type = types.path;
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
        ExecStart = "${cygnus-rs}/bin/cygnus auth -f ${cfg.userFile}";
        Restart = "always";
        RestartSec = 5;
      };
    };
  };
}