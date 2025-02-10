{ overlays }:

{
  miniflux-remove-youtube = import ./miniflux-remove-youtube-service.nix;

  overlayNixpkgsForThisInstance =
    { pkgs, ... }:
    {
      nixpkgs = {
        inherit overlays;
      };
    };
}
