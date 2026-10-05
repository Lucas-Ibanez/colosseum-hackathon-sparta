//! VeriCode devnet client (gate D7).
//!
//! Creates, funds, delivers, settles and refunds Jobs of the immutable
//! `vericode_escrow` program on Solana devnet, with the Groth16 receipts of
//! `vericode-prover`. The client decides nothing economic: every rule is
//! enforced by the program, and the client checks before sending so that a
//! mistake costs no fee. See `cli/README.md`.

pub mod escrow;
pub mod keys;
pub mod receipt;
pub mod rpc;
pub mod tx;

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn unhex32(text: &str) -> Result<[u8; 32], String> {
    if text.len() != 64 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("expected 64 hex digits, got {text:?}"));
    }
    let mut out = [0_u8; 32];
    for (index, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[2 * index..2 * index + 2], 16).map_err(|error| error.to_string())?;
    }
    Ok(out)
}

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes).into()
}

pub fn explorer_tx(signature: &str) -> String {
    format!("https://explorer.solana.com/tx/{signature}?cluster=devnet")
}

pub fn explorer_address(address: &solana_pubkey::Pubkey) -> String {
    format!("https://explorer.solana.com/address/{address}?cluster=devnet")
}
