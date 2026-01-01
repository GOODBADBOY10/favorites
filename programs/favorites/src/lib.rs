use anchor_lang::prelude::*;

declare_id!("6HxogBU2AvwFmQLqyxQsJMaWuhaDoZ4Qx2czCYsuZbTw");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
