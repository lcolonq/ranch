{
  inputs = {
    teleia.url = "github:lcolonq/teleia";
    nixpkgs.follows = "teleia/nixpkgs";
  };

  outputs = inputs@{ self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      ranch = inputs.teleia.native.build ./. "ranch_lib";
      wasm = inputs.teleia.wasm.build ./. "ranch_lib";
      wasmDeploy = inputs.teleia.wasm.buildAtUrl ./. "ranch_lib" "/ranch";
    in {
      packages.${system} = {
        default = ranch;
        inherit ranch wasm wasmDeploy;
      };
      applications.${system}.default = {
        type = "app";
        program = "${ranch}/bin/ranch";
      };
      devShells.${system} = {
        default = inputs.teleia.shell;
      };
      overlay = self: super: {
        ranch = {
          inherit wasm;
        };
      };
    };
}
