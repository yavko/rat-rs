{
  self,
  lib,
}: {
  rat-rs = final: _: {rat-rs = final.callPackage ./package.nix {};};

  default = self.overlays.rat-rs;
}
