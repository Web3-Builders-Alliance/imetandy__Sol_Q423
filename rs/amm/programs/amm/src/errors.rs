use anchor_lang::error_code;

#[error_code]
pub enum AmmError {
    #[msg("Fee is bigger than 100")]
    InvalidFee,
    #[msg("Authority is not the owner of the config account")]
    InvalidAuthority,
}