.DEFAULT_GOAL := help

.PHONY: help
help: ## List targets in this Makefile
	@awk 'BEGIN { FS = ":|:.*?## "; OFS="\t" }; /^[0-9a-zA-Z_-]+?:/ { print $$1, $$2 }' $(MAKEFILE_LIST) \
		| column --separator $$'\t' --table --table-wrap 2 --output-separator '    ' \
		| sort --dictionary-order

.PHONY: build
build: ## Build app in development mode
	cargo make build-development

.PHONY: clean
release: ## Remove output files
	rm -rf dist

.PHONY: release
release: clean ## Build app in release mode
	cargo build --release
	npm run build

.PHONY: watch
watch: ## Run a development server, automatically rebuilding on file changes
	cargo make watch
