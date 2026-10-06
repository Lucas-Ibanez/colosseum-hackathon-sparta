//! A fake Solana JSON-RPC server on 127.0.0.1 for the tests of `tx::run`
//! (gate D10a). Each test gives it a closure that answers by method; it
//! records the methods called. Only the standard library is used, nothing
//! leaves the loopback interface, and no keypair file exists: the tests sign
//! with in-memory keypairs.

#![allow(dead_code)]

use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};

/// Slot of the fake cluster; landed transactions are at `SLOT + 5`.
pub const SLOT: u64 = 600_000_000;
pub const LANDED_SLOT: u64 = SLOT + 5;

type Handler = dyn Fn(&str, &Value) -> Result<Value, Value> + Send + Sync;

pub struct FakeRpc {
    pub url: String,
    calls: Arc<Mutex<Vec<String>>>,
}

impl FakeRpc {
    /// `answer(method, params)` gives the `result` (`Ok`) or the JSON-RPC
    /// `error` object (`Err`).
    pub fn start(answer: impl Fn(&str, &Value) -> Result<Value, Value> + Send + Sync + 'static) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let calls = Arc::new(Mutex::new(Vec::new()));
        let answer: Arc<Handler> = Arc::new(answer);
        let recorded = calls.clone();
        thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let (answer, recorded) = (answer.clone(), recorded.clone());
                thread::spawn(move || serve(stream, &*answer, &recorded));
            }
        });
        Self { url, calls }
    }

    pub fn count(&self, method: &str) -> usize {
        self.calls.lock().unwrap().iter().filter(|called| *called == method).count()
    }
}

/// HTTP/1.1 with keep-alive: one JSON-RPC request per POST.
fn serve(stream: TcpStream, answer: &Handler, calls: &Mutex<Vec<String>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;
    loop {
        let mut length = 0_usize;
        let mut started = false;
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap_or(0) == 0 {
                return;
            }
            let line = line.trim_end();
            if line.is_empty() {
                if started {
                    break;
                }
                continue;
            }
            started = true;
            if let Some((name, value)) = line.split_once(':') {
                if name.eq_ignore_ascii_case("content-length") {
                    length = value.trim().parse().unwrap();
                }
            }
        }
        let mut body = vec![0_u8; length];
        reader.read_exact(&mut body).unwrap();
        let request: Value = serde_json::from_slice(&body).unwrap();
        let method = request["method"].as_str().unwrap().to_string();
        calls.lock().unwrap().push(method.clone());
        let mut response = json!({"jsonrpc": "2.0", "id": request["id"].clone()});
        match answer(&method, &request["params"]) {
            Ok(result) => response["result"] = result,
            Err(error) => response["error"] = error,
        }
        let body = response.to_string();
        let head = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
            body.len()
        );
        if writer.write_all(head.as_bytes()).and_then(|()| writer.write_all(body.as_bytes())).is_err() {
            return;
        }
    }
}

const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn base58(bytes: &[u8]) -> String {
    let mut digits: Vec<u8> = Vec::new();
    for &byte in bytes {
        let mut carry = u32::from(byte);
        for digit in digits.iter_mut() {
            carry += u32::from(*digit) << 8;
            *digit = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }
    let zeros = bytes.iter().take_while(|&&byte| byte == 0).count();
    let mut out = "1".repeat(zeros);
    out.extend(digits.iter().rev().map(|&digit| ALPHABET[digit as usize] as char));
    out
}

/// First signature of the base64 wire transaction in `params[0]`.
pub fn signature_of(params: &Value) -> String {
    let wire = STANDARD.decode(params[0].as_str().unwrap()).unwrap();
    base58(&wire[1..65])
}

pub fn custom(index: u8, code: u32) -> Value {
    json!({"InstructionError": [index, {"Custom": code}]})
}

pub fn failed(program: &impl std::fmt::Display, code: u32) -> String {
    format!("Program {program} failed: custom program error: {code:#x}")
}

/// The JSON-RPC error of a preflight that finds the signature already
/// processed (Agave: -32002, `TransactionError::AlreadyProcessed`).
pub fn already_processed() -> Value {
    json!({"code": -32002, "message": "Transaction simulation failed: This transaction has already been processed",
           "data": {"accounts": null, "err": "AlreadyProcessed", "logs": [], "unitsConsumed": 0}})
}

/// Answers for the methods every `tx::run` calls, from the simulated and
/// the landed outcome; `send` answers `sendTransaction`. `None` for the
/// landed outcome: the signature never appears and the blockhash expires.
pub fn answers(
    simulated: (Value, Vec<String>),
    landed: Option<(Value, Vec<String>)>,
    send: impl Fn(&Value) -> Result<Value, Value> + Send + Sync + 'static,
) -> impl Fn(&str, &Value) -> Result<Value, Value> + Send + Sync + 'static {
    let blockhash = base58(&[7_u8; 32]);
    move |method, params| match method {
        "getLatestBlockhash" => Ok(json!({"context": {"slot": SLOT},
                                          "value": {"blockhash": blockhash, "lastValidBlockHeight": 1_000}})),
        "simulateTransaction" => Ok(json!({"context": {"slot": SLOT},
                                           "value": {"err": simulated.0, "logs": simulated.1, "unitsConsumed": 1_234}})),
        "sendTransaction" => send(params),
        "getSignatureStatuses" => Ok(json!({"context": {"slot": SLOT}, "value": [landed.as_ref().map(|(err, _)| json!(
            {"slot": LANDED_SLOT, "confirmations": 1, "err": err, "confirmationStatus": "confirmed"}))]})),
        "getBlockHeight" => Ok(json!(if landed.is_some() { 1 } else { 2_000 })),
        "getTransaction" => Ok(match &landed {
            Some((err, logs)) => json!({"slot": LANDED_SLOT, "meta": {"err": err, "logMessages": logs,
                "computeUnitsConsumed": 1_234, "fee": 5_000, "preTokenBalances": [], "postTokenBalances": []},
                "transaction": {}}),
            None => Value::Null,
        }),
        other => Err(json!({"code": -32601, "message": format!("fake RPC: unexpected {other}")})),
    }
}
