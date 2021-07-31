.DEFAULT_GOAL := help

.PHONY: help
help: ## List targets in this Makefile
	@awk '\
		BEGIN { FS = ":$$|:[^#]+|:.*?## "; OFS="\t" }; \
		/^[0-9a-zA-Z_-]+?:/ { print $$1, $$2 } \
	' $(MAKEFILE_LIST) \
		| sort --dictionary-order \
		| column --separator $$'\t' --table --table-wrap 2 --output-separator '    '

.PHONY: clean
clean: ## Remove output files
	rm -rf dist

.PHONY: release
release: clean ## Build app in release mode
	npm install
	npm run build

.PHONY: watch
watch: ## Run both development servers, automatically rebuilding on file changes
	docker-compose up -d
	parallel --line-buffer --tagstring '[{}]' --verbose make ::: \
		watch-server \
		watch-web \
		| perl -pe 's/\t/ /'

.PHONY: watch-server
watch-server: ## Run backend server, automatically rebuilding on file changes
	cargo watch --exec run --workdir trellis_server

.PHONY: watch-web
watch-web: ## Run frontend server, automatically rebuilding on file changes
	npm run serve
