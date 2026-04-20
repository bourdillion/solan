use anchor_lang::prelude::*;

declare_id!("CotwLUmAuKEePbCfoziRuPEGgPKXhZ4RFYZmQWXdkxwA");

#[program]
pub mod day4 {
    use super::*;

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AIsTooSmall);
        require!(a <= 100, MyError::AIsTooBig);

        msg!("the value is {}", a);
        Ok(())
    }

    pub fn func(ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        return err!(MyError::AlwaysErrors);
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too big")]
    AIsTooBig,
    #[msg("Always errors")]
    AlwaysErrors,
    #[msg("a is too small")]
    AIsTooSmall,
}
