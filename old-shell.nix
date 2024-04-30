let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = "latest";
 # rustVersion = "1.76.0";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
    ];
  };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.openssl
    pkgs.pkg-config
    # other dependencies...
  ];

  RUST_BACKTRACE = 1;

  # Set the OPENSSL_DIR environment variable
  #OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_DIR = "${pkgs.openssl.out}";
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
}
