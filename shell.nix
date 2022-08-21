{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    nativeBuildInputs = [pkgs.pkgconfig];
    buildInputs = with pkgs; [
      alsaLib
      udev
      llvmPackages_latest.llvm
      llvmPackages_latest.bintools
      zlib.out
      rustup
      xorriso
      grub2
      qemu
      llvmPackages_latest.lld
      python3
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      glfw
      wayland
      vulkan-tools
      vulkan-headers
      vulkan-loader
      vulkan-validation-layers
    ];
    # Avoid polluting home dir with local project stuff.
    RUSTUP_HOME = toString ./.rustup;
    CARGO_HOME = toString ./.cargo;
    RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    LIBCLANG_PATH= pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
    HISTFILE=toString ./.history;
    shellHook = ''
      export PATH=$PATH:${CARGO_HOME}/bin
      export PATH=$PATH:${RUSTUP_HOME}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
      '';
    # Add libvmi precompiled library to rustc search path
    RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
      pkgs.libvmi
    ]);
    # Add libvmi, glibc, clang, glib headers to bindgen search path
    BINDGEN_EXTRA_CLANG_ARGS =
    # Includes with normal include path
    (builtins.map (a: ''-I"${a}/include"'') [
      pkgs.libvmi
      pkgs.glibc.dev
    ])
    # Includes with special directory paths
    ++ [
      ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
    ];
  LD_LIBRARY_PATH = with pkgs.xlibs; "${pkgs.mesa}/lib:${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXcursor}/lib:${libXxf86vm}/lib:${pkgs.xorg.libXi}/lib:${pkgs.xorg.libXrandr}/lib:${pkgs.vulkan-loader}/lib";

  }
