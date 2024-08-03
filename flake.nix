{
  description = "The Osmon Programming Language";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    # nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs: let
    lib = nixpkgs.lib;
    systems = [
      "aarch64-linux"
      "x86_64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
    ];

    forAllSystems = nixpkgs.lib.genAttrs systems;
    forEachSystem = f: lib.genAttrs systems (system: f pkgsFor.${system});
    pkgsFor = lib.genAttrs systems (system:
      import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      });
    devShellFor = system: let
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };
      script = import ./shell.nix {inherit pkgs;};
    in
      script;
  in {
    formatter =
      forAllSystems (system: nixpkgs.legacyPackages.${system}.nixpkgs-fmt);
    devShell = lib.mapAttrs (system: _: devShellFor system) (lib.genAttrs systems {});
  };
}
