{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    parts,
    nci,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [nci.flakeModule];
      perSystem = {
        config,
        pkgs,
        ...
      }: let
        outputs = config.nci.outputs;
        projectName = "capnproto-rust";      
      in {
        nci = {
          projects.${projectName} = {
            path = ./.;
            export = true;
          };
          crates = {
            "capnp" = {};
            "capnpc" = {};
            "capnpc-test" = {export = false;};
            "capnpc-test-edition-2015" = {export = false;};
            "capnpc-test-edition-2018" = {export = false;};
            "capnpc-test-edition-2021" = {export = false;};
            "capnp-futures" = {};
            "capnp-futures-test" = {export = false;};
            "capnp-rpc" = {};
            "capnp-rpc-test" = {export = false;};

            "async-byte-channel" = {export = false;};
            "benchmark" = {export = false;};
            "addressbook" = {export = false;};
            "addressbook_send" = {export = false;};
            "calculator" = {export = false;};
            "external-crate" = {export = false;};
            "fill_random_values" = {export = false;};
            "hello-world" = {export = false;};
            "pubsub" = {export = false;};
          };
        };
        devShells.default = outputs.${projectName}.devShell;
        packages = {
          "capnp" = outputs."capnp".packages.release;
          "capnpc" = outputs."capnpc".packages.release;
          "capnp-futures" = outputs."capnp-futures".packages.release;
          "capnp-rpc" = outputs."capnp-rpc".packages.release;
        };
      };
    };
}
