#![cfg(feature = "test-bpf")]

mod helpers;

use helpers::*;
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::instruction::approve;
use spl_token_lending::{
    instruction::deposit_obligation_collateral, processor::process_instruction,
    state::INITIAL_COLLATERAL_RATIO,
};

#[tokio::test]
async fn test_success() {
    let mut test = ProgramTest::new(
        "spl_token_lending",
        spl_token_lending::id(),
        processor!(process_instruction),
    );

    // limit to track compute unit increase
    test.set_compute_max_units(38_000);

    const GTH_DEPOSIT_AMOUNT_WEIS: u64 = 10 * WEIS_TO_GTH * INITIAL_COLLATERAL_RATIO;
    const GTH_RESERVE_COLLATERAL_WEIS: u64 = 2 * GTH_DEPOSIT_AMOUNT_WEIS;

    let user_accounts_owner = Keypair::new();
    let user_transfer_authority = Keypair::new();

    let lending_market = add_lending_market(&mut test);

    let gth_oracle = add_gth_oracle(&mut test);
    let gth_test_reserve = add_reserve(
        &mut test,
        &lending_market,
        &gth_oracle,
        &user_accounts_owner,
        AddReserveArgs {
            user_liquidity_amount: GTH_RESERVE_COLLATERAL_WEIS,
            liquidity_amount: GTH_RESERVE_COLLATERAL_WEIS,
            liquidity_mint_decimals: 9,
            liquidity_mint_pubkey: spl_token::native_mint::id(),
            config: TEST_RESERVE_CONFIG,
            mark_fresh: true,
            ..AddReserveArgs::default()
        },
    );

    let test_obligation = add_obligation(
        &mut test,
        &lending_market,
        &user_accounts_owner,
        AddObligationArgs::default(),
    );

    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    test_obligation.validate_state(&mut banks_client).await;

    let initial_collateral_supply_balance =
        get_token_balance(&mut banks_client, gth_test_reserve.collateral_supply_pubkey).await;
    let initial_user_collateral_balance =
        get_token_balance(&mut banks_client, gth_test_reserve.user_collateral_pubkey).await;

    let mut transaction = Transaction::new_with_payer(
        &[
            approve(
                &spl_token::id(),
                &gth_test_reserve.user_collateral_pubkey,
                &user_transfer_authority.pubkey(),
                &user_accounts_owner.pubkey(),
                &[],
                GTH_DEPOSIT_AMOUNT_WEIS,
            )
            .unwrap(),
            deposit_obligation_collateral(
                spl_token_lending::id(),
                GTH_DEPOSIT_AMOUNT_WEIS,
                gth_test_reserve.user_collateral_pubkey,
                gth_test_reserve.collateral_supply_pubkey,
                gth_test_reserve.pubkey,
                test_obligation.pubkey,
                lending_market.pubkey,
                test_obligation.owner,
                user_transfer_authority.pubkey(),
            ),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(
        &vec![&payer, &user_accounts_owner, &user_transfer_authority],
        recent_blockhash,
    );
    assert!(banks_client.process_transaction(transaction).await.is_ok());

    // check that collateral tokens were transferred
    let collateral_supply_balance =
        get_token_balance(&mut banks_client, gth_test_reserve.collateral_supply_pubkey).await;
    assert_eq!(
        collateral_supply_balance,
        initial_collateral_supply_balance + GTH_DEPOSIT_AMOUNT_WEIS
    );
    let user_collateral_balance =
        get_token_balance(&mut banks_client, gth_test_reserve.user_collateral_pubkey).await;
    assert_eq!(
        user_collateral_balance,
        initial_user_collateral_balance - GTH_DEPOSIT_AMOUNT_WEIS
    );
}
