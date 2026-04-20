use anchor_lang::prelude::*;

declare_id!("6YLSyMnkj59oohtsbuwSz3iocwb5aE3Lj5Kks5ES7935");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize> a: u64, b: u64) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
