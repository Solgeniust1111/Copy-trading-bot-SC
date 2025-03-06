use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramError {
    #[msg("One Tx is already processed")]
    DupHashError,
    #[msg("Only Admin can invoke this function")]
    InvalidAdmin,
}
