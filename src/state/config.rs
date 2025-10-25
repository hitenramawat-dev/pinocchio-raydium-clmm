use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

use crate::{try_from_account_info_mut, utils::{DataLen, Initialized}};


pub const AMM_CONFIG_SEED: &str = "amm_config";

pub const FEE_RATE_DENOMINATOR_VALUE: u32 = 1_000_000;



#[repr(C)] 
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct AmmConfig {
    pub init:u8,
    pub bump: u8,
    pub index: u16,
    pub owner: Pubkey,
    pub protocol_fee_rate: u32,
    pub trade_fee_rate: u32,
    pub tick_spacing: u16,
    pub fund_fee_rate: u32,
    pub padding_u32: u32,
    pub fund_owner: Pubkey,
    pub padding: [u64; 3],
}


impl DataLen for AmmConfig {
    const LEN: usize = 8 + 1 + 2 + 32 + 4 + 4 + 2 + 64;
}

impl  Initialized for AmmConfig{
    fn is_initialized(&self) -> bool {
        self.init > 0
    }
}

impl AmmConfig {
    pub fn is_authorized<'info>(
        &self,
        signer:&AccountInfo,
        expect_pubkey: &Pubkey,
    ) -> Result<(),ProgramError> {

        if signer.key() == &self.owner || expect_pubkey == signer.key() {
            return Err( ProgramError::InvalidAccountOwner);
        }
        Ok(())
    }

    pub fn initialize(amm_pda:&AccountInfo,data:&AmmConfig,bump:u8) -> ProgramResult {

        let current_state = unsafe {
            try_from_account_info_mut::<AmmConfig>(amm_pda)
        }?;

        current_state.bump = bump;
        current_state.owner = data.owner;

        current_state.index = data.index;
        current_state.trade_fee_rate = data.trade_fee_rate;

        current_state.fund_owner = data.fund_owner;
        current_state.fund_fee_rate = data.fund_fee_rate;

        current_state.tick_spacing = data.tick_spacing;
        current_state.protocol_fee_rate = data.protocol_fee_rate;


        Ok(())

    }
}