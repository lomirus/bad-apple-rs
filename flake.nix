{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
    in {
      packages = forAllSystems (system: let
        pkgs = pkgsFor.${system};
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in {
        default = pkgs.rustPlatform.buildRustPackage rec {
          pname = manifest.name;
          version = manifest.version;

          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
      });
    };
}
