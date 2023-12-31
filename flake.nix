{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        # Fix duplicate instances of these inputs by pointing them to my inputs
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Read from `rust-toolchain.toml` instead of adding `rust-bin.nightly.latest.default` to devShell `buildInputs`

        # Libraries that are mostly needed for tauri to work
        libraries = with pkgs; [
        ];

        packages = with pkgs; [
        ];

        # Inputs needed at compile-time
        nativeBuildInputs = with pkgs; [ rustToolchain ];
        # Inputs needed at runtime
        buildInputs = with pkgs; [ ] ++ packages ++ libraries;
      in
      {
        # packages.default = derivation {
        #   inherit system;
        #   name = "simple";
        #   builder = with pkgs; "${bash}/bin/bash";
        #   args = [ "-c" "echo foo > $out" ];
        #   src = ./.;
        # };

        devShells = {
          default = pkgs.mkShell {
            # inherit buildInputs nativeBuildInputs;
            # buildInputs = packages;

            shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}
            '';
          };

          device = let 
            rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./device/rust-toolchain.toml;
          in pkgs.mkShell {
            nativeBuildInputs = [ rustToolchain ];
            buildInputs = with pkgs; [
              # Working with RP Pico directly via USB
              picotool

              # Using debugprobe
              openocd-rp2040
              gdb
              minicom

              # Cargo stuff
              flip-link # A linker that protects against stack overflows
              probe-rs
            ];
          };
        };
      });
}
