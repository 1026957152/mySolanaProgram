use anchor_lang::prelude::*;

/*use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};*/
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;


declare_id!("2kx16UybH5HZkX1AimamBqqJ6mYstTgqR2jmVinV3GJS");

//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_solana_program {
    use mpl_token_metadata::instruction::create_metadata_accounts_v2;
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64,mint_seed: Vec<u8>,mint_bump: u8) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;

/*        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let cpi_accounts =puppet::cpi::accounts::SetData {
            puppet: ctx.accounts.my_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        puppet::cpi::set_data(cpi_ctx, data);
*/
        Ok(())
    }
/*    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }*/


    pub fn airdrop(ctx: Context<Airdrop>, mint_seed: Vec<u8>,mint_bump: u8) -> Result<()> {
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[&mint_seed, &[mint_bump]]],
            ),
            10000,
        )?;
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNft>, mint_seed: Vec<u8>,mint_bump: u8,
                    creator_key: Pubkey,
                    uri: String,
                    title: String) -> Result<()> {

/*        cpi_accounts = anchor_spl::token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        msg!("CPI Program Assigned");

        anchor_spl::token::mint_to(
            cpi_ctx,
            1,
        )?;
        msg!("Token Minted !!!");
*/


        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];

/*          let account_info = vec![
               metadata_account.clone(),
               mint_account.clone(),
               payer_accoount.clone(),
               payer_accoount.clone(),
               token_metadata_program_account.clone(),
               token_program_account.clone(),
               system_program_account.clone(),
               rent_account.clone(),
           ];*/
           let punk_num = 1;
        let symbol = std::string::ToString::to_string("symb");

/*           solana_program::program::invoke(
               &create_metadata_accounts_v2(
                   mpl_token_metadata::id(), //ctx.accounts.token_metadata_program.key(),
                   *metadata_account.key,
                   *mint_account.key,
                   *auth_account.key,
                   *payer_accoount.key,
                   *payer_accoount.key,
                   title,
                   symbol,
                   arweave_address_for_num(punk_num),
                   //uri,
                   Some(creator),
                   1,
                   true,
                   false,
                   None,
                   None,
               ),
               &account_info.as_slice(),
           )?;
*/

        Ok(())
    }



/*    pub fn make_offer(
        ctx: Context<MakeOffer>,
        escrowed_tokens_of_offer_maker_bump: u8,
        im_offering_this_much: u64,
        how_much_i_want_of_what_you_have: u64,
    ) -> Result<()> {
        handler(
            ctx,
            escrowed_tokens_of_offer_maker_bump,
            im_offering_this_much,
            how_much_i_want_of_what_you_have,
        );
        Ok(())

    }*/
}




#[derive(Accounts)]
#[instruction(bump: u8,mint_seed: Vec<u8>)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,

/*    #[account(
    init,
    seeds = [b"seed".as_ref()],
    bump ,
    payer = user,space = 8 + 8
    )]
    pub my_pda_account: Account<'info, MyAccount>,*/


/*    #[account(
    init_if_needed,
    payer = user,
    seeds = [&mint_seed],
    bump ,
    mint::decimals = 9,
    mint::authority = mint
    )]*/
   // pub mint: Account<'info, Mint>,


    //pub puppet_program: Program<'info, Puppet>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}













#[derive(Accounts)]
pub struct Airdrop<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

   // #[account(mut)]
    /// CHECK: 不知何故
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
   // pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
  //  pub associated_token_program: Program<'info, AssociatedToken>,
  //  pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct MyAccount {
    pub data: u64,

}







#[derive(Accounts)]
pub struct MintNft<'info> {

    #[account(mut)]
    pub metadata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // #[account(mut)]
    /// CHECK: 不知何故
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,



    pub token_metadata_program: Program<'info, Token>,
    pub token_program: Program<'info, Token>,



    pub system_program: Program<'info, System>,
    //  pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}