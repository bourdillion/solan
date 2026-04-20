use anchor_lang::prelude::*;

declare_id!("AA7CVC6ibLmMNW2LJfCs7rniUrkQh7rKRbU3WKPbV2AN");

#[program]
pub mod sysvars {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //get the clock sysvar
        let clock = Clock::get()?;
        msg!("cloc: {:?}", clock);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
