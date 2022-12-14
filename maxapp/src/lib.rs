use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
struct GreeterCounter {
    counter: u32,
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
