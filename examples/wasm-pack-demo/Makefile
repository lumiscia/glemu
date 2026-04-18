.PHONY: build
build:
	wasm-pack build --target web --dev --out-dir web/pkg

.PHONY: build_release
build_release:
	wasm-pack build --target web --release --out-dir web/pkg

.PHONY: serve
serve:
	python3 -m http.server --directory web 8001
