use anchor_lang::prelude::*;

declare_id!("EkkRQzE2pDWJ1Kfypu9tqaZkKJs9ae8iKugmCHXMKj5a");

#[program]
pub mod ai_trading_bot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
