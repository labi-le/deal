# deal

Ever had to unpack an archive and found yourself googling the flags for tar.gz for the tenth time, only to forget them a moment later? Me too\
deal provides one command to handle any archive, so you can finally stop googling "how to unpack X-format"

### Dependencies
see flake.nix

### Installation

- [Prebuilt binaries](https://github.com/labi-le/deal/releases)
- Nix flake
  <details> <summary>as profile</summary>

  ```sh
  nix profile install github:labi-le/deal
  ```
  </details>
  <details>
  <summary>import the module</summary>

  ```nix
  {
    # inputs
    deal.url = "github:labi-le/deal";
    # outputs
    overlay-deal = final: prev: {
      deal = deal.packages.${system}.default;
    };
  
    modules = [
      ({ config, pkgs, ... }: { nixpkgs.overlays = [ overlay-deal ]; })
    ];
  
    # add package
    environment.systemPackages = with pkgs; [
      deal
    ];
  }
  ```
  </details>

#### Build from source

see shell.nix for details
- rustc 1.89.0 (29483883e 2025-08-04)
- git
- makefile
- libarchive
- openssl.dev
- pkg-config

```sh
git clone https://github.com/labi-le/deal.git
cd deal
make build
```

### Usage

```
deal https://www.bamsoftware.com/hacks/zipbomb/zbsm.zip
deal https://www.bamsoftware.com/hacks/zipbomb/zbsm.zip -d # create a folder by file name
deal https://www.bamsoftware.com/hacks/zipbomb/zbsm.zip some_dir # create a folder using the conveyed name

# the same applies to local files
deal artifacts-build-local-x86_64-unknown-linux-gnu.zip 
deal artifacts-build-local-x86_64-unknown-linux-gnu.zip -d 
deal artifacts-build-local-x86_64-unknown-linux-gnu.zip some_dir 
```

### Todo

[ ] Add tests

