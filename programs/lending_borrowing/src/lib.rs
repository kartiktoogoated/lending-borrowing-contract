use anchor_lang::prelude::*;

declare_id!("6UYDtYy1tHWzVfX7YTG9GiHXyLU41vFzjjjRG1KjC6rE");

#[program]
pub mod lending_borrowing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
