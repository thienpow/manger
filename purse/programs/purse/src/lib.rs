use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::mem::size_of;

declare_id!("GCfgR2i7Q5G8LBFE2mhMb5x1qcP88q6SbdiTJo4ewC1C");

const TEXT_LENGTH: usize = 1024;
const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod purse {
    use super::*;

    pub fn create_state(ctx: Context<CreateState>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.post_count = 0;

        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, text: String, poster_name: String, poster_url: String)  -> Result<()> {
        let state = &mut ctx.accounts.state;
        let post = &mut ctx.accounts.post;
        post.authority = ctx.accounts.authority.key();
        post.text = text;
        post.poster_name = poster_name;
        post.poster_url = poster_url;
        post.comment_count = 0;
        post.index = state.post_count;
        post.post_time = ctx.accounts.clock.unix_timestamp;
        state.post_count += 1;

        Ok(())
    }
}

/// Contexts
/// CreateState context
#[derive(Accounts)]
pub struct CreateState<'info> {
    // Authenticating state account
    #[account(
        init,
        seeds = [b"state".as_ref()],
        bump,
        payer = authority,
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: System program
    pub system_program: UncheckedAccount<'info>,

    // Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

/// CreatePost context
#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(
        mut, 
        seeds=[b"state".as_ref()], 
        bump
    )]
    pub state: Account<'info, StateAccount>,

    #[account(
        init, 
        seeds=[b"post".as_ref(), state.post_count.to_be_bytes().as_ref()], 
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH
    )]
    pub post: Account<'info, PostAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: System program
    pub system_program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct StateAccount {
    pub authority: Pubkey,
    pub post_count: u64,
}


#[account]
pub struct PostAccount {
    pub authority: Pubkey,
    pub text: String,
    pub poster_name: String,
    pub poster_url: String,
    pub comment_count: u64,
    pub index: u64,
    pub post_time: i64
}
