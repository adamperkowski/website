{ pkgs ? import <nixpkgs> {} }:

let
  pkgInputs = with pkgs; [
    just dart-sass jq cargo cargo-watch cargo-shuttle
  ];
in
pkgs.mkShell {
  packages = pkgInputs;

  shellHook = ''
    echo -ne "-----------------------------------\n "

    echo -n "${toString (map (pkg: "• ${pkg.name}\n") pkgInputs)}"

    echo "-----------------------------------"
  '';
}
