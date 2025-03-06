use anchor_lang::prelude::*;

use crate::states::GlobalConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer=payer,
        seeds=[GlobalConfig::SEEDS],
        space=GlobalConfig::SIZE,
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// CHECK:
    pub fee_acc: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(&mut self, fee_amount: f32) -> Result<()> {
        msg!(
            "Admin Addr : {:?} , Fee Account Addr : {:?}",
            self.payer.key(),
            self.fee_acc.key()
        );

        self.global_config
            .set_config(fee_amount, self.fee_acc.key(), self.payer.key());

        Ok(())
    }
}
