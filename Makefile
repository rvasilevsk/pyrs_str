dev:
	uv run maturin develop
b:
	uv run maturin build
t:
	uv run pytest ./tests
ct:
	cargo test
ctd:
	cargo test --doc
doc_:
	cargo doc
doc:
	cargo doc --open
l:
	uv run jupyter lab ./lab --port=9999
