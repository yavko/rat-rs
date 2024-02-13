{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # <https://github.com/nix-systems/nix-systems>
    systems.url = "github:nix-systems/default-linux";
  };

  outputs = {
    self,
    nixpkgs,
    systems,
  }: let
    inherit (nixpkgs) lib;
    eachSystem = lib.genAttrs (import systems);
    pkgsFor = eachSystem (system:
      import nixpkgs {
        localSystem = system;
        overlays = [self.overlays.rat-rs];
      });
  in {
    overlays = import ./nix/overlays.nix {inherit self lib;};

    packages = eachSystem (system: {
      default = self.packages.${system}.rat-rs;
      inherit (pkgsFor.${system}) rat-rs;
    });

    # for `nix fmt`
    formatter = eachSystem (system: pkgsFor.${system}.alejandra);

    devShells = eachSystem (system: {
      default = with pkgsFor.${system};
        mkShell {
          # dependencies needed for `cargo build`
          buildInputs =
            [
              cargo
            ]
            ++ self.packages.${system}.rat-rs.buildInputs;
        };
    });
  };
}
