.DEFAULT_GOAL := help

.PHONY: help
help: ## List targets in this Makefile
	@awk '\
		BEGIN { FS = ":$$|:[^#]+|:.*?## "; OFS="\t" }; \
		/^[0-9a-zA-Z_-]+?:/ { print $$1, $$2 } \
	' $(MAKEFILE_LIST) \
		| sort --dictionary-order \
		| column --separator $$'\t' --table --table-wrap 2 --output-separator '    '

.PHONY: deps
deps: ## Install prerequisites and development tools
	cargo install diesel_cli --version 1.4.1

.PHONY: clean
clean: ## Remove output files
	rm -rf dist

.PHONY: server
server: ## Build server in release mode
	cargo build --release --bin trellis_server

.PHONY: web
web: clean ## Build webapp in release mode
	npm ci
	npm run build

.PHONY: migrate
migrate: ## Run database migrations
	diesel migration run --locked-schema

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

.PHONY: app-manifest
app-spec: .do/app.yaml ## Generate the production app spec

.do/app.yaml: .do/app.template.yaml
	@echo 'Checking that environment variables are set:'
	printenv ROCKET_SECRET_KEY_PRODUCTION
	> .do/app.yaml \
		sed \
		"s/__ROCKET_SECRET_KEY_PRODUCTION__/$${ROCKET_SECRET_KEY_PRODUCTION}/g" \
		.do/app.template.yaml
