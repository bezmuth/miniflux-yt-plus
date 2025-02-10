{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "miniflux-remove-youtube";
  version = "0.1";
  cargoLock.lockFile = ./Cargo.lock;
  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.installShellFiles
    pkgs.makeWrapper
    pkgs.openssl
  ];
  src = pkgs.lib.cleanSource ./.;
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
