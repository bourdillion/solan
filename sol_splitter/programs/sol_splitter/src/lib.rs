use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("9rfy2piRp5YHiiGpC3ekrJ5RmpRZjsSHWYxUikaK7VPo");

#[program]
pub mod sol_splitter {
    use super::*;

    pub fn send_sol(ctx: Context<SendSol>, amount1: u64, amount2: u64) -> Result<()> {
        let cpi_context1 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        );
        let cpi_context2 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.recipientb.to_account_info(),
            },
        );
        let res = system_program::transfer(cpi_context1, amount1);
        let res = system_program::transfer(cpi_context2, amount2);

        if res.is_ok() {
            return Ok(());
        } else {
            return err!(Errors::TransferFailed);
        }
    }
}

#[error_code]
pub enum Errors {
    #[msg("transfer failed")]
    TransferFailed,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    /// CHECK: we do not read or write the data of this account
    #[account(mut)]
    recipient: UncheckedAccount<'info>,

    /// CHECK: we do not read or write the data of this account
    #[account(mut)]
    recipientb: UncheckedAccount<'info>,

    system_program: Program<'info, System>,
    #[account(mut)]
    signer: Signer<'info>,
}
