dfx canister call avqkn-guaaa-aaaaa-qaaea-cai get_escrow_account

dfx ledger transfer 7a1572cb607fdc16e81f220000e7df981402bd138578bd1b81e5289d3fa27673 --icp 1 --memo 12345

dfx canister call avqkn-guaaa-aaaaa-qaaea-cai book_tokens '(record { quantity = 5; })'
