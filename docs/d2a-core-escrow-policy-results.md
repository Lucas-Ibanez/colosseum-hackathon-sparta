# D2a — política pura de escrow no core

Data: 2026-10-04

## Resultado

**CONCLUÍDO no escopo autorizado, com subescopo de refund/prazo em
`AGUARDANDO_AUTORIZAÇÃO`.** A política pura de escrow foi implementada em
`crates/vericode-core/src/escrow.rs`, sem Solana, Anchor ou RISC Zero, e
passou 37/37 testes com Rust `1.85.0` e `1.89.0`, locked/offline.

A política presume que um adaptador futuro já verificou a receipt. Este gate
não implementa nem alega verificação de receipt, seal, Groth16, Router ou
CPI. Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Preflight

```text
pwd                     /home/lucas/src/vericode
git rev-parse --show-toplevel  /home/lucas/src/vericode
branch                  main
HEAD                    a3eb9e067c6810a4c72fff250047007a5f672acb
git status --short      (vazio)
git diff --check        exit 0
/home/lucas/.rustup     ausente
target/ no clone        ausente
```

## Verificação do D1c2b nos arquivos atuais

- `docs/agent-control.md` (antes deste gate) registrava D1c2b.3j
  `CONCLUÍDO`; `docs/decisions.md` e `docs/evidence.md` contêm as entradas
  3i/3j; `docs/d1c2b3j-final-audit.md` declara receipts `Composite`, não
  Groth16, e Router/CPI/devnet `STATUS: NÃO VALIDADO`.
- Nenhum documento atual alega Groth16, Router, CPI ou verificação ZK
  on-chain. Receipts locais não são tratados como tal neste gate.
- Locks atuais iguais aos registrados no D1c2b: host
  `f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226`,
  guest `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`.
- Receipts, vendors e ELF do D1c2b estão em `/tmp` e não foram relidos nem
  reexecutados neste gate (sem Docker/proving autorizado).

### Errata encontrada

`docs/d1c2b3i-local-receipts-results.md` tem erro de transcrição:

| Local | Valor no relatório | Comprimento | Valor canônico (`lib.rs`, testado) |
| --- | --- | ---: | --- |
| linha 55, harness hash | `01124025…996b50d5aa` | 68 hex | `01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50` |
| linha 49, artifact PASS | `9223d6d2…442a224c` | 60 hex | `d5aa9223d6d2a1ba23bd73ca325b411c75027a739c285a95ff63b963442a224c` |

Os quatro hex `d5aa` foram deslocados entre os dois valores. O relatório
histórico foi preservado e a errata foi registrada em `docs/decisions.md`.
As receipts do D1c2b foram verificadas contra bytes exatos pelo host; a
errata afeta somente o texto do relatório.

## Decisões humanas desta sessão

1. O `artifact_hash` esperado é registrado uma única vez, somente pelo
   executor do Job, no estado `Funded`.
2. `JobV1` rejeita `buyer == executor`.

## Diff

| Arquivo | Alteração |
| --- | --- |
| `crates/vericode-core/src/escrow.rs` | novo: tipos, `JobV1`, estados, erros, transições e 17 testes |
| `crates/vericode-core/src/lib.rs` | `+2` linhas: `pub mod escrow;` após o código não-teste existente |
| `docs/escrow-state-machine.md` | novo: especificação da máquina de estados |
| `docs/d2a-core-escrow-policy-results.md` | novo: este relatório |
| `docs/architecture.md` | seção "Estado D2a" e linha do Core na tabela |
| `docs/decisions.md` | entrada D2a, pendências e errata 3i |
| `docs/evidence.md` | linha D2a |
| `docs/agent-control.md` | controle substituído pelo marco D2a |

A declaração do módulo foi colocada depois de `evaluate_restricted_artifact`
para não deslocar linhas do código não-teste já compilado no guest.
`Cargo.toml`, `Cargo.lock`, `zkvm/`, `JournalV1`, wire format e
`docs/manifest-schema.md` não foram alterados.

## Comandos e saídas reais

Todos os comandos usaram `env -i`, `RUSTUP_AUTO_UPDATE=0`,
`CARGO_NET_OFFLINE=true`, `CARGO_TARGET_DIR` no scratchpad da sessão (fora do
clone) e as homes isoladas:

```text
lane-a: CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/cargo
        RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/rustup
        PATH=<lane-a>/cargo/bin:/usr/bin:/bin   toolchain +1.85.0
lane-b: mesmas variáveis com lane-b               toolchain +1.89.0
```

