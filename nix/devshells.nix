{ pkgs, ... }:
rs:
let
  inherit (rs) rslib;
in
{
  default = rslib.default.devShell {
    packages = with pkgs; [
      nixd
      nixfmt-rfc-style
    ];
  };
}
