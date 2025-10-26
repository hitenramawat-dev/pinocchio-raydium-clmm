use pinocchio::program_error::ProgramError;
use thiserror::Error;
use num_derive::FromPrimitive;




#[derive(shank::ShankType,Clone, Debug, Eq,PartialEq,Error,FromPrimitive)]
pub enum CustomError {

    #[error("invalid instruction data")]
    InvalidInstructionData
}




impl From<CustomError> for ProgramError {
    fn from(value: CustomError) -> Self {
        Self::Custom(value as u32)
    }
}


