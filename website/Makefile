build-dev:
	npm install && wasm-pack build --debug

publish:
	npm install && wasm-pack build --release
	du -k ./pkg
	npm publish

build:
	npm run webpack && du -h ./docs

local:
	npm run serve
