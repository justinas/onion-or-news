{ pkgs ? import <nixpkgs> { }
, lib ? pkgs.lib
, naersk ? pkgs.callPackage
    (builtins.fetchTarball {
      url = "https://github.com/nix-community/naersk/archive/c3e56b8a4ffb6d906cdfcfee034581f9a8ece571.tar.gz";
      sha256 = "0mq4jqvvqmy35bapybsqqpngy0r6j43n3pzm1y75bbfnyq5f4gab";
    })
    { }
}:
let
  stdenv = pkgs.stdenv;
  binaries = naersk.buildPackage {
    name = "onion-or-news";
    src = lib.sourceFilesBySuffices ./. [
      "Cargo.lock"
      "Cargo.toml"
      ".rs"
      ".sql"
    ];
    nativeBuildInputs = with pkgs; [ openssl pkg-config postgresql.lib ];
  };
  static = stdenv.mkDerivation {
    name = "onion-or-news-static";
    src = lib.sourceFilesBySuffices ./. [ ".html" ".js" ];
    phases = [ "unpackPhase" "installPhase" ];
    installPhase = ''
      mkdir -p $out/share/static
      cp -r oon_web/static/. $out/share/static
    '';
  };
in
pkgs.symlinkJoin {
  name = "onion-or-news-deploy";
  paths = [ binaries static ];
}
