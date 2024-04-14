use anchor_lang::prelude::*;

declare_id!("HdZDgSANZbYPc439ktrx4G4M6f58HVA4hnyUZD1kawnZ");

#[program]
pub mod hello_world_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world, form the first solana program written by me");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
