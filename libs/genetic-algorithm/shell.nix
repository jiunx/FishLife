let 
    pkgs = imort <nixpkgs> {};

in
    pkgs.mkShell {
        buildInputs = with pkgs; [
            nodeJs
            wasm-pack
        ];
    }