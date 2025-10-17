use anchor_lang::prelude::*;

// Program ID remains the same
declare_id!("4vWfuD581LqXkKEiB2WLScSvpxWj2ssgwLJPbd6qZ3Kc");

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter_account.counter = 0;
        msg!(
            "Counter account initialized to: {}",
            ctx.accounts.counter_account.counter
        );
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = counter_account
            .counter
            .checked_add(1)
            .ok_or(ProgramError::InvalidArgument)?;
        msg!("Counter incremented to: {}", counter_account.counter);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = counter_account
            .counter
            .checked_sub(1)
            .ok_or(ProgramError::InvalidArgument)?;
        msg!("Counter decremented to: {}", counter_account.counter);
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: i128) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = value;
        msg!("Counter set to: {}", counter_account.counter);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + Counter::INIT_SPACE 
    )]
    pub counter_account: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub counter: i128,
}
