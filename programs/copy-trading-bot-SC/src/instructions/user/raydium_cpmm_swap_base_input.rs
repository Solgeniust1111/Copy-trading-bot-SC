use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use raydium_cpmm_cpi::{
    cpi,
    program::RaydiumCpmm,
    states::{AmmConfig, ObservationState, PoolState},
};

#[derive(Accounts)]
pub struct RaydiumCpmmSwapBaseInput<'info> {
    pub cp_swap_program: Program<'info, RaydiumCpmm>,
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// CHECK: pool vault and lp mint authority
    #[account(
      seeds = [
        raydium_cpmm_cpi::AUTH_SEED.as_bytes(),
      ],
      seeds::program = cp_swap_program.key(),
      bump,
  )]
    pub authority: UncheckedAccount<'info>,

    /// The factory state to read protocol fees
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for input token
    #[account(
      mut,
      constraint = input_vault.key() == pool_state.load()?.token_0_vault || input_vault.key() == pool_state.load()?.token_1_vault
  )]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for output token
    #[account(
      mut,
      constraint = output_vault.key() == pool_state.load()?.token_0_vault || output_vault.key() == pool_state.load()?.token_1_vault
  )]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// SPL program for input token transfers
    pub input_token_program: Interface<'info, TokenInterface>,

    /// SPL program for output token transfers
    pub output_token_program: Interface<'info, TokenInterface>,

    /// The mint of input token
    #[account(
      address = input_vault.mint
  )]
    pub input_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of output token
    #[account(
      address = output_vault.mint
  )]
    pub output_token_mint: Box<InterfaceAccount<'info, Mint>>,
    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,
}

impl<'info> RaydiumCpmmSwapBaseInput<'info> {
    pub fn proxy_swap_base_input(&mut self, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        let cpi_accounts = cpi::accounts::Swap {
            payer: self.payer.to_account_info(),
            authority: self.authority.to_account_info(),
            amm_config: self.amm_config.to_account_info(),
            pool_state: self.pool_state.to_account_info(),
            input_token_account: self.input_token_account.to_account_info(),
            output_token_account: self.output_token_account.to_account_info(),
            input_vault: self.input_vault.to_account_info(),
            output_vault: self.output_vault.to_account_info(),
            input_token_program: self.input_token_program.to_account_info(),
            output_token_program: self.output_token_program.to_account_info(),
            input_token_mint: self.input_token_mint.to_account_info(),
            output_token_mint: self.output_token_mint.to_account_info(),
            observation_state: self.observation_state.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.cp_swap_program.to_account_info(), cpi_accounts);
        cpi::swap_base_input(cpi_context, amount_in, minimum_amount_out)
    }
}
