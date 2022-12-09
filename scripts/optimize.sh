#!/bin/bash

docker run --rm -v "$(pwd)":/code \
  -e CARGO_TERM_COLOR=always \
  --mount type=volume,source="cosmwasm_sccache",target=/root/.cache/sccache \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/workspace-optimizer:0.12.10
