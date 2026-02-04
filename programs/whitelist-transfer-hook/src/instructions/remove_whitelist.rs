use anchor_lang::prelude::*;

use crate::state::whitelist::Whitelist;
use crate::error::WhiteListError;

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct RemoveWhitelist<'info> {
    #[account(
        mut,
    )]
    pub admin: Signer<'info>,
    #[account(
        mut,
        close = admin,
        seeds = [b"whitelist", address.as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveWhitelist<'info> {
    pub fn remove_whitelist(&mut self) -> Result<()> {  
        require!(!self.whitelist.is_whitelisted, WhiteListError::AlreadyWhitelisted);
            self.whitelist.is_whitelisted = false;
        
        Ok(())
    }

}