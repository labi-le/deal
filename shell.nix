{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustup
    cargo
    pkg-config
  ];

  buildInputs = with pkgs; [
    libarchive
    openssl.dev
    cargo-release
  ];
}
