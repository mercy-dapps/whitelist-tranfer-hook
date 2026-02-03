use anchor_lang::{ 
    prelude::*, 
};
use anchor_spl::{token_2022::{self, MintTo}, token_interface::{
    Mint, TokenAccount, TokenInterface
}};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::Whitelist;
use crate::ID;

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct TokenFactory<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        mint::token_program = token_program,
        extensions::transfer_hook::authority = user,
        extensions::transfer_hook::program_id = ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    // ATA to mint tokens to
    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: ExtraAccountMetaList Account, will be checked by the transfer hook
    #[account(mut)]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    #[account(
        seeds = [b"whitelist", address.as_ref()], 
        bump
    )]
    pub blocklist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> TokenFactory<'info> {
    pub fn init_mint(&mut self, _bumps: &TokenFactoryBumps) -> Result<()> {

        let mint_amount = 100_000_000_000; // 100 token with 9 decimals

        let mint_to_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), mint_to_accounts);
        token_2022::mint_to(cpi_ctx, mint_amount)?;

        msg!("Minted {} tokens to {}", mint_amount, self.ata.key());

        Ok(())
    }
}