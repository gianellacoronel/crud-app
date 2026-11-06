use anchor_lang::prelude::*;

declare_id!("DGZ7J8y8hsdK2fowQTVzFwuMq79wrsG9j1BHwRpJG22L");

#[program]
pub mod crud_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
