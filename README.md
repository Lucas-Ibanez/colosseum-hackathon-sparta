# Hive

Escrow for development tasks between agents, released after deterministic verification and a zkVM proof.

> **Name.** The product is called Hive; until recently it was called VeriCode. The repository (`colosseum-hackathon-sparta`) and the code keep the previous name, and every technical identifier appears here exactly as it exists: the crates `vericode-core`, `vericode-escrow`, `vericode-cli`, `vericode-prover`; the binaries `vericode` and `vericode-prover`; the program `vericode_escrow` and its error type `VericodeEscrowError`; the worker `worker/vericode_worker.py` and its header `X-VeriCode-Token`; the `VERICODE_*` environment variables.
>
> Português: [`docs/README.pt-BR.md`](docs/README.pt-BR.md).

## What is demonstrated

In a Hive Job, the buyer deposits Test USDC into a vault controlled by the program. The executor delivers a restricted artifact. A RISC Zero guest evaluates the artifact with a fixed rule and publishes a journal with the verdict. The program moves the money only after checking the binding between the journal, the Job and the delivery and verifying the proof:

> Hive's Groth16 receipt is **verified on devnet via CPI to the immutable Groth16 verifier of risc0-solana v3.0.0**, in the same instruction that releases or refunds the Job's Test USDC. Example: the PASS settlement of Job P₃, [`2gB9djwM…`](https://explorer.solana.com/tx/2gB9djwM3WpApHLXYf5riF974mTK5W4UW7HCv1xJamLPu3NEwnPcCY4oT6zAwvbMkaKRcATf1LCQPyXP8rmpj3o?cluster=devnet).

There is no Verifier Router in the path, and none of this exists on mainnet.

| Case on devnet | Result | Transaction |
| --- | --- | --- |
| PASS, Job P₃ `26df9ef4…` (D11–D12, through the Hive interface on the worker) | `deliver`+`release`: proof verified (99,541 CU in the verifier) and 1 Test USDC to the executor | [`2gB9djwM…`](https://explorer.solana.com/tx/2gB9djwM3WpApHLXYf5riF974mTK5W4UW7HCv1xJamLPu3NEwnPcCY4oT6zAwvbMkaKRcATf1LCQPyXP8rmpj3o?cluster=devnet) |
| FAIL, Job F₃ `7395a76d…` (D11–D12, through the interface) | `deliver`+`refund_on_fail`: proof of the `FAIL` verdict verified (99,541 CU) and 1 Test USDC back to the buyer | [`4jcugMyX…`](https://explorer.solana.com/tx/4jcugMyXqhAttG7cjcjWZnMiyZe3SosHC2cgphQ6rf48n5KfrWZ4TRfzR1ZioP5NwYE2qeo2zUDEcD4rJQDTCmSJ?cluster=devnet) |
| Timeout, Job T₃ `2c38ca66…` (D11–D12, through the interface) | refund to the buyer after the deadline | [`5oTbFiTX…`](https://explorer.solana.com/tx/5oTbFiTXjkv8aBgTQPrKSqTShfNgVqSePZuP5x57GXxfEi6mTPMLfXFCWpnZSQzgcXbyqSjBHSJy65dEy5aAKVzi?cluster=devnet) |
| Negatives of D11–D12 (through the interface) | refund before the deadline (6021), receipt of Job `bc334093…` on P₃ (6014), tampered seal rejected by the verifier (6003) and second settlement (6007); nothing moves | [`4jDL97DG…`](https://explorer.solana.com/tx/4jDL97DGJRLQhB2m2NKK9kof85EFHSQMyPDPMmdhQ9mm6nkRKcbFF9HCuXcEiooWKKWomBCuhhTd2KbpYFAGiRHF?cluster=devnet), [`5CBFajsa…`](https://explorer.solana.com/tx/5CBFajsaFvYreCJkQwiPuR8rHWtpBKCULoVAeysUJGRwTDfF2CXYzeFkuit6azjBjqQkoif8TbYwxByv8Ckmo4en?cluster=devnet), [`kTQdVojx…`](https://explorer.solana.com/tx/kTQdVojxjzJULzS97Jo5HBFRcroqnYVMaK9Crtx2LmyAY1pUhnBgh7v1Xs686wY3gvvxGz3aiN9URWhNgepRg39?cluster=devnet), [`j2pmqFWu…`](https://explorer.solana.com/tx/j2pmqFWuTSe4H6isKyEFX3cBYmufEANP9Ue1p74tTUawXbiPSQ4KqckPW6WX3gnoAWsXE5XBdeVHEevpLa852ky?cluster=devnet) |
| PASS, Job `bc334093…` (D10, through the local worker on this repository's CLI and prover) | `deliver`+`release`: proof verified (99,541 CU in the verifier) and 1 Test USDC to the executor | [`ByGF4BFP…`](https://explorer.solana.com/tx/ByGF4BFPD8zkyqc6bg3Hd2FnQ1bF9gjo1EWp4fyVQkcGWfcj53gi99kWPLgyherzmvYp8knoqAzgjLLyhgpLnVR?cluster=devnet) |
| Timeout, Jobs `8ab4ee8d…` and `8bce67f2…` of the recordings (D10, through the worker) | refund to the buyer after the deadline | [`57UYbVX9…`](https://explorer.solana.com/tx/57UYbVX9gEXdp7dproD5mEyxj5UzxuQZdjwhQZe96XKTfCKbrWSxrfhKTmqrTdnAby8PGyJVSa1gDZsdk7P9Xd9v?cluster=devnet), [`625JvmR8…`](https://explorer.solana.com/tx/625JvmR85QE6LbRZaE49kf3Y2cSFVnbKGaynVBbb8eeqM1Dj6r8UVUZiUbDoA4g5egJfqYUW175pG6d4NHYZWc3y?cluster=devnet) |
| Negatives of D10 (through the worker) | refund before the deadline (6021), tampered seal rejected by the verifier (6003) and second settlement (6007); nothing moves | [`3PJfg2jU…`](https://explorer.solana.com/tx/3PJfg2jUmmDVc5RuN31DgF3pJwcYnog8GUCYy9vvyB56n1drScwiY8dTTsXwZWsX59DCuRDn33YhwViff3shxv4L?cluster=devnet), [`4cKK83JB…`](https://explorer.solana.com/tx/4cKK83JBYxRVfrAT5t6visKQdEWim5NANMe3eMH4bHsDmPumzq3WZb83F4eWiuYC694uivPqhsip3D8ySxJBmX2u?cluster=devnet), [`5ug6Pggx…`](https://explorer.solana.com/tx/5ug6PggxL9qZdgwkCu1Vt3oEYSFiBKXLdCi9bE6W1bewLjfa2yKpYjf8BeHzEjtvdkYDNLHZY4cjjBvkqtF9Bn5w?cluster=devnet) |
| PASS, Job P′ (D9, clean environment, this repository's CLI and prover) | `deliver`+`release`: proof verified (99,541 CU in the verifier) and 1 Test USDC to the executor | [`5tjezXYh…`](https://explorer.solana.com/tx/5tjezXYhN361HHUiZcMcJQFXwViWc4cSUfdreHfokdXuB89LrLDt6WPE8KRLhHwNE7rx5nGAgxjdfod6r7pPvDs1?cluster=devnet) |
| Timeout, Job T′ (D9) | refund to the buyer after the deadline | [`33ezPvow…`](https://explorer.solana.com/tx/33ezPvow48vSFHYS43i76pDJwkpyKTnDCN3Tdr1p7he8mbHjEpXvsBkt2MrMHKPUNW6AbqWE7oRJ7MriRZNxrjkh?cluster=devnet) |
| Negatives of D9 | refund before the deadline (6021), receipt of Job P of D7 on P′ (6014), tampered seal rejected by the verifier (6003) and second settlement (6007); nothing moves | [`3BKnczeK…`](https://explorer.solana.com/tx/3BKnczeKPQd15iCkYfPbnEvDbYayRfbVV9Zokqn3xLyVehJuZSNffgDy1paFTxof8tpYfaN92N789LzB2w4DQTd6?cluster=devnet), [`8Kk5UkX5…`](https://explorer.solana.com/tx/8Kk5UkX5XW8tssPMxNSCEBt71jiv4RriHLDtYqRzb1f1U5WLZfgQmotQTD1ZqecEbyqMmhWSjQqWpXxCJUubJLg?cluster=devnet), [`HXgnGUhh…`](https://explorer.solana.com/tx/HXgnGUhhB4zRgD8Bubs6ULCqpuhoqBSuim3Vbx1kz7V3bK2FFJG3ND2jcTtRFf6QxD9i1bpwKW4z32yFYYEofVr?cluster=devnet), [`5T9XE5Y3…`](https://explorer.solana.com/tx/5T9XE5Y3DzqKffoeFFzEhgqxsuzqMstpxtMCQikrZPe1RyP1KWvRmPNs9CFuke1yBgtwg5vVnGZVpwHWZr31Dfrs?cluster=devnet) |
| PASS, Job P (D7, this repository's CLI and prover) | `deliver`+`release`: proof verified (99,541 CU in the verifier) and 1 Test USDC to the executor | [`4oWhwZfU…`](https://explorer.solana.com/tx/4oWhwZfUzZhVhrTydrhdBx1TJxWVH2mdWdKwiUhtHtMnhmeJg3MprEshvp592hYMgsaiwwhyvsDHek1egS9zKM1L?cluster=devnet) |
| PASS, Job A (D4b) | `release` to the executor | [`4yWq28Gw…`](https://explorer.solana.com/tx/4yWq28GwkT9uLbG8haXbQYQyMqNWd6Qc29hWrez9cT4tGu36mS1fY8w6ocu75d5JxfQSLzTKKbMPc131fbL7chhR?cluster=devnet) |
| FAIL, Job B (D4b) | `refund_on_fail` to the buyer | [`2osG9m8J…`](https://explorer.solana.com/tx/2osG9m8JcE1CpAKt8JribtJCw8ffrdhBh6hYm5xKeLPptM9YLsugouMRH2KnmRXAymwAMvBABUmuZY6tkwHoVbP4?cluster=devnet) |
| Timeout, Job T (D7) | refund to the buyer after the deadline | [`3fiNWgTW…`](https://explorer.solana.com/tx/3fiNWgTWdscCB8kRTE4gxacQgNXCZ36NHNhazzgUtRBiF7ub7Zs3NDBBZ7qwEdXGEy2HGUsZyyZX9sv7gxFsVZ7?cluster=devnet) |
| Journal of another Job (D7) | rejected (6014), nothing moves | [`5eNRKgfH…`](https://explorer.solana.com/tx/5eNRKgfHii8gbWvJCNRTUehsT7Z46mXDC4Xorj2mfEfF4QjJ2nvFKwt3wmevJ5L81VXweaYpLzTxk3adKDAcBbJ7?cluster=devnet) |
| Tampered seal (D4b) | rejected by the verifier (6003), nothing moves | [`5UwKqSJs…`](https://explorer.solana.com/tx/5UwKqSJsku7rvYCXWz96BYNDNo3DjA8Yn7YXC4eMj1SYMhiUUvDLBvTLwWQEeabffLe8GtVFLC6x8Z1KH1PvCsUU?cluster=devnet) |
| Second settlement (D7) | `release` again on Job A → 6007; `refund_on_fail` again on Job B → 6008 | [`61de4rBk…`](https://explorer.solana.com/tx/61de4rBkBSN7UjqFrMed8a4KncQvRiBG4rNHJtqVA46B74tMUBoZ6stXVKx3RH2gMydCbRM4Ki9G7EXemgsqY7na?cluster=devnet), [`5vZ6TGRo…`](https://explorer.solana.com/tx/5vZ6TGRoF8hnNMCARsw9AdUkrQSQccavrNVxJLuXbiu1VH3rZ8wM1wfuk2DGpFGq1g4YqdHeysBFKdmi4Zag3pjG?cluster=devnet) |
| Refund before the deadline (D7) | rejected (6021), nothing moves | [`8DmQFdjE…`](https://explorer.solana.com/tx/8DmQFdjEjRsVHTfU1DmgjA6GmXZs52G1fdcuCaKmkSV9qXR6eEKh8MiGpKAKv6xa7LDdNPzK7AS9fbP9DgiyX1S?cluster=devnet) |

The full list, with CU, sizes and balances, is in [`docs/d4b-devnet-results.md`](docs/d4b-devnet-results.md), [`docs/d7-cli-results.md`](docs/d7-cli-results.md), [`docs/d9-demo-results.md`](docs/d9-demo-results.md), [`docs/d10-worker-results.md`](docs/d10-worker-results.md) and [`docs/d11-ui-results.md`](docs/d11-ui-results.md).

**What the proof does not say.** It attests that the admitted guest ran the fixed rule on the artifact the executor committed to when delivering. It does not prove that any code is correct: the v1 rule is trivial (`output = 2 × input`) and serves only to demonstrate the flow.

## Frozen phrases (English, ratified R-UI)

The README, the scripts, the videos, the worker and the screens use only these phrases, or paraphrases that say no more than they do (decision D-EN-1, entry "R-UI" of [`docs/decisions.md`](docs/decisions.md)). Phrase 1 carries the brand Hive. The equivalent Portuguese list, still valid, is in [`docs/README.pt-BR.md`](docs/README.pt-BR.md#frases-permitidas-congeladas-no-d9). Changing either list requires a decision recorded in `docs/decisions.md`.

1. **Claim:** "Hive's Groth16 receipt is verified on devnet via CPI to the immutable Groth16 verifier of risc0-solana v3.0.0, in the same instruction that releases or refunds the Job's Test USDC." Always with the link of a transaction.
2. **Statement:** "A previously committed deterministic evaluator ran on the artifact delivered in the Job and produced the published verdict. The program moves the Test USDC only after checking the binding between journal, Job and delivery and verifying the proof."
3. **Limit:** "The proof attests to the execution of the fixed rule on this artifact, not to the quality of a piece of software. The v1 rule is trivial (output = 2 × input) and serves to demonstrate the flow."
4. **No administrator:** "The escrow and the verifier are immutable (upgrade authority `none`). No one, not even the project, changes the rules or decides the payment; there is also no e-stop."
5. **Network:** "Devnet and Test USDC only; none of this exists on mainnet."
6. **Negatives:** "On devnet, a tampered proof, a journal from another Job, a refund before the deadline and a second settlement were rejected without moving funds." Always with the links.
7. **Reproduction:** "The flow was reproduced with this repository's CLI and prover in a clean environment: a fresh clone and fresh targets, with copied isolated toolchains, not a new machine." (D9)
8. **Proof:** "The proof is generated locally and compressed to Groth16 in a local Docker container, with no network access."

**Do not say:** "the code is correct"; "trustless" or "nobody needs to trust anybody"; "any repository"; "Verifier Router" as the current path; "mainnet" or "real money"; "ZK on-chain" without phrase 1; "audited", "private" or "new machine". Also do not present an earlier run as live, nor show a receipt without saying which Job it belongs to.

## Videos

| Video | Length | Link |
| --- | --- | --- |
| Pitch | up to 3 min | `TODO(video)` |
| Technical demo, a labeled cut of the continuous take ([cut list](docs/demo-script.md#demo-técnica-corte-de-23-min)) | 2–3 min | `TODO(video)` |
| Continuous take through the Hive interface, uncut (full evidence) | ~13 min | `TODO(video)` |

Scripts: [`docs/pitch-script.md`](docs/pitch-script.md) and [`docs/demo-script.md`](docs/demo-script.md). Submission text: [`docs/submission.md`](docs/submission.md).

## How it works

1. **Create and fund.** The buyer's `create_job` and `fund` go in a single transaction. The Job fixes the executor, the Test USDC mint, the amount, the deadline slot and the v1 terms (spec hash, harness hash and admitted image ID). The vault is a token account whose authority is the Job's PDA.
2. **Prove locally.** The executor runs `vericode-prover`: the RISC Zero guest evaluates the artifact `(input, claimed_output)` and publishes a 165-byte `JournalV1` (`schema_version`, `job_id`, `spec_hash`, `harness_hash`, `artifact_hash`, `image_id`, `verdict`). The `Composite` receipt is compressed to `Groth16` in a local Docker container, pinned by digest, with no network access.
3. **Deliver and settle.** `vericode job settle --deliver` sends `deliver` and `release` (verdict `PASS`) or `refund_on_fail` (verdict `FAIL`) in one transaction. Inside the settlement instruction, the escrow checks the journal against the Job and the delivery, calls the Groth16 verifier by CPI and moves the Test USDC with `TransferChecked`. Any mismatch or failed verification reverts the whole transaction.
4. **Timeout.** After the deadline slot, anyone can call `refund_on_timeout`, and the Test USDC goes back to the buyer.

The direct CPI replaced the Verifier Router in gate D4a, because the upstream Router on devnet is not initialized ([`docs/d4a-direct-verifier-results.md`](docs/d4a-direct-verifier-results.md)).

Stack: a pure Rust core (`no_std` + `alloc`, no Solana, Anchor or RISC Zero dependency) shared by the guest and the program; a RISC Zero `3.0.3` guest with a deterministic build; an Anchor `0.31.1` program; the `vericode` CLI and the `vericode-prover` prover in Rust; a local worker in Python (standard library only) that serves the Hive interface (HTML, CSS and JavaScript with no dependency).

## MVP scope

- A single restricted serialized artifact, evaluated by a deterministic rule and a fixed harness.
- A RISC Zero receipt with a public journal and a `PASS` or `FAIL` verdict.
- Escrow on Solana devnet with Test USDC: `PASS` pays the executor; `FAIL` or timeout refunds the buyer.

The MVP does not verify arbitrary repositories or general patches.

## Versions, hashes and addresses

| Item | Value |
| --- | --- |
| Escrow (devnet) | [`GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`](https://explorer.solana.com/address/GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH?cluster=devnet): 395,064 bytes, SHA-256 `cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133`; upgrade authority **`none`** since the [finalization](https://explorer.solana.com/tx/4AsofYxry7vjdpo2CQxSKSg9MdczuCB4GnRCLzz6ZeVFffPkWdedFBuBQDw58audRHJLcAQ1sGrN5G3t2CoW7eYH?cluster=devnet) |
| Groth16 verifier (devnet) | [`THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge`](https://explorer.solana.com/address/THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge?cluster=devnet), `risc0-solana v3.0.0` (commit `ee415935`), upgrade authority `None`; devnet bytes `34ae6e5c…`, local rebuild `dab6746d…`, structurally and functionally equivalent |
| Test USDC mint | [`9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F`](https://explorer.solana.com/address/9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F?cluster=devnet): SPL Token, 6 decimals, no freeze authority |
| Admitted guest | [`prover/artifacts/vericode-guest.bin`](prover/artifacts/README.md): 180,300 bytes `e09ba8cf…`; ELF `63fac491…`; ImageID `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a` |
| v1 terms | spec `af642b56…b778`, harness `01124025…6b50`, deadline from 1,500 to 1,512,000 slots ([`docs/manifest-schema.md`](docs/manifest-schema.md)) |
| `JournalV1` | v1 frozen, 165 bytes |
| Toolchains | Rust `1.89.0` (host; the core also on `1.85.0`); RISC Zero `3.0.3` (guest Rust `1.88.0`); Anchor `0.31.1` + Agave `2.3.9` |
| Groth16 prover (Docker) | `risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331` |
| Locks | root `191802b2…`; `zkvm` `f5236689…`; guest `1116acef…`; `anchor` `19a1db26…`; `anchor/tests-local` `be94760a…`; `cli` `4d979577…`; `prover` `8b76f1e1…` |

## Reproduce from a clone

Requirements:
- Linux x86_64, Rust `1.89.0` via rustup and a C/C++ compiler;
- about 8 GiB of RAM: run one heavy build or one proof at a time;
- Docker, only for Groth16, with the image `risczero/risc0-groth16-prover@sha256:7f173963…` (5.21 GB) already pulled by digest. The prover never pulls;
- the Agave `2.3.9` CLI (`solana`), only to download the programs in section 1.

**Network on the first build:** rustup (toolchain `1.89.0`), crates.io and, in the prover build, `recursion_zkr.zip` from RISC Zero's S3 (SHA-256 `744b999f…`), unless `RECURSION_SRC_PATH` points to a local copy ([`prover/README.md`](prover/README.md)). With these present, the builds and tests run with `--offline`.

### 1. Local tests

```bash
cargo +1.89.0 test --locked                                             # core, 42 tests
cargo +1.89.0 test --locked --release --manifest-path prover/Cargo.toml # prover: guest, frame, journal == core
cargo +1.89.0 test --locked --manifest-path cli/Cargo.toml              # CLI: bytes == the suite's builders
```

The program suite (`anchor/tests-local`, 61 tests) runs in process with the escrow `.so` and the verifier `.so`. Both can come from devnet. The 2.3.9 CLI requires a default signer even for reads; `-k` points to any devnet keypair, and nothing is signed.

```bash
SBF=~/vericode-sbf; mkdir -p $SBF            # outside the clone
solana -u devnet -k ~/devnet-key.json program dump GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH $SBF/vericode_escrow.so   # cdf6967f…
solana -u devnet -k ~/devnet-key.json program dump THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge $SBF/groth_16_verifier.so # 34ae6e5c…
(cd anchor/tests-local && SBF_OUT_DIR=$SBF cargo +1.89.0 test --locked)
```

To rebuild the escrow instead of downloading it: `cargo-build-sbf --manifest-path anchor/programs/vericode-escrow/Cargo.toml -- --locked` with Agave `2.3.9` (see [`docs/escrow-program.md`](docs/escrow-program.md)).

### 2. Check devnet (read only)

```bash
cargo +1.89.0 build --locked --release --manifest-path cli/Cargo.toml
V=cli/target/release/vericode
$V check        # escrow cdf6967f… with authority none, immutable verifier, mint
$V job show --job-id 91ea6fcdd2d0c29942246c664d893634b7c66f6cb2babbe436734a74bef5418e   # P′ (D9): Released
$V job show --job-id ec9afb74ceb080e09cb3d605d94883ff5ea90cc638c3f086f5e5c6d4cfed5c48   # T′ (D9): RefundedOnTimeout
$V job show --job-id 3e4ca0269e4af134738120703ccbfd751ef0d51fa6dcd2d99525d0d3f24c9c57   # P: Released
$V job show --job-id 05f7493483cba42146cada07822160d2fbec4e67d854fafa32f70ef0bb01d758   # T: RefundedOnTimeout
$V job show --job-id 3f0dd1c833714e193744c5b95879662c1d8b440cdd3c6467a1585ecc60baec8a   # A: Released
$V job show --job-id 5a25ae4808b6da928f57c8787c4fbb173791239d18342b540d4c485647fbc309   # B: RefundedOnFail
```

### 3. Prove locally

```bash
export RISC0_PROVER=local RISC0_EXECUTOR=local; unset RISC0_DEV_MODE
RUN=~/vericode-run; mkdir -p $RUN          # receipts and logs outside the clone
cargo +1.89.0 build --locked --release --manifest-path prover/Cargo.toml
P=prover/target/release/vericode-prover
$P check
$P prove <JOB_ID_HEX> 21 42 $RUN/P    # Composite, journal == core
$P compress $RUN/P                    # Groth16 (local Docker by digest, no network)
$P verify $RUN/P
```

Details, memory and `RECURSION_SRC_PATH` are in [`prover/README.md`](prover/README.md).

### 4. Full flow on devnet

Requires devnet keypairs (buyer, executor) outside the clone, with mode `0600`, devnet SOL and **Test USDC from the admitted mint**. The mint authority is the project's deployer key, so a third party has to receive Test USDC from it.

```bash
$V job create --buyer-keypair ~/keys/buyer.json --executor <PUBKEY_EXECUTOR> --deadline-offset 9000 --job-file $RUN/job.json
$P prove <JOB_ID_HEX> 21 42 $RUN/P && $P compress $RUN/P
$V job settle --job-id <JOB_ID_HEX> --receipt $RUN/P --deliver --executor-keypair ~/keys/executor.json
# or, without a delivery, after the deadline:
$V job refund-timeout --job-id <JOB_ID_HEX> --payer-keypair ~/keys/buyer.json --wait
```

Each operation checks the cluster, program, mint, terms, receipt and state before sending, simulates the transaction and prints the Explorer link. The negative scenarios use `--expect-error`. See [`cli/README.md`](cli/README.md) and the script [`docs/demo-script.md`](docs/demo-script.md).

## The Hive interface

The Hive interface is served by the local worker ([`worker/README.md`](worker/README.md)), which runs only on `127.0.0.1` and calls only the D10a binaries of the CLI and the prover:

```bash
python3 -B worker/vericode_worker.py --config <worker.json outside the clone>
```

Open exactly `http://127.0.0.1:8710/ui/` (with `localhost` the worker answers 421) and paste the token that the worker prints in the operator's terminal. The token stays only in the tab's memory; a reload asks for it again. There is no wallet in the browser (decision P3): every signature goes through the CLI in the worker. The interface shows the Jobs, a new Job (only the deadline is editable) and the Job detail: state timeline, commitments × journal, local and on-chain verification, settlement anatomy and balances, the adversarial scenarios (demonstration), the limits of the proof and the CLI command of each operation.

## Limitations

- Devnet and Test USDC only; no mainnet and no real money. Verification does not go through the Verifier Router, whose upstream on devnet is not initialized.
- **No e-stop.** The escrow (upgrade authority `none`) and the verifier are immutable. A bug can only be fixed with a new program ID, and a soundness bug in the verifier could not be paused. This was accepted for the MVP with Test USDC.
- The v1 rule is trivial, and the executor chooses the input: any `(n, 2n)` passes. The delivery commitment (`deliver`) prevents third parties from swapping the artifact, but does not make the task hard.
- `job_id` and Jobs are public, and the CLI generates a random `job_id` per Job. The rent of the Job and the vault (about 0.0036 SOL) stays locked, because there is no `close`.
- The Test USDC mint authority is the project's deployer key.
- The ImageID comes from the deterministic guest of D1c2b and has not been recertified since; the core only gained the `escrow` module, which the guest does not use.
- The Groth16 prover requires x86 Docker and a lot of memory; on the 7.6 GiB WSL the compression takes about 2 minutes.
- Findings of R-D7 in the CLI and the prover ([`docs/r-d7-review-results.md`](docs/r-d7-review-results.md)), fixed in D10a with tests ([`docs/d10a-hardening-results.md`](docs/d10a-hardening-results.md)) and confirmed by the R-D10a delta review ([`docs/r-d10a-review-results.md`](docs/r-d10a-review-results.md)):
  - `--expect-error escrow:N` only matches when the escrow is the innermost failure (RD7-01);
  - the Docker shim is an exact argv allowlist, with the image by digest, no network and the local daemon (RD7-02);
  - the prover uses only the local prover and refuses `RISC0_PROVER` other than `local`, `BONSAI_*` and `RISC0_DEV_MODE` (RD7-03);
  - the `--expect-error` matching and `--tamper-seal` have tests (RD7-04);
  - an "already processed" is resolved by the signature status, and the reads after the transaction use `minContextSlot` (RD7-07).

  The D9 binaries, used in the "Gravação" section of the script, predate these fixes. The documented negatives are still 6014, 6007/6008, 6021 and `verifier:6003`. RD7-08 remains open: an `escrow:6021` negative sent close to the deadline can turn into a real refund, and the CLI then reports `UNEXPECTED`.
- **Local worker, no wallet in the browser (D10, decision P3).** The worker ([`worker/`](worker/README.md)) runs only on `127.0.0.1` and calls only the D10a binaries. It holds, by path, the project's devnet keys (buyer and executor; never the deployer's), and the same local operator runs both roles. Proving (`Proving`) is a local step of the executor, off-chain. The Hive interface (D11–D12) is served by this worker at `http://127.0.0.1:8710/ui/`: the operator pastes the token from the terminal, it stays only in the tab's memory, and every signature still goes through the CLI in the worker.
- Phrase 8 holds for receipts whose compression recorded the shim's `docker_run` line (`--context default run --pull=never --network=none … @sha256:7f173963…`; condition C10-2 of R-D10a). The worker refuses the others.

## Repository structure

- `crates/vericode-core/`: canonical types, serialization, hashes, harness and pure escrow policy.
- `zkvm/`: RISC Zero guest and host (deterministic build of D1c2b).
- `anchor/`: escrow program and in-process tests (`anchor/tests-local`).
- `prover/`: local prover of the admitted guest (Composite → Groth16).
- `cli/`: devnet client (`vericode`).
- `worker/`: local HTTP worker on `127.0.0.1` over the CLI and the prover (D10) and the Hive interface in `worker/static/` (D11–D12); interface development tools in `worker/ui-tools/`.
- `brand/`: Hive brand assets (provisional SVGs, see [`brand/MANIFEST.md`](brand/MANIFEST.md)) and fonts.
- `docs/`: product context, decisions, evidence and the report of each gate (in Portuguese). Start with [`docs/project-context.md`](docs/project-context.md).

## Evidence

Commands, versions, hashes and real outputs are in [`docs/evidence.md`](docs/evidence.md) and in the `docs/d*-results.md` reports.

## License

This project is distributed under the [Apache-2.0](LICENSE) license.
