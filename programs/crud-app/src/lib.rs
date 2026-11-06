use anchor_lang::prelude::*;

declare_id!("DGZ7J8y8hsdK2fowQTVzFwuMq79wrsG9j1BHwRpJG22L");

#[program]
pub mod crud_app {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        initialize::handler(ctx)
    }
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
