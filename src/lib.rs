use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use solana_program::{program::invoke_signed, system_program};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgWRHPxMKP6S");

#[program]
pub mod minimal_nft {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, name: String, symbol: String, uri: String) -> Result<()> {
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        let bump = *ctx.bumps.get("mint_authority").unwrap();
        let seeds = &[b"authority", &[bump]];
        let signer_seeds = &[&seeds[..]];

        let metadata_ix = create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.mint_authority.key(),
            name,
            symbol,
            uri,
            None,
            500,  // 5% royalty, adjust as needed
            true,
            false,
            None,
            None,
            None,
        );

        invoke_signed(
            &metadata_ix,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
            ],
            signer_seeds,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: Mint authority PDA or key
    #[account(seeds = [b"authority"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Metadata account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: Metaplex metadata program
    pub token_metadata_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

