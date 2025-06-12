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

# Push the Docker image to the server
docker-push:
    docker save -o fsr.tar fsr:latest
    scp fsr.tar {{REMOTE}}:{{REMOTE_DIR}}/
    ssh {{REMOTE}} "docker load -i {{REMOTE_DIR}}/fsr.tar && cd /var/www/ && docker compose up -d"

# Pull the data from the server
pull-data: 
    rsync -azvhe ssh {{REMOTE}}:{{REMOTE_DIR}}/ {{LOCAL_DIR}}/

# Push the data to the server
push-data: 
    rsync -azvhe ssh {{LOCAL_DIR}}/ {{REMOTE}}:{{REMOTE_DIR}}

# # Push the latest Docker image to GHCR
# ghcr-push: 
#     docker tag fsr:latest ghcr.io/romac/fsr:latest
#     docker push ghcr.io/romac/fsr:latest
# 
# # Pull the latest Docker image from GHCR
# ghcr-pull: 
#     ssh {{REMOTE}} "docker pull ghcr.io/romac/fsr:latest"
#
# # Remotely reload the webserver
# ghrc-reload: 
#     ssh {{REMOTE}} "cd /var/www/ && docker compose pull fsr && docker compose up -d"
