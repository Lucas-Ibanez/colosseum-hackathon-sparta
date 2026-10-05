//! Keypair files: devnet keys only, passed by path, never read from inside a
//! Git work tree and never printed.

use std::{fs, os::unix::fs::PermissionsExt, path::Path};

use solana_keypair::{read_keypair_file, Keypair};
use solana_signer::Signer;

/// Whether `path` (already canonical) lies inside a Git work tree.
pub fn inside_git_work_tree(path: &Path) -> bool {
    path.ancestors().skip(1).any(|dir| dir.join(".git").exists())
}

/// Loads the keypair of `role` from `path`.
///
/// Refuses files inside a Git work tree (such as this repository) and files
/// readable or writable by group or others. A read error never shows the
/// file content.
pub fn load(path: &Path, role: &str) -> Result<Keypair, String> {
    let canonical = fs::canonicalize(path).map_err(|_| format!("{role} keypair: cannot resolve the path"))?;
    if inside_git_work_tree(&canonical) {
        return Err(format!(
            "{role} keypair: refusing a file inside a Git work tree; keep devnet keys outside the clone"
        ));
    }
    let metadata = fs::metadata(&canonical).map_err(|_| format!("{role} keypair: cannot read the file metadata"))?;
    if !metadata.is_file() {
        return Err(format!("{role} keypair: not a regular file"));
    }
    check_mode(metadata.permissions().mode(), role)?;
    read_keypair_file(&canonical).map_err(|_| format!("{role} keypair: not a readable keypair file (content not shown)"))
}

/// Accepts only owner-only permissions (`0600` or `0400`).
pub fn check_mode(mode: u32, role: &str) -> Result<(), String> {
    let mode = mode & 0o777;
    if mode & 0o077 != 0 {
        return Err(format!("{role} keypair: mode {mode:o} gives access to group or others; use 0600"));
    }
    Ok(())
}

/// Pubkey line for the output; the only key material ever printed.
pub fn describe(role: &str, keypair: &Keypair) -> String {
    format!("{role}={}", keypair.pubkey())
}
