{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/22.04";
  };

  outputs = { nixpkgs }:
      with nixpkgs; {
        devShell = mkShell {
          buildInputs=[
            nodePackages.pnpm
            nodePackages.typescript
            nodePackages.typescript-language-server
          ];
        };
      };
}
