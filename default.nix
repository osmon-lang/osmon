{ pkgs ? import <nixpkgs> { } }:
let
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
}
