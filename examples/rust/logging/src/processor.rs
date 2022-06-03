//! Program instruction processor

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    log::{gth_log_compute_units, gth_log_params, gth_log_slice},
    msg,
    pubkey::Pubkey,
};

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Log a string
    msg!("static string");

    // Log a slice
    gth_log_slice(instruction_data);

    // Log a formatted message, use with caution can be expensive
    msg!("formatted {}: {:?}", "message", instruction_data);

    // Log a public key
    program_id.log();

    // Log all the program's input parameters
    gth_log_params(accounts, instruction_data);

    // Log the number of compute units remaining that the program can consume.
    gth_log_compute_units();

    Ok(())
}
