.DEFAULT_GOAL := help

REMOTE := romac.me
REMOTE_DIR := /var/www/france-schmid.ch

.PHONY: docker-build docker-push pull-data stop start reload full help

docker-build: ## Build the latest Docker image
	docker build -t fsr:latest .

docker-push: ## Push the latest Docker image to GHCR
	docker tag fsr:latest ghcr.io/romac/fsr:latest
	docker push ghcr.io/romac/fsr:latest

pull-data: ## Pull the data from the server
	rsync -azvhe ssh ${REMOTE}:${REMOTE_DIR}/_data/ _data

push-data: ## Push the data to the server
	rsync -azvhe ssh _data/ ${REMOTE}:${REMOTE_DIR}/_data/

deploy: ## Deploy the binary to the server
	echo "TODO"

stop: ## Remotely stop the webserver
	ssh -t fsr 'tmux send-keys -t fsr C-c ENTER'

start: ## Remotely start the webserver
	ssh -t fsr 'tmux send-keys -t fsr ./fsr ENTER'

reload: ## Remotely reload the webserver
	$(MAKE) stop
	$(MAKE) start

full: ## Alias for `stop`, `deploy`, `start`
	$(MAKE) stop
	$(MAKE) deploy
	$(MAKE) start

help: ## Show the available Makefile targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

