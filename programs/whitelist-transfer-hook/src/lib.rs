#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;

use instructions::*;

use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::{
    instruction::{
        ExecuteInstruction, 
        InitializeExtraAccountMetaListInstruction
    },
};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;

declare_id!("DhzyDgCmmQzVC4vEcj2zRGUyN8Mt5JynfdGLKkBcRGaX");

#[program]
pub mod whitelist_transfer_hook {
    use super::*;

    pub fn mint_token(ctx: Context<TokenFactory>) -> Result<()> {
        ctx.accounts.init_mint(&ctx.bumps)
    }

    pub fn initialize_whitelist_config(ctx: Context<InitializeWhitelistConfig>) -> Result<()> {
        ctx.accounts.initialize_whitelist_config(ctx.bumps)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, address: Pubkey) -> Result<()> {
        ctx.accounts.add_to_whitelist(address, ctx.bumps)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, address: Pubkey) -> Result<()> {
        ctx.accounts.remove_from_whitelist(address)
    }

    pub fn initialize_transfer_hook(
        ctx: Context<InitializeExtraAccountMetaList>,
        address: Pubkey
    ) -> Result<()> {

        msg!("Initializing Transfer Hook...");

        // Get the extra account metas for the transfer hook
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas(address)?;

        msg!("Extra Account Metas: {:?}", extra_account_metas);
        msg!("Extra Account Metas Length: {}", extra_account_metas.len());

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas
        ).unwrap();

        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // Call the transfer hook logic
        ctx.accounts.transfer_hook(amount)
    }
}
