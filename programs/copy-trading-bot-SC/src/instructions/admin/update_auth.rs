use anchor_lang::prelude::*;

use crate::{states::GlobalConfig, validation::is_admin};

#[derive(Accounts)]
pub struct UpdateAuth<'info> {
    #[account(
        mut,
        seeds=[GlobalConfig::SEEDS],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateAuth<'info> {
    pub fn process(&mut self, new_admin: Pubkey) -> Result<()> {
        is_admin(self.payer.key(), self.global_config.admin_addr.key());

        self.global_config.update_admin(new_admin);

        Ok(())
    }
}
