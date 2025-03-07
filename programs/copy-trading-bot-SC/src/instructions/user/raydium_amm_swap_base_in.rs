use anchor_lang::{
    prelude::*,
    solana_program::{native_token::lamports_to_sol, program::invoke, system_instruction},
    system_program,
};
use anchor_spl::{
    associated_token::{self, AssociatedToken, Create},
    token::{spl_token, Mint, Token},
};
use raydium_amm_cpi::SwapBaseIn;

use crate::{
    consts::{
        OPENBOOK_DEV_ID, RAY_AMM_AUTH_SEED, RAY_AMM_COIN_VAULT_SEED, RAY_AMM_DEV_ID,
        RAY_AMM_OPENBOOK_ORDER_SEED, RAY_AMM_PC_VAULT_SEED, RAY_AMM_SEED, WRAP_SOL_MINT,
    },
    states::GlobalConfig,
};

#[derive(Accounts, Clone)]
pub struct RaydiumAMMSwapBaseIn<'info> {
    /// CHECK:
    #[account(mut)]
    pub fee_acc: UncheckedAccount<'info>,

    #[account(
            seeds=[GlobalConfig::SEEDS],
            bump
        )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    /// CHECK: Safe
    #[account(mut , address=RAY_AMM_DEV_ID)]
    pub amm_program: UncheckedAccount<'info>,
    /// CHECK: Safe. amm Account
    #[account(
        mut,
        seeds=[amm_program.key().as_ref() , market.key().as_ref() , RAY_AMM_SEED],
        seeds::program=amm_program.key(),
        bump
    )]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority Account
    #[account(
        mut,
        seeds=[RAY_AMM_AUTH_SEED],
        seeds::program=amm_program.key(),
        bump
    )]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(
        mut,
        seeds=[amm_program.key().as_ref() , market.key().as_ref() , RAY_AMM_OPENBOOK_ORDER_SEED],
        seeds::program=amm_program.key(),
        bump
    )]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Amm Account to swap FROM or To,
    #[account(
        mut,
        seeds=[amm_program.key().as_ref() , market.key().as_ref() , RAY_AMM_COIN_VAULT_SEED],
        seeds::program=amm_program.key(),
        bump
    )]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Amm Account to swap FROM or To,
    #[account(
        mut,
        seeds=[amm_program.key().as_ref() , market.key().as_ref() , RAY_AMM_PC_VAULT_SEED],
        seeds::program=amm_program.key(),
        bump
    )]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut , address=OPENBOOK_DEV_ID)]
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market Account. OpenBook program is the owner.
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. bids Account
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    /// CHECK: Safe. asks Account
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    /// CHECK: Safe. event_q Account
    #[account(mut)]
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. coin_vault Account
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. pc_vault Account
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. vault_signer Account
    #[account(mut)]
    pub market_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    pub user_token_source_mint: Account<'info, Mint>,
    pub user_token_destination_mint: Account<'info, Mint>,

    /// CHECK: Safe. user source token Account. user Account to swap from.
    #[account(
        mut,
        seeds = [
            &user_source_owner.key().to_bytes(),
            &token_program.key().to_bytes(),
            &user_token_source_mint.key().to_bytes(),
            ],
        seeds::program=associated_token_program.key(),
        bump,
    )]
    pub user_token_source: UncheckedAccount<'info>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(
        mut,
        seeds = [
            &user_source_owner.key().to_bytes(),
            &token_program.key().to_bytes(),
            &user_token_destination_mint.key().to_bytes(),
            ],
        seeds::program=associated_token_program.key(),
        bump,
    )]
    pub user_token_destination: UncheckedAccount<'info>,
    /// CHECK: Safe. user owner Account
    #[account(mut)]
    pub user_source_owner: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'a, 'b, 'c, 'info> From<&mut RaydiumAMMSwapBaseIn<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>>
{
    fn from(
        accounts: &mut RaydiumAMMSwapBaseIn<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>> {
        let cpi_accounts = SwapBaseIn {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),
            market_program: accounts.market_program.clone(),
            market: accounts.market.clone(),
            market_bids: accounts.market_bids.clone(),
            market_asks: accounts.market_asks.clone(),
            market_event_queue: accounts.market_event_queue.clone(),
            market_coin_vault: accounts.market_coin_vault.clone(),
            market_pc_vault: accounts.market_pc_vault.clone(),
            market_vault_signer: accounts.market_vault_signer.clone(),
            user_token_source: accounts.user_token_source.clone(),
            user_token_destination: accounts.user_token_destination.clone(),
            user_source_owner: accounts.user_source_owner.clone(),
            token_program: accounts.token_program.clone(),
        };
        let cpi_program = accounts.amm_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> RaydiumAMMSwapBaseIn<'info> {
    pub fn wrap_sol(&mut self, amount_in: u64) -> Result<()> {
        let user_token_source: &UncheckedAccount<'info> = &self.user_token_source;
        let user_token_destination = &self.user_token_destination;

        // Ensure the source ATA exists or create it
        if self.user_token_source_mint.key() == WRAP_SOL_MINT {
        } else {
            // If the account doesn't exist, create it
        }
        // Ensure the destination ATA exists or create it
        if !user_token_destination.to_account_info().data_is_empty() {
            msg!("User destination token account already exists.");
        } else {
            // If the account doesn't exist, create it

            msg!("Created user destination token account.");
        }

        Ok(())
    }

    pub fn unwrap_sol(&mut self, minimum_amount_out: u64) -> Result<()> {
        // Ensure the source ATA exists or create it
        if self.user_token_destination_mint.key() == WRAP_SOL_MINT {
            msg!("Sell Token");

            // Ensure the destination ATA exists or create it
            if !self
                .user_token_destination
                .to_account_info()
                .data_is_empty()
            {
            } else {
                msg!("No Wrap Sol Account");
            }

            let cpi_context = CpiContext::new(
                self.system_program.to_account_info(),
                system_program::Transfer {
                    from: self.user_source_owner.to_account_info(),
                    to: self.fee_acc.to_account_info(),
                },
            );

            let fee_sol_amount =
                (self.global_config.fee_point * (minimum_amount_out as f32)) as u64;

            let _ = system_program::transfer(cpi_context, fee_sol_amount);
        }
        Ok(())
    }

    pub fn process(
        &mut self,
        amount_in: u64,
        minimum_amount_out: u64,
        timestamp: u64,
    ) -> Result<()> {
        let buy_amount;
        if self.user_token_source_mint.key() == WRAP_SOL_MINT {
            buy_amount = ((1.0 - self.global_config.fee_point) * (amount_in as f32)) as u64;
        } else {
            buy_amount = amount_in;
        }

        Ok(())
    }
}
