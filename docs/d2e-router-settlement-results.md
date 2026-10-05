# D2e — `release` e `refund_on_fail` com CPI ao Verifier Router

Data: 2026-10-04 · Executor: Claude Code (Opus 5.5) · Commit do código:
`2f10a8f` · Gate anterior: D2b.1 (`0e9838e`, `5736565`, `75a1971`)

## Resultado

**CONCLUÍDO (local).** O `vericode_escrow` liquida por veredito:
- `release` (`Pass` → executor);
- `refund_on_fail` (`Fail` → buyer).

Cada uma só aceita um journal do **artefato entregue**, vinculado ao Job pelo
core, cuja receipt Groth16 o **Verifier Router** de `risc0-solana v3.0.0`
verificou por CPI, na mesma instrução e antes da transferência. O destino é
a ATA canônica da parte paga.

Os testes rodam em `solana-program-test` com:
- o Router e o verificador reais, reconstruídos offline do commit pinado;
- as receipts PASS/FAIL versionadas.

Claim máximo: **"verificado por CPI ao Verifier Router em
`solana-program-test` local"**. Router em devnet, deploy e "ZK on-chain" em
cluster: `STATUS: NÃO VALIDADO`.

Não houve rede, Docker, devnet, deploy, keypair novo nem push. Core,
`JournalV1`, `zkvm/` e locks não mudaram.

## Preflight e checagem do D2b.1

| Checagem | Resultado |
| --- | --- |
| Git | `/home/lucas/src/vericode`, `main`, HEAD `75a1971`; árvore limpa; `git diff --check` 0 |
| Perfil padrão | `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes |
| Snapshots no início | `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| Core lane-a `+1.85.0` / lane-b `+1.89.0` `test --locked --offline` | 42 passed cada, 0 warnings |
| `cargo-build-sbf -- --locked` (Perfil A) | `vericode_escrow.so` 367.288 bytes `ea0dd92c…1a37` (igual) |
| `anchor/tests-local` | escrow 24, layout 4, fixtures 2 |
| Vetores D2d `sha256sum -c` | 17/17 OK |
| `.so` D2d | Router `1b26b017…`, verificador `dab6746d…` (iguais) |
| Harness do Router (D2d) | `1 passed`; FIB/PASS/FAIL aceitos a 110.851 CU; adulterado 6003, selector desconhecido 3012, ImageID e digest errados 6000 |
| Locks | raiz `191802b2…`, host `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…` |

Helpers, logs e artefatos ficam em `~/.local/share/vericode-spikes/d2e/`
(fora do clone e de `/tmp`).

## Decisões confirmadas no Plan Mode

1. **CPI manual**, sem dependência nova: discriminador `verify`
   `85a18d3078c65896` + `RouterSeal` (Borsh do `Seal`) + `image_id` +
   digest. Compatibilidade provada contra o `.so` real.
2. **Router ID** fixo `6JvFfBrvCcWgANKh1Eae9xDq4RC6cfJuBcf71rp2k9Y7` (upstream),
   sem features de cluster.
3. **`release`/`refund_on_fail` permissionless**, porque o destino é fixo
   (ATA canônica) e a prova é verificada.
4. **Contas do Router no genesis dos testes:**
   - `VerifierRouter` e `VerifierEntry` com o layout do fonte pinado, dono em
     memória;
   - verificador por `add_program`;
   - `.so` reconstruídos offline do commit pinado.
5. **Program ID do verificador também fixado**
   (`THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge`).

Decisões já tomadas e aplicadas:
- política D2b + D2b.1;
- ordem decode → core → SHA-256 → CPI → transfer → estado;
- F-03 (selector `73c457ba` e PDA da entrada);
- F-06 (ATA canônica, também em `refund_on_timeout`);
- F-08 (`address = job.mint`);
- F-12 (digest sobre os 165 bytes decodificados; `job.image_id`).

