use anchor_lang::prelude::*;

declare_id!("BLbK6c6DPAerbsyCWa8FiPjT6MJBT5HBbh6SHo9SNYKF");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
