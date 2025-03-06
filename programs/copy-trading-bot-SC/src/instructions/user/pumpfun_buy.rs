use std::ops::Div;

use crate::{
    consts::{
        BONDING_CURVE_SEED, PUMPFUN_EVENT_AUTH, PUMPFUN_FEE_ACC, PUMPFUN_GLOBAL,
        PUMPFUN_PROGRAM_ADDRESS,
    },
    states::GlobalConfig,
    utils::buy_ix,
};
use anchor_lang::{
    prelude::*,
    solana_program::{native_token::lamports_to_sol, program::invoke, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct PumpfunBuy<'info> {
    /// CHECK:
    #[account(mut)]
    pub fee_acc: UncheckedAccount<'info>,

    #[account(
        seeds=[GlobalConfig::SEEDS],
        bump
    )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    pub mint_addr: Box<Account<'info, Mint>>,

    /// CHECK:
    #[account(
        seeds=[PUMPFUN_GLOBAL],
        bump,
        seeds::program=pumpfun_program.key()
    )]
    pub pumpfun_global_acc: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut, address=PUMPFUN_FEE_ACC)]
    pub pumpfun_fee_acc: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        mut,
        seeds=[BONDING_CURVE_SEED , mint_addr.key().as_ref()],
        seeds::program=pumpfun_program.key(),
        bump,
    )]
    pub pumpfun_bonding_curve: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint_addr,
        associated_token::authority = pumpfun_bonding_curve,
    )]
    pub pumpfun_bonding_curve_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    #[account(executable, address = PUMPFUN_PROGRAM_ADDRESS)]
    pub pumpfun_program: UncheckedAccount<'info>,

    /// CHECK:
    #[account(address = PUMPFUN_EVENT_AUTH)]
    pub event_authority: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer=payer,
        associated_token::mint = mint_addr,
        associated_token::authority = payer,
    )]
    pub payer_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> PumpfunBuy<'info> {
    pub fn process(&mut self, token_amount: u64, max_sol_cost: u64, timestamp: u64) -> Result<()> {
        Ok(())
    }
}
