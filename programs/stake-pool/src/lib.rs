use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{MintTo, Transfer, Burn},
    token::Token,
    associated_token::AssociatedToken,
};
use std::cmp::max;
use std::mem::size_of;

declare_id!("3Ss6H3oL9uBKkfDP1jTzaHJ3twfn9BZnnMAHcfE4xhyj");

#[program]
pub mod anchor_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    
    pub fn new_staker(ctx: Context<NewStaker>) -> Result<()> {
        Ok(())
    }

    pub fn add(ctx: Context<Operation>, deposit_amount: u64) -> Result<()> {
        let receipt = &mut ctx.accounts.receipt;
        if receipt.is_valid == 0 {
            receipt.is_valid = 1;
            receipt.created_ts = ctx.accounts.clock.unix_timestamp;
            receipt.amount_deposited = deposit_amount;
        } else {
            return Err(ErrorCode::AccountAlreadyStakedError.into());
        }

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_token_x.to_account_info(),
                to: ctx.accounts.vault_x.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, deposit_amount)?;

        let mint_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                to: ctx.accounts.sender_token_synth_x.to_account_info(),
                mint: ctx.accounts.synthetic_x.to_account_info(),
                authority: ctx.accounts.synthetic_x.to_account_info(),
            },
        );
        let (_synthetic, synthetic_bump) =
            Pubkey::find_program_address(&[b"synthetic"], ctx.program_id);
        let seed1: &[u8] = b"synthetic";
        let seed2: &[u8] = &[synthetic_bump];
        let pda_sign: &[&[u8]] = &[seed1, seed2];
        token::mint_to(mint_ctx.with_signer(&[pda_sign]), deposit_amount)?;
        Ok(())
    }

    pub fn remove(ctx: Context<Operation>) -> Result<()> {
        let receipt = &mut ctx.accounts.receipt;
        if receipt.is_valid == 0 {
            return Err(ProgramError::InvalidAccountData.into());
        }
        let deposited_amount = receipt.amount_deposited;
        let start_time = receipt.created_ts;
        let curr_time = ctx.accounts.clock.unix_timestamp;
        let diff_time = curr_time - start_time;
        let burn_amount = max(0_u64, deposited_amount - diff_time as u64);
        receipt.is_valid = 0;

        if burn_amount > 0 {
            let burn_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.synthetic_x.to_account_info(),
                    from: ctx.accounts.sender_token_synth_x.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            );
            token::burn(burn_ctx, burn_amount)?;
        }

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_x.to_account_info(),
                to: ctx.accounts.sender_token_x.to_account_info(),
                authority: ctx.accounts.vault_x.to_account_info(),
            },
        );
        let (_vault, vault_bump) =
            Pubkey::find_program_address(&[b"vault"], ctx.program_id);
        let seed1: &[u8] = b"vault";
        let seed2: &[u8] = &[vault_bump];
        let pda_sign: &[&[u8]] = &[seed1, seed2];
        token::transfer(transfer_ctx.with_signer(&[pda_sign]), deposited_amount)?;
        Ok(())
    }
}

/// CHECK: `token_x` is an external SPL token mint account; no on-chain checks are performed.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: External token mint.
    #[account(mut)]
    pub token_x: UncheckedAccount<'info>,
    /// CHECK: This account will be initialized as the synthetic mint. No checks are necessary because it is controlled by the program.
    #[account(
        init,
        payer = payer,
        seeds = [b"synthetic"],
        bump,
        space = 82
    )]
    pub synthetic_x: UncheckedAccount<'info>,
    /// CHECK: This account will be initialized as the vault token account. No checks are performed.
    #[account(
        init,
        payer = payer,
        seeds = [b"vault"],
        bump,
        space = 165
    )]
    pub vault_x: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

/// CHECK: `token_x` is an external SPL token mint.
#[derive(Accounts)]
pub struct NewStaker<'info> {
    /// CHECK: External token mint.
    #[account(mut)]
    pub token_x: UncheckedAccount<'info>,
    #[account(
        init,
        payer = sender,
        seeds = [b"receipt", b"unique0"],
        bump,
        space = 8 + size_of::<Receipt>()
    )]
    pub receipt: Account<'info, Receipt>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// CHECK: All external token accounts are unchecked.
#[derive(Accounts)]
pub struct Operation<'info> {
    /// CHECK: External token mint.
    #[account(mut)]
    pub token_x: UncheckedAccount<'info>,
    /// CHECK: Synthetic mint account.
    #[account(mut, seeds = [b"synthetic"], bump)]
    pub synthetic_x: UncheckedAccount<'info>,
    /// CHECK: Vault token account.
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault_x: UncheckedAccount<'info>,
    pub sender: Signer<'info>,
    /// CHECK: Sender's token account.
    #[account(mut)]
    pub sender_token_x: UncheckedAccount<'info>,
    /// CHECK: Sender's synthetic token account.
    #[account(mut)]
    pub sender_token_synth_x: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    #[account(mut, seeds = [b"receipt", b"unique0"], bump)]
    pub receipt: Account<'info, Receipt>,
}

#[account]
#[derive(Default)]
pub struct Receipt {
    pub is_valid: u8,
    pub created_ts: i64,
    pub amount_deposited: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account has already staked.")]
    AccountAlreadyStakedError,
}
