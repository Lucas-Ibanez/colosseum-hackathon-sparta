# D2b.1 — liquidação vinculada à entrega do executor e aos termos admitidos da v1

Data: 2026-10-04 · Executor: Claude Code (Opus 5.5) · Commits: `0e9838e` (core), `5736565` (anchor) · Gate anterior: R-D2
(`12529b4`, **REPROVADO** para o D2e)

## Resultado

**CONCLUÍDO.** Só o artefato **entregue e assinado pelo executor** pode
liquidar o Job por veredito, e só com a spec, o harness e o guest da v1.

| Achado | Correção |
| --- | --- |
| F-01 | `deliver(artifact_hash)` grava `Delivered { h }`; `release` e `refund_on_fail` exigem um journal com `artifact_hash == h`. Um terceiro que prova o FAIL de outro artefato recebe `ArtifactHashMismatch` |
| F-02 | `create_job` rejeita spec, harness e ImageID fora da v1 |
| F-07 | prazo dentro da janela de 1.500 a 1.512.000 slots; executor ≠ PDAs do Job e do vault. O timeout sempre devolve, de `Funded` ou de `Delivered` |
| F-08 | conta mint amarrada a `job.mint` |
| F-10 | claims corrigidos (ver "Errata F-10") |
| F-11 | lacunas de teste fechadas |

Não houve rede, instalação, Docker, devnet, deploy, keypair novo nem push.
`JournalV1`, wire format, `crates/vericode-core/src/lib.rs`, guest e locks
não mudaram. O guest não foi reconstruído.

## Preflight

- `/home/lucas/src/vericode`, branch `main`, HEAD `12529b4`
  (`docs: record R-D2 adversarial review`) sobre `42b4f58`.
- `git status --short` vazio; `git diff --check` exit 0.
- `docs/r-d2-adversarial-review-results.md` e a entrada "Decisões humanas
  para o D2b.1" presentes.
- `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes.
- Listagem de `~/.cargo` com SHA-256 `d9e12578…` no início e no fim.
- Helpers, logs e artefatos em `~/.local/share/vericode-spikes/d2b1/`
  (`bin/env.sh`, `logs/`, `out/`, `targets/`), fora do clone e fora de
  `/tmp`.

## Checagem da tarefa anterior (baseline D2c.1/R-D2)

| Comando | Saída real |
| --- | --- |
| core lane-a `cargo +1.85.0 test --locked --offline` | exit 0; 36 passed |
| core lane-b `cargo +1.89.0 test --locked --offline` | exit 0; 36 passed |
| `cargo-build-sbf -- --locked` (Perfil A, `d2c` lane-b) | exit 0, 1m20s; `vericode_escrow.so` 298.224 bytes, `d66ac76bc8ef66488facc6473b7679eac2fa5e605fba409a6cd61acce20baaae` (igual) |
| `anchor/tests-local` `cargo +1.89.0 test --locked` | exit 0; escrow 12 passed; fixtures 2 passed |
| Locks | raiz `191802b2…`, host `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…` (iguais) |

## Decisões aplicadas e desenho técnico (aprovado no Plan Mode)

Fonte: `docs/decisions.md`, "Decisões humanas para o D2b.1" e entrada D2b.1.

| Regra | Local |
| --- | --- |
| `deliver`; exigência de `Delivered { h }` e comparação do artefato; timeout de `Funded` e `Delivered`; `fund` em `Delivered` → `AlreadyFunded` | core, `crates/vericode-core/src/escrow.rs` |
| `JobV1::admit(admitted_image_id, current_slot)`: spec e harness calculados pelo core; ImageID igual ao parâmetro; janela por `deadline_slot.checked_sub(current_slot) ∈ [MIN, MAX]`; hash incalculável não admite nada | core |
| `ADMITTED_IMAGE_ID_V1 = 4da06f90…fb1a`; leitura do `Clock` | programa |
| executor ≠ PDA do Job e do vault (`ExecutorIsProgramAccount`) | programa |
| `#[account(address = job.mint @ MintMismatch)]` em `Fund` e `RefundOnTimeout` | programa |

