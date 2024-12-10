# !/bin/bash



# Specify the path to your Wasm.gz file
wasm="wasm/asset/assetstorage.wasm.gz"


# Display the hexdump or use the variable as needed
# echo "$(hexdump -ve '1/1 "%.2x"' "$wasm" | sed 's/../\\&/g')"

# Use xxd to convert the file content to a hexadecimal string
char=$(hexdump -ve '1/1 "%.2x"' "$wasm")

# Escape special characters in the hexadecimal string
char_escaped=$(printf "%s" "$char" | sed 's/../\\&/g')

# Create a shell script with the escaped hexadecimal string
printf "(blob \"%s\")"  "$char_escaped" > argument
dfx canister call provision --network ic add_asset_wasm --argument-file argument 
