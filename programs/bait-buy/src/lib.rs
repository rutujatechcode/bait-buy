use anchor_lang::prelude::*;

declare_id!("CZAPsG85K3jyiLnVowmc3PACJXHHP4sNcwvhUW1gdoZ2");

#[program]
pub mod bait_buy {
    use super::*;

    pub fn buy(ctx: Context<Buy>, id: String, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let admin = &ctx.accounts.admin;

        // Transfer lamports from the user to the admin
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            user.to_account_info().key,
            admin.to_account_info().key,
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                user.to_account_info(),
                admin.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Emit an event after a successful transaction
        emit!(TransactionEvent {
            user: *user.key,
            id: id.clone(),
            amount: amount,
        });

        Ok(())
    }

    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.admin = new_admin;

        Ok(())
    }

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.admin = *ctx.accounts.admin.key;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(init, payer = admin, space = 8 + 32, seeds = [b"admin-account".as_ref()], bump)]
    pub admin_account: Account<'info, AdminAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub admin: SystemAccount<'info>,
    #[account(
        seeds = [b"admin-account".as_ref()],
        bump,
        has_one = admin // Ensures the stored admin matches the admin account
    )]
    pub admin_account: Account<'info, AdminAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"admin-account".as_ref()],
        bump,
        has_one = admin 
    )]
    pub admin_account: Account<'info, AdminAccount>,
}

#[account]
pub struct AdminAccount {
    pub admin: Pubkey,
}

#[event]
pub struct TransactionEvent {
    pub user: Pubkey,
    pub id: String,
    pub amount: u64,
}
