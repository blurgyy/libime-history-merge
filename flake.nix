{
  description = "Merge fcitx5 histories from multiple machines";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    hsz = {
      url = "gitlab:highsunz/flames";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, hsz, ... }: flake-utils.lib.eachSystem ["x86_64-linux" "aarch64-linux"] (system: let
    pkgs = import nixpkgs { inherit system; };
    lib = nixpkgs.lib;
  in {
    packages = rec {
      default = libime-history-merge;
      libime-history-merge = pkgs.rustPlatform.buildRustPackage {
        pname = "libime-history-merge";
        version = "0.2.0";
        src = lib.cleanSource ./.;
        cargoLock.lockFile = ./Cargo.lock;
        meta = {
          homepage = "https://github.com/blurgyy/libime-history-merge";
          description = "Merge fcitx5 histories from multiple machines";
          license = lib.licenses.lgpl21;
        };
      };
    };
    devShells = rec {
      default = libime-history-merge;
      libime-history-merge = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          cargo-edit
          rust-analyzer
          rustfmt
        ];
        shellHook = ''
          source ${hsz.packages.${system}.common-shell-hook}
        '';
      };
    };
  }) // {
    hydraJobs = self.packages;
  };
}
