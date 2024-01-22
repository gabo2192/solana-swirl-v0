pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
use {anchor_lang::prelude::*, instructions::*};

declare_id!("8vqgdewQChSQVdnBcYCUftq5RGKFjoandbzMFPJ3n69u");

#[program]
pub mod solana_swirl_v0 {
    use super::*;
    pub fn init_pool(ctx: Context<InitializePool>)-> Result<()>{
        init_pool::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
