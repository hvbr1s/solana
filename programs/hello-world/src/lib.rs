use anchor_lang::prelude::*;

declare_id!("6ZFL2Q7eaqL7LCvjKEALjYHcCeN8hTFBfcVtJfyfef6k");

#[program]
mod hello_world {
use super::*;

pub fn hello(_ctx: Context<Hello>) -> Result<()> {
	msg!("Hello, World!");
	Ok(())
}

#[derive(Accounts)]
pub struct Hello {
    
}
}