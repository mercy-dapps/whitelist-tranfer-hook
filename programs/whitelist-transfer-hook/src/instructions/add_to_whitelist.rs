use anchor_lang::prelude::*;

use crate::{error::WhiteListError, state::*};

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"whitelist_config"],
        bump = whitelist_config.bump,
        constraint = whitelist_config.admin == admin.key() @ WhiteListError::Unauthorized
    )]
    pub whitelist_config: Account<'info, WhitelistConfig>,

    #[account(
        init,
        payer = admin,
        space = 8 + Whitelist::INIT_SPACE,
        seeds = [b"whitelist", address.as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddToWhitelist<'info> {
    pub fn add_to_whitelist(
        &mut self,
        address: Pubkey,
        bumps: AddToWhitelistBumps) -> Result<()> {
        // add address to whitelist
        self.whitelist.set_inner(Whitelist { 
            address,
            is_whitelisted: true,
            bump: bumps.whitelist
        });

        msg!("Address added to whitelist");

        Ok(())
    }
}