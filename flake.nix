{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let inherit (flake-utils.lib) eachSystem system;
    in eachSystem [ system.x86_64-linux ] (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages = {
          default =
            let toml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
            in pkgs.rustPlatform.buildRustPackage {
              pname = toml.package.name;
              version = toml.package.version;
              src = self;
              cargoLock.lockFile = ./Cargo.lock;
            };
        };
      });
}
