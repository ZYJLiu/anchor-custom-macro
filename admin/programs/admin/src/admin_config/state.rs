use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub auth: Pubkey,
    pub bool: bool,
    pub first_number: u8,
    pub second_number: u64,
}

impl Config {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 8;
}
