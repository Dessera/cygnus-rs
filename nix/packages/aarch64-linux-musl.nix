{
  lib,
  libiconv,
  openssl,
  pkg-config,
  craneLib,
  stdenv,
}:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource ./.;

  nativeBuildInputs =
    [
      pkg-config
      stdenv.cc
    ]
    ++ lib.optionals stdenv.buildPlatform.isDarwin [
      libiconv
    ];

  buildInputs = [
    openssl
  ];

  strictDeps = true;
  CARGO_BUILD_TARGET = "aarch64-unknown-linux-musl";
  CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";

  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";

  HOST_CC = "${stdenv.cc.nativePrefix}cc";
  TARGET_CC = "${stdenv.cc.targetPrefix}cc";
}
