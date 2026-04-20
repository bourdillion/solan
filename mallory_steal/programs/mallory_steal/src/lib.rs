use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("7NN6hrseaG1HTt39SiM61cRpedyJHtc3pDfkbTyNVAza");

const STARTING_POINTS: u32 = 10;

#[program]
pub mod mallory_steal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.player.points = STARTING_POINTS;
        ctx.accounts.player.authority = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn transferPoints(ctx: Context<TransferPoints>, amount: u32) -> Result<()> {
        require!(
            ctx.accounts.from.authority == ctx.accounts.signer.key(),
            Errors::SignerIsNotAuthority
        );
        require!(
            ctx.accounts.from.points >= amount,
            Errors::InsufficientPoints
        );
        ctx.accounts.from.points -= amount;
        ctx.accounts.to.points += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer,
space = size_of::<Player>() + 8, seeds = [&(signer.as_ref().key().to_bytes())], bump)]
    pub player: Account<'info, Player>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferPoints<'info> {
    #[account(mut)]
    from: Account<'info, Player>,
    #[account(mut)]
    to: Account<'info, Player>,
    #[account(mut)]
    signer: Signer<'info>,
}

#[account]
pub struct Player {
    points: u32,
    authority: Pubkey,
}

#[error_code]
pub enum Errors {
    #[msg("Signer is not authority")]
    SignerIsNotAuthority,
    #[msg("Insufficient points")]
    InsufficientPoints,
}
