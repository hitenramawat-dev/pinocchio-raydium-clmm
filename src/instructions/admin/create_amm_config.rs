
use pinocchio::instruction::{Seed, Signer};
use pinocchio::pubkey::{self, Pubkey};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, sysvars::rent::Rent, ProgramResult};
use pinocchio_system::instructions::CreateAccount;

use crate::state::{AmmConfig, AMM_CONFIG_SEED};
use crate::{require, DataLen, ID};
use crate::utils::load_ix_data;



pub fn process_create_amm(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    data:&[u8]
) -> ProgramResult {

    let [owner,amm_confg,sysvar_rent_acc, _system_program] = accounts else {
        return Err(ProgramError::InsufficientFunds);
    };

    require!(owner.is_signer(),ProgramError::MissingRequiredSignature);
    require!(amm_confg.data_is_empty(),ProgramError::AccountAlreadyInitialized);
    require!(amm_confg.is_owned_by(&ID),ProgramError::IllegalOwner);

    let rent = Rent::from_account_info(sysvar_rent_acc).unwrap();

    let ix_data = unsafe {load_ix_data::<AmmConfig>(data)?};

    require!(ix_data.owner.ne(owner.key()),ProgramError::IllegalOwner);


    let index = ix_data.index.to_le_bytes();


    let seeds = [AMM_CONFIG_SEED.as_bytes(),&index];

    let (amm_pda,bump)  = pubkey::find_program_address(&seeds, &program_id);

    require!(amm_pda.ne(amm_confg.key()),ProgramError::InvalidAccountOwner);

    let bumps = [bump];

    let signer_seeds = [
        Seed::from(AMM_CONFIG_SEED.as_bytes()),
        Seed::from(&ix_data.owner),
        Seed::from(&bumps),
    ];

    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: owner,
        to: amm_confg,
        space: AmmConfig::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(AmmConfig::LEN),
    }
    .invoke_signed(&signers)?;

    AmmConfig::initialize(amm_confg, &ix_data, bump)?;

    Ok(())
}