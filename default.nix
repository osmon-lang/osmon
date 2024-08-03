{ pkgs ? import <nixpkgs> { } }:
let
  lib = pkgs.lib;
  getLibFolder = pkg: "${pkg}/lib";
  libiconvPath = "${pkgs.libiconv}/lib";
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  buildInputs = [
    pkgs.gcc
    pkgs.libiconv
    pkgs.libgccjit
    pkgs.llvmPackages.llvm
    pkgs.llvmPackages.clang
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    (getLibFolder pkgs.gcc)
    (getLibFolder pkgs.llvmPackages.llvm)
    libiconvPath
  ];

  NIX_LDFLAGS = "-L${libiconvPath} -L${./lib}";

  meta = with lib; {
    homepage = "https://osmon-lang.uz";
    description = ''
      Highly experimental programming language developed by @orzklv.";
    '';
    licencse = licenses.mit;
    platforms = with platforms; linux ++ darwin;

    maintainers = [
      {
        name = "Sokhibjon Orzikulov";
        email = "sakhib@orzklv.uz";
        handle = "orzklv";
        github = "orzklv";
        githubId = 54666588;
        keys = [{
          fingerprint = "00D2 7BC6 8707 0683 FBB9  137C 3C35 D3AF 0DA1 D6A8";
        }];
      }
    ];
  };
}
