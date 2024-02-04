
help:
	@cat Makefile

serve: bindgen.wasm http.server

http.server:
	@cd dist && python3 -m http.server 3333
bindgen.wasm: clean.dist release.wasm
	@wasm-bindgen --out-dir dist --target web ../target/wasm32-unknown-unknown/release/domik.wasm
	@cp -v assets/** dist/
clean.dist:
	@rm -rf dist
	@mkdir -p dist

run: release
	@cargo run --release

all: test release release.wasm

release:
	@cargo rustc --release -- -C prefer-dynamic
release.wasm:
	@cargo build --release --target wasm32-unknown-unknown
test:
	@cargo test
check:
	@cargo rustc -- -Awarnings

configure:
	@cargo install -f wasm-bindgen-cli

git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

clean: clean.dist
	@cargo clean
