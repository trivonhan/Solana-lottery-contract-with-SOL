use anchor_lang::prelude::*;

#[constant]
pub const LOTTERY_SEED: &[u8] = b"INIT_LOTTERY";

#[constant]
pub const  LOTTERY_WALLET_SEED: &[u8] = b"LOTTERY_WALLET";

#[constant]
pub const TODO_TAG: &[u8] = b"TODO_STATE";

pub const ROOT_KEYS: &[&str] = &[
  "3BiVpSVqGw9VX9Dp1SdBvKaGwBtWEhpG8eWkfLPZyMhK",
];