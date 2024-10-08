mod claim;
mod close;
mod initialize;
mod mine;
mod open;
mod reset;
mod stake;
mod update;

use claim::*;
use close::*;
use initialize::*;
use mine::*;
use open::*;
use reset::*;
use stake::*;
use update::*;

use coal_api::instruction::*;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub(crate) use coal_utils as utils;

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&coal_api::id()) {
        println!("Program ID mismatch");
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    println!("Validated instruction data");
    match OreInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        OreInstruction::Claim => process_claim(accounts, data)?,
        OreInstruction::Close => process_close(accounts, data)?,
        OreInstruction::Mine => process_mine(accounts, data)?,
        OreInstruction::Open => process_open(accounts, data)?,
        OreInstruction::Reset => process_reset(accounts, data)?,
        OreInstruction::Stake => process_stake(accounts, data)?,
        OreInstruction::Update => process_update(accounts, data)?,
        OreInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}
