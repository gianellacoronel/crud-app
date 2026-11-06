use anchor_lang::prelude::*;

declare_id!("DGZ7J8y8hsdK2fowQTVzFwuMq79wrsG9j1BHwRpJG22L");

#[program]
pub mod crud_app {
    use super::*;

    pub fn create(ctx: Context<Create>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.title = title;
        journal_entry.message = message;

        Ok(())
    }

    pub fn update(ctx: Context<Update>,) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + Journal::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub journal_entry: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Create<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Journal::INIT_SPACE,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub journal_entry: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Journal {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(200)]
    pub message: String,
}
