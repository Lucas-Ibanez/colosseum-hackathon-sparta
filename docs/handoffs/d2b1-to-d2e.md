# D2e — `release` e `refund_on_fail` vinculados à entrega, com CPI ao Verifier Router

> Substitui `docs/handoffs/d2d-to-d2e.md` (histórico, reprovado pelo R-D2).
> Este prompt incorpora o D2b.1 e as condições F-03, F-06, F-08 e F-12 da
> revisão `docs/r-d2-adversarial-review-results.md`.

## Identificação
- Gate: `D2e`  ·  Dias da sequência: D6/D7 (instrução Anchor integrada ao
  Router; prova errada rejeitada antes do happy path)  ·  Marcos do guia: M3
  e M5 (local)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D2b.1`. Commits:
  - `0e9838e` `core: bind settlement to the executor delivery (D2b.1)`;
  - `5736565` `anchor: add deliver and admitted v1 terms (D2b.1)`;
  - `docs: record D2b.1 delivery binding` (HEAD esperado).
- Relatório: `docs/d2b1-delivery-binding-results.md`.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh** — liquidação
  econômica, CPI, autoridade e destino de tokens.
- **Plan Mode obrigatório** (`CLAUDE.md`: escrow, programa Anchor e
  integração do Router). As decisões de produto estão abaixo; o Plan Mode
  aprova o desenho técnico e as decisões marcadas "a confirmar".

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§2, §5, §7, §8, §9, §11)
- `docs/r-d2-adversarial-review-results.md`: F-01, F-03, F-06, F-08, F-12,
  F-15
- `docs/d2b1-delivery-binding-results.md`, `docs/d2d-groth16-router-spike-results.md`
- `docs/escrow-program.md`, `docs/escrow-state-machine.md`,
  `docs/manifest-schema.md`, `docs/router-notes.md`
- `docs/d1a3-spike-results.md`: ABI do `verify`
- `docs/decisions.md`: D2b, D2c, D2d, D2c.1, R-D2, "Decisões humanas para
  o D2b.1" e D2b.1
- Histórico: `docs/handoffs/d2d-to-d2e.md`. **Não executar.** Lê-lo apenas
  pelos detalhes da ABI e do spike.
- Código:
  - `anchor/programs/vericode-escrow/src/lib.rs`;
  - `anchor/tests-local/tests/*.rs`;
  - `anchor/tests-local/fixtures/groth16/*`;
  - `crates/vericode-core/src/escrow.rs`, somente leitura.
- Harness do spike, fora do clone:
  - `~/.local/share/vericode-spikes/d2d/router-test/tests/router.rs`;
  - `…/d2d/receipts/src/main.rs`.

## Preflight
- `pwd`; raiz Git; branch; HEAD (esperado: commit
  `docs: record D2b.1 delivery binding`); `git status --short` (vazio);
  `git diff --check`.
- `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes.
- Snapshots de `~/.cargo` (listagem `find . -printf '%p %s %T@\n' | sort`,
  SHA-256 `d9e12578…`), `~/.avm` e `~/.docker` no início e no fim.
  `~/.docker/buildx/current` existe desde o D2d e é esperado.
- O WSL pode reiniciar e limpar `/tmp`. Helpers, logs e artefatos ficam em
  `~/.local/share/vericode-spikes/<gate>`. O helper do D2b.1 é
  `~/.local/share/vericode-spikes/d2b1/bin/env.sh` (`core_lane`, `d2c`,
  `sbf_build`, `prog_tests`).
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou
  stash.

## Checagem da tarefa anterior

Reexecutar com targets fora do clone:

| Item | Esperado |
| --- | --- |
| Core `cargo +1.85.0` (lane-a) e `+1.89.0` (lane-b) `test --locked --offline` | `42 passed` em cada, sem warnings |
| `cargo-build-sbf -- --locked` (Perfil A, ambiente `d2c` lane-b) | `vericode_escrow.so` de 367.288 bytes, SHA-256 `ea0dd92c039dcc6d8c9ab6dc4d808f43dd71b20616b82b1384f060690a941a37` |
| `anchor/tests-local`, `cargo +1.89.0 test --locked` | escrow 24, layout 4, fixtures 2 |
| Locks | raiz `191802b2…`, host `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…` |

Antes do build SBF, copiar `d2c/keys/vericode_escrow-keypair.json` para o
out-dir, sem exibi-lo.

Spike D2d:
- `sha256sum` de `d2d/vectors/*` igual a `d2d/logs/vectors.sha256`;
- `.so` do Router `1b26b017…` e do verificador `dab6746d…`;
- harness do Router (comando do relatório D2d) → `1 passed`.

Divergência → parar, registrar e reportar.

## Objetivo
Implementar `release` (`Pass` → executor) e `refund_on_fail` (`Fail` →
buyer) no `vericode_escrow`. Cada liquidação parte de `Delivered { h }` e
aceita somente:
- um `JournalV1` vinculado ao Job **e a `h`**, pelo core;
- e um seal Groth16 verificado por **CPI ao Verifier Router**, com o selector
  fixado, na mesma instrução e antes de qualquer transferência.

## Decisões já tomadas
- Política do core (D2b + D2b.1, `docs/escrow-state-machine.md`):
  - `release` só de `Delivered { h }`, com `Clock.slot <= deadline_slot`;
  - `refund_on_fail` só de `Delivered { h }`, em qualquer slot;
  - de `Funded` → `NotDelivered` (6025); journal de outro artefato →
    `ArtifactHashMismatch` (6017);
  - destino fixo no Job. O core não muda neste gate.
- Ordem obrigatória na instrução:
  1. `JournalV1::decode_candidate` sobre exatamente 165 bytes;
  2. `JobV1::release`/`refund_on_fail` com o estado, `Clock.slot`, dono e mint
     do destino;
  3. `SHA-256` on-chain **sobre os mesmos 165 bytes decodificados** (F-12);
  4. CPI `verify(seal, job.image_id, digest)`, com o `image_id` **do Job**,
     nunca do chamador nem do journal (F-12);
  5. `transfer_checked` de `job.amount` do vault, assinado pela PDA;
  6. persistir o estado terminal.

  Qualquer falha reverte tudo.
- **F-03, selector fixado:**
  - constante `SELECTOR = [0x73, 0xc4, 0x57, 0xba]`;
  - o escrow exige `seal.selector == SELECTOR` antes da CPI, com erro próprio
    no fim do enum;
  - `verifier_entry` igual à PDA `["verifier", SELECTOR]` do Router fixado,
    por constraint de endereço;
  - opcional, a confirmar: fixar também o Program ID do verificador.
  - Documentar a confiança residual: e-stop do dono do Router (bloqueia
    `release`/`refund_on_fail`; o timeout continua) e a upgrade authority.
- **F-06, ATA canônica:**
  - o destino de `release`, `refund_on_fail` e `refund_on_timeout` é a ATA
    canônica da parte para `job.mint`;
  - endereço = `find_program_address([parte, spl_token::ID, job.mint],
    ATA_PROGRAM_ID)`, com a constante
    `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`;
  - sem dependência nova. Nos testes, a criação da ATA é montada à mão, pelo
    programa ATA que o `solana-program-test` já embute.
  - Adicionar a feature `associated_token` do `anchor-spl` exige decisão
    explícita, porque pode mudar o lock.
  - Muda o comportamento de `refund_on_timeout`: registrar e atualizar os
    testes existentes.
- **F-08:** `address = job.mint @ MintMismatch` em toda conta mint nova.
- Fixtures: usar as versionadas em `anchor/tests-local/fixtures/groth16/`;
  não regenerar.
  - Job `0x11`; `pass` = artefato `(7,14)`, o mesmo `artifact_hash` que os
    testes do D2b.1 entregam; `fail` = `(7,15)`.
  - Seal cru com `pi_a` não negado; o cliente nega ao montar o `Seal`.
  - O Job dos testes precisa ser criado com os termos admitidos e um prazo
    na janela.
- Compute budget: `verify` consome cerca de 111 k CU (D2d). A transação
  precisa de instrução de compute budget acima de 200 k, montada sem
  dependência nova ou com decisão explícita.

## Decisões a confirmar no Plan Mode
1. **Como chamar o Router:**
   - (a) **recomendado:** CPI manual, sem dependência nova; compatibilidade
     provada contra o `.so` real.
     - discriminador `verify` `85a18d3078c65896`;
     - Borsh de `Seal{selector[4], proof{pi_a[64], pi_b[128], pi_c[64]}}`,
       `image_id[32]` e `journal_digest[32]`;
     - contas `[router PDA, verifier_entry, verifier_program, system_program]`.
   - (b) dependência git `verifier_router` pinada em `ee415935…`: rede ao
     GitHub e lock alterado.
2. **Program ID do Router:** o `declare_id` upstream `6JvFfBrv…`, usado no
   spike, registrando que o Router de devnet continua `NÃO VALIDADO`; ou uma
   constante por feature de cluster.
3. **Quem aciona `release`/`refund_on_fail`:** recomendação permissionless,
   porque o destino é fixo e a prova é verificada.
4. **Contas do Router no teste:** `VerifierRouter` e `VerifierEntry` montadas
   no genesis (dono em memória); `.so` do Router e do verificador
   reconstruídos do commit pinado por comando documentado.

Sem confirmação: `AGUARDANDO_AUTORIZAÇÃO`.

## Rede e instalação autorizadas
- Nenhuma por padrão; a opção 1(a) não precisa de rede.
- `index.crates.io`/`static.crates.io` com `--locked`, somente se o Plan Mode
  aprovar uma dependência.
- GitHub somente se a opção 1(b) for aprovada, e apenas
  `boundless-xyz/risc0-solana` no commit `ee415935…`.
- Sem Docker: as receipts Groth16 já existem nas fixtures.

## Escopo autorizado
- Código e testes:
  - `anchor/programs/vericode-escrow/src/lib.rs` (instruções, contas, erros
    a partir de 6033);
  - `anchor/tests-local/tests/` (arquivos existentes e novos);
  - locks de `anchor/` só com dependência aprovada.
- Docs a atualizar: `escrow-program.md`, `escrow-state-machine.md` (só
  pendências), `router-notes.md`, `decisions.md`, `evidence.md`,
  `agent-control.md`, `project-context.md`, `architecture.md` e `README.md`,
  se a capacidade passar.
- Docs a criar: `docs/d2e-router-settlement-results.md` e
  `docs/handoffs/d2e-to-<próximo>.md`.

## Fora de escopo / proibido
- `crates/vericode-core`, `JournalV1`, wire format, `zkvm/`, lock raiz,
  `docs/context/*`.
- Devnet, deploy, airdrop, RPC remoto; push.
- Aceitar `image_id`, selector, destino ou Program ID do Router vindos do
  chamador sem validação.
- Mock de Router ou verificador, seal sintético como prova, dev mode,
  receipt `Fake`.
- Alegar "ZK on-chain" em cluster. O máximo é "verificado por CPI ao Router
  em `solana-program-test` local".

## Implementação esperada
- Instruções `release(journal: [u8;165] ou Vec<u8>, seal)` e
  `refund_on_fail(journal, seal)`.
  - Contas: job, vault, mint (`address = job.mint`), destino (ATA canônica),
    token program, router (Program ID fixado), router PDA, verifier entry
    (PDA do selector fixado), verifier program, system program.
- Erros novos a partir de 6033, sempre no fim. 6000–6032 e as tags de
  `EscrowStatus` 0–5 ficam inalterados; `tests/layout.rs` estendido com os
  literais novos.
- IDL com exatamente 6 instruções (`create_job`, `fund`, `deliver`,
  `release`, `refund_on_fail`, `refund_on_timeout`), nenhuma administrativa.

## Testes obrigatórios (negativos antes dos positivos; toda rejeição com snapshot byte a byte de Job, vault e saldos)

Os dois primeiros são o PoC-1 do R-D2 on-chain, com seal válido e
verificável:
- **"FAIL de artefato não entregue":** entregar `h(7,14)` e chamar
  `refund_on_fail` com a fixture `fail` real `(7,15)` → 6017 antes da CPI;
- simétrico: entregar `h(7,15)` e chamar `release` com a fixture `pass` →
  6017.

Rejeições:

| Caso | Esperado |
| --- | --- |
| liquidação a partir de `Funded` | 6025 |
| seal adulterado | revert sem movimento |
| seal válido de outro journal (troca PASS↔FAIL) | revert |
| seal com selector ≠ `73c457ba` | erro do escrow (F-03) |
| `verifier_entry` de outro selector ou falso | rejeitado |
| Program ID de router falso | rejeitado |
| journal com ≠ 165 bytes ou tag inválida | rejeitado |
| FAIL em `release` | `VerdictNotPass` |
| PASS em `refund_on_fail` | `VerdictNotFail` |
| `release` após o prazo | `DeadlinePassed`, com `Clock.slot` conferido |
| destino ≠ ATA canônica da parte | rejeitado |
| ATA de outro mint | rejeitado |
| mint estranho | 6011 (F-08) |

Positivos:
- PASS: o executor recebe exatamente `amount`; estado `Released { h }`.
- FAIL, antes e depois do prazo: o buyer recebe `amount`; estado
  `RefundedOnFail { h }`.
- Timeout de `Delivered` para a ATA do buyer.

Dupla liquidação: release→release, release→refund_on_fail,
refund_on_fail→release, timeout→release, release→timeout e release→deliver.

Também:
- CU de `release`/`refund_on_fail`;
- testes anteriores ajustados (ATA) e passando; fixtures 2/2; core 42/42.

Comandos: os da checagem anterior e
`anchor-0.31.1 idl build -p vericode_escrow`. O binário está em
`d2c/homes/lane-b/avm/bin`, e o target fica fora do clone.

## Evidências exigidas
- `docs/d2e-router-settlement-results.md` com:
  - decisões, comandos e saídas;
  - SHA-256 do `.so` novo, da IDL e dos locks;
  - CU;
  - matriz testes × invariantes 3, 4, 6, 8, 9 e 10 × achados R-D2 (F-03,
    F-06, F-08, F-12);
  - riscos e fronteiras.
- Entradas D2e em `docs/decisions.md` e `docs/evidence.md`;
  `escrow-program.md` e `router-notes.md` atualizados.

## Critério de pronto
- Build SBF no Perfil A e todos os testes obrigatórios passam em processo,
  com o Router e o verificador reais.
- Códigos 6000–6032 inalterados.
- Nenhum segredo no Git; `git diff --check` exit 0; diff integral revisado;
  perfil padrão inalterado, salvo o já registrado.

## Condições de parada
- ABI do `verify` divergente do D1a.3/D2d, ou CPI não confirmável no fonte
  pinado.
- Necessidade de alterar o core, o journal ou o lock raiz.
- Rede fora do autorizado; resultado obtido com mock ou dev mode.

## Commit
- Commits locais autorizados ao final, somente se o critério de pronto for
  atendido:
  - `anchor: settle escrow through Verifier Router CPI (D2e)`;
  - `docs: record Router settlement (D2e)`.
- Identidade dos commits anteriores via `git -c`. Push proibido.

## Relatório final
1. arquivos modificados;
2. testes e saídas reais;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras (sem devnet, deploy, push nem claim ZK on-chain
   em cluster);
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`.

Depois do D2e: **revisão adversarial separada de D2b.1 + D2e** (Opus 5.5,
max, somente leitura) antes de qualquer gate devnet. Em seguida, o gate
devnet (D4): F-04 upgrade authority, F-05 allowlist do Test USDC e
confirmação do Router em devnet.
