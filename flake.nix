{
  description = "Composable exact number checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      forAllSystems =
        function:
        nixpkgs.lib.genAttrs systems (
          system:
          function {
            pkgs = import nixpkgs { inherit system; };
          }
        );
    in
    {
      apps = forAllSystems (
        { pkgs }:
        let
          check = pkgs.writeShellApplication {
            name = "number-check";
            runtimeInputs = [
              pkgs.cargo-hack
              pkgs.rustc
              pkgs.cargo
              pkgs.clippy
              pkgs.rustfmt
            ];
            text = ''
              cargo number-fmt
              cargo number-clippy
              cargo number-test
              cargo number-example
              cargo number-powerset
            '';
          };
        in
        {
          check = {
            type = "app";
            program = "${check}/bin/number-check";
          };
          default = {
            type = "app";
            program = "${check}/bin/number-check";
          };
        }
      );

      devShells = forAllSystems (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo-hack
              pkgs.rustc
              pkgs.cargo
              pkgs.clippy
              pkgs.rustfmt
            ];
          };
        }
      );

      formatter = forAllSystems ({ pkgs }: pkgs.nixfmt);
    };
}