## Fatos do fonte pinado (`ee415935`) usados

| Item | Fato |
| --- | --- |
| Router `Verify` | `router` (PDA `["router"]`), `verifier_entry` (PDA `["verifier", seal.selector]`, `selector` igual ao do seal), `verifier_program` (executável, `== entry.verifier`), `system_program`; nenhuma conta writable ou signer |
| Checagens do `verify` | rejeita entrada `estopped` (`SelectorDeactivated`); **não** checa a upgrade authority do verificador (só `add_verifier` checa) |
| Layouts | `VerifierRouter { ownership: Ownership { owner: Option<Pubkey>, pending_owner: Option<Pubkey> } }`, espaço `8+33+33`; `VerifierEntry { selector, verifier, estopped }`, espaço `8+32+4+1` |
| `pi_a` | o verificador espera `pi_a` negado (`y' = Q − y`, `Q` = módulo BN254) |

## Desenho implementado

Detalhado em [`docs/escrow-program.md`](escrow-program.md).

Constantes:
- `VERIFIER_ROUTER_ID`, `GROTH16_VERIFIER_ID`, `ATA_PROGRAM_ID`;
- `GROTH16_SELECTOR`, `ROUTER_VERIFY_DISCRIMINATOR`;
- PDAs pré-calculadas `ROUTER_PDA` (`4Sh5ofCz…`) e `GROTH16_VERIFIER_ENTRY`
  (`4Z7ok78x…`), conferidas por `tests/layout.rs`.

Instruções `release(journal: Vec<u8>, seal: RouterSeal)` e
`refund_on_fail(journal, seal)` sobre a struct `SettleWithProof`. A ordem,
num helper comum `settle_with_proof`, é:

| Passo | Ação | Rejeição |
| ---: | --- | --- |
| 1 | `JournalV1::decode_candidate` | 6034 |
| 2 | core | 6025/6017/6019/6020/6022/6010/6011… |
| 3 | ATA canônica da parte paga | 6035 |
| 4 | selector `73c457ba` | 6033 |
| 5 | SHA-256 dos mesmos bytes | — |
| 6 | CPI `verify(seal, job.image_id, digest)` | erro do Router ou do verificador |
| 7 | `transfer_checked` assinado pela PDA | — |
| 8 | estado terminal | — |

`refund_on_timeout` passou a exigir a ATA canônica do buyer, depois do core;
6010 e 6011 continuam iguais. Erros novos no fim: 6033
`UnexpectedSelector`, 6034 `JournalMalformed`, 6035 `DestinationNotCanonical`.

## Diff

| Arquivo | Alteração |
| --- | --- |
| `anchor/programs/vericode-escrow/src/lib.rs` | constantes do Router; `RouterSeal`; `release`, `refund_on_fail`, `settle_with_proof`, `require_canonical_destination`, `verify_with_router`, `transfer_from_vault` (extraído do timeout); `SettleWithProof`; ATA canônica no timeout; erros 6033–6035 |
| `anchor/tests-local/tests/common/mod.rs` | novo: helpers movidos de `escrow.rs` (sem mudança de lógica), ATA manual, base64, negação de `pi_a`, fixtures, `assert_failure` com logs da simulação, instruções de liquidação |
| `anchor/tests-local/tests/escrow.rs` | usa `common`; `setup` cria a ATA do buyer; novo teste F-06/PoC-4; os corpos dos 24 testes anteriores ficaram iguais (só o helper de fixture mudou de nome) |
| `anchor/tests-local/tests/settlement.rs` | novo: 16 testes com Router e verificador reais |
| `anchor/tests-local/tests/layout.rs` | literais 6033–6035; IDs, PDAs, discriminador e selector; layout Borsh de `RouterSeal` |
| docs | `escrow-program.md`, `escrow-state-machine.md` (introdução e pendências), `router-notes.md`, `architecture.md`, `README.md`, `decisions.md`, `evidence.md`, `agent-control.md`, `project-context.md`, este relatório, `handoffs/d2e-to-r-d2e.md` |

