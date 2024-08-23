use anchor_lang::prelude::*;

use crate::constants::MASTER;
use crate::states::master::Master;

#[derive(Accounts)]
pub struct UpdateMasterCtx<'info> {
    #[account(
        mut,
        seeds = [MASTER.as_ref()],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

impl<'info> UpdateMasterCtx<'info> {
    pub fn update_start_sale_time(&mut self, start_sale_time: u64) -> Result<()> {
        self.master.start_sale_time = start_sale_time;
        Ok(())
    }

    pub fn update_end_sale_time(&mut self, end_sale_time: u64) -> Result<()> {
        self.master.end_sale_time = end_sale_time;
        Ok(())
    }

    pub fn update_cliff_time(&mut self, cliff_time: u64) -> Result<()> {
        self.master.cliff_time = cliff_time;
        Ok(())
    }

    pub fn update_cycle(&mut self, cycle: u64) -> Result<()> {
        self.master.cycle = cycle;
        Ok(())
    }
}
