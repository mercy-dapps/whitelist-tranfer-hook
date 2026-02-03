use anchor_lang::prelude::*;

use crate::state::WhitelistConfig;

#[derive(Accounts)]
pub struct InitializeWhitelistConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + WhitelistConfig::INIT_SPACE,
        seeds = [b"whitelist_config"],
        bump
    )]
    pub whitelist_config: Account<'info, WhitelistConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelistConfig<'info> {
    pub fn initialize_whitelist_config(
        &mut self, 
        bumps: InitializeWhitelistConfigBumps
    ) -> Result<()> {
        // Initialize the whitelistConfig to control access
        self.whitelist_config.set_inner(WhitelistConfig { 
            admin: self.admin.key(), 
            bump: bumps.whitelist_config 
        });

        msg!("created a whitelist config");

        Ok(())
    }
}