# Deploy Canisters with Specific IDs
dfx deploy --specified-id 4sc7j-7iaaa-aaaaj-qnfva-cai asset
dfx deploy --specified-id 43buv-jaaaa-aaaaj-qnfuq-cai asset_proxy
dfx deploy --specified-id 44asb-eyaaa-aaaaj-qnfua-cai provision
dfx deploy --specified-id ewirk-vqaaa-aaaaj-qa57q-cai backend
dfx canister create token

# Update Controller Permissions (Check if "2vxsx-fae" is intended)
dfx canister update-settings --add-controller "2vxsx-fae" provision 
dfx canister update-settings --add-controller "2vxsx-fae" asset_proxy 
dfx canister update-settings --add-controller "2vxsx-fae" asset 

# Ensure Script Paths Are Correct
./scripts/local/copy_token_to_wasm.sh
./scripts/local/upload_token_wasm.sh
./scripts/local/upload_asset_wasm.sh

# Set Canister Relationships
dfx canister call provision set_asset_proxy_canister '(principal "43buv-jaaaa-aaaaj-qnfuq-cai")'
dfx canister call asset_proxy set_provision_canister '(principal "44asb-eyaaa-aaaaj-qnfua-cai")'
dfx canister call asset_proxy set_temp_asset_canister '(principal "4sc7j-7iaaa-aaaaj-qnfva-cai")'

# Grant Asset Canister Permissions
dfx canister call asset grant_permission '(record { permission = variant { Prepare }; to_principal = principal "43buv-jaaaa-aaaaj-qnfuq-cai" })'
dfx canister call asset grant_permission '(record { permission = variant { Commit }; to_principal = principal "43buv-jaaaa-aaaaj-qnfuq-cai" })'
dfx canister call asset grant_permission '(record { permission = variant { ManagePermissions }; to_principal = principal "43buv-jaaaa-aaaaj-qnfuq-cai" })'
