use pinocchio::{program_error::ProgramError};

pub mod admin;
pub use admin::*;




#[repr(u8)]
pub enum ClmmInstructions {
    // Config
    CreateAmmConfig,
    UpdateAmmConfig,
    CreateOperationAccount,
    UpdateOperationAccount,

    // Pool
    CreateSupportMintAta,
    CreatePool,
    UpdatePool,
    InitiaReward,
    SetRewardParams,
    UpdateRewardInfo,
    TransferRewardOwner,

    // LP
    OpenPosition,
    IncreaseLiquidity,
    DecreaseLiquidity,
    ClosePosition,

    // Trading
    Swap,

    // Admin
    CollectProtocolFee,
    CollectFundFee,
    CollectRemainingReward,
    CloseProtocolPosition,
}

impl TryFrom<&u8> for ClmmInstructions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0  => Ok(Self::CreateAmmConfig),
            1  => Ok(Self::UpdateAmmConfig),
            2  => Ok(Self::CreateOperationAccount),
            3  => Ok(Self::UpdateOperationAccount),
            4  => Ok(Self::CreateSupportMintAta),
            5  => Ok(Self::CreatePool),
            6  => Ok(Self::UpdatePool),
            7  => Ok(Self::InitiaReward),
            8  => Ok(Self::SetRewardParams),
            9  => Ok(Self::UpdateRewardInfo),
            10 => Ok(Self::TransferRewardOwner),
            11 => Ok(Self::OpenPosition),
            12 => Ok(Self::IncreaseLiquidity),
            13 => Ok(Self::DecreaseLiquidity),
            14 => Ok(Self::ClosePosition),
            15 => Ok(Self::Swap),
            16 => Ok(Self::CollectProtocolFee),
            17 => Ok(Self::CollectFundFee),
            18 => Ok(Self::CollectRemainingReward),
            19 => Ok(Self::CloseProtocolPosition),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}