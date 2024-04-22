# Variables
REMOTE     := "romac.me"
REMOTE_DIR := "/home/fsr/site"
LOCAL_DIR  := "_data"

# Help recipe to show available commands
help:
    @just --list

# Build the latest Docker image
docker-build: 
    docker build -t fsr:latest .

# Push the latest Docker image to GHCR
docker-push: 
    docker tag fsr:latest ghcr.io/romac/fsr:latest
    docker push ghcr.io/romac/fsr:latest

# Pull the latest Docker image from GHCR
docker-pull: 
    ssh {{REMOTE}} "docker pull ghcr.io/romac/fsr:latest"

# Remotely reload the webserver
reload: 
    ssh {{REMOTE}} "cd /var/www/ && docker compose pull fsr && docker compose up -d"

# Pull the data from the server
pull-data: 
    rsync -azvhe ssh {{REMOTE}}:{{REMOTE_DIR}}/ {{LOCAL_DIR}}/

# Push the data to the server
push-data: 
    rsync -azvhe ssh {{LOCAL_DIR}}/ {{REMOTE}}:{{REMOTE_DIR}}

