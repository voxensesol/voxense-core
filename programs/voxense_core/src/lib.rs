use anchor_lang::prelude::*;

declare_id!("VoXense1111111111111111111111111111111111");

#[program]
pub mod voxense_core {
    use super::*;

    pub fn register_node(ctx: Context<RegisterNode>, node_id: Pubkey) -> Result<()> {
        let node = &mut ctx.accounts.node;
        node.owner = ctx.accounts.authority.key();
        node.node_id = node_id;
        node.bump = *ctx.bumps.get("node").unwrap();
        Ok(())
    }

    pub fn submit_proof(
        ctx: Context<SubmitProof>,
        data_hash: [u8; 32],
        sensor_type: u8,
    ) -> Result<()> {
        let proof = &mut ctx.accounts.proof;
        proof.node = ctx.accounts.node.key();
        proof.data_hash = data_hash;
        proof.sensor_type = sensor_type;
        proof.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterNode<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1,
        seeds = [b"node", authority.key().as_ref()],
        bump
    )]
    pub node: Account<'info, Node>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitProof<'info> {
    #[account(mut, has_one = owner)]
    pub node: Account<'info, Node>,
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 1 + 8,
        seeds = [b"proof", node.key().as_ref()],
        bump
    )]
    pub proof: Account<'info, Proof>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Node {
    pub owner: Pubkey,
    pub node_id: Pubkey,
    pub bump: u8,
}

#[account]
pub struct Proof {
    pub node: Pubkey,
    pub data_hash: [u8; 32],
    pub sensor_type: u8,
    pub timestamp: i64,
}
