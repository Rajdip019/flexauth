{
  description = "FlexAuth - inhouse-auth";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: 
  flake-utils.lib.eachDefaultSystem (system:
    let
      # name = "FlexAuth";
      # src = ./.;
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
            inherit system overlays;
      };
    in
    with pkgs;
        {
          devShells.default = mkShell {
            nativeBuildInputs = [
              rust-bin.stable.latest.default
              openssl_1_1
              pkg-config
            ];

            buildInputs = [
              openssl_1_1
              pkg-config 
              zlib 
            ];

            CARGO_HOME = "${pkgs.writeTextDir "cargo-home" ""}";
            OPENSSL_DIR = "${openssl_1_1}";
          };

          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "inhouse-auth";
            version = "0.1.0";
            src = ./.;

            nativeBuildInputs = [
              openssl
              pkg-config
            ];

            cargoHash = "sha256-wZ5q6Ghkr9/14cLZqASlPZZI4pktxmHy0BxOmzIbrMM=";
          };
        }
  );
}
