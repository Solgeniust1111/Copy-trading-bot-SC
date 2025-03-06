use anchor_lang::{
    prelude::*,
    solana_program::{native_token::lamports_to_sol, program::invoke, system_instruction},
    system_program,
};
use anchor_spl::{
    associated_token::{self, AssociatedToken, Create},
    token::{self, spl_token, Mint, Token},
};
use raydium_amm_cpi::SwapBaseOut;

use crate::{
    consts::{
        OPENBOOK_DEV_ID, RAY_AMM_AUTH_SEED, RAY_AMM_COIN_VAULT_SEED, RAY_AMM_DEV_ID,
        RAY_AMM_OPENBOOK_ORDER_SEED, RAY_AMM_PC_VAULT_SEED, RAY_AMM_SEED, WRAP_SOL_MINT,
    },
    states::GlobalConfig,
};

#[derive(Accounts, Clone)]
pub struct RaydiumAMMSwapBaseOut<'info> {
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

impl<'a, 'b, 'c, 'info> From<&mut RaydiumAMMSwapBaseOut<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SwapBaseOut<'info>>
{
    fn from(
        accounts: &mut RaydiumAMMSwapBaseOut<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SwapBaseOut<'info>> {
    }
}

impl<'info> RaydiumAMMSwapBaseOut<'info> {
    pub fn wrap_sol(&mut self, max_amount_in: u64) -> Result<()> {
        let user_token_source: &UncheckedAccount<'info> = &self.user_token_source;
        let user_token_destination = &self.user_token_destination;

        // Ensure the source ATA exists or create it
        if self.user_token_source_mint.key() == WRAP_SOL_MINT {
            // Ensure the destination ATA exists or create it
            if !user_token_source.to_account_info().data_is_empty() {
                msg!("Wrap Sol Account is existing ... ");
            } else {
                // If the account doesn't exist, create it
                let create_ata_acc = Create {
                    associated_token: self.user_token_source.to_account_info(),
                    authority: self.user_source_owner.to_account_info(),
                    mint: self.user_token_source_mint.to_account_info(),
                    payer: self.user_source_owner.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                };
            }

            let cpi_context = CpiContext::new(
                self.system_program.to_account_info(),
                system_program::Transfer {
                    from: self.user_source_owner.to_account_info(),
                    to: self.user_token_source.to_account_info(),
                },
            );

            let _ = system_program::transfer(cpi_context, buy_amount);

            let ix: spl_token::solana_program::instruction::Instruction =
                spl_token::instruction::sync_native(
                    &spl_token::id(),
                    &self.user_token_source.key(),
                )?;
            let _ = invoke(&ix, &[self.user_token_source.to_account_info()]);
        }

        // Ensure the destination ATA exists or create it
        if !user_token_destination.to_account_info().data_is_empty() {
            msg!("User destination token account already exists.");
        } else {
            // If the account doesn't exist, create it
            let create_ata_acc = Create {
                associated_token: self.user_token_destination.to_account_info(),
                authority: self.user_source_owner.to_account_info(),
                mint: self.user_token_destination_mint.to_account_info(),
                payer: self.user_source_owner.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            };

            let cpi_program = self.associated_token_program.to_account_info();

            let cpi_ctx = CpiContext::new(cpi_program, create_ata_acc);
            let _ = associated_token::create(cpi_ctx);
            msg!("Created user destination token account.");
        }

        Ok(())
    }

    pub fn unwrap_sol(&mut self, amount_out: u64) -> Result<()> {
        // Ensure the source ATA exists or create it
        if self.user_token_destination_mint.key() == WRAP_SOL_MINT {
            msg!("Sell Token");

            // Ensure the destination ATA exists or create it
            if !self
                .user_token_destination
                .to_account_info()
                .data_is_empty()
            {
                msg!("Wrap Sol Account is existing ... ");
                let ix: spl_token::solana_program::instruction::Instruction =
                    spl_token::instruction::close_account(
                        &spl_token::id(),
                        &self.user_token_destination.key(),
                        &self.user_source_owner.key(),
                        &self.user_source_owner.key(),
                        &[&self.user_source_owner.key()],
                    )?;
                let _ = invoke(
                    &ix,
                    &[
                        self.user_token_destination.to_account_info(),
                        self.user_token_source.to_account_info(),
                        self.user_source_owner.to_account_info(),
                    ],
                );
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

            let fee_sol_amount = (self.global_config.fee_point * (amount_out as f32)) as u64;

            let _ = system_program::transfer(cpi_context, fee_sol_amount);
        }
        Ok(())
    }

    pub fn process(&mut self, max_amount_in: u64, amount_out: u64, timestamp: u64) -> Result<()> {
        let buy_amount;
        if self.user_token_source_mint.key() == WRAP_SOL_MINT {
            buy_amount = ((1.0 - self.global_config.fee_point) * (max_amount_in as f32)) as u64;
        } else {
            buy_amount = max_amount_in;
        }

        Ok(())
    }
}
