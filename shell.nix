{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    clippy # Our favorite linter
    glibc
    gnumake
    libxkbcommon
    #openssl # Needed for the reqwest library
    rustc
    rustfmt
    rust-analyzer
  ];
}
