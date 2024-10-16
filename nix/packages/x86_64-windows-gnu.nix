{
  craneLib,
  craneSrc,
  pkgsCross,
  openssl,
}:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource craneSrc;

  strictDeps = true;
  doCheck = false;

  CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";

  # fixes issues related to libring
  TARGET_CC = "${pkgsCross.mingwW64.stdenv.cc}/bin/${pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";

  #fixes issues related to openssl
  OPENSSL_DIR = "${openssl.dev}";
  OPENSSL_LIB_DIR = "${openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${openssl.dev}/include/";

  depsBuildBuild = [
    pkgsCross.mingwW64.stdenv.cc
    pkgsCross.mingwW64.windows.pthreads
  ];
}
