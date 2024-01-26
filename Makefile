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

# common container parameters
container_run_parameters = --rm -t -h $(build_env_image) \
	-e AT_CONTAINER=true -e CARGO_HOME=/root/$(project)/.cargo \
	--mount type=bind,source=$(CURDIR),target=/root/$(project) -w /root/$(project) \
	$(container_run_options) \
	--pull never $(build_env_image)

# Command for executing command in rust build environment (compilation, and executing)
container_run = $(CONTAINER_RUNTIME) run $(container_run_parameters)

# Command for running rust build environment as daemon
container_daemon_run = $(CONTAINER_RUNTIME) run -d $(container_network) --name $(project) $(container_run_parameters)
#container_network = --network macvlan_enp1s0f1 --ip=192.168.56.56

# Test Parameter for testing command line argument
test_arguments = --name Rust --age 10
#test_arguments = 


.PHONY: build-env
build-env: container_build_options := -t $(build_env_image) --target build-env
build-env:
	$(container_build)

.PHONY: build
build:
ifdef AT_CONTAINER
	cargo build --release
else
	@$(container_run) cargo build --release 
endif

.PHONY: local
local:
	cargo build --release

.PHONY: run
run: build
ifdef AT_CONTAINER
	target/release/hello $(test_arguments)
else
	@$(container_run) target/release/hello $(test_arguments)
endif

.PHONY: local-run
local-run: local
	target/release/hello $(test_arguments)

.PHONY: docker-run
docker-run:
# execute like belows
# make docker-run CMD="...."
	@echo $(container_run)
	@echo docker-run [command line: '$(CMD)']

	@echo
	@$(container_run) $(CMD) 

.PHONY: daemon-run
daemon-run:
	@echo $(container_daemon_run)
	@$(container_daemon_run)

.PHONY: daemon
daemon:
	@echo $(container_daemon_run)
	$(CONTAINER_RUNTIME) exec -it $(project) /bin/bash

.PHONY: clean
clean:
ifdef AT_CONTAINER
	cargo clean 
else
	@$(container_run) cargo clean 
endif

.PHONY: local-clean
local-clean:
	cargo clean 
