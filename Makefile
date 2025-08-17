build:
	nix-shell shell.nix --run "cargo build --release"

build-windows:
	nix-shell shell-windows.nix --run "cargo build --target=x86_64-pc-windows-gnu"

.PHONY: dump
dump:
	@{ \
	   echo "=== START PROJECT CODE DUMP ==="; \
	   echo ""; \
	   echo "=== PROJECT TREE ==="; \
	   nix run "nixpkgs#tree" -- . -I target || echo "(tree failed)"; \
	   echo ""; \
	   find . -path ./target -prune -o -type f \( \
	      -name "*.rs" -o \
	      -name "*.nix" -o \
	      -name "Makefile" \
	   \) -print | sort | while read file; do \
	      echo "=== FILE: $$file ==="; \
	      echo "=== START CODE ==="; \
	      cat "$$file"; \
	      echo "=== END CODE ==="; \
	      echo ""; \
	   done; \
	} | wl-copy