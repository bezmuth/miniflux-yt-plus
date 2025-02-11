# Automated service to remove youtube shorts from miniflux
A simple (slightly scuffed) NixOS module to periodically (every 5 minutes) mark youtube shorts as read. This applies to both shorts and livestreams. 

Short detection is done by making a request to https://www.youtube.com/shorts/<videoid> and checking for a redirect. Youtube shorts do not redirect.
## Installation (NixOS)
Add this repo to your flake inputs and ensure the module is loaded:
```nix
{
  inputs = {
    # ...
    miniflux-remove-youtube.url = "github:bezmuth/miniflux-remove-yt-shorts";
  };
  outputs = { miniflux-remove-youtube, ... }: {
    nixosConfigurations.<host> = nixpkgs.lib.nixosSystem {
      modules = [
        miniflux-remove-youtube.nixosModules.miniflux-remove-youtube
        ./configuration.nix
      ];
    };
  };
}
```
And then configure the service, here is an example config using [Agenix](https://github.com/ryantm/agenix) to store the token:
```nix
  age.secrets.miniflux-token = {
    file = miniflux-token.age;
    owner = "miniflux-remove-youtube";
  };
  services = {
    miniflux-remove-youtube = {
      enable = true;
      miniflux-url = "http://localhost:8080/";
      tokenfile-path = config.age.secrets.miniflux-token.path;
    };
  };
```
Ensure that whatever file you store the token in can be read by the "miniflux-remove-youtube" user.

You can find my NixOS config using this module here: https://github.com/bezmuth/nix-config/blob/822cf2e6b2e6672f441c4a31a36aa3ff223545f9/modules/miniflux/default.nix
## FAQ
### "I don't use NixOS!"
This is just a rust program, clone this repo and you should be able to run `cargo install`. The first argument to the program is your miniflux url and the second is a path to file containing the api token. Create a cron job and you're off to the races.
### "*X* thing doesn't work!"
This isn't really "production ready" code, there's very little error handling outside of a `panic!()`. If you run into any issues you can't fix yourself just file an issue.
### "My entire business imploded because of your bad code!"
womp womp
