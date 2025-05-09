dev:
	pnpm tauri dev
install:
	pnpm install

clean:
	@echo "Cleaning Rust build artifacts in src-tauri..."
	(cd src-tauri && cargo clean)
	@echo "Removing Rust lock file in src-tauri..."
	rm -f src-tauri/Cargo.lock
	@echo "Removing node_modules from project root..."
	rm -rf node_modules
	@echo "Removing package lock files (pnpm-lock.yaml, package-lock.json, yarn.lock) from project root..."
	rm -f pnpm-lock.yaml package-lock.json yarn.lock
	@echo "Project cleaned. Reinstalling dependencies and updating Rust crates..."
	$(MAKE) install
	(cd src-tauri && cargo update && cd ..)
	@echo "Dependencies reinstalled and Rust crates updated."
