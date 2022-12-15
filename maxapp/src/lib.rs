use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    stake_history::Epoch,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GreeterCounter {
    pub counter: u32,
}

entrypoint!(process_instuction);

pub fn process_instuction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("owner should be the program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    let mut greeter_counter = GreeterCounter::try_from_slice(&account.data.borrow())?;
    greeter_counter.counter += 1;

    greeter_counter.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[test]
fn test_helloworld() {
    let program_id = Pubkey::default();
    let key = Pubkey::default();
    let owner = Pubkey::default();
    let mut data = vec![0_u8; 4];

    let mut lamports = 0;
    let account = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let instruction_data: [u8; 1] = [0];
    let accounts = [account];

    assert_eq!(
        GreeterCounter::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter,
        0
    );

    process_instuction(&program_id, &accounts, &instruction_data);

    assert_eq!(
        GreeterCounter::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter,
        1
    );
}
