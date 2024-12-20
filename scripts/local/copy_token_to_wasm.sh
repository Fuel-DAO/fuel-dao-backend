#!/usr/bin/env bash

function copy_did_and_wasm() {
  local canister=$1
  local canister_root="wasm/$canister"

  # Create the destination directory if it doesn't exist
  mkdir -p "$canister_root"

  # Extract candid interface and save to can.did
  candid-extractor "target/wasm32-unknown-unknown/release/$canister.wasm" > "$canister_root/can.did"

  # Gzip the wasm file
  gzip -c ".dfx/local/canisters/$canister/$canister.wasm" > "$canister_root/$canister.wasm.gz"

  echo "Copied can.did and gzipped wasm for $canister to $canister_root"
}

# The list of canisters in your project (comma-separated if more than one)
CANISTERS=token

for canister in $(echo $CANISTERS | sed "s/,/ /g"); do
  dfx build "$canister"
  copy_did_and_wasm "$canister"
done
