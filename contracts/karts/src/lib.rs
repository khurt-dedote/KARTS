#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contract]
pub struct KartsContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
Invoice(u64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderInvoice {
pub seller: Address,
pub buyer: Address,
pub token: Address,
pub amount: i128,
pub auto_release_time: u64,
pub is_settled: bool,
}

#[contractimpl]
impl KartsContract {
pub fn secure_invoice(
env: Env,
invoice_id: u64,
seller: Address,
buyer: Address,
token: Address,
amount: i128,
delivery_duration: u64,
) {
buyer.require_auth();

    let token_client = token::Client::new(&env, &token);
    token_client.transfer(&buyer, &env.current_contract_address(), &amount);

    let auto_release_time = env.ledger().timestamp() + delivery_duration;

    let invoice = OrderInvoice {
        seller,
        buyer,
        token,
        amount,
        auto_release_time,
        is_settled: false,
    };

    env.storage().persistent().set(&StorageKey::Invoice(invoice_id), &invoice);
}

pub fn claim_payment(env: Env, invoice_id: u64, seller: Address) {
    seller.require_auth();

    let key = StorageKey::Invoice(invoice_id);
    let mut invoice: OrderInvoice = env.storage().persistent().get(&key).unwrap();

    assert_eq!(seller, invoice.seller, "Caller is not the authorized seller.");
    assert!(!invoice.is_settled, "Payment has already been settled.");
    assert!(
        env.ledger().timestamp() >= invoice.auto_release_time,
        "Delivery window has not expired yet."
    );

    invoice.is_settled = true;
    env.storage().persistent().set(&key, &invoice);

    let token_client = token::Client::new(&env, &invoice.token);
    token_client.transfer(&env.current_contract_address(), &invoice.seller, &invoice.amount);
    
    env.storage().persistent().remove(&key);
}
}

#[cfg(test)]
mod test;