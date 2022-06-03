//! Program instruction processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Create an iterator to safely reference accounts in the slice
    let account_info_iter = &mut accounts.iter();

    // As part of the program specification the first account is the source
    // account and the second is the destination account
    let source_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;

    // Withdraw five weis from the source
    **source_info.try_borrow_mut_weis()? -= 5;
    // Deposit five weis into the destination
    **destination_info.try_borrow_mut_weis()? += 5;

    Ok(())
}
