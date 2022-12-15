use borsh::BorshDeserialize;
use maxapp::{process_instuction, GreeterCounter};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, instruction::AccountMeta,
    msg, program_error::ProgramError, pubkey::Pubkey, stake_history::Epoch,
};
use solana_program_test::*;
use std::{borrow::Borrow, str::FromStr};

use solana_sdk::{
    account::Account, instruction::Instruction, signer::Signer, transaction::Transaction,
};

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::from_str("2B43kW97uTFhhFsZx3mBkKPXVPr3VoB5rU3UjYn9q5oy").unwrap();
    let mut data = vec![0_u8; 4];

    let mut lamports = 10;
    let account = Pubkey::new_unique();
    let mut program_test = ProgramTest::new("max", program_id, processor!(process_instuction));
    program_test.add_account(
        account,
        Account {
            lamports: lamports,
            data: data,
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut client, payer, recblock_hash) = program_test.start().await;
    let greeter_account = client
        .get_account(account)
        .await
        .expect("get_account")
        .expect("failed to get account");
    assert_eq!(
        GreeterCounter::try_from_slice(greeter_account.data.borrow())
            .unwrap()
            .counter,
        0
    );

    let mut txn = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0],
            vec![AccountMeta::new(account, false)],
        )],
        Some(&payer.pubkey()),
    );

    txn.sign(&[&payer], recblock_hash);
    client.process_transaction(txn).await.unwrap();
    let greeter_account = client
        .get_account(account)
        .await
        .expect("get_account")
        .expect("failed to get account");
    assert_eq!(
        GreeterCounter::try_from_slice(greeter_account.data.borrow())
            .unwrap()
            .counter,
        1
    );
}
