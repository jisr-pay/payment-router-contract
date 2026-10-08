#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, token,
    Address, Env,
};

#[contracttype]
#[derive(Clone)]
pub struct Policy {
    pub treasury: Address,
    pub token: Address,
    pub fee_bps: u32,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidPolicy = 1,
    InvalidAmount = 2,
    PolicyMismatch = 3,
    InvalidRecipient = 4,
}

#[contract]
pub struct PaymentRouter;

#[contractimpl]
impl PaymentRouter {
    // Constructor executes only at deployment; no public reinitialization or upgrades.
    pub fn __constructor(env: Env, treasury: Address, token: Address, fee_bps: u32) {
        if fee_bps > 1_000
            || treasury == env.current_contract_address()
            || treasury == token
            || token == env.current_contract_address()
        {
            panic_with_error!(&env, Error::InvalidPolicy);
        }
        env.storage().instance().set(
            &symbol_short!("policy"),
            &Policy {
                treasury,
                token,
                fee_bps,
            },
        );
    }

    pub fn policy(env: Env) -> Policy {
        env.storage()
            .instance()
            .get(&symbol_short!("policy"))
            .unwrap()
    }

    // ABI matches the existing Jisr SDK. Amount is the total debit in token units.
    pub fn route_payment(
        env: Env,
        sender: Address,
        recipient: Address,
        platform_treasury: Address,
        token_address: Address,
        amount: i128,
    ) -> i128 {
        let policy = Self::policy(env.clone());
        if platform_treasury != policy.treasury || token_address != policy.token {
            panic_with_error!(&env, Error::PolicyMismatch);
        }
        if amount <= 0 {
            panic_with_error!(&env, Error::InvalidAmount);
        }
        if sender == recipient
            || sender == policy.treasury
            || sender == env.current_contract_address()
            || recipient == env.current_contract_address()
        {
            panic_with_error!(&env, Error::InvalidRecipient);
        }
        sender.require_auth();
        // Floor basis-point fee without multiplying the full i128 amount.
        let bps = i128::from(policy.fee_bps);
        let fee = (amount / 10_000) * bps + ((amount % 10_000) * bps) / 10_000;
        let net = amount - fee;
        let client = token::Client::new(&env, &policy.token);
        client.transfer(&sender, &recipient, &net);
        if fee > 0 {
            client.transfer(&sender, &policy.treasury, &fee);
        }
        env.events().publish(
            (symbol_short!("routed"), sender, recipient),
            (policy.token, amount, net, fee),
        );
        net
    }
}

#[cfg(test)]
mod test;
