use anchor_lang::prelude::{borsh::de, *};

declare_id!("D7avnnLHYnvZMHVSpc8gBcr6esG1ZpqQm9vpR2E8ubSA");

#[program]
pub mod counter {
    use super::*;

     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter_account.count += 1;
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"counter", user.key().as_ref()],
        bump,
        space = 8 + 8 // 8 for anchor's discriminator + 8 for u64
    )]
    pub counter_account: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()],
        bump,
    )]
    pub counter_account: Account<'info, Counter>,

    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
