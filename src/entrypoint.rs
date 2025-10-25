#![allow(unexpected_cfgs)]

use pinocchio::{account_info::AccountInfo, default_panic_handler, program_entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

use crate::{create_amm_config, instructions::ClmmInstructions, process_create_amm};




program_entrypoint!(process_instruction);


default_panic_handler!();




pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
) -> ProgramResult {

    let (ix_disc,ix_data) = instruction_data.split_first().ok_or(ProgramError::InvalidArgument)?;


    match ClmmInstructions::try_from(ix_disc) {
        Ok(instruction) => match instruction {
            ClmmInstructions::CreateAmmConfig => { process_create_amm(program_id,accounts,ix_data) ?}
            ClmmInstructions::UpdateAmmConfig => {}
            ClmmInstructions::CreateOperationAccount => {}
            ClmmInstructions::UpdateOperationAccount => {}


            ClmmInstructions::CreateSupportMintAta => {}
            ClmmInstructions::CreatePool => {}
            ClmmInstructions::UpdatePool => {}
            ClmmInstructions::InitiaReward => {}
            ClmmInstructions::SetRewardParams => {}
            ClmmInstructions::UpdateRewardInfo => {}
            ClmmInstructions::TransferRewardOwner => {}


            ClmmInstructions::OpenPosition => {}
            ClmmInstructions::IncreaseLiquidity => {}
            ClmmInstructions::DecreaseLiquidity => {}
            ClmmInstructions::ClosePosition => {}


            ClmmInstructions::Swap => {}

            ClmmInstructions::CollectProtocolFee => {}
            ClmmInstructions::CollectFundFee => {}
            ClmmInstructions::CollectRemainingReward => {}
            ClmmInstructions::CloseProtocolPosition => {}

        },
        Err(_) => {
          return   Err(ProgramError::InvalidInstructionData) ;
        }
    }


    Ok(())
}