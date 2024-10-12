{ fenix, system }:
with fenix.packages.${system};
combine [
  minimal.rustc
  minimal.cargo
  targets.x86_64-pc-windows-gnu.stable.rust-std
]
