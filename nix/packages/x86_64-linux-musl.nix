{ craneLib, craneSrc }:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource craneSrc;

  strictDeps = true;

  CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
  CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
}
