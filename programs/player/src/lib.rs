use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("6riKjKqR6aoqevBgaqaU9Kzu85jjoFKPBPAVP3ZKukyz");

#[program]
pub mod game {
    use super::*;
    // handler function
    pub fn create_user_stats(ctx: Context<CreateUserStats>, name: String) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.level = 0;
        if name.as_bytes().len() > 200 {
            // proper error handling omitted for brevity
            panic!();
        }
        // if action == true {
        //     user_stats.deposit.push(amount);
        //     user_stats.balance+=amount;
        // }else {
        //     user_stats.deposit.push(amount);
        //     user_stats.balance-=amount;
        // }
        user_stats.player_name = name;
        user_stats.deposit = vec![0];
        user_stats.withdrawl = vec![0];
        user_stats.balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Update>, amount: u32) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.deposit.push(amount);
        msg!("deposit");
        user_stats.balance += amount;
        Ok(())
    }
    pub fn withdrawl(ctx: Context<Update>, amount: u32) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.deposit.push(amount);
        msg!("withdrawl");
        user_stats.balance -= amount;
        Ok(())
    }

}

// validation struct
#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = size_of::<UserStats>() + 16, seeds = ["user-stats".as_bytes(), user.key().as_ref()], bump
    )]
    pub user_stats: Account<'info, UserStats>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
            mut,
        seeds = ["user-stats".as_bytes(), user.key().as_ref()], 
        bump = user_stats.bump,
    )]
    pub user_stats: Account<'info, UserStats>,
}

#[account]
pub struct UserStats {
    level: u16,
    player_name: String,
    deposit: Vec<u32>,
    withdrawl: Vec<u32>,
    balance: u32,
    bump: u8,
}
