use super::*;
use soroban_sdk::{testutils::Address as _, IntoVal};

fn setup(fee: u32, balance: i128) -> (Env, Address, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let treasury = Address::generate(&env);
    let admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(admin).address();
    token::StellarAssetClient::new(&env, &token).mint(&sender, &balance);
    let router = env.register(PaymentRouter, (treasury.clone(), token.clone(), fee));
    (env, router, sender, recipient, treasury, token)
}

#[test]
fn split_and_sender_authorization() {
    let (env, router, sender, recipient, treasury, asset) = setup(125, 20_000);
    let client = PaymentRouterClient::new(&env, &router);
    assert_eq!(
        client.route_payment(&sender, &recipient, &treasury, &asset, &10_001),
        9_876
    );
    let auths = env.auths();
    let token = token::Client::new(&env, &asset);
    assert_eq!(token.balance(&sender), 9_999);
    assert_eq!(token.balance(&recipient), 9_876);
    assert_eq!(token.balance(&treasury), 125);
    assert_eq!(auths[0].0, sender);
    assert_eq!(
        auths[0].1.function,
        soroban_sdk::testutils::AuthorizedFunction::Contract((
            router,
            soroban_sdk::Symbol::new(&env, "route_payment"),
            (sender, recipient, treasury, asset, 10_001i128).into_val(&env)
        ))
    );
}

#[test]
fn missing_and_wrong_sender_auth_are_rejected() {
    let (env, router, sender, recipient, treasury, asset) = setup(100, 20_000);
    let client = PaymentRouterClient::new(&env, &router);
    env.set_auths(&[]);
    assert!(client
        .try_route_payment(&sender, &recipient, &treasury, &asset, &10_000)
        .is_err());
    client.mock_auths(&[soroban_sdk::testutils::MockAuth {
        address: &recipient,
        invoke: &soroban_sdk::testutils::MockAuthInvoke {
            contract: &router,
            fn_name: "route_payment",
            args: (
                sender.clone(),
                recipient.clone(),
                treasury.clone(),
                asset.clone(),
                10_000i128,
            )
                .into_val(&env),
            sub_invokes: &[],
        },
    }]);
    assert!(client
        .try_route_payment(&sender, &recipient, &treasury, &asset, &10_000)
        .is_err());
    assert_eq!(token::Client::new(&env, &asset).balance(&sender), 20_000);
}

#[test]
fn failed_fee_transfer_rolls_back_recipient_transfer() {
    let (env, router, sender, recipient, treasury, asset) = setup(1_000, 9_000);
    assert!(PaymentRouterClient::new(&env, &router)
        .try_route_payment(&sender, &recipient, &treasury, &asset, &10_000)
        .is_err());
    let token = token::Client::new(&env, &asset);
    assert_eq!(token.balance(&sender), 9_000);
    assert_eq!(token.balance(&recipient), 0);
    assert_eq!(token.balance(&treasury), 0);
}

#[test]
fn invalid_inputs_and_policy_cannot_move_funds() {
    let (env, router, sender, recipient, treasury, asset) = setup(100, 20_000);
    let client = PaymentRouterClient::new(&env, &router);
    for amount in [0, -1, i128::MIN] {
        assert_eq!(
            client.try_route_payment(&sender, &recipient, &treasury, &asset, &amount),
            Err(Ok(soroban_sdk::Error::from_contract_error(
                Error::InvalidAmount as u32
            )))
        );
    }
    let stranger = Address::generate(&env);
    assert_eq!(
        client.try_route_payment(&sender, &recipient, &stranger, &asset, &1),
        Err(Ok(soroban_sdk::Error::from_contract_error(
            Error::PolicyMismatch as u32
        )))
    );
    assert_eq!(
        client.try_route_payment(&sender, &recipient, &treasury, &stranger, &1),
        Err(Ok(soroban_sdk::Error::from_contract_error(
            Error::PolicyMismatch as u32
        )))
    );
    assert_eq!(
        client.try_route_payment(&sender, &sender, &treasury, &asset, &1),
        Err(Ok(soroban_sdk::Error::from_contract_error(
            Error::InvalidRecipient as u32
        )))
    );
    assert_eq!(token::Client::new(&env, &asset).balance(&sender), 20_000);
}

#[test]
fn maximum_amount_and_small_fee_rounding() {
    for (fee, amount) in [(1_000, i128::MAX), (100, 1), (0, 10_000)] {
        let (env, router, sender, recipient, treasury, asset) = setup(fee, amount);
        let net = PaymentRouterClient::new(&env, &router)
            .route_payment(&sender, &recipient, &treasury, &asset, &amount);
        let token = token::Client::new(&env, &asset);
        assert_eq!(token.balance(&sender), 0);
        assert_eq!(token.balance(&recipient), net);
        assert_eq!(net + token.balance(&treasury), amount);
    }
}

#[test]
#[should_panic]
fn excessive_fee_policy_is_rejected_at_deployment() {
    setup(1_001, 10);
}