## Comandos e saídas reais

Ambiente `d2c` lane-b (Perfil A), `env -i`, homes isoladas, offline, targets
em `d2e/targets`.

```text
Rebuild offline do commit pinado (staging/lane-b/risc0-solana/solana-verifier,
INITIAL_OWNER=8pS2iUcJ…, keypairs de programa do D2d copiados sem leitura):
  cargo-build-sbf --manifest-path programs/groth_16_verifier/Cargo.toml -- --locked
  → exit 0, 77 s; groth_16_verifier.so dab6746d4a24f1d263f303ef91103166e97d03c5ae0e37893ed517b831e01e0c (igual ao D2d)
  cargo-build-sbf --manifest-path programs/verifier_router/Cargo.toml -- --locked
  → exit 0, 20 s; verifier_router.so 1b26b017b08eb62e79cee1d58dcec429edb2a1076dd5d4dc4b238c3ae79ce1c5 (igual ao D2d)
  staging: git status vazio antes e depois

cargo-build-sbf --manifest-path programs/vericode-escrow/Cargo.toml -- --locked   (target limpo)
→ exit 0, 84 s; 15 warnings de macro do Anchor (14 antes: mais um
  `anchor-debug` do `#[derive(Accounts)]` de `SettleWithProof`); nenhum aviso de stack
→ vericode_escrow.so 398.504 bytes,
  SHA-256 6457aecf471e6d2cb38796fd7dc4442572001b3025fdf8330cde29b93ca9ca96
  (igual ao build incremental)

SBF_OUT_DIR=d2e/out/d2e (vericode_escrow, verifier_router, groth_16_verifier)
cargo +1.89.0 test --locked --no-fail-fast   (anchor/tests-local)
→ exit 0
  tests/escrow.rs: 25 passed
  tests/settlement.rs: 16 passed
  tests/layout.rs: 6 passed
  tests/groth16_fixtures.rs: 2 passed
  nenhum warning no crate de testes

anchor-0.31.1 idl build -p vericode_escrow
→ exit 0; 21.779 bytes;
  SHA-256 37a3028a90ebbe0ebd080807b24cda730270d4bfd08279b1ec9bdfc3368c212c (não versionada)
  instruções: create_job, deliver, fund, refund_on_fail, refund_on_timeout,
  release (6; nenhuma administrativa); erros: 36 (6000–6035);
  tipos: EscrowStatus, JobAccount, RouterSeal
