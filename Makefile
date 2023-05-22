.DEFAULT_GOAL := help

REMOTE := romac.me
REMOTE_DIR := /var/www/france-schmid.ch

.PHONY: docker-build docker-push pull-data reload help

docker-build: ## Build the latest Docker image
	docker build -t fsr:latest .

docker-push: ## Push the latest Docker image to GHCR
	docker tag fsr:latest ghcr.io/romac/fsr:latest
	docker push ghcr.io/romac/fsr:latest

reload: ## Remotely reload the webserver
	ssh ${REMOTE} "cd ${REMOTE_DIR}/.. && docker compose pull fsr && docker compose up -d"

pull-data: ## Pull the data from the server
	rsync -azvhe ssh ${REMOTE}:${REMOTE_DIR}/_data/ _data

push-data: ## Push the data to the server
	rsync -azvhe ssh _data/ ${REMOTE}:${REMOTE_DIR}/_data/

help: ## Show the available Makefile targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

