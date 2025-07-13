use anchor_lang::prelude::*;

declare_id!("ESkQzdDp8aRca8rn4kvCmpBUmZ2Sm9hTxHxZLL5tzRLf");

#[program]
pub mod sol_vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, title: String, description: String, options: Vec<String>, duration_secs: i64) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        let counter = &mut ctx.accounts.poll_counter;

        // detects if PollCounter is initialized for first time
        if counter.count == 0 && poll.created_at == 0 {
            poll.poll_id = 0;
            counter.count = 1;
        } else {
            poll.poll_id = counter.count;
            counter.count = counter.count.checked_add(1).ok_or(ErrorCode::CounterOverflow)?;
        }
        // atleast one option required
        require!(options.len() != 0, ErrorCode::NoOption);
        require!(options.len() <= 5, ErrorCode::TooManyOptions);
        require!(title.len() <= 100, ErrorCode::TitleTooLong);
        require!(description.len() <= 300, ErrorCode::DescriptionTooLong);

        poll.title = title;
        poll.description = description;
        poll.author = ctx.accounts.user.key();
        poll.options = options;
        let options_size = poll.options.len();
        poll.votes = vec![0; options_size];
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        poll.created_at = now;
        poll.end_at = now + duration_secs;
        Ok(())
    }

    pub fn cast_vote(ctx: Context<Vote>, option: u16) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        let vote = &mut ctx.accounts.user_vote;
        let user = &mut ctx.accounts.user;
        let clock = Clock::get()?;
        require!(clock.unix_timestamp <= poll.end_at, ErrorCode::PollClosed);
        // since u16 doesn't directly get converted to usize, i typecasted it.
        require!((option as usize) < poll.options.len(), ErrorCode::InvalidOption);
        poll.votes[option as usize] += 1;
        vote.option = option;
        vote.poll = poll.key();
        vote.user = user.key();
        Ok(())
    }
}

#[account]
pub struct PollCounter {
    pub count: u64
}

#[account]
#[derive(InitSpace)]
pub struct PollData {
    pub poll_id: u64,
    #[max_len(100)] // limit title size to save account space
    pub title: String,
    #[max_len(300)] // limit description size to save account space
    pub description: String,
    /* 5 options with 30 characters each
    if you wish to change max length here, change in VoteData too.*/
    #[max_len(5, 30)]
    pub options: Vec<String>,
    #[max_len(5)] // will count votes for each options, initial -> [0,0,0,0,0]
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
    /* since option size won't be more than 2^16, it's safe to use u16,
    you can increase this if you have more than 2^16 options in your poll */
    pub option: u16,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Poll has ended.")]
    PollClosed,
    #[msg("Invalid option index.")]
    InvalidOption,
    #[msg("No option provided")]
    NoOption,
    #[msg("Counter exceeded 2^64")]
    CounterOverflow,
    #[msg("Title too long")]
    TitleTooLong,
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Too many options provided")]
    TooManyOptions
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, seeds=[b"poll_counter"], bump, payer = user, space=8+8)]
    pub poll_counter: Account<'info, PollCounter>,
    #[account(init, seeds=[b"poll", user.key().as_ref(), &poll_counter.count.to_le_bytes()], bump, payer=user, space=8+PollData::INIT_SPACE)]
    pub poll_account: Account<'info, PollData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(init, seeds=[b"user_vote", user.key().as_ref(), poll_account.key().as_ref()], bump, payer=user, space=8+VoteData::INIT_SPACE)]
    pub user_vote: Account<'info, VoteData>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub poll_account: Account<'info, PollData>,
    pub system_program: Program<'info, System>
}