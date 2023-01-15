use anchor_lang::prelude::*;

declare_id!("8H1uF9UVq3y8FPimZAjXA9NDLf2jPUwr7eDqrg97WWPG");

#[program]
pub mod game {
    use super::*;
    // handler function
    pub fn create_user_stats(ctx: Context<CreateUserStats>, name: String,amount: u32,action:bool) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.level = 0;
        if name.as_bytes().len() > 200 {
            // proper error handling omitted for brevity
            panic!();
        }
        if action == true {
            user_stats.deposit.push(amount);
            user_stats.balance+=amount;
        }else {
            user_stats.deposit.push(amount);
            user_stats.balance-=amount;
        }
        user_stats.player_name = name;
        user_stats.bumps = *ctx.bumps.get("user_balance").unwrap();
        user_stats.action = action;
        Ok(())
    }
}

#[account]
pub struct UserStats {
    level: u16,
    player_name: String,
    deposit: Vec<u32>,
    withdrawl: Vec<u32>,
    balance: u32,
    action:bool,
    bumps: u8
}

// validation struct
#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // space: 8 discriminator + 2 level + 4 name length + 200 name + 1 bump
    #[account(
        init,
        payer = user,
        space = 8 + 2 + 4 + 200 +4 + 4+ 4+4+4+1  + 1, seeds = [b"user-stats", user.key().as_ref()], bump
    )]
    pub user_stats: Account<'info, UserStats>,
    pub system_program: Program<'info, System>,
}