//! Minimal Solana JSON-RPC client over HTTPS.
//!
//! The public devnet endpoint rate-limits bursts (HTTP 429), so every call
//! is paced and retried with exponential backoff. Only the configured URL is
//! contacted; the client refuses any cluster other than devnet by genesis
//! hash (see `main.rs`).

use std::{
    cell::{Cell, RefCell},
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::Path,
    thread::sleep,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};
use solana_pubkey::Pubkey;

pub const DEFAULT_RPC_URL: &str = "https://api.devnet.solana.com";
const MIN_INTERVAL: Duration = Duration::from_millis(250);
const ATTEMPTS: u32 = 8;
// JSON-RPC errors that clear by waiting: minimum context slot not reached,
// node behind, block not available yet.
const TRANSIENT_RPC_CODES: [i64; 3] = [-32016, -32005, -32004];

/// One account as returned by `getAccountInfo` with base64 data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Account {
    pub lamports: u64,
    pub owner: Pubkey,
    pub executable: bool,
    pub data: Vec<u8>,
}

pub struct Rpc {
    url: String,
    client: reqwest::blocking::Client,
    last: Cell<Option<Instant>>,
    log: Option<RefCell<File>>,
}

fn now_unix() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs())
}

fn parse_account(value: &Value) -> Result<Option<Account>, String> {
    if value.is_null() {
        return Ok(None);
    }
    let data = value["data"][0].as_str().ok_or("account data is not base64")?;
    Ok(Some(Account {
        lamports: value["lamports"].as_u64().ok_or("account lamports")?,
        owner: value["owner"]
            .as_str()
            .ok_or("account owner")?
            .parse()
            .map_err(|_| "account owner is not a pubkey")?,
        executable: value["executable"].as_bool().unwrap_or(false),
        data: STANDARD.decode(data).map_err(|error| error.to_string())?,
    }))
}

impl Rpc {
    /// `log`: optional JSON Lines file (created `0600`) for every recorded
    /// exchange.
    pub fn new(url: &str, log: Option<&Path>) -> Result<Self, String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent(concat!("vericode-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|error| error.to_string())?;
        let log = match log {
            Some(path) => Some(RefCell::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .mode(0o600)
                    .open(path)
                    .map_err(|error| format!("log {}: {error}", path.display()))?,
            )),
            None => None,
        };
        Ok(Self {
            url: url.to_string(),
            client,
            last: Cell::new(None),
            log,
        })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    /// Appends one JSON object to the log, with the Unix time.
    pub fn record(&self, mut entry: Value) {
        if let Some(log) = &self.log {
            entry["unix_time"] = json!(now_unix());
            let _ = writeln!(log.borrow_mut(), "{entry}");
        }
    }

    fn pace(&self) {
        if let Some(last) = self.last.get() {
            let elapsed = last.elapsed();
            if elapsed < MIN_INTERVAL {
                sleep(MIN_INTERVAL - elapsed);
            }
        }
        self.last.set(Some(Instant::now()));
    }

    /// Calls `method` and returns its `result`, retrying rate limits, server
    /// errors, timeouts and transient JSON-RPC errors.
    pub fn call(&self, method: &str, params: Value) -> Result<Value, String> {
        let body = json!({"jsonrpc": "2.0", "id": 1, "method": method, "params": params});
        let mut delay = Duration::from_secs(1);
        let mut reason = String::new();
        for attempt in 1..=ATTEMPTS {
            self.pace();
            match self.client.post(&self.url).json(&body).send() {
                Ok(response) if response.status().as_u16() == 429 || response.status().is_server_error() => {
                    reason = format!("HTTP {}", response.status().as_u16());
                }
                Ok(response) => {
                    let value: Value = response.json().map_err(|error| format!("rpc {method}: {error}"))?;
                    match value.get("error") {
                        Some(error) => {
                            let code = error["code"].as_i64().unwrap_or(0);
                            if !TRANSIENT_RPC_CODES.contains(&code) {
                                return Err(format!("rpc {method}: {error}"));
                            }
                            reason = format!("rpc error {code}");
                        }
                        None => return Ok(value["result"].clone()),
                    }
                }
                Err(error) => reason = error.to_string(),
            }
            if attempt < ATTEMPTS {
                eprintln!("rpc {method}: {reason}; retry {attempt}/{ATTEMPTS} in {} s", delay.as_secs());
                sleep(delay);
                delay = (delay * 2).min(Duration::from_secs(16));
            }
        }
        Err(format!("rpc {method}: {reason} after {ATTEMPTS} attempts"))
    }

    pub fn genesis_hash(&self) -> Result<String, String> {
        self.call("getGenesisHash", json!([]))?
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| "getGenesisHash".into())
    }

