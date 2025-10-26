use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

use crate::error::CustomError;




pub trait DataLen {
    const LEN: usize;
}

pub trait Initialized {
    fn is_initialized(&self) -> bool;
}


#[inline(always)]
pub fn load_ix_data<T:DataLen>(data:&[u8]) -> Result<&T,ProgramError> {

    if data.len() != T::LEN {
        return Err(CustomError::InvalidInstructionData.into());
    }

    Ok(unsafe { &*( data.as_ptr() as *const T)})
}


#[inline(always)]
pub fn try_from_account_info_mut<T:DataLen>(
    account:&AccountInfo
) -> Result<&mut T ,ProgramError> {

    if account.owner() != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    let mut bytes = account.try_borrow_mut_data()?;

    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok( unsafe {
        &mut *(bytes.as_mut_ptr() as *mut T) 
    })
}