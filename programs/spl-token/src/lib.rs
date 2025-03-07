use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Token, InitializeMint, MintTo, Transfer, Burn, FreezeAccount, ThawAccount,
    Approve, Revoke, CloseAccount,
};

declare_id!("DuuNR8eW4BuwJJ4VLL1uPo49ELyec6WTjPpdWpi7S76w");

#[program]
pub mod spl_token_devnet {
    use super::*;

    // SPL Token mint 계정을 초기화 (생성)
    pub fn initialize(ctx: Context<InitializeMints>, decimals: u8) -> Result<()> {
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::initialize_mint(cpi_ctx, decimals, ctx.accounts.authority.key, None)?;
        Ok(())
    }

    // 토큰 민팅 (새 토큰 발행)
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    // 토큰 전송
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    // 토큰 소각
    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::burn(cpi_ctx, amount)?;
        Ok(())
    }

    // 토큰 계정 동결
    pub fn freeze_account(ctx: Context<FreezeAccountContext>) -> Result<()> {
        let cpi_accounts = FreezeAccount {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::freeze_account(cpi_ctx)?;
        Ok(())
    }

    // 토큰 계정 해동
    pub fn thaw_account(ctx: Context<ThawAccountContext>) -> Result<()> {
        let cpi_accounts = ThawAccount {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::thaw_account(cpi_ctx)?;
        Ok(())
    }

    // Delegate 승인: 특정 금액을 delegate에게 사용권한 부여
    pub fn approve_tokens(ctx: Context<ApproveTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Approve {
            to: ctx.accounts.token_account.to_account_info(),
            delegate: ctx.accounts.delegate.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::approve(cpi_ctx, amount)?;
        Ok(())
    }

    // Delegate 승인 취소
    pub fn revoke_tokens(ctx: Context<RevokeTokens>) -> Result<()> {
        let cpi_accounts = Revoke {
            source: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::revoke(cpi_ctx)?;
        Ok(())
    }

    // 토큰 계정 종료 (close account): 남은 토큰과 램포트를 지정한 계정으로 회수
    pub fn close_token_account(ctx: Context<CloseTokenAccount>) -> Result<()> {
        let cpi_accounts = CloseAccount {
            account: ctx.accounts.token_account.to_account_info(),
            destination: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::close_account(cpi_ctx)?;
        Ok(())
    }
}

/// CHECK: 이 계정은 SPL Token mint 계정입니다.
#[derive(Accounts)]
pub struct InitializeMints<'info> {
    /// CHECK: 초기화할 mint 계정 (프로그램이 소유)
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// CHECK: mint된 토큰을 받을 토큰 계정입니다.
#[derive(Accounts)]
pub struct MintTokens<'info> {
    /// CHECK: SPL Token mint 계정.
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: 토큰을 받을 계정.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// CHECK: 토큰 전송 시 사용되는 계정들.
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    /// CHECK: 보내는 계정.
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: 받는 계정.
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// CHECK: 토큰 소각 시 사용되는 계정들.
#[derive(Accounts)]
pub struct BurnTokens<'info> {
    /// CHECK: SPL Token mint 계정.
    pub mint: UncheckedAccount<'info>,
    /// CHECK: 토큰 소각 대상 계정.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// CHECK: 토큰 계정 동결 기능에 필요한 계정들.
#[derive(Accounts)]
pub struct FreezeAccountContext<'info> {
    /// CHECK: 동결할 토큰 계정.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: SPL Token mint 계정.
    pub mint: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// CHECK: 토큰 계정 해동 기능에 필요한 계정들.
#[derive(Accounts)]
pub struct ThawAccountContext<'info> {
    /// CHECK: 해동할 토큰 계정.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: SPL Token mint 계정.
    pub mint: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// CHECK: Delegate 승인 기능에 필요한 계정들.
#[derive(Accounts)]
pub struct ApproveTokens<'info> {
    /// CHECK: delegate 권한 부여할 토큰 계정.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: delegate로 지정할 계정.
    pub delegate: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// CHECK: Delegate 승인 취소 기능에 필요한 계정들.
#[derive(Accounts)]
pub struct RevokeTokens<'info> {
    /// CHECK: delegate 권한이 부여된 토큰 계정.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// CHECK: 토큰 계정 종료(close) 기능에 필요한 계정들.
#[derive(Accounts)]
pub struct CloseTokenAccount<'info> {
    /// CHECK: 종료할 토큰 계정.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: 토큰 계정 종료 후 램포트 회수 대상 계정.
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
