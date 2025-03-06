use anchor_lang::prelude::*;

use crate::consts::GLOBAL_SEED;

#[account]
pub struct GlobalConfig {
    pub fee_point: f32, //  sol percentage which moves to pumpfun after bonding curve
    pub fee_account: Pubkey, //  sol percentage which moves to pumpfun after bonding curve
    pub admin_addr: Pubkey, //  sol percentage which moves to pumpfun after bonding curve
}

impl GlobalConfig {
    pub const SIZE: usize = 72 + 8; // Size of the struct
    pub const SEEDS: &'static [u8] = GLOBAL_SEED; // Size of the struct

    pub fn set_config(&mut self, fee_point: f32, fee_acc: Pubkey, admin_addr: Pubkey) {
        self.fee_point = fee_point;
        self.fee_account = fee_acc;
        self.admin_addr = admin_addr;
    }

    pub fn udpate_config(&mut self, fee_point: f32, fee_acc: Pubkey) {
        self.fee_point = fee_point;
        self.fee_account = fee_acc;
    }

    pub fn update_admin(&mut self, new_admin: Pubkey) {
        self.admin_addr = new_admin;
    }

    pub fn get_config(&self) -> GlobalConfig {
        self.clone()
    }
}
