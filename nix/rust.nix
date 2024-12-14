{
  pkgs,
  crane,
  fenix,
}:
let
  inherit (pkgs) system;
in
rec {
  toolchain = {
    default = fenix.packages.${system}.stable.withComponents [
      "cargo"
      "rustc"
      "rust-src"
      "rust-analyzer"
    ];
  };

  rslib = {
    default = (crane.mkLib pkgs).overrideToolchain toolchain.default;
  };
}
