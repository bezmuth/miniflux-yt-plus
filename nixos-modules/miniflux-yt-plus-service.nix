{
  config,
  pkgs,
  lib ? pkgs.lib,
  ...
}:
with lib; let
  cfg = config.services.miniflux-yt-plus;
in {
  options = {
    services.miniflux-yt-plus = rec {
      enable = mkOption {
        type = types.bool;
        default = false;
        description = ''
          Whether to run miniflux-yt-plus service
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
      remove-shorts = mkOption {
        type = types.bool;
        default = true;
        description = ''
          Remove shorts or not
        '';
      };
      remove-livestreams = mkOption {
        type = types.bool;
        default = false;
        description = ''
          Remove livestreams or not
        '';
      };

    };
  };

  config = mkIf cfg.enable {
    users.extraGroups.miniflux-yt-plus = {};
    users.extraUsers.miniflux-yt-plus = {
      description = "miniflux-yt-plus";
      group = "miniflux-yt-plus";
      isSystemUser = true;
      useDefaultShell = true;
    };

    environment.systemPackages = [pkgs.miniflux-yt-plus];

    systemd.timers."miniflux-yt-plus" = {
      wantedBy = ["timers.target"];
      timerConfig = {
        OnBootSec = "5m";
        OnUnitActiveSec = "5m";
        Unit = "miniflux-yt-plus.service";
      };
    };

    systemd.services.miniflux-yt-plus = {
      serviceConfig = {
        ExecStart = "${pkgs.miniflux-yt-plus}/bin/miniflux-yt-plus ${cfg.miniflux-url} ${cfg.tokenfile-path} ${if cfg.remove-shorts then "-s" else ""} ${if cfg.remove-livestreams then "-l" else ""}";
        Type = "oneshot";
        User = "miniflux-yt-plus";
      };
    };
  };
}
