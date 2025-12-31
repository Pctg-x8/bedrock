{
  description = "bedrock devenv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };
  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      deps = [
        pkgs.rustup
        pkgs.vulkan-loader
        # required for workflow generator(also included in githooks)
        pkgs.stack
        # debugging
        pkgs.vulkan-validation-layers
        # git hook
        rusty-hook
      ];
      rusty-hook = pkgs.rustPlatform.buildRustPackage rec {
        pname = "rusty-hook";
        version = "0.11.2";
        cargoHash = "sha256-HC+1Cs2BeIPHuuGxcFEB8GqcyrrUEYcSM+KgE/INxIw=";
        src = pkgs.fetchFromGitHub {
          owner = "swellaby";
          repo = pname;
          rev = "30162426ec354d70ffdb91d1fe93549a58770c3d"; # master at 2025/12/31
          hash = "sha256-enqEsI0TSazVpIP9Awt/ZWjbxE6j1zzccggLF4SF358=";
        };
      };
    in
    {
      devShells."${system}" = {
        default = pkgs.mkShell {
          buildInputs = deps;
        };
        fish =
          let
            fishPrehook = pkgs.writeScriptBin "startup" ''
              # prepend devenv prompt
              functions -c fish_prompt __fish_prompt_org
              function fish_prompt
                # preserve status code
                set -l last_status $status
                printf "[Bedrock] "
                echo "exit $last_status" | .
                __fish_prompt_org
              end
            '';
          in
          pkgs.mkShell {
            buildInputs = deps ++ [ pkgs.fish ];
            shellHook = ''
              exec ${pkgs.fish.outPath}/bin/fish -C "source ${fishPrehook}/bin/startup"
            '';
          };
      };
    };
}
