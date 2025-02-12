{overlays}: {
  miniflux-yt-plus = import ./miniflux-yt-plus-service.nix;

  overlayNixpkgsForThisInstance = {pkgs, ...}: {
    nixpkgs = {
      inherit overlays;
    };
  };
}
