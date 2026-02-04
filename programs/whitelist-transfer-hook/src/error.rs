use anchor_lang::prelude::*;

#[error_code]
pub enum WhiteListError {
    #[msg("Unauthorized access")]
    Unauthorized,

     #[msg("whitelisted already")]
    AlreadyWhitelisted
}