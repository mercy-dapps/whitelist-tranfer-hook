 use anchor_lang::prelude::*;

use crate::{error::WhiteListError, state::*};

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"whitelist_config"],
        bump = whitelist_config.bump,
        constraint = whitelist_config.admin == admin.key() @ WhiteListError::Unauthorized
    )]
    pub whitelist_config: Account<'info, WhitelistConfig>,

    #[account(
        mut,
        close = admin,
        seeds = [b"whitelist", address.as_ref()],
        bump = whitelist.bump
    )]
    pub whitelist: Account<'info, Whitelist>,

}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(
        &mut self,
        _address: Pubkey) -> Result<()> {

        msg!("Address removed from whitelist");

        Ok(())
    }
}