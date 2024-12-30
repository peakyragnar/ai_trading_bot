use anchor_lang::prelude::*;
pub mod vault;
use crate::vault::{DepositTokens, WithdrawTokens}; // Import structs directly

declare_id!("Rqr8zpYgDDhiEY9DWM4E4deyQU2wuQbyvVBnFr3yw6K");

#[program]
pub mod ai_trading_bot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello from the main program!");
        Ok(())
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        self::vault::deposit_tokens(ctx, amount)
    }

    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, amount: u64) -> Result<()> {
        self::vault::withdraw_tokens(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}