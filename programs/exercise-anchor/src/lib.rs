use anchor_lang::prelude::*;

declare_id!("4vzULsyMxjoaMGqNGzST3ema18rwuapDb7nZcVfhUtph");

#[program]
pub mod exercise_anchor {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }

    pub fn instruction_one(ctx: Context<InstructionAccounts>, instruction_data: u64) -> Result<()> {
        ctx.accounts.account_name.data = instruction_data
        Ok(())
    }
}

#[derive(Accounts)] // mendefinisikan struktur yang menyimpan semua akun yang diperlukan oleh instruksi smart contract, Menyediakan informasi tentang akun-akun yang terlibat dalam transaksi.
pub struct InstructionAccounts {
    #[account(init, payer = user, space = 8 + 8)] // akun akan diinisialisasi, user akan membayar biaya inisialisasi akun, menyediakan space 8 + 8 (8 awal untuk metadata dan 8 akhir untuk data yang ada di AccountStruct)
    pub account_name: Account<'info, AccountStruct>,
    #[account(mut)] // akun dapat dimodifikasi
    pub user: Signer<'info> // akun ini yang menandatangani transaksi
    pub sytem_program: Program<'info, System> // kalau gada ini gabisa inisialisasi akun atau transfer (semacam operasi dasar di Solana)
}

#[account] // Mengatur detail spesifik dari akun, seperti inisialisasi, modifikasi, dan ruang yang dibutuhkan.
pub struct AccountStruct {
    data: u64
}
