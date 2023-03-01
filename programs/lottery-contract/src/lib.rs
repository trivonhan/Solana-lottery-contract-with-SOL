
use anchor_lang::prelude::*;
pub mod constant;
pub mod error;

use anchor_lang::solana_program::{
    system_instruction::{
        transfer
    },
    program::{
        invoke, invoke_signed
    }
};

use crate::{
    constant::*,
    error::{
        ErrorCode
    }
};

declare_id!("53kjJ9cywtGofrGyQ3hv8XLAhAJRWqdWRTXTNfdAn46d");

#[program]
pub mod lottery_contract {
    use super::*;

    #[access_control(is_root(*ctx.accounts.root.key))]
    pub fn init_lottery_master(ctx: Context<InitializeLotteryMaster>) -> Result<()> {
        let lottery_master = &mut ctx.accounts.lottery_master;

        lottery_master.lottery_count = 0;

        Ok(())
    }

    #[access_control(is_root(*ctx.accounts.root.key))]
    pub fn init_lottery(ctx: Context<InitLottery>) -> Result<()>{
        msg!("Initialize lottery");
        let lottery_account = &mut ctx.accounts.lottery_account;
        let lottery_master = &mut ctx.accounts.lottery_master;

        lottery_account.amount = 0;
        lottery_account.is_starting = true;
        lottery_account.player = vec![];
        lottery_account.claimed = false;
        lottery_account.id = lottery_master.lottery_count;

        lottery_master.lottery_count += 1;

        Ok(())
    }

    pub fn add_money_to_lottery(ctx: Context<AddMoney>, _lottery_index: u8) -> Result<()> {
        let player = &ctx.accounts.player;
        let lottery_account = &mut ctx.accounts.lottery_account;
        let lottery_signer =  &mut ctx.accounts.lottery_signer;

        require!(lottery_account.is_starting==true, ErrorCode::LotteryNotStart);
        
        let instruction = &transfer(player.key, &lottery_signer.key, 1000000000);
        

        invoke(&instruction, &[player.clone().to_account_info(), player.clone().to_account_info(), lottery_signer.clone()])
            .expect("CPI failed");
        msg!("DEBUG: Transfer Instruction {:?}", instruction);

        lottery_account.amount += 1000000000;
        lottery_account.player.push(player.key());

        Ok(())
    }

    #[access_control(is_root(*ctx.accounts.root.key))]
    pub fn pick_winner(ctx: Context<PickWinner>, _lottery_index: u8) -> Result<()> {
        let lottery_account = &mut ctx.accounts.lottery_account;

        let amount_player = lottery_account.player.len();
        let player = &lottery_account.player;
        let now_ts = Clock::get().unwrap().unix_timestamp;

        msg!("Time is: {:?}", now_ts);
        let winner_index = now_ts % amount_player as i64;
        msg!("Winner is: {:?}", player[winner_index as usize]);


        lottery_account.is_starting = false;
        lottery_account.winner = lottery_account.player[winner_index as usize];

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, _lottery_index: u8, _bump: u8) -> Result<()> {
        let lottery_account = &mut ctx.accounts.lottery_account;
        let lottery_signer = &mut ctx.accounts.lottery_signer;

        let player  = &ctx.accounts.player;
        require!(lottery_account.winner == player.key(), ErrorCode::NotTheWinner);

        msg!("Player: {:?}", player.key());

        let lottery_id = lottery_account.id;

        let seed  = &[
            LOTTERY_WALLET_SEED, 
            &[lottery_id],
            &[_bump]];

        let instruction = &transfer(&lottery_signer.to_account_info().key(), &player.to_account_info().key(), lottery_account.amount);
        

        invoke_signed(&instruction, &[lottery_signer.to_account_info().clone(), player.to_account_info().clone(), player.to_account_info().clone()],&[seed])
            .expect("CPI failed");
        msg!("DEBUG: Transfer Instruction {:?}", instruction);

        lottery_account.amount = 0;
        lottery_account.claimed = true;
        lottery_account.is_starting = false;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLotteryMaster<'info> {
        /// CHECK: program owner, verified using #access_control
        #[account(mut)]
        pub root: Signer<'info>,
    
        #[account(
            init,
            seeds = [LOTTERY_SEED, root.key().as_ref()],
            bump,
            payer = root,
            space = 8 + 8,
        )]
        pub lottery_master: Box<Account<'info, LotteryMaster>>,
    
        pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct InitLottery<'info> {
    /// CHECK: program owner, verified using #access_control
    #[account(mut)]
    pub root: Signer<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, root.key().as_ref()],
        bump,
    )]
    pub lottery_master: Box<Account<'info, LotteryMaster>>,

    #[account(
        init,
        seeds = [LOTTERY_SEED, &[lottery_master.lottery_count]],
        bump,
        payer = root,
        space = 1024,
    )]
    pub lottery_account: Box<Account<'info, Lottery>>,

    /// CHECK: Signer for lottery account
    #[account(
        seeds = [LOTTERY_WALLET_SEED, &[lottery_master.lottery_count]],
        bump,
    )]
    pub lottery_signer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_lottery_index: u8)]
pub struct AddMoney<'info> {

    /// CHECK: Who can send money to lottery
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, &[_lottery_index].as_ref()],
        bump
    )]
    pub lottery_account: Box<Account<'info, Lottery>>,

    /// CHECK: Signer for lottery account
    #[account(
        mut,
        seeds = [LOTTERY_WALLET_SEED, &[_lottery_index]],
        bump
    )]
    pub lottery_signer: AccountInfo<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(_lottery_index: u8)]
pub struct PickWinner<'info> {

    /// CHECK: program owner, verified using #access_control
    #[account(mut)]
    pub root: Signer<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, &[_lottery_index].as_ref()],
        bump
    )]
    pub lottery_account: Box<Account<'info, Lottery>>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(_lottery_index: u8, _bump: u8)]
pub struct Claim<'info> {

    /// CHECK: Who can send money to lottery
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, &[_lottery_index].as_ref()],
        bump
    )]
    pub lottery_account: Box<Account<'info, Lottery>>,

    /// CHECK: Signer for lottery account
    #[account(
        mut,
        seeds = [LOTTERY_WALLET_SEED, &[_lottery_index]],
        bump
    )]
    pub lottery_signer: AccountInfo<'info>,

    pub system_program: Program<'info, System>
}

#[account]
#[derive(Default)]
pub struct LotteryMaster {
    lottery_count: u8,
}

#[account]
#[derive(Default)]
pub struct Lottery {
    id: u8,
    amount: u64,
    is_starting: bool,
    player: Vec<Pubkey>,
    winner: Pubkey,
    claimed: bool,
}

pub fn is_root(user: Pubkey) -> Result<()> {
    let user_key = user.to_string();
    let result = ROOT_KEYS.iter().position(|&key| key == &user_key[..]);
    if result == None {
      return Err(ErrorCode::Unauthorized.into());
    }
  
    Ok(())
}
