use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod biblioteca {
    use super::*;

    pub fn crear_libro(
        ctx: Context<CrearLibro>,
        titulo: String,
        autor: String
    ) -> Result<()> {

        let libro = &mut ctx.accounts.libro;

        libro.titulo = titulo;
        libro.autor = autor;
        libro.disponible = true;

        Ok(())
    }
}

#[account]
pub struct Libro {
    pub titulo: String,
    pub autor: String,
    pub disponible: bool,
}

#[derive(Accounts)]
pub struct CrearLibro<'info> {

    #[account(init, payer = usuario, space = 200)]
    pub libro: Account<'info, Libro>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}
