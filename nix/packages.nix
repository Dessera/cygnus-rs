_: rs:
let
  inherit (rs) rslib;
  src = rslib.default.cleanCargoSource ../.;
in
{
  default = rslib.default.buildPackage {
    inherit src;
    strictDeps = true;
  };
}
