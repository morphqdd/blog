{
  description = "Dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          openssl
          zlib
          fish
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.openssl
          pkgs.zlib
        ];

        shellHook = ''
          export SHELL=${pkgs.fish}/bin/fish
          echo "  - LD_LIBRARY_PATH: $LD_LIBRARY_PATH"
        '';
      };
    };
}
