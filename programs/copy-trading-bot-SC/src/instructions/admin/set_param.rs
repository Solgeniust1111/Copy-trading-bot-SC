use anchor_lang::prelude::*;

use crate::{states::GlobalConfig, validation::is_admin};

#[derive(Accounts)]
pub struct SetParam<'info> {
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

impl<'info> SetParam<'info> {
    pub fn process(&mut self, fee_point: f32, new_fee_acc: Pubkey) -> Result<()> {
        is_admin(self.payer.key(), self.global_config.admin_addr.key());

        msg!(
            "New Fee Addr : {:?} , New Fee Point : {:?}",
            new_fee_acc.key(),
            fee_point
        );

        self.global_config.udpate_config(fee_point, new_fee_acc);

        Ok(())
    }
}