    pub fn slot(&self, commitment: &str) -> Result<u64, String> {
        self.call("getSlot", json!([{"commitment": commitment}]))?
            .as_u64()
            .ok_or_else(|| "getSlot".into())
    }

    pub fn block_height(&self) -> Result<u64, String> {
        self.call("getBlockHeight", json!([{"commitment": "confirmed"}]))?
            .as_u64()
            .ok_or_else(|| "getBlockHeight".into())
    }

    /// Recent blockhash and its last valid block height.
    pub fn latest_blockhash(&self) -> Result<(String, u64), String> {
        let value = self.call("getLatestBlockhash", json!([{"commitment": "confirmed"}]))?;
        let blockhash = value["value"]["blockhash"].as_str().ok_or("blockhash")?.to_string();
        let height = value["value"]["lastValidBlockHeight"].as_u64().ok_or("lastValidBlockHeight")?;
        Ok((blockhash, height))
    }

    pub fn account(&self, key: &Pubkey) -> Result<Option<Account>, String> {
        let value = self.call(
            "getAccountInfo",
            json!([key.to_string(), {"encoding": "base64", "commitment": "confirmed"}]),
        )?;
        parse_account(&value["value"])
    }

    /// `length` bytes of the account data from `offset` (for large accounts).
    pub fn account_slice(&self, key: &Pubkey, offset: usize, length: usize) -> Result<Option<Account>, String> {
        let value = self.call(
            "getAccountInfo",
            json!([key.to_string(), {"encoding": "base64", "commitment": "confirmed",
                                      "dataSlice": {"offset": offset, "length": length}}]),
        )?;
        parse_account(&value["value"])
    }

    /// Several accounts read at one slot, at least `min_slot` when given.
    pub fn accounts(&self, keys: &[Pubkey], min_slot: Option<u64>) -> Result<(u64, Vec<Option<Account>>), String> {
        let mut config = json!({"encoding": "base64", "commitment": "confirmed"});
        if let Some(slot) = min_slot {
            config["minContextSlot"] = json!(slot);
        }
        let keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
        let value = self.call("getMultipleAccounts", json!([keys, config]))?;
        let slot = value["context"]["slot"].as_u64().ok_or("context slot")?;
        let accounts = value["value"]
            .as_array()
            .ok_or("getMultipleAccounts value")?
            .iter()
            .map(parse_account)
            .collect::<Result<Vec<_>, _>>()?;
        Ok((slot, accounts))
    }

    pub fn balance(&self, key: &Pubkey) -> Result<u64, String> {
        self.call("getBalance", json!([key.to_string(), {"commitment": "confirmed"}]))?["value"]
            .as_u64()
            .ok_or_else(|| "getBalance".into())
    }

    pub fn rent_exempt_minimum(&self, length: usize) -> Result<u64, String> {
        self.call("getMinimumBalanceForRentExemption", json!([length]))?
            .as_u64()
            .ok_or_else(|| "getMinimumBalanceForRentExemption".into())
    }

    /// `simulateTransaction` with signature verification and the
    /// transaction's own blockhash.
    pub fn simulate(&self, transaction: &[u8]) -> Result<Value, String> {
        Ok(self.call(
            "simulateTransaction",
            json!([STANDARD.encode(transaction), {"encoding": "base64", "sigVerify": true,
                   "commitment": "processed", "replaceRecentBlockhash": false}]),
        )?["value"]
            .clone())
    }

    pub fn send(&self, transaction: &[u8], skip_preflight: bool) -> Result<String, String> {
        self.call(
            "sendTransaction",
            json!([STANDARD.encode(transaction), {"encoding": "base64", "skipPreflight": skip_preflight,
                   "preflightCommitment": "processed"}]),
        )?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| "sendTransaction".into())
    }

    pub fn signature_status(&self, signature: &str, history: bool) -> Result<Value, String> {
        Ok(self.call(
            "getSignatureStatuses",
            json!([[signature], {"searchTransactionHistory": history}]),
        )?["value"][0]
            .clone())
    }

    pub fn transaction(&self, signature: &str) -> Result<Value, String> {
        self.call(
            "getTransaction",
            json!([signature, {"encoding": "json", "commitment": "confirmed", "maxSupportedTransactionVersion": 0}]),
        )
    }
}
