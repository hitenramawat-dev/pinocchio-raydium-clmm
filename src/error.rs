use pinocchio::program_error::ProgramError;








#[derive(shank::ShankType,Clone)]
pub enum CustomError {
    InvalidInstructionData
}

impl From<CustomError> for ProgramError {
    fn from(value: CustomError) -> Self {
        Self::Custom(value as u32)
    }
}