Assinaturas novas do core:

```rust
pub const MIN_DEADLINE_WINDOW_SLOTS: u64 = 1_500;
pub const MAX_DEADLINE_WINDOW_SLOTS: u64 = 1_512_000;
impl JobV1 {
    pub fn admit(&self, admitted_image_id: ImageId, current_slot: u64) -> Result<(), JobError>;
    pub fn deliver(&self, state: EscrowState, deliverer: ExecutorId,
                   artifact_hash: Hash32, current_slot: u64) -> Result<EscrowState, EscrowError>;
}
```

Erros novos:

| Core | Programa |
| --- | --- |
| `EscrowError::{NotDelivered, AlreadyDelivered, DelivererMismatch}` | 6025–6027 |
| `JobError::{SpecNotAdmitted, HarnessNotAdmitted, ImageIdNotAdmitted, DeadlineOutOfWindow}` | 6028–6031 |
| — (só no programa) | 6032 `ExecutorIsProgramAccount` |

`EscrowStatus::Delivered` foi acrescentado com tag 5. Os códigos 6000–6024 e
as tags 0–4 não mudaram, e `JobAccount::INIT_SPACE` continua 276.

**Divergência do guia §5 e do D2a.1:** o `artifact_hash` deixa de ser
registrado "apenas na liquidação". Motivo: R-D2 F-01, decisão humana
registrada.

## Diff

| Arquivo | Alteração |
| --- | --- |
| `crates/vericode-core/src/escrow.rs` | `Delivered`, `deliver`, `admit`, constantes de janela, erros novos no fim. `release`/`refund_on_fail` passam a exigir `Delivered { h }` e a comparar `h`; timeout aceita `Delivered`. Testes reescritos: 22, antes 16 |
| `anchor/programs/vericode-escrow/src/lib.rs` | `ADMITTED_IMAGE_ID_V1`; `create_job` com executor ≠ PDAs e `admit`; instrução `deliver`; `address = job.mint`; `EscrowStatus::Delivered` (tag 5); erros 6025–6032 |
| `anchor/tests-local/tests/escrow.rs` | literais de erro; `DEADLINE` 1.000 → 5.000 (janela); `Clock.slot` conferido em cada warp; 12 novos testes (24 no total) |
| `anchor/tests-local/tests/layout.rs` | novo: códigos 6000–6032 por literal; tags e ida e volta das 6 variantes; `INIT_SPACE`; valores admitidos |
| `docs/escrow-state-machine.md`, `escrow-program.md`, `manifest-schema.md` (só "Validação pelo contrato"), `architecture.md`, `README.md` | D2b.1 e errata F-10 |
| `docs/decisions.md`, `evidence.md`, `agent-control.md`, `project-context.md` | registro |
| `docs/handoffs/d2b1-to-d2e.md` | prompt do D2e reescrito |

`anchor/tests-local/tests/groth16_fixtures.rs` e as fixtures não mudaram.

## Comandos e saídas reais

Ambiente: homes isoladas D1a.3 (core) e `d2c` lane-b (Perfil A), via
`env -i`; targets em `~/.local/share/vericode-spikes/d2b1/targets`.

### Core

```text
lane-a: cargo +1.85.0 test --locked --offline → exit 0; 42 passed; 0 warnings
lane-b: cargo +1.89.0 test --locked --offline → exit 0; 42 passed; 0 warnings
```

### Testes de mutação do core

Cópia de `Cargo.toml`, `Cargo.lock` e `crates/vericode-core` em
`d2b1/mutants/`, fora do clone; `cargo +1.89.0 test --locked --offline --lib
escrow`.

