use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount}
};

declare_id!("C8Rd2i4EFiKmJxAfucJvavFU7QBwK8Vb9MQCAxEabB5a");

// #[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn init_nft(ctx: Context<InitNFT>) -> Result<()> {
        // create mint account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo{
                        mint:
                ctx.accounts.mint.to_account_info(),
                to:
                ctx.accounts.associated_token_account.to_account_info(),
                authority:
                ctx.accounts.signer.to_account_info(),
            },
        );

        mint_to(cpi_context, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing in this account by ourselves
    #[account(mut, signer)]
    signer: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: Account<'info,  Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>

}
