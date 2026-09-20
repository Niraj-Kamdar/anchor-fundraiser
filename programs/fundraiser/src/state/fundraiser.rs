use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Fundraiser {
    pub maker: Pubkey,
    pub mint_to_raise: Pubkey,
    pub amount_to_raise: u64,
    pub current_amount: u64, // Is this safe?? Donation attack: consider gift but messes with when balance reached amount to raise vs amount raised by contributor
    pub time_started: i64,
    pub duration: u8, // $explore
    pub bump: u8,
}
