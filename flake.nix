{
  description = "bedrock devenv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
      ];
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