Antes dos testes, os 29 pacotes registry do lock raiz foram conferidos nas
duas caches; nenhum faltava.

### `cargo +1.85.0 test --locked --offline` — exit `0`

```text
running 37 tests
test escrow::tests::delivery_is_registered_once_by_the_job_executor_after_funding ... ok
test escrow::tests::eligible_pass_releases_only_to_the_job_executor_and_mint ... ok
test escrow::tests::fail_verdict_matching_the_job_is_not_eligible ... ok
test escrow::tests::funding_requires_the_exact_buyer_mint_and_amount_once ... ok
test escrow::tests::job_construction_rejects_invalid_terms ... ok
test escrow::tests::rejects_a_journal_with_a_different_harness_hash ... ok
test escrow::tests::rejects_a_journal_with_a_different_image_id ... ok
test escrow::tests::rejects_a_duplicate_release ... ok
test escrow::tests::no_state_journal_recipient_or_mint_bypasses_the_policy ... ok
test escrow::tests::rejects_a_journal_for_a_different_job_id ... ok
test escrow::tests::rejects_a_journal_with_a_different_spec_hash ... ok
test escrow::tests::rejects_release_of_a_job_that_is_not_funded_or_delivered ... ok
test escrow::tests::rejects_transitions_out_of_the_terminal_state ... ok
test escrow::tests::rejects_a_mint_other_than_the_job_mint ... ok
test escrow::tests::transitions_do_not_change_the_job_terms ... ok
test escrow::tests::rejects_a_passing_journal_for_a_different_artifact_than_registered ... ok
test escrow::tests::rejects_a_recipient_other_than_the_job_executor ... ok
(+ 20 testes D1c1/D1c2a em tests::, todos ok)
test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Doc-tests vericode_core: 0 passed; 0 failed
```

Nenhum warning de compilação.

### `cargo +1.89.0 test --locked --offline` — exit `0`

