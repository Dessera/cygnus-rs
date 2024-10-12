{ fenix, system }:
fenix.packages.${system}.stable.withComponents [
  "cargo"
  "rustc"
  "rust-src"
  "rust-analyzer"
]
