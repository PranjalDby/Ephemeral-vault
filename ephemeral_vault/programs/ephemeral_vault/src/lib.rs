use anchor_lang::prelude::*;

// this file contains the Anchor Program for our solena engine
declare_id!("2Y2AseLPmKvaGRXsU4yB3hjjMgXyhh9Y4LVgsgkSzCoT");

#[program]
pub mod ephemeral_vault {
    use super::*;

    pub fn create_ephemeral_vault(
        ctx: Context<CreateEphemeralVault>,
        session_duration: i64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.parent_wallet = ctx.accounts.parent_wallet.key();
        vault.ephemeral_wallet = ctx.accounts.ephemeral_wallet.key();
        vault.session_expires_at = Clock::get()?.unix_timestamp + session_duration;
        Ok(())
    }
    
    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let clock = Clock::get()?;

        // Session expired check
        require!(
            clock.unix_timestamp <= vault.session_expires_at,
            VaultError::SessionExpired
        );

        // Transfer SOL from parent wallet to PDA vault account
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.parent_wallet.key(),
            &ctx.accounts.vault.key(),
            amount,
        );
            /// Parent can revoke the session early by expiring it now.
        pub fn revoke_session(ctx: Context<RevokeSession>) -> Result<()> {
            let vault = &mut ctx.accounts.vault;
            let now = Clock::get()?.unix_timestamp;
            vault.session_expires_at = now;
            Ok(())
        }

        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.parent_wallet.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;

        Ok(())
    }
    /// Parent wallet can revoke the trading session before expiry.
    /// This makes the ephemeral wallet lose authority immediately.
    pub fn revoke_session(ctx: Context<RevokeSession>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let now = Clock::get()?.unix_timestamp;
        vault.session_expires_at = now; // expire instantly
        Ok(())
    }

    /// Open or update a perpetual futures position using the ephemeral wallet.
    /// No SOL transfers needed here — trade is purely risk accounting.
    pub fn place_trade(ctx: Context<PlaceTrade>, size: i64, price: i64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let now = Clock::get()?.unix_timestamp;

        require!(now <= vault.session_expires_at, VaultError::SessionExpired);

        // overwrite previous position (simple model for assignment)
        vault.position_size = size;
        vault.entry_price = price;

        Ok(())
    }


}

#[account]
pub struct VaultAccount {
    pub parent_wallet: Pubkey,
    pub ephemeral_wallet: Pubkey,
    pub session_expires_at: i64,
    pub position_size: i64,
    pub entry_price: i64,
    pub bump:u8
}

#[derive(Accounts)]
#[instruction(session_duration: i64)]
pub struct CreateEphemeralVault<'info> {
    #[account(mut)]
    pub parent_wallet: Signer<'info>,

    /// CHECK: we only store and verify this public key
    pub ephemeral_wallet: UncheckedAccount<'info>,

    #[account(
    init_if_needed,
    seeds = [b"vault".as_ref(), b"v2".as_ref(), parent_wallet.key().as_ref()],
    bump,
    payer = parent_wallet,
    space = 8 + std::mem::size_of::<VaultAccount>(),
    )]

    pub vault: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub parent_wallet: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", b"v2", parent_wallet.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}
// revoke session 
#[derive(Accounts)]
pub struct RevokeSession<'info> {
    #[account(mut)]
    pub parent_wallet: Signer<'info>,

   #[account(
        mut,
        seeds = [b"vault", b"v2", parent_wallet.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, VaultAccount>,
}
// placing the perpetual trades
#[derive(Accounts)]
pub struct PlaceTrade <'info> {
    #[account(mut)]
    pub parent_wallet: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", b"v2", parent_wallet.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, VaultAccount>,

    /// CHECK: must match stored key inside VaultAccount
    #[account(signer)]
    pub ephemeral_wallet: UncheckedAccount<'info>,
}

// for error handling
#[error_code]
pub enum VaultError {
    #[msg("Session expired")]
    SessionExpired,
}