```text
running 37 tests
test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### `cargo build --locked --offline` (lib `no_std`, perfil dev)

```text
lane-a +1.85.0: exit 0, Finished `dev` profile
lane-b +1.89.0: exit 0, Finished `dev` profile
```

### `cargo tree --locked --offline`

```text
lane-a +1.85.0: exit 0, SHA-256 da saída 9d99d866853c7d0c1bf4a57e094f9454ba65c23559cb35b52d9fbe76c774beea
lane-b +1.89.0: exit 0, SHA-256 da saída 9d99d866853c7d0c1bf4a57e094f9454ba65c23559cb35b52d9fbe76c774beea
cmp: idênticas
ocorrências de solana|anchor|risc0: 0
```

A árvore contém somente `vericode-core`, `borsh 0.10.4`, `sha2 0.10.9` e suas
dependências transitivas já registradas no D1c2a.

### Locks e fontes após o gate

```text
191802b234a6aa0f6bb9ce58a61963c377aed435a2baa6dec9576d13d8283b87  Cargo.lock                     (inalterado)
f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226  zkvm/Cargo.lock                (inalterado)
1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa  zkvm/methods/guest/Cargo.lock  (inalterado)
2d7845c7a5c0681c49fc9b02a47d1b5dca408119aede97f8719b837b2df42b3e  crates/vericode-core/Cargo.toml (inalterado)
c51a810f058d3d9e8042f6261708e1bea574f54c71088454a28ead253dbcc372  crates/vericode-core/src/lib.rs
f03110d31d1c79475b51e8e52d14ef9901bd5cbbe9ad989ffb44eaa37b2ad85e  crates/vericode-core/src/escrow.rs
```

### Higiene

- `git diff --check`: exit `0`; espaços finais nos arquivos novos: nenhum.
- Busca de padrões de segredo (chave privada, seed/mnemonic, `api_key`,
  `id.json`, `.keypair`, arrays de 64 bytes, tokens base58 longos) nos
  arquivos alterados: nenhuma ocorrência.
- `git status --short --ignored`: somente os arquivos listados no diff; nenhum
  `target/`, `.env`, keypair, `Anchor.toml` ou `programs/`.
- Termos `admin|override|bypass|force` em `escrow.rs`: somente nos
  comentários que declaram a ausência de override e no nome do teste de
  ausência de bypass.
- Rustfmt e Clippy não foram executados: `rustup component list --installed`
  nas duas toolchains isoladas (exit `0`) lista somente `cargo`, `rust-std` e
  `rustc`; nada foi instalado.

## Invariantes implementadas e cobertura de teste

| Invariante | Mecanismo | Teste |
| --- | --- | --- |
| identidades de 32 bytes distintas por papel | `BuyerId`, `ExecutorId`, `MintId` | todos |
| valor validado | `Amount::new` rejeita zero | `job_construction_rejects_invalid_terms` |
| `JobV1` imutável e válido | campos privados; rejeita identidade zero e `buyer == executor` | `job_construction_rejects_invalid_terms`, `transitions_do_not_change_the_job_terms` |
| funding exato pelo buyer, uma vez | `fund` | `funding_requires_the_exact_buyer_mint_and_amount_once` |
| entrega registrada uma vez pelo executor | `register_delivery` | `delivery_is_registered_once_by_the_job_executor_after_funding` |
| release elegível | `check_release`/`release` | `eligible_pass_releases_only_to_the_job_executor_and_mint` |
| `Verdict::Fail` não libera | `VerdictNotPass` | `fail_verdict_matching_the_job_is_not_eligible` |
| cada compromisso coincide | reuso de `JournalV1::validate_against` | `rejects_a_journal_for_a_different_job_id`, `…_spec_hash`, `…_harness_hash`, `rejects_a_passing_journal_for_a_different_artifact_than_registered`, `…_image_id` |
| executor do Job | `RecipientMismatch` | `rejects_a_recipient_other_than_the_job_executor` |
| mint do Job | `MintMismatch` | `rejects_a_mint_other_than_the_job_mint` |
| Job não funded/entregue | `NotFunded`, `DeliveryNotRegistered` | `rejects_release_of_a_job_that_is_not_funded_or_delivered` |
| release duplicado | `AlreadyReleased` | `rejects_a_duplicate_release` |
| estado terminal | `TerminalState` | `rejects_transitions_out_of_the_terminal_state` |
| sem admin bypass | nenhum parâmetro de autoridade; payout copiado do Job | `no_state_journal_recipient_or_mint_bypasses_the_policy` (5 estados × 2 journals × 3 recipients × 2 mints = 60 casos; único `Ok` é o elegível) |

`schema_version` divergente é coberto por `validate_against` e pelo decoder
(`incompatible_schema_version_is_rejected`); `JournalV1::new` não permite
construir outro schema.

## Decisões pendentes — `AGUARDANDO_AUTORIZAÇÃO`

Os documentos não definem uma condição objetiva de refund nem a política de
prazo; por isso o subescopo foi parado sem inventar regra:

- condição de refund e estado terminal correspondente;
- prazo, timeout, quem aciona cada transição temporal e se release após o
  prazo é permitido;
- efeito econômico de um `Verdict::Fail` válido;
- quem pode acionar `release` on-chain;
- layout, seeds e serialização do Job on-chain.

## Riscos abertos

- O ImageID `4da06f90…fb1a` foi calculado sobre o `lib.rs` anterior. O guest
  compila o core inteiro; o novo módulo não é usado pelo guest e as linhas do
  código não-teste não foram deslocadas, mas o ELF/ImageID não foi
  recertificado neste gate (sem Docker autorizado).
- Um Job `Delivered` com artefato que produz `FAIL` não tem transição de saída
  até que o refund seja decidido.
- O executor escolhe o artefato registrado; com a regra trivial de
  desenvolvimento, qualquer par `(n, 2n)` válido passa. Isso é limitação do
  harness de desenvolvimento, não da política.
- A política pura não garante atomicidade, custódia por PDA, validação de
  contas nem verificação de prova; tudo isso pertence a gates futuros
  Anchor/Router.
- O guia operacional ainda descreve o core sem "decisão de pagamento"; a
  fronteira atualizada está registrada em `docs/decisions.md` e
  `docs/architecture.md`, mas o guia não foi editado.
- Rustfmt/Clippy não executados.
- Router/CPI/devnet: `STATUS: NÃO VALIDADO`.

## Confirmação de fronteiras

Neste gate não foram usados nem criados: Anchor, `Anchor.toml`, programas,
IDL, Solana/Agave CLI, validator, wallet, seed, keypair, chave privada,
`.env`, Program ID, airdrop, transação, deploy, Router, CPI, Docker, rede,
instalação de ferramentas, commit ou push.
