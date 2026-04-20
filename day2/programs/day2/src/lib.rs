use anchor_lang::prelude::*;

declare_id!("AQ7nqdoypwz4oZyis7yqeTt2xAj84DambxuqJnFzmr6k");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array{:?}", arr);
        Ok(())
    }

    pub fn subtract(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let answer: u64 = a.checked_sub(b).unwrap();
        msg!("Your answer is {}", answer);
        Ok(())
    }

    pub fn cbrt(ctx: Context<Initialize>, a: f32) -> Result<()> {
        let answer: f32 = a.cbrt();
        msg!("Your cbrt answer is {}", answer);
        Ok(())
    }
    pub fn calculator(ctx: Context<Initialize>, a: f32, b: f32) -> Result<()> {
        let addition: f32 = a + b;
        let subtraction: f32 = a - b;
        let multiplication: f32 = a * b;
        let division: f32 = a / b;
        let sqrt: f32 = a.sqrt();
        let base10: f32 = a.log10();
        msg!("Your addition answer is {}", addition);
        msg!("Your substraction answer is {}", subtraction);
        msg!("Your multiplication answer is {}", multiplication);
        msg!("Your division answer is {}", division);
        msg!("Your sqrt answer is {}", sqrt);
        msg!("Your base10 answer is {}", base10);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
