{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };

      package = pkgs.rustPlatform.buildRustPackage {
        pname = "myaoc";
        version = "2024-day1-v1";

        src = ./.;

        cargoHash = "sha256-R1vq8gEf99ACamqWKlJ3pqLqt6enABN7cvJtKkDlFL4=";
      };
    in
    {
      packages.x86_64-linux.default = package;
      
      devShells.x86_64-linux.default = pkgs.mkShell {
        
        packages  = with pkgs; [ rustfmt clippy ];
        
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        inputsFrom = [
          package
        ];
      };
    };
}