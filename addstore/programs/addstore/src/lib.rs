use anchor_lang::prelude::*;

declare_id!("CaTNJvizMo6CoCMvgqswe2gGTeLtL2GfoMKEGM4kSR5A");

#[program]
pub mod addstore {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
