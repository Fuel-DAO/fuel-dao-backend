dfx canister call avqkn-guaaa-aaaaa-qaaea-cai get_escrow_account

dfx ledger transfer --memo 1  --amount 1 'c40332503c0d45fc94934d39312b3bd0176d1cdf68f5ed02accfcf7b2b876003' --ledger-canister-id "b77ix-eeaaa-aaaaa-qaada-cai"

dfx canister call avqkn-guaaa-aaaaa-qaaea-cai book_tokens '(record { quantity = 5; })'
