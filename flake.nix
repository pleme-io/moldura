{
  description = "Moldura — canonical pleme-io TUI app framework";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    crate2nix.url = "github:nix-community/crate2nix";
    flake-utils.url = "github:numtide/flake-utils";
    substrate = {
      url = "github:pleme-io/substrate";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crate2nix, flake-utils, substrate }:
    (import "${substrate}/lib/rust-library.nix" {
      inherit nixpkgs crate2nix flake-utils;
    }) {
      libraryName = "moldura";
      src = self;
      repo = "pleme-io/moldura";

      module = {
        description = "Moldura — canonical pleme-io TUI app framework";
        hmNamespace = "blackmatter.components";
      };
    };
}
