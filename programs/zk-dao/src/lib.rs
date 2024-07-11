use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("DnfKVNRvbJUuVpwXx3Cfn2jawHkCzwuEtV6gedbw4cE4");

#[program]
pub mod zk_dao {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote: bool) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        if vote {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }
        let voter = &mut ctx.accounts.voter;
        voter.has_voted = true;
        Ok(())
    }

    pub fn get_results(ctx: Context<GetResults>) -> Result<()> {
        let proposal = &ctx.accounts.proposal;
        msg!("Title: {}", proposal.title);
        msg!("Yes Votes: {}", proposal.yes_votes);
        msg!("No Votes: {}", proposal.no_votes);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(init, payer = user, space = 8 + 1)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn distribute_rewards(ctx: Context<DistributeRewards>, amount: u64) -> Result<()> {
    let voter = &ctx.accounts.voter;
    if voter.has_voted {
        let user = &ctx.accounts.user;
        let reward_account = &mut ctx.accounts.reward_account;
        reward_account.points += amount;
    }
    Ok(())
}

#[account]
pub struct RewardAccount {
    pub points: u64,
}

#[account]
pub struct Proposal {
    pub title: String,
    pub yes_votes: u32,
    pub no_votes: u32,
}

#[account]
pub struct Voter {
    pub has_voted: bool,
}