| Mutação | Resultado |
| --- | --- |
| M1: comparar o journal com o próprio `artifact_hash` (política F-01 do D2b) | 6 testes falham: `refund_on_fail_rejects_a_fail_of_an_undelivered_artifact`, `release_rejects_a_pass_of_an_undelivered_artifact`, os dois de compromissos divergentes, a matriz e a propriedade de destino único |
| M2: `Funded` liquida por veredito | `verdict_settlement_requires_a_delivery` falha |
| M3: `deliver` sem checar o deliverer | `deliver_rejects_other_deliverers_states_and_late_slots` e a matriz falham |
| M4: janela mínima `MIN - 1` | `admission_bounds_the_deadline_window` falha |
| M5: timeout recusa `Delivered` | 3 testes falham, entre eles a matriz e a propriedade |

### Programa

```text
cargo-build-sbf --manifest-path programs/vericode-escrow/Cargo.toml --sbf-out-dir d2b1/out/final -- --locked
  (target limpo d2b1/targets/sbf-final)
→ exit 0, 98 s; 14 warnings de macro do Anchor (13 antes: mais um
  `anchor-debug` gerado pelo `#[derive(Accounts)]` de `Deliver`)
→ vericode_escrow.so 367.288 bytes,
  SHA-256 ea0dd92c039dcc6d8c9ab6dc4d808f43dd71b20616b82b1384f060690a941a37
  (igual ao primeiro build, em outro target)

SBF_OUT_DIR=d2b1/out/final cargo +1.89.0 test --locked --no-fail-fast   (anchor/tests-local)
→ exit 0
  tests/escrow.rs: 24 passed; 0 failed
  tests/groth16_fixtures.rs: 2 passed; 0 failed
  tests/layout.rs: 4 passed; 0 failed

anchor-0.31.1 idl build -p vericode_escrow -o d2b1/out/idl/vericode_escrow.json
→ exit 0; 15.142 bytes;
  SHA-256 1026640831c0dce96d58e73e87e9b4de5c7a6e9696729eead6179e53e878ae70 (não versionada)
  instruções: create_job, deliver, fund, refund_on_timeout (4; nenhuma administrativa)
  deliver: contas executor (signer), job (writable); args artifact_hash
  create_job: mesmos 7 argumentos do D2c
  erros: 33 (6000–6032); EscrowStatus: Created, Funded, Released,
  RefundedOnFail, RefundedOnTimeout, Delivered
