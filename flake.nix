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
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "vault-unseal";
            version = "0.1.0";
            src = self;
            cargoSha256 = "sha256-nCOHQU62fzJ9uwUK8n5JsVkKmqQwhG/5GI6rvtejZjY=";
          };
        };
      });
}
