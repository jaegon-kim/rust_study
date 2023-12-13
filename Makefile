include Container.mk
version := $(file < VERSION)

.PHONY: default
default: build

src_dir := src
output_dir := output
project := rust_study

# Rust build environment container image name
build_env_image := cont-rust-build-env:$(version)

# Command for constructing Rust build environment
container_build = $(CONTAINER_RUNTIME) build \
	-f Dockerfile \
	$(container_build_options) \
	.

# Command for executing command in rust build environment (compilation, and executing)
container_run = $(CONTAINER_RUNTIME) run \
	--rm -t -h $(build_env_image) -e AT_CONTAINER=true -e CARGO_HOME=/root/$(project)/.cargo \
	--mount type=bind,source=$(CURDIR),target=/root/$(project) -w /root/$(project) \
	$(container_run_options) \
	--pull never $(build_env_image)

# Test Parameter for testing command line argument
test_arguments = --name Rust --age 10
#test_arguments = 


.PHONY: build-env
build-env: container_build_options := -t $(build_env_image) --target build-env
build-env:
	$(container_build)

.PHONY: build
build:
	@$(container_run) cargo build --release 

.PHONY: local
local:
	cargo build --release

.PHONY: run
run: build
	@$(container_run) target/release/hello $(test_arguments)

.PHONY: local-run
local-run:
	target/release/hello $(test_arguments)

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

.PHONY: local-clean
local-clean:
	cargo clean 
