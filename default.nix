{ pkgs ? import <nixpkgs> { }
, naersk ? pkgs.callPackage
    (pkgs.fetchFromGitHub {
      owner = "nmattia";
      repo = "naersk";
      rev = "d5a23213d561893cebdf0d251502430334673036";
      sha256 = "0ifvqv3vjg80hhgxr7b22i22gh2gxw0gm5iijd9r7y4qd7n2yrcp";
    })
    { }
}:
let
  stdenv = pkgs.stdenv;
  binaries = naersk.buildPackage {
    name = "onion-or-news";
    src = stdenv.lib.sourceFilesBySuffices ./. [
      "Cargo.lock"
      "Cargo.toml"
      ".rs"
      ".sql"
    ];
    nativeBuildInputs = with pkgs; [ openssl pkgconfig postgresql.lib ];
  };
  static = stdenv.mkDerivation {
    name = "onion-or-news-static";
    src = stdenv.lib.sourceFilesBySuffices ./. [ ".html" ".js" ];
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
