include Container.mk
version := $(file < VERSION)

.PHONY: default
default: build

src_dir := src
output_dir := output
project := rust_study

build_env_image := cont-rust-build-env:$(version)

container_build = $(CONTAINER_RUNTIME) build \
	-f Dockerfile \
	$(container_build_options) \
	.

container_run = $(CONTAINER_RUNTIME) run \
	--rm -t -h $(build_env_image) -e AT_CONTAINER=true -e CARGO_HOME=/root/$(project)/.cargo \
	--mount type=bind,source=$(CURDIR),target=/root/$(project) -w /root/$(project) \
	$(container_run_options) \
	--pull never $(build_env_image)

.PHONY: build-env
build-env: container_build_options := -t $(build_env_image) --target build-env
build-env:
	$(container_build)

.PHONY: build
build:
	@$(container_run) cargo build --release 

.PHONY: run
run:
	@$(container_run) target/release/hello --name Me --count 1

.PHONY: local
local:
	cargo build --release

.PHONY: local-run
local-run:
	target/release/hello

.PHONY: docker-run
docker-run:
# execute like belows
# make docker-run CMD="...."
	@echo $(container_run)
	@echo docker-run [command line: '$(CMD)']

	@echo
	@$(container_run) $(CMD) 

.PHONY: clean
clean:
	@$(container_run) cargo clean 
