# Automated service to remove youtube shorts from miniflux
A simple (slightly scuffed) NixOS module to periodically (every 5 minutes) mark youtube shorts and livestreams as read.

Short detection is done by making a request to https://www.youtube.com/shorts/<videoid> and checking for a redirect. Youtube shorts do not redirect.
## Installation (NixOS)
Add this repo to your flake inputs and ensure the module is loaded:
```nix
{
  inputs = {
    # ...
    miniflux-yt-plus.url = "github:bezmuth/miniflux-yt-plus";
  };
  outputs = { miniflux-yt-plus, ... }: {
    nixosConfigurations.<host> = nixpkgs.lib.nixosSystem {
      modules = [
        miniflux-yt-plus.nixosModules.miniflux-yt-plus
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
    owner = "miniflux-yt-plus";
  };
  services = {
    miniflux-yt-plus = {
      enable = true;
      miniflux-url = "http://localhost:8080/";
      tokenfile-path = config.age.secrets.miniflux-token.path;
      remove-shorts = true;
      remove-livestreams = true;
    };
  };
```
Ensure that whatever file you store the token in can be read by the "miniflux-yt-plus" user.

You can find my NixOS config using this module here: https://github.com/bezmuth/nix-config/blob/822cf2e6b2e6672f441c4a31a36aa3ff223545f9/modules/miniflux/default.nix
## FAQ
### "I don't use NixOS!"
This is just a rust program, clone this repo and you should be able to run `cargo install`. Run `miniflux-yt-plus --help` to see all the arguemnts and then create a cron job.
### "*X* thing doesn't work!"
This isn't really "production ready" code, there's very little error handling outside of a `panic!()`. If you run into any issues you can't fix yourself just file an issue.
### "My entire business imploded because of your bad code!"
womp womp
