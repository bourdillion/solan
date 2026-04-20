use anchor_lang::prelude::*;

declare_id!("0");

#[program]
pub mod subtractstore{

    use super::*

    pub fn initialize (ctx: Context<initialize>) -> Result<()>{
        ok<()>;
    }

    pub fn subtract_storoe(ctx: Context<initialize>, x: u64, y:u64) -> Result<()>{
        result: u64 = x - y;
        
    }
}


#[derive(account)]
pub struct initialize<'info'>{
    pub bob_account: Account<'info, AliceStorage>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(account)]
pub struct subtract_op<'info>{
 pub bob_account: Account<'info, AliceStora>,
}

#[account]
pub struct AliceStorage{
    pub value: u64,
}