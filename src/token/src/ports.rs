use candid::{Nat, Principal};
use ic_cdk::caller;
use ic_ledger_types::AccountIdentifier;
use crate::state::escrow::EscrowTokenBalance;
use crate::state::metadata::UpdateMetadataArgs;
use crate::state::State;
use crate::validations::{check_collection_owner, check_not_anonymous};
use crate::{BookTokensArg, Icrc7BalanceOfArgItem, Icrc7OwnerOfRetItemInner, Icrc7TokenMetadataRetItemInnerItem1, Icrc7TokensOfArg, Icrc7TransferArgItem, Icrc7TransferArgItemTo, Icrc7TransferRetItemInner};
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


#[update(guard = "check_collection_owner")]
pub fn update_annonymous_investor(new_investor: Principal) -> Result<(), String> {
    STATE.with_borrow_mut(|f|  {f.escrow.update_annonymous_investor(new_investor); Ok(())} )
}


#[update(guard = "check_not_anonymous")]
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

#[update(guard = "check_collection_owner")]
pub async fn refund_icp_amount_after_sale_from_annonymous( amount: f64 ) -> Result<bool, String> {
    let    f  =  STATE.with( |f|  f.borrow().clone() );
    let invester = Principal::anonymous();
    f.refund_icp_amount_after_sale_from_annonymous(invester, amount).await
}

#[update(guard = "check_collection_owner")]
pub async fn transfer_icp_amount_from_annonymous_to_investor( amount: f64, invester: Principal ) -> Result<bool, String> {
    let    f  =  STATE.with( |f|  f.borrow().clone() );
    f.refund_icp_amount_from_annonymous_to_investor( amount, invester).await
}

#[query]
pub async fn get_booked_tokens( arg0: Option<Principal>) -> u128 {
    STATE.with( |f|  f.borrow().clone() )
    .get_booked_tokens(arg0).await 
}

#[update]
pub fn token_transfer_to_principal(token_id: u32, user_principal: Principal  ) -> Vec<Option<Icrc7TransferRetItemInner>>  {
    let args = Icrc7TransferArgItem {
        to: Icrc7TransferArgItemTo {
            owner: user_principal, 
            subaccount: Some(crate::state::subaccount::Subaccount::from(&user_principal).to_vec())
        }, 
        token_id, 
        memo: None, 
        from_subaccount: Some(crate::state::subaccount::Subaccount::from(&caller()).to_vec()), 
        created_at_time: None, 
    };
    STATE.with( |f|  f.borrow_mut().icrc_7_transfer(vec![args]) )
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
pub fn tokens_of( user: Option<Principal>,
    prev: Option<u32>,
    take: Option<u32>,) -> Vec<u32>  {
        let user = user.unwrap_or(caller());
        let account = Icrc7TokensOfArg {
            owner: user, 
            subaccount: Some(crate::state::subaccount::Subaccount::from(&user).to_vec())
        };
    STATE.with( |f|  f.borrow().icrc_7_tokens_of(account,prev, take) )
}

#[query]
pub fn icrc7_tokens_of(   account: Icrc7TokensOfArg,
    prev: Option<u32>,
    take: Option<u32>,) -> Vec<u32>  {
    STATE.with( |f|  f.borrow().icrc_7_tokens_of(account,prev, take) )
}


#[query(guard = "check_not_anonymous")]
pub async fn get_escrow_account() -> Result<GetEscrowAccountRet, String> {
    STATE.with( |f|  f.borrow().clone() )
    .get_escrow_account().await 
}

#[query]
pub async fn get_metadata() -> Result<GetMetadataRet, String> {
    STATE.with( |f|  f.borrow().clone() )
    .get_metadata() 
}

#[query]
pub async fn get_participating_investors() -> Vec<Principal> {
    STATE.with( |f|  f.borrow().clone() )
    .get_participating_investors().await 
}

#[query]
pub async fn canister_escrow_account() -> String {
    State::canister_escrow_account()
}

#[update]
pub async fn canister_balance_in_icp() -> Result<f64, String> {
    STATE.with( |f|  f.borrow().clone() )
    .canister_balance_in_icp().await 
}

#[update]
pub async fn get_escrow_balance_and_token_balance_for_user(user: Option<Principal>) -> Result<EscrowTokenBalance, String> {
    let user  = user.unwrap_or(caller());
    let state = STATE.with( |f|  f.borrow().clone() );
    let icp_balance =  state.get_escrow_balance_for_principal(user).await?;
    let subaccount = Some(crate::state::subaccount::Subaccount::from(&user).to_vec());
    let token_balance = state.tokens.token_count_for_investor(user, subaccount);
    Ok(EscrowTokenBalance {
        icp_balance, token_balance,
    })
}

#[update(guard = "check_collection_owner")]
pub async fn trasfer_icp_to_investors(icp: f64) -> Result<String, String> {
    STATE.with( |f: &std::cell::RefCell<State>|  f.borrow().clone() )
    .trasfer_icp_to_investors(icp).await 
}

#[update]
pub async fn transfer_my_balance_from_escrow(icp: f64, account_id: String) -> Result<f64, String> {
    let _ = AccountIdentifier::from_hex(&account_id)?;
    let user = caller();
    let state = STATE.with( |f|  f.borrow().clone() );
    let icp_balance =  state.get_escrow_balance_for_principal(user).await?;
    if icp_balance < icp {
        Err("Insufficient funds to perform this request".into())
    } else {
        let amount_to_transfer = (icp * 1e8 ) as u64 ;
        let meta = state.get_metadata()?;
        let ledger = meta.token;
        let from_principal = user;
        let transfer_to_account_id = account_id;
        let _ = state.escrow.transfer_amount_from_escrow_to_account_id( &from_principal, ledger,  amount_to_transfer, transfer_to_account_id  ).await.map_err(|f| format!("Failed for investor: {}\n{f}", user.to_text()))?;
        let icp_balance =  state.get_escrow_balance_for_principal(user).await?;
        Ok(icp_balance)
    }
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

#[query]
pub async fn get_escrow_account_id_for_principal( canister_id: Option<Principal>,principal: Principal,) -> String {
    State::genral_escrow_account(canister_id, principal)
}
