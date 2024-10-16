{
  lib,
  libiconv,
  openssl,
  pkg-config,
  craneLib,
  craneSrc,
  stdenv,
}:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource craneSrc;

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
  doCheck = false;
  CARGO_BUILD_TARGET = "aarch64-unknown-linux-gnu";

  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";

  HOST_CC = "${stdenv.cc.nativePrefix}cc";
  TARGET_CC = "${stdenv.cc.targetPrefix}cc";
}
