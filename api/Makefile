GIT_HASH=$(shell git rev-parse --short HEAD)
IMAGE_NAME=ghcr.io/aspedrosa/courses-calculator-api

# to have multiple github accounts on the same computer, we must user a different docker config
#  directory per github project, so we can push images to such projects
DOCKER_CONFIG=$(shell if [ -d .docker ]; then echo "--config .docker"; fi)

build:
	docker build -t $(IMAGE_NAME):latest -t $(IMAGE_NAME):$(GIT_HASH) .

push:
	docker $(DOCKER_CONFIG) push $(IMAGE_NAME):latest
	docker $(DOCKER_CONFIG) push $(IMAGE_NAME):$(GIT_HASH)

RUST_DIR=csvProcessor
RUST_LIB_NAME=csv_processor
LIB_DIR=$(shell pwd)/$(RUST_DIR)/target/release

build-rust:
	cd $(RUST_DIR) && cargo build --release

build-go:
	CGO_LDFLAGS="-L$(LIB_DIR) -l$(RUST_LIB_NAME) -lm" CGO_ENABLED=1 go build -o bin/courses-calculator main.go
