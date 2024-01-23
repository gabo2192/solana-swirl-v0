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
        init_pool::handler_init_pool(ctx)
    }
    pub fn init_stake_entry(ctx: Context<InitEntryCtx>) -> Result<()> {
        init_stake_entry::handler(ctx)
    }
    pub fn stake(ctx: Context<StakeCtx>, amount: u64) -> Result<()> {
        stake::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<UnstakeCtx>) -> Result<()> {
        unstake::unstake(ctx)
    }
}
