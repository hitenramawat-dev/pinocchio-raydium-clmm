use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::require;








pub fn process_update_config(
    accounts:&[AccountInfo],
    data:&u8
) -> ProgramResult {

    let [owner,amm_config, _system_program] = accounts else {
        return Err(ProgramError::InsufficientFunds);
    };

     require!(owner.is_signer(),ProgramError::MissingRequiredSignature);
     require!(amm_config.is_writable(),ProgramError::InvalidAccountData);
     require!(amm_config.is_owned_by(&crate::ID),ProgramError::IllegalOwner);


    Ok(())
}