```

### Os testes novos contra o `.so` do D2c.1

`SBF_OUT_DIR=d2b1/out/baseline` (`d66ac76b…`), mesmo código de teste:
exit 101; **13 passed, 11 failed**.

| Teste | Resultado no programa antigo |
| --- | --- |
| `create_job_rejects_terms_not_admitted_for_v1` | termos fora da v1 aceitos |
| `create_job_bounds_the_deadline_window` | `expected 6031, got Ok` |
| `create_job_rejects_an_executor_that_is_a_program_account` | `expected 6032, got Ok` |
| `timeout_refund_rejects_other_recipients_and_mints` | `left: 3, right: 6011`, o `MintMismatch` do SPL Token, como no PoC-3 |
| os 6 testes com `deliver` | `InstructionFallbackNotFound` (101) |
| `unsupported_account_version_is_rejected` | 101 no `deliver` |
| `accounts_bound_to_the_job_reject_substitutes` | 101 no `deliver` |

Os 13 que passam são os testes que já existiam e os de Token-2022 e
squatting, que valem nas duas versões.

### Compute units (simulação da mesma transação, 2 execuções cada, determinístico)

| Instrução | D2c.1 (`d66ac76b…`) | D2b.1 (`ea0dd92c…`) |
| --- | ---: | ---: |
| `create_job` | 22.083 | 36.267 (+14.184: `hash_restricted_spec` + `hash_harness_version` + janela) |
| `deliver` | — | 4.949 |

O teste `create_job_compute_units` usa um prazo relativo ao `Clock`, para
rodar sem mudança nos dois programas.

## Testes

### Core (22 de escrow; 42 no total)

| Teste | Cobre |
| --- | --- |
| `refund_on_fail_rejects_a_fail_of_an_undelivered_artifact` | **PoC-1 invertido**: entregue `(7,14)`; FAILs `(7,15)`, `(7,0)`, `(8,15)`, `(1000000,1)` → `Journal(ArtifactHashMismatch)`; no mesmo estado, o release do PASS entregue é aceito em {0, prazo-1, prazo} |
| `release_rejects_a_pass_of_an_undelivered_artifact` | entregue `(7,15)`; 4 PASS de outros artefatos → `ArtifactHashMismatch` em {0, prazo-1, prazo}; em prazo+1, `DeadlinePassed` (ordem D2b); o FAIL entregue reembolsa |
| `verdict_settlement_requires_a_delivery` | `Funded` → `NotDelivered`; `Created` → `NotFunded` |
| `deliver_rejects_other_deliverers_states_and_late_slots` | buyer/terceiro → `DelivererMismatch`; `Created`; repetição com o mesmo ou outro artefato → `AlreadyDelivered`; prazo+1 e `u64::MAX` → `DeadlinePassed` |
| `deliver_records_the_executor_commitment_up_to_the_deadline_inclusive` | slots 0, prazo-1 e prazo; `Delivered` não terminal |
| `timeout_refunds_the_buyer_only_after_the_deadline` | `Funded`, `Delivered{pass}` e `Delivered{fail}`: até o prazo `DeadlineNotReached`; prazo+1 devolve |
| `admission_accepts_only_the_v1_terms` | spec, spec/harness trocados, harness, ImageID; termos antes da janela |
| `admission_bounds_the_deadline_window` | MIN-1, MAX+1, prazo ≤ criação, prazo 0, `u64::MAX`, criação perto de `u64::MAX` (sem overflow); MIN e MAX aceitos |
| `rejects_double_settlement_and_funding_of_a_terminal_job` | 3 terminais × 5 operações |
| `no_input_combination_bypasses_the_policy` | 7 estados × 5 journals × 3 slots × 3 partes × 2 mints, mais `deliver`; **oráculo independente**: lista explícita das 30 chamadas aceitas, comparada por igualdade de conjunto |
| `at_most_one_destination_per_state_and_slot` | 47 estados (Created, Funded, 42 `Delivered`, 3 terminais) × 5 slots × 44 journals do harness × 3 partes × 2 mints: no máximo um destino; casos-chave nomeados |
| `settlement_records_the_delivered_commitment` | o estado terminal grava o `h` entregue, igual a `hash_restricted_artifact` |
| demais | release/refund nos limites, compromissos divergentes (com o caso de artefato), destinatário, mint, construção e funding (`Delivered` → `AlreadyFunded`) |

Removido: `transitions_do_not_change_the_job_terms` (F-11/T7: trivial, `&self`
sobre tipo `Copy`).

### Programa (escrow 24, layout 4, fixtures 2)

Toda rejeição confere snapshot byte a byte (lamports e dados) de Job, vault,
saldos e, na criação, da conta do buyer. Os códigos esperados são literais.

| Teste | Cobre |
| --- | --- |
| `create_job_rejects_terms_not_admitted_for_v1` | spec 6028; spec/harness trocados 6028; harness 6029; ImageID 6030; contas não criadas |
| `create_job_bounds_the_deadline_window` | `Clock.slot` lido: `slot+MIN-1`, `slot+MAX+1`, `0`, `slot`, `u64::MAX` → 6031; `slot+MIN` e `slot+MAX` aceitos; slot inalterado no fim |
| `create_job_rejects_an_executor_that_is_a_program_account` | executor = PDA do Job ou do vault → 6032 |
| `create_job_rejects_token_2022` | mint Token-2022 → 3007; programa Token-2022 → 3008 (F-11/T1) |
| `create_job_rejects_a_duplicate_job_id` | squatter com mint próprio e amount 1; criação legítima → `Custom(0)`, o erro real (T3) |
| `deliver_rejects_other_signers_and_an_unfunded_job` | `Created` → 6005; buyer e terceiro → 6027; executor sem assinatura → 3010 |
| `deliver_is_rejected_after_the_deadline_and_accepted_at_it` | warp ao prazo com `Clock.slot == DEADLINE` conferido; Job com prazo `DEADLINE-1` → 6022; Job com prazo `DEADLINE` → `Delivered` |
| `deliver_records_the_executor_commitment_once` | `h` igual ao artefato da fixture `pass.txt` (Job `0x11`); nenhum token move; segundo deliver (mesmo `h` ou `fail`) → 6026; fund depois → 6006 |
| `timeout_refund_returns_a_delivered_job_to_the_buyer` | `Delivered`: antes e no prazo → 6021; prazo+1 devolve `amount`; deliver depois → 6008 |
| `timeout_refund_rejects_other_recipients_and_mints` | +F-08: conta mint estranha → 6011 (antes `0x3`) |
| `fund_rejections_move_no_tokens_and_keep_the_state` | +F-08 em `fund` |
| `accounts_bound_to_the_job_reject_substitutes` | vault falso → 2006 em fund e refund; cópia do Job com owner System → 3007 em refund, fund e deliver (T5) |
| `unsupported_account_version_is_rejected` | `version = 2` injetado → 6023 em fund e deliver (T6) |
| `timeout_refund_fails_before_and_at_the_deadline` e demais warps | `Clock.slot` conferido depois de cada warp (T2) |
| `create_job_compute_units`, `deliver_compute_units` | CU < 200.000 e impressão |
| `layout.rs` | 33 códigos por literal (T4); ida e volta e tags Borsh 0–5 das 6 variantes (T8); `EscrowStatus::INIT_SPACE == 33`, `JobAccount::INIT_SPACE == 276`; ImageID, spec, harness e janela iguais aos decididos; ImageID das fixtures |

## Matriz invariantes × testes × achados R-D2

| Achado / invariante | Testes | Situação |
| --- | --- | --- |
| **F-01** (inv. 3, 4, 8) | core: `refund_on_fail_rejects_a_fail_of_an_undelivered_artifact`, `release_rejects_a_pass_of_an_undelivered_artifact`, matriz, propriedade; programa: `deliver_*` | **resolvido na política**; on-chain, `release`/`refund_on_fail` ficam para o D2e |
| **F-02** (inv. 8) | `admission_accepts_only_the_v1_terms`; `create_job_rejects_terms_not_admitted_for_v1`; `layout::admitted_terms_are_the_decided_v1_values` | **resolvido** |
| **F-07** (inv. 5, 7) | `admission_bounds_the_deadline_window`; `create_job_bounds_the_deadline_window`; `create_job_rejects_an_executor_that_is_a_program_account`; timeout de `Delivered` | **resolvido** |
| **F-08** (inv. 2) | `timeout_refund_rejects_other_recipients_and_mints`; `fund_rejections_*` | **resolvido** em `fund`/`refund_on_timeout`; contas novas do D2e no handoff |
| **F-10** | docs (abaixo) | **corrigido** |
| **F-11** T1–T8 | ver tabela acima | **fechado** |
| inv. 6 (atomicidade) | snapshots em toda rejeição | testado localmente |
| inv. 9 (terminais) | `rejects_double_settlement_*` (5 operações); `timeout_refund_requires_funding_and_settles_once`; deliver após refund | testado |
| "um destino por estado/slot" | `at_most_one_destination_per_state_and_slot` | testado, sob a premissa de journals do harness (receipts verificadas) |

## Errata F-10

| Local | Antes | Agora |
| --- | --- | --- |
| `escrow-state-machine.md:74` | "Em nenhum slot dois destinos diferentes competem" | tabela por estado × slot e propriedade testada com premissa explícita |
| `escrow-program.md`; `decisions.md` (D2c.1) | `MintCannotFreeze` atribuído ao "SPL Token 7.0.0" | programa executado `spl_token-3.5.0.so` (do `solana-program-test 2.3.9`); 7.0.0 é o crate cliente; repetir em devnet. O relatório `d2c1-mint-freeze-and-fixtures-results.md` ficou fora do escopo; a errata está aqui e em `decisions.md` |
| `escrow-program.md` | "constraints validam somente estrutura" | estrutura + duas pré-condições de custódia + ImageID e `Clock` |
| `manifest-schema.md` "Validação pelo contrato" | compromisso "aceito/registrado" inexistente | compromisso `deliver` descrito; schema inalterado |
| `README.md`, `architecture.md` | "artefato vinculado ao Job" sem compromisso | o artefato do `deliver`; limitação da spec v1 declarada |
| `escrow-program.md` | "só pode corresponder a um Job" | por implantação (F-15) |

## Incidentes durante o gate (registrados, sem efeito no resultado)

1. `accounts_bound_to_the_job_reject_substitutes` falhou na primeira
   execução com pânico do `solana-accounts-db`
   (`calculate_accounts_hash_with_verify mismatch`).
   - Causa: `set_account` injetou lamports antes de `warp_to_slot`, que
     verifica a capitalização do banco.
   - Correção no teste: o warp vem antes da injeção.
2. `deliver_compute_units` falhou com `AccountInUse` ao usar
   `process_transaction_with_metadata` logo após o `fund`. A execução direta
   disputa locks com a fila interna do banks-server.
   - Correção: medir por `simulate_transaction`, que não trava contas, e
     depois enviar a mesma transação.
3. A primeira chamada de `anchor-0.31.1 idl build` falhou com exit 127: o
   binário fica em `d2c/homes/lane-b/avm/bin`, fora do PATH do helper. Foi
   repetida com esse diretório no PATH.

## Locks e fronteiras

- Inalterados:
  - raiz `191802b2…8b87`;
  - host `f5236689…e226`;
  - guest `1116acef…dbfa`;
  - `anchor/` `19a1db26…6765`;
  - `anchor/tests-local` `be94760a…a377`.
- Nenhuma dependência nova: `spl_token_2022` vem do re-export do
  `anchor-spl`, e as contas forjadas são injetadas com o tipo inferido de
  `get_account`.
- `~/.cargo` com o mesmo SHA-256 `d9e12578…`; `~/.rustup`,
  `~/.cache/solana` e `~/.config/solana` ausentes.
- Único uso de keypair: cópia do keypair do programa para o out-dir do
  build (`0600`), sem leitura nem exibição.
- `git diff --check` exit 0; busca de segredos no diff e nos arquivos novos
  sem ocorrências.

## Riscos abertos

- **ImageID não recertificado.**
  - `ADMITTED_IMAGE_ID_V1` é o do ELF D1c2b preservado.
  - O guest não foi reconstruído. `escrow.rs` faz parte da crate que o guest
    compila, então um rebuild pode gerar outro ImageID.
  - Recertificar antes de mudar a constante.
- **Spec v1 trivial:** o executor escolhe a entrada, e qualquer `(n, 2n)`
  passa. O `deliver` impede a troca do artefato por terceiros, mas não torna
  a tarefa difícil.
- **Preimagem do artefato:** com a spec v1, o espaço de artefatos PASS tem
  cerca de 10⁶ entradas, então o `h` de um PASS entregue é invertível por
  força bruta. Isso só permite provar o PASS, que paga o executor (resultado
  correto). O guia não promete privacidade.
- **Prazo apertado para o executor:** com a janela mínima (cerca de 10 min),
  o executor precisa entregar e provar (cerca de 90 s de Groth16 no D2d)
  antes do prazo. Depois dele, só resta o timeout ao buyer.
- **Condições do D2e:** F-03 (selector), F-06 (ATA canônica) e F-12 (digest
  sobre os 165 bytes e `job.image_id`). F-08 vale nas contas novas.
- **Antes do D4:** F-04 (upgrade authority) e F-05 (allowlist do mint).
- **F-09, F-13, F-14, F-15** permanecem baixos ou informativos.
- Revisão adversarial separada de D2b.1 + D2e pendente.

## Próximo gate

D2e, conforme [`docs/handoffs/d2b1-to-d2e.md`](handoffs/d2b1-to-d2e.md).
