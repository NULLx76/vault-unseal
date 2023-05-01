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
              cargoSha256 =
                "sha256-eOvTR7TpFpi83J3G8HPXgOBryTzkq4XWp6CER6UDCbo=";
            };
        };
      });
}
