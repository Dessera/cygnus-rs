{ fenix, system }:
with fenix.packages.${system};
combine [
  minimal.rustc
  minimal.cargo
  targets.x86_64-pc-windows-gnu.latest.rust-std
  targets.x86_64-unknown-linux-musl.latest.rust-std
  targets.aarch64-unknown-linux-musl.latest.rust-std
]
