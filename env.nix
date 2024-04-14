{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.openssl
    pkgs.pkg-config
    pkgs.rustup
    pkgs.postgresql
    pkgs.diesel-cli
  ];

  shellHook = ''
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
    export OPENSSL_INCLUDE_DIR="${pkgs.openssl.out}/include"
    export PKG_CONFIG_PATH="${pkgs.openssl.out}/lib/pkg-config"
  '';
}
