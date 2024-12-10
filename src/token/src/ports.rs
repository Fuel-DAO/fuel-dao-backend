use candid::{Nat, Principal};
use ic_cdk::caller;
use crate::state::metadata::UpdateMetadataArgs;
use crate::state::subaccount::Subaccount;
use crate::validations::{check_collection_owner,check_not_anonymous};
use crate::{BookTokensArg, Icrc7BalanceOfArgItem, Icrc7OwnerOfRetItemInner, Icrc7TokenMetadataRetItemInnerItem1, Icrc7TokensOfArg, Icrc7TransferArgItem, Icrc7TransferRetItemInner};
use crate::{state::{escrow::SaleStatus, models::{GetEscrowAccountRet, GetMetadataRet}}, STATE};
use ic_cdk_macros::*;


#[update(guard = "check_collection_owner")]
pub async fn change_ownership( arg0: Principal) -> Result<Nat, String> {
    let   f  =  STATE.with_borrow_mut( |f|  f.clone() );
    f.change_ownership(arg0).await 
}
#[update(guard = "check_collection_owner")]
pub async fn update_metadata( arg0: UpdateMetadataArgs) -> Result<Nat, String> {
    STATE.with_borrow_mut( |f|{  f.metadata.as_mut().map(|f| f.metadata.update(arg0)); Ok(f.transactions.index().clone()) } )
}


// #[update(guard = "check_not_anonymous")]
#[update]
pub async fn book_tokens( arg: BookTokensArg) -> Result<bool, String> {
    let   f  =  STATE.with_borrow( |f|  f.clone() );
    let qunatity =  arg.quantity.clone();
    let res = f.book_tokens(arg).await?;

    STATE.with(|f|{  f.borrow_mut().escrow.book_tokens(caller(), qunatity.into());  } );
    Ok(res)
}

#[query]
pub async fn get_excess_escrow_balance() -> Result<Vec<Principal>, String> {
  STATE.with_borrow( |f|  f.clone() ).get_excess_escrow_balance().await
}


#[update(guard = "check_collection_owner")]
pub async fn accept_sale() -> Result<bool, String> {
    let state = STATE.with(|s| s.borrow().clone());
    state.accept_sale().await 
}

#[update(guard = "check_collection_owner")]
pub async fn reject_sale() -> Result<bool, String> {
    let    f  =  STATE.with( |f|  f.borrow().clone() );
    f.reject_sale().await
}

// #[update(guard = "check_collection_owner")]
// pub async fn reject_sale_individual(invester: Principal) -> Result<bool, String> {
//     let    f  =  STATE.with( |f|  f.borrow().clone() );
//     f.reject_sale_individual(invester).await
// }

#[update(guard = "check_collection_owner")]
pub async fn refund_excess_after_sale(invester: Principal) -> Result<bool, String> {
    let    f  =  STATE.with( |f|  f.borrow().clone() );
    f.refund_excess_after_sale(invester).await
}

#[query]
pub async fn get_booked_tokens( arg0: Option<Principal>) -> u128 {
    STATE.with( |f|  f.borrow().clone() )
    .get_booked_tokens(arg0).await 
}

#[update]
pub fn icrc7_transfer( args: Vec<Icrc7TransferArgItem>) -> Vec<Option<Icrc7TransferRetItemInner>>  {
    STATE.with( |f|  f.borrow_mut().icrc_7_transfer(args) )
}

#[query]
pub fn icrc7_balance_of( args: Vec<Icrc7BalanceOfArgItem>) -> Vec<u64>  {
    STATE.with( |f|  f.borrow().icrc_7_balance_of(args) )
}
#[query]
pub fn icrc7_owner_of( args: Vec<u32>) -> Vec<Option<Icrc7OwnerOfRetItemInner>>  {
    STATE.with( |f|  f.borrow().icrc_7_owner_of(args) )
}

#[query]
pub fn icrc7_token_metadata( args: Vec<u32>) -> Vec<Option<Vec<(String, Icrc7TokenMetadataRetItemInnerItem1)>>>  {
    STATE.with( |f|  f.borrow().icrc_7_token_metadata(args) )
}

#[query]
pub fn icrc7_tokens(  prev: Option<u32>,
    take: Option<u32>,) -> Vec<u32>  {
    STATE.with( |f|  f.borrow().icrc_7_tokens(prev, take) )
}
#[query]
pub fn icrc7_tokens_of(   account: Icrc7TokensOfArg,
    prev: Option<u32>,
    take: Option<u32>,) -> Vec<u32>  {
    STATE.with( |f|  f.borrow().icrc_7_tokens_of(account,prev, take) )
}


#[query]
pub async fn get_escrow_account() -> Result<GetEscrowAccountRet, String> {
    STATE.with( |f|  f.borrow().clone() )
    .get_escrow_account().await 
}

#[query]
pub async fn get_metadata() -> Result<GetMetadataRet, String> {
    STATE.with( |f|  f.borrow().clone() )
    .get_metadata().await 
}

#[query]
pub async fn get_participating_investors() -> Vec<Principal> {
    STATE.with( |f|  f.borrow().clone() )
    .get_participating_investors().await 
}


#[query]
pub async fn get_sale_status() -> SaleStatus {
    STATE.with( |f|  f.borrow().clone() )
    .get_sale_status().await 
}

#[update(guard = "check_collection_owner")]
pub async fn update_sale_status(status: SaleStatus) -> SaleStatus {
    STATE.with( |f|  f.borrow_mut().escrow.update_sale_status(status.clone()) );
    status
}


#[query]
pub async fn get_total_booked_tokens() -> u128 {
    STATE.with( |f|  f.borrow().clone() )
    .get_total_booked_tokens().await 
}
