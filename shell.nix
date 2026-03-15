{
  pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  },
}:
let
  rustToolchain = pkgs.rust-bin.stable.latest.complete;
in
pkgs.mkShell {
  packages = with pkgs; [
    rustToolchain
    qemu-user
  ];
}
