dfx canister call br5f7-7uaaa-aaaaa-qaaca-cai get_escrow_account

dfx ledger transfer --memo 1  --amount 1 'c40332503c0d45fc94934d39312b3bd0176d1cdf68f5ed02accfcf7b2b876003' --ledger-canister-id "b77ix-eeaaa-aaaaa-qaada-cai"

dfx canister call br5f7-7uaaa-aaaaa-qaaca-cai book_tokens '(record { quantity = 1000; })'

dfx canister call br5f7-7uaaa-aaaaa-qaaca-cai debit_profit_entry_for_investor '(1000000000, "8e08b3744f6bd323b958235aa9a823bfcf44a1fdbb358d1f67981eb564259799")'
