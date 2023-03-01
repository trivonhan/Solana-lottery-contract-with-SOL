use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

  #[msg("Lottery: Unauthorized")]
  Unauthorized,

  #[msg("Lottery: Lottery is not start")]
  LotteryNotStart,

  #[msg("Lottery: You're not the winner")]
  NotTheWinner,
}
