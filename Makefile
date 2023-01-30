#!/usr/bin/make -f
VERSION := $(shell echo $(shell git describe --tags) | sed 's/^v//')
COMMIT := $(shell git log -1 --format='%H')

CURRENT_DIR := $(shell pwd)
BASE_DIR := $(shell basename $(CURRENT_DIR))

compile:
	@echo "Compiling Contract $(COMMIT)..."	
	@docker run --rm -v "$(CURRENT_DIR)":/code \
	--mount type=volume,source="$(BASE_DIR)_cache",target=/code/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	cosmwasm/rust-optimizer:0.12.11

all:
	cargo schema	
	cargo fmt
	cargo test
	cargo clippy -- -D warnings	

test:
	cargo test -- --nocapture

test-e2e:
	sh ./e2e/test_e2e.sh

clippy-pedantic:	
	cargo clippy -- -W clippy::pedantic -W clippy::missing-errors-doc -W clippy::doc-markdown -W clippy::module-name-repetitions
clippy-format:	
	cargo clippy -- -W clippy::useless_format
clippy-correctness:	
	cargo clippy -- -W clippy::correctness
clippy-suspicious:	
	cargo clippy -- -W clippy::suspicious
clippy-style:	
	cargo clippy -- -W clippy::style
clippy-perf:	
	cargo clippy -- -W clippy::perf
clippy-complexity:	
	cargo clippy -- -W clippy::complexity
clippy-new:	
	cargo clippy -- -W clippy::nursery	

# -W clippy::missing-errors-doc