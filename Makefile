all: css js

css:
	tailwindcss -i tailwind.css -o style.css --minify

js:
	tsc -p .
	esbuild main.ts --minify --target=es2022 --outfile=main.js

.PHONY: all css js
