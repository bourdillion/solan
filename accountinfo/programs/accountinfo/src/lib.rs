use anchor_lang::prelude::*;

declare_id!("7d1z2TeayKX2rQ4tahdy92SUQHUU8kFNAZA5dCQJFjEi");

// NOTE: Replace with your wallet's public key
const OWNER: &str = "AUHT8ECJVEHp9Fanykgxk78DtKmmnHMYzgcXFVmkaCMy";

#[program]
pub mod day14 {
    use super::*;

    #[access_control(check(&ctx))]
    pub fn initialize(ctx: Context<OnlyOwner>) -> Result<()> {
        // Function logic...

        msg!("Holla, I'm the owner.");
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

// An enum for custom error codes
#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}
