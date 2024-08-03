{ pkgs ? import <nixpkgs> {} }:

let
  # Extract the lib folder from a package
  getLibFolder = pkg: "${pkg}/lib";
  libiconvPath = "${pkgs.libiconv}/lib";
in
pkgs.mkShell {
  buildInputs = [
    pkgs.gcc
    pkgs.libiconv
    pkgs.libgccjit
    pkgs.llvmPackages.llvm
    pkgs.llvmPackages.clang
  ];

  # Set the LD_LIBRARY_PATH so that the dynamic linker can find shared libraries
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    (getLibFolder pkgs.gcc)
    (getLibFolder pkgs.llvmPackages.llvm)
    libiconvPath
  ];

  NIX_LDFLAGS = "-L${libiconvPath} -L${./lib}";

  shellHook = ''
    # export DYLD_LIBRARY_PATH="${./lib}:$DYLD_LIBRARY_PATH"

    echo "Loaded development environment with libgccjit, GCC, LLVM, Clang!"
  '';
}
