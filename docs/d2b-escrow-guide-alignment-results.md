# D2b — política pura de escrow alinhada ao guia de produto

Data: 2026-10-04

## Resultado

**CONCLUÍDO.** `crates/vericode-core/src/escrow.rs` foi alinhado ao guia de
produto (§5 e §7):
- release ao executor em `Pass` válido até o prazo;
- refund ao buyer em `Fail` válido, em qualquer slot;
- refund por timeout somente após `deadline_slot`;
- `artifact_hash` registrado apenas na liquidação.

Resultado dos testes: 36/36 com Rust `1.85.0` e `1.89.0`, locked/offline, sem
warnings.

A política presume receipt já verificada por adaptador futuro. Nada aqui
verifica receipt, seal, Groth16, Router ou CPI. Router/CPI/devnet permanecem
`STATUS: NÃO VALIDADO`.

## Preflight e checagem da tarefa anterior

```text
pwd / raiz Git         /home/lucas/src/vericode
branch                 main
HEAD                   402426fe367fefd8b467814befb44230fdcb2ebe (D2a.2)
git status --short     (vazio)
git diff --check       exit 0
```

O prompt colado esperava como HEAD o commit D2a.1 (`0622709`). O HEAD real é
o D2a.2, `402426f`, criado imediatamente antes por autorização humana
explícita na mesma mensagem que iniciou este gate. A versão salva de
`docs/handoffs/d2a1-to-d2b.md` já esperava o D2a.2. Portanto, não houve
divergência de estado.

- `git log --oneline -4`: `402426f`, `0622709`, `4d7e18f`, `a3eb9e0`.
- Arquivos de contexto e de handoff presentes.
- Hashes antes da edição, iguais aos esperados:
  - `Cargo.lock` `191802b2…3b87`;
  - `zkvm/Cargo.lock` `f5236689…e226`;
  - guest lock `1116acef…dbfa`;
  - `escrow.rs` `f03110d3…ad85e`.
- Baseline na raia A: `test result: ok. 37 passed; 0 failed`, exit `0`.

## Decisões humanas confirmadas no Plan Mode

1. Estados de política: `Created` (≙ `Draft`), `Funded`,
   `Released { artifact_hash }` e `Refunded { reason }`.
   - `reason` é `Fail { artifact_hash }` ou `Timeout`.
   - `Proving`, `Submitted` e `Failed` são estados de worker/UI.
2. Release por `Pass` somente com `current_slot <= deadline_slot`.
3. Refund por `Fail` permitido antes ou depois do prazo.

Decisões já tomadas e apenas aplicadas:
- timeout somente com `current_slot > deadline_slot`;
- o slot é entrada, o core não lê relógio;
- `artifact_hash` registrado na liquidação (D2a.1).

## Diff

| Arquivo | Alteração |
| --- | --- |
| `crates/vericode-core/src/escrow.rs` | `deadline_slot` no `JobV1`; estados `Released`/`Refunded`; `RefundReason`, `PayoutRecipient`, `Settlement`; `release`, `refund_on_fail`, `refund_on_timeout`. Removidos `Delivered`, `register_delivery`, `check_release`, `expected_commitments` e os erros associados. Os 17 testes D2a foram substituídos por 16 |
| `docs/escrow-state-machine.md` | reescrito para o D2b; removida a nota "Revisão pendente" |
| `docs/d2b-escrow-guide-alignment-results.md` | este relatório |
| `docs/handoffs/d2b-to-d2c.md` | prompt da próxima fase |
| `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`, `docs/project-context.md`, `docs/architecture.md` | registro do D2b |

`lib.rs`, `Cargo.toml`, locks, `zkvm/`, `JournalV1`, wire format e
`docs/manifest-schema.md` permaneceram inalterados.

O vínculo do journal reutiliza `JournalV1::validate_against`, passando como
compromisso o próprio `artifact_hash` do journal. Assim:
- schema, Job, spec, harness e image são comparados;
- o artefato é apenas registrado.

## Comandos e saídas reais

Ambiente de todos os comandos:
- `env -i`, `RUSTUP_AUTO_UPDATE=0`, `CARGO_NET_OFFLINE=true`;
- `CARGO_TARGET_DIR` no scratchpad da sessão;
- homes isoladas D1a.3: `lane-a` com `+1.85.0` e `lane-b` com `+1.89.0`.

### `cargo +1.85.0 test --locked --offline` — exit `0`

```text
running 36 tests
test escrow::tests::funding_requires_the_exact_buyer_mint_and_amount_once ... ok
test escrow::tests::job_construction_rejects_invalid_terms ... ok
test escrow::tests::fail_never_releases_and_pass_never_refunds_on_fail ... ok
test escrow::tests::artifact_hash_is_recorded_from_the_journal_at_settlement ... ok
test escrow::tests::no_input_combination_bypasses_the_policy ... ok
test escrow::tests::fail_refunds_the_buyer_before_and_after_the_deadline ... ok
test escrow::tests::pass_releases_to_the_executor_up_to_the_deadline_inclusive ... ok
test escrow::tests::pass_after_the_deadline_is_rejected ... ok
test escrow::tests::refund_on_fail_rejects_each_divergent_commitment ... ok
test escrow::tests::rejects_a_mint_other_than_the_job_mint ... ok
test escrow::tests::rejects_double_settlement_and_funding_of_a_terminal_job ... ok
test escrow::tests::rejects_recipients_other_than_the_paid_job_party ... ok
test escrow::tests::rejects_settlement_of_an_unfunded_job ... ok
test escrow::tests::timeout_refunds_the_buyer_only_after_the_deadline ... ok
test escrow::tests::release_rejects_each_divergent_commitment ... ok
test escrow::tests::transitions_do_not_change_the_job_terms ... ok
(+ 20 testes D1c1/D1c2a em tests::, todos ok e inalterados)
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Doc-tests vericode_core: 0 passed; 0 failed
```

