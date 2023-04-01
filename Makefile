docs:
	@cargo doc --no-deps
.PHONY: docs

docs-dev:
	@cargo doc --no-deps
	@echo "Crate documentation: http://localhost:8787/hyper_middleware"
	@static-web-server -p 8787 -d target/doc/ \
		& watchman-make -p 'src/**/*.rs' --run 'cargo doc'
.PHONY: docs-dev
