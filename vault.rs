use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");
#[program]
pub mod VaultProgram {
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        Ok(())
    }
    pub fn deposit(ctx: Context<DepositContext>, amount: u64) -> Result<()> {
        Ok(())
    }
    pub fn withdraw(ctx: Context<WithdrawContext>, amount: u64) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account()]
    pub vault: SystemAccount<'info>,
    # [account (init , payer = owner , seeds = [b"vault"] , bump)]
    pub state: Account<'info, Vault>,
    #[account()]
    pub auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DepositContext<'info> {
    #[account()]
    pub auth: UncheckedAccount<'info>,
    #[account()]
    pub state: Account<'info, Vault>,
    #[account()]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
}
#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    #[account()]
    pub vault: SystemAccount<'info>,
    #[account()]
    pub state: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account()]
    pub auth: UncheckedAccount<'info>,
}
#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub stateBump: u8,
    pub authBump: u8,
    pub vaultBump: u8,
}