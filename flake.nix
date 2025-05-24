{
  inputs = {
    teleia.url = "github:lcolonq/teleia";
    nixpkgs.follows = "teleia/nixpkgs";
  };

  outputs = inputs@{ self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      oub = inputs.teleia.native.build ./. "ranch";
    in {
      packages.${system} = {
        default = ranch;
        inherit ranch;
      };
      applications.${system}.default = {
        type = "app";
        program = "${ranch}/bin/ranch";
      };
    };
}
