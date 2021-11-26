use anchor_lang::prelude::*;

declare_id!("71WZw3fAcjv77vKgwLJXnPdrHyY14EV8d5P26KMKiikD");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            upvotes: 0,
        };

        base_account.gifs_list.push(item);
        base_account.total_gifs += 1;

        Ok(())
    }

    pub fn update_item(ctx: Context<UpdateItem>, index: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        let i: usize = index.parse().unwrap();

        base_account.gifs_list[i].upvotes += 1;

        Ok(())
    }

    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        )
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init,payer=user,space=9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    from: Signer<'info>,
    #[account(mut)]
    to: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub upvotes: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gifs_list: Vec<ItemStruct>,
}
