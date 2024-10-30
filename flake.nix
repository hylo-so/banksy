{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.05";
    rust-overlay.url =
      "github:oxalica/rust-overlay/7df2ac544c203d21b63aac23bfaec7f9b919a733";
  };
  outputs = { self, nixpkgs, rust-overlay }:
    with import nixpkgs {
      system = "aarch64-darwin";
      overlays = [ (import rust-overlay) ];
    }; {
      devShell.aarch64-darwin = mkShell {
        buildInputs = [
          rust-bin.stable.latest.default
          rust-analyzer
          nodePackages.pnpm
          nodePackages.typescript
          nodePackages.typescript-language-server
        ];
      };
    };
}
