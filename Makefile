.DEFAULT_GOAL := help

.PHONY: help
help: ## List targets in this Makefile
	@awk 'BEGIN { FS = ":|:.*?## "; OFS="\t" }; /^[0-9a-zA-Z_-]+?:/ { print $$1, $$2 }' $(MAKEFILE_LIST) \
		| column --separator $$'\t' --table --table-wrap 2 --output-separator '    ' \
		| sort --dictionary-order

.PHONY: clean
clean: ## Remove output files
	rm -rf dist

.PHONY: release
release: clean ## Build app in release mode
	npm install
	npm run build

.PHONY: watch
watch: ## Run a development server, automatically rebuilding on file changes
	npm run serve
