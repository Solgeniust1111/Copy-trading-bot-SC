use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;
pub mod validation;

use crate::instructions::*;

declare_id!("8Bt6v2aVESaAf2yGw8xjFY8R1YKgXbt4eDq5YcVKQVCf");

#[program]
pub mod copy_trading_bot_SC {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee_point: f32) -> Result<()> {
        let _ = ctx.accounts.process(fee_point);
        Ok(())
    }

    pub fn set_param(ctx: Context<SetParam>, fee_point: f32, new_fee_acc: Pubkey) -> Result<()> {
        let _ = ctx.accounts.process(fee_point, new_fee_acc);
        Ok(())
    }

    pub fn update_auth(ctx: Context<UpdateAuth>, new_fee_acc: Pubkey) -> Result<()> {
        let _ = ctx.accounts.process(new_fee_acc);
        Ok(())
    }

    pub fn pumpfun_buy(
        ctx: Context<PumpfunBuy>,
        amount: u64,
        max_sol_cost: u64,
        timestamp: u64,
    ) -> Result<()> {
        let _ = ctx.accounts.process(amount, max_sol_cost, timestamp);
        Ok(())
    }

    pub fn pumpfun_sell(
        ctx: Context<PumpfunSell>,
        amount: u64,
        min_sol_output: u64,
        timestamp: u64,
    ) -> Result<()> {
        let _ = ctx.accounts.process(
            amount,
            min_sol_output,
            timestamp,
            ctx.bumps.pumpfun_bonding_curve,
        );
        Ok(())
    }

    pub fn raydium_amm_swap_base_in(
        ctx: Context<RaydiumAMMSwapBaseIn>,
        amount_in: u64,
        minimum_amount_out: u64,
        timestamp: u64,
    ) -> Result<()> {
        let _ = ctx.accounts.wrap_sol(amount_in);
        let _ = ctx
            .accounts
            .process(amount_in, minimum_amount_out, timestamp);
        let _ = ctx.accounts.unwrap_sol(minimum_amount_out);

        Ok(())
    }

    pub fn raydium_amm_swap_base_out(
        ctx: Context<RaydiumAMMSwapBaseOut>,
        max_amount_in: u64,
        amount_out: u64,
        timestamp: u64,
    ) -> Result<()> {
        let _ = ctx.accounts.wrap_sol(max_amount_in);
        let _ = ctx.accounts.process(max_amount_in, amount_out, timestamp);
        let _ = ctx.accounts.unwrap_sol(amount_out);

        Ok(())
    }
}
