{
  description = "A simple CLI for inspecting, merging and editing libime pinyin histories from multiple machines.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        lib = nixpkgs.lib;
      in
      {
        packages = rec {
          default = libime-history-merge;
          libime-history-merge = pkgs.rustPlatform.buildRustPackage {
            pname = "libime-history-merge";
            version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
            meta = {
              homepage = "https://github.com/blurgyy/libime-history-merge";
              description = "A simple CLI for inspecting, merging and editing libime pinyin histories from multiple machines.";
              license = lib.licenses.lgpl21;
            };
          };
        };
        devShells = rec {
          default = libime-history-merge;
          libime-history-merge = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustc
              cargo
              cargo-edit
              rust-analyzer
              rustfmt
            ];
            shellHook = ''
              SHELL=$(grep "$USER:" /etc/passwd | awk -F: '{ print $NF }')
              [[ $- == *i* ]] && exec $SHELL
            '';
          };
        };
      }
    )
    // {
      hydraJobs = self.packages;
    };
}
