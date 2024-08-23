use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct HelloCtx {}

pub fn hello(_ctx: Context<HelloCtx>) -> Result<()> {
    msg!("hello from Event prototol IVO");
    Ok(())
}