#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, testutils::Ledger, Address, Env, token};

fn setup_test_env<'a>() -> (Env, Address, Address, token::Client<'a>) {
let env = Env::default();
env.mock_all_auths();

let seller = Address::generate(&env);
let buyer = Address::generate(&env);

let token_admin = Address::generate(&env);
let contract_id = env.register_stellar_asset_contract(token_admin.clone());
let token_client = token::Client::new(&env, &contract_id);
let token_admin_client = token::StellarAssetClient::new(&env, &contract_id);

token_admin_client.mint(&buyer, &10000);

(env, seller, buyer, token_client)
}

#[test]
fn test_1_happy_path_escrow_lifecycle() {
let (env, seller, buyer, token_client) = setup_test_env();
let contract_id = env.register_contract(None, KartsContract);
let contract_client = KartsContractClient::new(&env, &contract_id);

env.ledger().set_timestamp(1000);
let delivery_window = 172800; // 48 Hours

contract_client.secure_invoice(&5001, &seller, &buyer, &token_client.address, &3500, &delivery_window);
assert_eq!(token_client.balance(&contract_id), 3500);

env.ledger().set_timestamp(1000 + delivery_window + 1);
contract_client.claim_payment(&5001, &seller);

assert_eq!(token_client.balance(&seller), 3500);
assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "Delivery window has not expired yet.")]
fn test_2_edge_case_early_claim_failure() {
let (env, seller, buyer, token_client) = setup_test_env();
let contract_id = env.register_contract(None, KartsContract);
let contract_client = KartsContractClient::new(&env, &contract_id);

env.ledger().set_timestamp(1000);
contract_client.secure_invoice(&5002, &seller, &buyer, &token_client.address, &2000, &172800);

env.ledger().set_timestamp(50000);
contract_client.claim_payment(&5002, &seller);
}

#[test]
fn test_3_state_verification_after_deposit() {
let (env, seller, buyer, token_client) = setup_test_env();
let contract_id = env.register_contract(None, KartsContract);
let contract_client = KartsContractClient::new(&env, &contract_id);

env.ledger().set_timestamp(5000);
contract_client.secure_invoice(&5003, &seller, &buyer, &token_client.address, &1200, &100);

let key = StorageKey::Invoice(5003);
let data: OrderInvoice = env.storage().persistent().get(&key).unwrap();

assert_eq!(data.seller, seller);
assert_eq!(data.buyer, buyer);
assert_eq!(data.amount, 1200);
assert_eq!(data.auto_release_time, 5100);
assert!(!data.is_settled);
}

#[test]
#[should_panic(expected = "Caller is not the authorized seller.")]
fn test_4_unauthorized_seller_claim() {
let (env, seller, buyer, token_client) = setup_test_env();
let contract_id = env.register_contract(None, KartsContract);
let contract_client = KartsContractClient::new(&env, &contract_id);

env.ledger().set_timestamp(1000);
contract_client.secure_invoice(&5004, &seller, &buyer, &token_client.address, &4000, &100);

env.ledger().set_timestamp(2000);
let completely_different_person = Address::generate(&env);
contract_client.claim_payment(&5004, &completely_different_person);
}

#[test]
#[should_panic]
fn test_5_insufficient_buyer_funds() {
let (env, seller, buyer, token_client) = setup_test_env();
let contract_id = env.register_contract(None, KartsContract);
let contract_client = KartsContractClient::new(&env, &contract_id);

env.ledger().set_timestamp(1000);
contract_client.secure_invoice(&5005, &seller, &buyer, &token_client.address, &99999, &100);
}