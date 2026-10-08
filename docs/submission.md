# Hive: submission text

Text for the Colosseum submission form, in English.

> `TODO(form)`: the fields, character limits and rules of the current edition are not in
> the repository. Until the team supplies them, this text uses generic sections. When the
> form is known, map each field to a section below and cut to its limit without adding
> claims; every technical claim keeps its evidence link.

Claims about what was proven are only the ratified phrases
([`README.md`](../README.md#frozen-phrases-english-ratified-r-ui), F1–F8), quoted as they
are. Every Explorer link is on devnet.

## Project name

Hive

The repository and the code keep the previous name, `vericode`: crates `vericode-*`,
binaries `vericode` and `vericode-prover`, program `vericode_escrow`.

## One-liner

Escrow for development tasks between agents, released after deterministic verification
and a zkVM proof, on Solana devnet.

## Problem

AI coding agents are starting to take on delegated development tasks. When an agent
delivers work, someone still has to decide whether it gets paid: a person reviews the
delivery by hand, or both sides rely on the platform in the middle. That decision is hard
to check afterwards.

## Solution

Hive fixes the acceptance criteria of a Job before any work starts and keeps the payment
in an escrow on Solana devnet. The executor proves, with a zkVM, that the committed
evaluator ran on the delivered artifact; the program settles from that proof, with no
administrator and no button that decides the result.

> "A previously committed deterministic evaluator ran on the artifact delivered in the Job
> and produced the published verdict. The program moves the Test USDC only after checking
> the binding between journal, Job and delivery and verifying the proof." (F2)

## How it works

1. **Create and fund.** The buyer's `create_job` and `fund` go in one transaction. The Job
   fixes the executor, the Test USDC mint, the amount, the deadline slot and the v1 terms
   (spec hash, harness hash and admitted image ID).
2. **Prove locally.** The RISC Zero guest evaluates the artifact and publishes a 165-byte
   journal with `job_id`, `spec_hash`, `harness_hash`, `artifact_hash`, `image_id` and the
   `PASS`/`FAIL` verdict. The receipt is compressed to Groth16.
   > "The proof is generated locally and compressed to Groth16 in a local Docker container,
   > with no network access." (F8; evidence: [`docs/d11-ui-results.md`](d11-ui-results.md))
3. **Deliver and settle** in one transaction: `deliver` + `release` (`PASS`, pays the
   executor) or `deliver` + `refund_on_fail` (`FAIL`, refunds the buyer).
4. **Timeout.** After the deadline, `refund_on_timeout` returns the Test USDC to the buyer.

Stack: a pure Rust core (`no_std` + `alloc`) shared by the guest and the program; RISC
Zero `3.0.3`; Anchor `0.31.1`; the `vericode` CLI and the `vericode-prover` prover in Rust;
a local Python worker (standard library only) that serves the Hive interface (HTML, CSS and
JavaScript, no dependency), with no wallet in the browser.

## Solana integration

- The vault is a token account whose authority is the Job's PDA; the Job's state machine
  lives in an on-chain account.
- The settlement instruction (`release` or `refund_on_fail`) checks the journal against the
  Job and the delivery, calls the immutable Groth16 verifier of risc0-solana v3.0.0
  (`THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge`) by CPI (99,541 CU) and moves the Test
  USDC with SPL Token `TransferChecked`, all in the same instruction. Any mismatch or
  failed verification reverts the whole transaction.
- The escrow calls the verifier directly, without the Verifier Router: the upstream Router
  on devnet is not initialized ([`docs/d4a-direct-verifier-results.md`](d4a-direct-verifier-results.md)).
- Programs on devnet: escrow `vericode_escrow`
  [`GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`](https://explorer.solana.com/address/GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH?cluster=devnet);
  Test USDC mint
  [`9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F`](https://explorer.solana.com/address/9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F?cluster=devnet).

## What is proven (with evidence)

- **F1.** "Hive's Groth16 receipt is verified on devnet via CPI to the immutable Groth16
  verifier of risc0-solana v3.0.0, in the same instruction that releases or refunds the
  Job's Test USDC." PASS settlement of Job P₃:
  [`2gB9djwM…`](https://explorer.solana.com/tx/2gB9djwM3WpApHLXYf5riF974mTK5W4UW7HCv1xJamLPu3NEwnPcCY4oT6zAwvbMkaKRcATf1LCQPyXP8rmpj3o?cluster=devnet);
  FAIL settlement of Job F₃:
  [`4jcugMyX…`](https://explorer.solana.com/tx/4jcugMyXqhAttG7cjcjWZnMiyZe3SosHC2cgphQ6rf48n5KfrWZ4TRfzR1ZioP5NwYE2qeo2zUDEcD4rJQDTCmSJ?cluster=devnet).
- **F4.** "The escrow and the verifier are immutable (upgrade authority `none`). No one,
  not even the project, changes the rules or decides the payment; there is also no
  e-stop." Escrow finalization:
  [`4AsofYxr…`](https://explorer.solana.com/tx/4AsofYxry7vjdpo2CQxSKSg9MdczuCB4GnRCLzz6ZeVFffPkWdedFBuBQDw58audRHJLcAQ1sGrN5G3t2CoW7eYH?cluster=devnet).
- **F6.** "On devnet, a tampered proof, a journal from another Job, a refund before the
  deadline and a second settlement were rejected without moving funds." Through the Hive
  interface: refund before the deadline (6021)
  [`4jDL97DG…`](https://explorer.solana.com/tx/4jDL97DGJRLQhB2m2NKK9kof85EFHSQMyPDPMmdhQ9mm6nkRKcbFF9HCuXcEiooWKKWomBCuhhTd2KbpYFAGiRHF?cluster=devnet),
  receipt of Job `bc334093…` on Job P₃ (6014)
  [`5CBFajsa…`](https://explorer.solana.com/tx/5CBFajsaFvYreCJkQwiPuR8rHWtpBKCULoVAeysUJGRwTDfF2CXYzeFkuit6azjBjqQkoif8TbYwxByv8Ckmo4en?cluster=devnet),
  tampered seal rejected by the verifier (6003)
  [`kTQdVojx…`](https://explorer.solana.com/tx/kTQdVojxjzJULzS97Jo5HBFRcroqnYVMaK9Crtx2LmyAY1pUhnBgh7v1Xs686wY3gvvxGz3aiN9URWhNgepRg39?cluster=devnet),
  second settlement (6007)
  [`j2pmqFWu…`](https://explorer.solana.com/tx/j2pmqFWuTSe4H6isKyEFX3cBYmufEANP9Ue1p74tTUawXbiPSQ4KqckPW6WX3gnoAWsXE5XBdeVHEevpLa852ky?cluster=devnet).
- **F7.** "The flow was reproduced with this repository's CLI and prover in a clean
  environment: a fresh clone and fresh targets, with copied isolated toolchains, not a new
  machine." ([`docs/d9-demo-results.md`](d9-demo-results.md))
- All cases, with CU and balances: [`README.md`](../README.md#what-is-demonstrated).

## What is not proven

> "The proof attests to the execution of the fixed rule on this artifact, not to the
> quality of a piece of software. The v1 rule is trivial (output = 2 × input) and serves
> to demonstrate the flow." (F3)

## Limitations

- "Devnet and Test USDC only; none of this exists on mainnet." (F5)
- No e-stop: a bug can only be fixed with a new program ID.
- The v1 rule is trivial, and the executor chooses the input.
- No wallet in the browser: the local worker holds the project's devnet keys, and the same
  operator runs buyer and executor (decision P3).
- `job_id`s are public; the rent of each Job stays locked (no `close`); the Test USDC mint
  authority is the project's deployer key.
- Full list: [`README.md`](../README.md#limitations).

## Team

`TODO(team)`: names, roles and background, supplied by the team.

## Validation

`TODO(validation)`: only real feedback from users or design partners, supplied by the
team. If there is none, write: "No user validation yet."

## Business model

`TODO(business)`: written by the team, as a plan.

## Links

- Repository: <https://github.com/Lucas-Ibanez/colosseum-hackathon-sparta>
  (`TODO(repo)`: confirm that the judges can open it).
- Pitch video (up to 3 min): `TODO(video)`.
- Technical demo (2–3 min, a labeled cut of the continuous take): `TODO(video)`.
- Continuous take through the Hive interface, uncut (~13 min): `TODO(video)`.
- Evidence: [`docs/evidence.md`](evidence.md) and the `docs/d*-results.md` reports (in
  Portuguese).
