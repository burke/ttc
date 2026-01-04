{
  description = "ttc - Token count CLI tool for counting tokens in files";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          ttc = pkgs.rustPlatform.buildRustPackage {
            pname = "ttc";
            version = "1.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            meta = with pkgs.lib; {
              description = "Token count - like wc -l but for tokens";
              homepage = "https://github.com/burke/ttc";
              license = licenses.mit;
              maintainers = [ ];
              mainProgram = "ttc";
            };
          };

          default = self.packages.${system}.ttc;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      });
}