### `cargo +1.89.0 test --locked --offline` — exit `0`

```text
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### `build` e `tree --locked --offline`

```text
build lane-a +1.85.0: exit 0, 0 linhas de warning/error
build lane-b +1.89.0: exit 0, 0 linhas de warning/error
tree lane-a: exit 0, SHA-256 9d99d866853c7d0c1bf4a57e094f9454ba65c23559cb35b52d9fbe76c774beea
tree lane-b: exit 0, SHA-256 9d99d866853c7d0c1bf4a57e094f9454ba65c23559cb35b52d9fbe76c774beea
cmp A/B: idênticas; cmp com a árvore D2a: idêntica
ocorrências de solana|anchor|risc0: 0
```

### Hashes após o gate

```text
191802b234a6aa0f6bb9ce58a61963c377aed435a2baa6dec9576d13d8283b87  Cargo.lock                     (inalterado)
f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226  zkvm/Cargo.lock                (inalterado)
1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa  zkvm/methods/guest/Cargo.lock  (inalterado)
2d7845c7a5c0681c49fc9b02a47d1b5dca408119aede97f8719b837b2df42b3e  crates/vericode-core/Cargo.toml (inalterado)
c51a810f058d3d9e8042f6261708e1bea574f54c71088454a28ead253dbcc372  crates/vericode-core/src/lib.rs (inalterado)
8ea483d207552cded42869c243327516f466ac1bc4f3b8c764d70ba1ad4a1711  crates/vericode-core/src/escrow.rs
```

### Higiene

- `git diff --check`: exit `0`.
- Termos `admin|override|bypass|force` em `escrow.rs`: só aparecem no
  comentário que declara a ausência de override e no nome do teste de
  ausência de bypass.
- Busca de segredos nos arquivos alterados: nenhuma ocorrência.
- Rustfmt e Clippy: não executados, porque não estão instalados nas
  toolchains isoladas.

## Invariantes × testes

| Invariante (guia §7) | Teste |
| --- | --- |
| 2. buyer, executor, mint, amount e deadline do Job | `funding_requires_the_exact_buyer_mint_and_amount_once`, `rejects_a_mint_other_than_the_job_mint`, `transitions_do_not_change_the_job_terms` |
| 3. `Pass` paga só o executor | `pass_releases_to_the_executor_up_to_the_deadline_inclusive`, `rejects_recipients_other_than_the_paid_job_party` |
| 4. `Fail` devolve só ao buyer | `fail_refunds_the_buyer_before_and_after_the_deadline`, `fail_never_releases_and_pass_never_refunds_on_fail` |
| 5. timeout só após o prazo (lógica) | `timeout_refunds_the_buyer_only_after_the_deadline` (`0`, `deadline-1` e `deadline` falham; `deadline+1` passa); `pass_after_the_deadline_is_rejected` |
| 7. sem admin/destino livre | `no_input_combination_bypasses_the_policy` |
| 8. journal de outro Job/spec/harness/ImageID | `release_rejects_each_divergent_commitment`, `refund_on_fail_rejects_each_divergent_commitment` |
| 9. replay/dupla liquidação | `rejects_double_settlement_and_funding_of_a_terminal_job` (3 estados terminais × 4 operações) |
| artifact registrado na liquidação | `artifact_hash_is_recorded_from_the_journal_at_settlement` |
| Job não financiado | `rejects_settlement_of_an_unfunded_job` |
| construção do Job | `job_construction_rejects_invalid_terms` |

A matriz exaustiva cobre 810 chamadas, todas comparadas a um oráculo
independente:
- 5 estados × 3 journals × 3 slots × 3 identidades × 2 mints × 3 operações;
- os journals são PASS, FAIL e PASS de outro Job.

Resultado da matriz:
- exatamente 8 casos aceitos: 2 releases, 3 refunds por `Fail` e 3 por
  timeout;
- todos os aceitos pagam a parte correta com mint e amount do Job.

As invariantes 1 (vault PDA), 6 (atomicidade estado + transferência) e 10
(falha de CPI reverte) estão fora do core e continuam requisitos do programa.

## Decisões pendentes

- Quem assina e aciona cada liquidação on-chain. O destino já é fixo pelo Job.
- Layout, seeds e serialização do Job e do vault: gate Anchor.
- Caminho de verificação da receipt: Router/CPI ou fallback atestado rotulado.

## Riscos

- O ImageID `4da06f90…fb1a` não foi recertificado depois das mudanças no
  core. `escrow.rs` mudou, e o guest compila o crate inteiro, embora não use
  o módulo.
- Um executor que prova depois do prazo perde o release. É o comportamento
  decidido; deadlines curtos demais em relação ao tempo de proving são risco
  de produto.
- Com o harness de desenvolvimento, qualquer par `(n, 2n)` passa. É limitação
  do artefato, não da política.
- A revisão adversarial separada (guia §11) ainda não foi executada:
  **PENDENTE**.
- Rustfmt e Clippy não foram executados.

## Fronteiras

Neste gate não foram usados nem criados:
- Anchor, `Anchor.toml`, programas, IDL, Solana/Agave CLI, SPL;
- validator, wallet, keypair, seed, `.env`, Program ID;
- airdrop, transação, deploy, Router, CPI;
- Docker, rede, instalação de ferramentas, push.
