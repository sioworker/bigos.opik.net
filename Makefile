all: css wasm

css:
	tailwindcss -i tailwind.css -o style.css --minify

wasm:
	cargo build -q --release --target wasm32-unknown-unknown
	wasm-bindgen --target web --no-typescript --out-dir pkg target/wasm32-unknown-unknown/release/bigos.wasm
	esbuild pkg/bigos.js --minify --format=esm --allow-overwrite --outfile=pkg/bigos.js --log-level=warning

site: all
	rm -rf _site && mkdir _site && cp -r index.html 404.html init.js style.css favicon.svg me.jpg CNAME pkg _site/

serve: site
	cd _site && python3 -m http.server -b 127.0.0.1 8099

.PHONY: all css wasm site serve
