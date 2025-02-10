{
  config,
  pkgs,
  lib ? pkgs.lib,
  ...
}:
with lib; let
  cfg = config.services.miniflux-remove-youtube;
in {
  options = {
    services.miniflux-remove-youtube = rec {
      enable = mkOption {
        type = types.str;
        default = false;
        description = ''
          Whether to run miniflux-remove-youtube service
        '';
      };
      miniflux-url = mkOption {
        type = types.str;
        default = "http://localhost:8080/";
        description = ''
          Url of the running miniflux instance
        '';
      };
      tokenfile-path = mkOption {
        type = types.str;
        default = "";
        description = ''
          Path of the file containing the miniflux token
        '';
      };
    };
  };

  config = mkIf cfg.enable {
    users.extraGroups.miniflux-remove-youtube = {};
    users.extraUsers.miniflux-remove-youtube = {
      description = "miniflux-remove-youtube";
      group = "miniflux-remove-youtube";
      isSystemUser = true;
      useDefaultShell = true;
    };

    environment.systemPackages = [pkgs.miniflux-remove-youtube];

    systemd.timers."miniflux-remove-youtube" = {
      wantedBy = ["timers.target"];
      timerConfig = {
        OnBootSec = "5m";
        OnUnitActiveSec = "5m";
        Unit = "miniflux-remove-youtube.service";
      };
    };

    systemd.services.miniflux-remove-youtube = {
      serviceConfig = {
        ExecStart = "${pkgs.miniflux-remove-youtube}/bin/miniflux-remove-youtube ${cfg.miniflux-url} ${cfg.tokenfile-path}";
        Type = "oneshot";
        User = "miniflux-remove-youtube";
      };
    };
  };
}