```

Observação da IDL: ela mostra o endereço fixo de `router_program` e `router`,
mas não o de `verifier_entry` e `verifier_program`. É limitação do gerador de
IDL do Anchor 0.31.1. As constraints existem no código, e
`router_accounts_are_fixed` prova 2012 para as quatro contas.

### Testes novos contra o `.so` do D2b.1 (`ea0dd92c…`)

| Suíte | Resultado |
| --- | --- |
| escrow | **24 passed, 1 failed**; só `timeout_refund_pays_only_the_canonical_buyer_account` (F-06) falha |
| settlement | **0 passed, 16 failed**; as instruções não existem |
| layout, fixtures | passam |

### Compute units (por simulação, sem instrução de compute budget)

| Instrução | CU por transação (5 execuções) | Dos quais |
| --- | --- | --- |
| `release` | 135.516 – 141.516 | Router 110.701 (verificador 99.541 dentro dele); SPL `transfer_checked` 6.174 |
| `refund_on_fail` | 135.345 – 139.845 | idem |

- A variação vem de `find_program_address` da ATA, cerca de 1.500 CU por
  tentativa de bump, que depende das chaves aleatórias de cada teste.
- **Cabe no limite padrão de 200 k CU por instrução**: o teste
  `settlement_fits_the_default_compute_limit` envia sem `SetComputeUnitLimit`.
- Isso corrige a premissa do D2d/handoff de que a transação "precisa" de mais
  de 200 k. O harness do D2d usava 1,4 M por margem.
- Os demais testes de liquidação incluem `SetComputeUnitLimit(400 000)`,
  montado à mão, como margem.

## Testes de liquidação (`tests/settlement.rs`)

Router e verificador reais; contas do Router no genesis; Job `0x11` com os
termos admitidos e `DEADLINE = 5 000`. Para cada rejeição, o teste:
- simula para ler os logs e identificar o programa que falhou, porque os
  códigos 6000+ do verificador, do Router e do VeriCode se sobrepõem;
- envia a transação;
- confere o snapshot byte a byte de Job, vault, ATA do buyer e ATA do
  executor.

"Antes do Router" significa que os logs não contêm invocação do Router.

| Teste | Esperado e observado |
| --- | --- |
| `refund_on_fail_rejects_a_valid_fail_of_an_undelivered_artifact` | **PoC-1 on-chain**: entregue `(7,14)`, fixture FAIL real `(7,15)` → 6017 do escrow, antes do Router |
| `release_rejects_a_valid_pass_of_an_undelivered_artifact` | entregue `(7,15)`, fixture PASS → 6017, antes do Router |
| `settlement_by_verdict_requires_a_delivery` | `Funded`: `release` e `refund_on_fail` → 6025, antes do Router |
| `malformed_journals_are_rejected` | 164 bytes, 166 bytes e tag de verdict 2 → 6034, antes do Router |
| `each_instruction_accepts_only_its_verdict` | FAIL em `release` → 6019; PASS em `refund_on_fail` → 6020 |
| `release_after_the_deadline_is_rejected` | `Clock.slot == DEADLINE+1` → 6022 |
| `settlement_pays_only_the_canonical_account_of_the_paid_party` | conta do executor fora da ATA → 6035; ATA do executor de outro mint → 6011; conta mint estranha → 6011 (F-08); ATA do buyer em `release` → 6010 |
| `seals_of_another_selector_are_rejected_by_the_escrow` | selector `deadbeef` → 6033, antes do Router (F-03) |
| `router_accounts_are_fixed` | Router, router PDA, entrada de outro selector ou verificador substituídos → 2012, antes do Router |
| `invalid_proofs_are_rejected_by_the_verifier` | `pi_c` adulterado → verificador 6003 (`PairingError`), via Router; seal válido do outro journal → verificador 6000 (`VerificationError`) |
| `an_emergency_stop_blocks_proofs_but_not_the_timeout` | entrada `estopped` → Router 6001 (`SelectorDeactivated`), verificador não chamado; após o prazo, o timeout devolve `BUYER_START_BALANCE` |
| `release_pays_the_executor_once` | ATA do executor recebe exatamente `AMOUNT`; vault 0; `Released { h(7,14) }`. Depois: release→release, release→refund_on_fail, release→deliver e release→timeout → 6007, sem movimento |
| `refund_on_fail_returns_the_amount_to_the_buyer_before_the_deadline` | ATA do buyer volta a `BUYER_START_BALANCE`; `RefundedOnFail { h(7,15) }`; refund→release → 6008 |
| `refund_on_fail_is_accepted_after_the_deadline` | em `DEADLINE+1` |
| `release_after_a_timeout_refund_is_rejected` | timeout→release → 6008 |
| `settlement_fits_the_default_compute_limit` | ver CU acima |

Em `escrow.rs`, `timeout_refund_pays_only_the_canonical_buyer_account`
reproduz o PoC-4: uma conta do buyer com delegate antigo → 6035; a ATA do
buyer recebe o valor, e a conta delegada fica com 0.

## Matriz invariantes × testes × achados R-D2

| Item | Testes | Situação |
| --- | --- | --- |
| inv. 3 (`Pass` paga só o executor) | `release_pays_the_executor_once`; `settlement_pays_only_the_canonical_account_of_the_paid_party` | **testado em processo** |
| inv. 4 (`Fail` devolve só ao buyer) | `refund_on_fail_*`; `each_instruction_accepts_only_its_verdict` | **testado em processo** |
| inv. 6 (atomicidade) | snapshot em toda rejeição, inclusive dentro do verificador | **testado localmente** |
| inv. 8 (journal de outro Job/artefato/ImageID) | 6017 com seal válido do outro artefato; `image_id` da CPI = `job.image_id`; termos admitidos (D2b.1) | **testado** |
| inv. 9 (dupla liquidação) | as 6 combinações | **testado** |
| inv. 10 (falha de verificação reverte) | seal adulterado, seal trocado, e-stop | **testado** |
| **F-01** on-chain | os dois PoC-1 com seals reais e verificáveis | **resolvido** |
| **F-03** | `seals_of_another_selector_*`; entrada fixa (2012); `layout::router_accounts_are_the_pinned_release` | **resolvido** |
| **F-06** | 6035 em `release` e no timeout (PoC-4) | **resolvido**; resíduo: delegate posto pela própria parte na sua ATA |
| **F-08** | conta mint estranha → 6011 na liquidação | **resolvido** |
| **F-12** | digest = SHA-256 dos mesmos 165 bytes decodificados; `image_id` do Job | **resolvido no código** |

Sobre o F-12: como o core exige `journal.image_id == job.image_id` antes da
CPI, um teste não distingue as duas fontes do `image_id`.

## Locks e fronteiras

- Locks inalterados (os cinco acima).
- Nenhuma dependência nova:
  - ATA, compute budget e CPI montados à mão;
  - contas do Router injetadas por `add_account_with_base64_data`.
- Snapshots no fim iguais aos do início: `~/.cargo` `d9e12578…`, `~/.avm`
  `7d29f7f8…`, `~/.docker` `6046f67f…`. `~/.rustup`, `~/.cache/solana` e
  `~/.config/solana` ausentes.
- Staging `risc0-solana` limpo.
- Keypairs:
  - nenhum criado;
  - copiados sem leitura para out-dirs (`0600`): o do `vericode_escrow` e os
    dois de programa do D2d;
  - o dono do Router nos testes é uma chave pública fixa, sem keypair.
- `git diff --check` exit 0; busca de segredos sem ocorrências.

## Riscos abertos

- **Router em devnet** (`STATUS: NÃO VALIDADO`): Program ID, dono, entrada
  `73c457ba` e verificador implantados não foram confirmados. Usar um Router
  próprio exige decidir o dono e as upgrade authorities.
- **Contas do Router nos testes:** montadas no genesis com o layout do fonte
  pinado, sem `initialize`/`add_verifier` nesta suíte. O `add_verifier` real
  foi exercitado no D2d.
- **Confiança residual:**
  - e-stop do dono do Router (testado: congela a liquidação por veredito; o
    timeout continua);
  - upgrade authorities do Router, do verificador e do escrow (F-04).
- **ATA:** precisa existir antes da liquidação (a CLI deve criá-la de forma
  idempotente). Um delegate posto pela própria parte continua válido.
- **ImageID admitido não recertificado** (guest não reconstruído).
- **Spec v1 trivial.**
- **CU variável** (135–142 k) pela busca do bump da ATA; ainda cabe no
  limite padrão.
- **F-05** (allowlist do mint), **F-09**, **F-13**, **F-14** e **F-15**
  inalterados.
- **Revisão adversarial separada de D2b.1 + D2e pendente**, obrigatória
  antes de qualquer gate devnet.

## Próximo gate

`R-D2e`: revisão adversarial somente leitura de D2b.1 + D2e, conforme
[`docs/handoffs/d2e-to-r-d2e.md`](handoffs/d2e-to-r-d2e.md).
