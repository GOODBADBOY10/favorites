use anchor_lang::prelude::*;

declare_id!("6HxogBU2AvwFmQLqyxQsJMaWuhaDoZ4Qx2czCYsuZbTw");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn set_favorites() -> Result<()> {}
}

#[account]
#[derive(InitSpace)]
pub struct Favorite {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct Initialize {}