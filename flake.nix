{
  description = "Belphegor clipboard manager flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      version = "0.1.1";
      pname = "deal";
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];

      systemConfigs = {
        x86_64-linux = {
          url = "https://github.com/labi-le/deal/releases/download/v${version}/deal-x86_64-unknown-linux-gnu.tar.xz";
          hash = "sha256-AnU497Kyo5XwO2KXfZ3ibzNwivcMoAVl/1UXEIr6zco="; # x86_64-linux
        };
        aarch64-linux = {
          url = "https://github.com/labi-le/deal/releases/download/v${version}/deal-aarch64-unknown-linux-gnu.tar.xz";
          hash = "sha256-zdopsNTB/BLPNo7nB82bA68Q2MopOdLQuqv1kfLcMKM="; # aarch64-linux
        };
      };
    in
    flake-utils.lib.eachSystem supportedSystems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        config = systemConfigs.${system};
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          inherit pname version;

          src = pkgs.fetchurl {
            url = config.url;
            hash = config.hash;
          };

          nativeBuildInputs = [ pkgs.autoPatchelfHook ];
          buildInputs = [ pkgs.stdenv.cc.cc.lib pkgs.xz ];

          installPhase = ''
            mkdir -p $out/bin
            install -m 755 deal $out/bin/${pname}
          '';

          meta = with pkgs.lib; {
            description = "";
            homepage = "https://github.com/labi-le/deal";
            license = licenses.mit;
            platforms = supportedSystems;
          };
        };
      }
    );
}