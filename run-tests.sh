#!/usr/bin/env bash

diesel migration redo
cargo test --test mod pages -- --test-threads=1
cargo test --test mod users -- --test-threads=1
cargo test --test mod session -- --test-threads=1