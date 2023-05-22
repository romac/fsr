.DEFAULT_GOAL := help

REMOTE := romac.me
REMOTE_DIR := /var/www/france-schmid.ch

.PHONY: cross pull-content push-templates deploy stop start reload full help

cross: ## Cross compile the web server for Linux x86-64
	cross build --release --target x86_64-unknown-linux-gnu

pull-content: ## Pull the content from the server
	rsync -azvhe ssh ${REMOTE}:${REMOTE_DIR}/content/ content/

push-content: ## Push the content to the server
	rsync -azvhe ssh content/ ${REMOTE}:${REMOTE_DIR}/content/

push-static: ## Push the static assets to the server
	rsync -azvhe ssh templates/ ${REMOTE}:${REMOTE_DIR}/templates/
	rsync -azvhe ssh static/ ${REMOTE}:${REMOTE_DIR}/static/

deploy: ## Deploy the binary to the server
	scp target/x86_64-unknown-linux-gnu/release/fsr fsr:${REMOTE_DIR}/

stop: ## Remotely stop the webserver
	ssh -t fsr 'tmux send-keys -t fsr C-c ENTER'

start: ## Remotely start the webserver
	ssh -t fsr 'tmux send-keys -t fsr ./fsr ENTER'

reload: ## Remotely reload the webserver
	$(MAKE) stop
	$(MAKE) start

full: ## Alias for `cross`, `stop`, `deploy`, `start`
	$(MAKE) cross
	$(MAKE) stop
	$(MAKE) deploy
	$(MAKE) start

help: ## Show the available Makefile targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

