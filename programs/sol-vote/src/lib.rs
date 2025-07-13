use anchor_lang::prelude::*;

declare_id!("ESkQzdDp8aRca8rn4kvCmpBUmZ2Sm9hTxHxZLL5tzRLf");

#[program]
pub mod sol_vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, title: String, description: String, options: Vec<String>, duration_secs: i64) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        poll.title = title;
        poll.description = description;
        poll.author = ctx.accounts.user.key();
        poll.options = options;
        let options_size = poll.options.len();
        poll.votes = vec![064; options_size];
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        poll.created_at = now;
        poll.end_at = now + duration_secs;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct PollData {
    #[max_len(100)] // 100 characters
    pub title: String,
    #[max_len(300)]
    pub description: String,
    #[max_len(5, 30)] // 5 options with 30 characters each
    pub options: Vec<String>,
    #[max_len(5)] // initial -> [0,0,0,0,0]
    pub votes: Vec<u64>,
    pub author: Pubkey,
    pub created_at: i64,
    pub end_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct VoteData {
    pub poll: Pubkey,
    pub user: Pubkey,
    pub option: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds=[b"create_poll", user.key().as_ref()], bump, payer=user, space=8+PollData::INIT_SPACE)]
    pub poll_account: Account<'info, PollData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(init, seeds=[b"user_vote", user.key().as_ref(), poll_account.key().as_ref()], bump, payer=user, space=8+VoteData::INIT_SPACE)]
    pub user_vote: Account<'info, VoteData>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub poll_account: Account<'info, PollData>,
    pub system_program: Program<'info, System>
}