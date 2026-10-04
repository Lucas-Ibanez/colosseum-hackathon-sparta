# D2e — `release` e `refund_on_fail` no `vericode_escrow` com CPI ao Verifier Router

## Identificação
- Gate: `D2e`  ·  Dias da sequência: D6/D7 (instrução Anchor integrada ao
  Router; prova errada rejeitada antes do happy path)  ·  Marcos do guia: M3
  e M5 (local)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D2d` (GO), commit
  `docs: record Groth16 and Router spike (D2d)`; relatório
  `docs/d2d-groth16-router-spike-results.md`

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh** — liquidação
  econômica, CPI, autoridade e destino de tokens.
- **Plan Mode obrigatório** (`CLAUDE.md`: escrow, `programs`/`anchor` e
  integração do Router).

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§5, §7, §8, §9, §11)
- `docs/d2d-groth16-router-spike-results.md`, `docs/router-notes.md`,
  `docs/escrow-program.md`, `docs/escrow-state-machine.md`,
  `docs/manifest-schema.md`, `docs/d1a3-spike-results.md` (ABI do `verify`)
- `docs/decisions.md` (D2b, D2c e D2d)
- `anchor/programs/vericode-escrow/src/lib.rs`, `anchor/tests-local/tests/escrow.rs`,
  `crates/vericode-core/src/escrow.rs`
- Harness do spike, fora do clone:
  `~/.local/share/vericode-spikes/d2d/router-test/tests/router.rs` e
  `…/d2d/receipts/src/main.rs`

## Preflight
- `pwd`; raiz Git; branch; HEAD (esperado: commit D2c.1
  `anchor: reject freezable mints and add Groth16 fixtures (D2c.1)` sobre
  `9a18f71`, ou posterior se a revisão adversarial tiver gerado commits de
  correção autorizados);
  `git status --short` (vazio); `git diff --check`.
- `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes.
- Snapshots de `~/.cargo`, `~/.avm` e `~/.docker` no início e no fim.
  `~/.docker/buildx/current` existe desde o D2d e é esperado.
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou
  stash.

## Checagem da tarefa anterior
- `git log --oneline -10` contém o commit D2c.1, `9a18f71` (D2d), `ec980e9` e
  `a10f026`.
- Achados da revisão adversarial do D2b/D2c/D2c.1
  (`docs/handoffs/d2c1-to-review-d2b-d2c.md`): ler o relatório e tratar os
  bloqueantes antes de começar; se a revisão não tiver ocorrido, parar e
  perguntar.
- Core 36/36 nas duas raias.
- Programa:
  - `cargo-build-sbf -- --locked` deve repetir `vericode_escrow.so`
    `d66ac76bc8ef66488facc6473b7679eac2fa5e605fba409a6cd61acce20baaae`
    (D2c.1);
  - `anchor/tests-local`: escrow 12/12 e fixtures 2/2.
- Spike D2d:
  - `sha256sum` de `d2d/vectors/*` igual a `d2d/logs/vectors.sha256`;
  - `.so` Router `1b26b017…` e verificador `dab6746d…`;
  - reexecutar o harness do Router (comando do relatório D2d) → `1 passed`.
- Locks do repositório:
  - raiz `191802b2…`;
  - host `f5236689…`;
  - guest `1116acef…`;
  - `anchor/` `19a1db26…`;
  - `anchor/tests-local` `be94760a…`.

## Objetivo
Implementar no `vericode_escrow` as liquidações por veredito:
- `release`: `Pass` paga o executor;
- `refund_on_fail`: `Fail` devolve ao buyer.

Cada uma aceita somente um journal `JournalV1` vinculado ao Job (pelo core)
**e** um seal Groth16 verificado por **CPI ao Verifier Router** na mesma
instrução, antes de qualquer transferência. Os testes rodam em processo, com
as receipts Groth16 reais do D2d e o Router/verificador reais.

## Decisões já tomadas
- Política econômica do core (D2b):
  - `release` só com `current_slot <= deadline_slot`;
  - `refund_on_fail` em qualquer slot;
  - destino fixo (executor ou buyer do Job);
  - `artifact_hash` do journal registrado na liquidação.
- Caminho forte comprovado no D2d. A transação precisa de compute budget
  acima de 200 k (`verify` consome cerca de 111 k CU).
- Ordem obrigatória na instrução:
  1. decodificar o journal (`JournalV1::decode_candidate`) e aplicar
     `JobV1::release`/`refund_on_fail` com `Clock.slot`, dono e mint da token
     account de destino;
  2. calcular `SHA-256(journal)` on-chain;
  3. CPI `verify(seal, job.image_id, digest)`;
  4. transferir `job.amount` do vault, assinado pela PDA;
  5. persistir o estado terminal.

  Qualquer falha reverte a transação inteira.
- O `image_id` enviado ao Router vem **do Job**, nunca do chamador. O core
  exige `journal.image_id == job.image_id`.
- O Program ID do Router deve ser **fixado** no programa (constraint de
  endereço). Router PDA, entrada do verificador e verificador são validados
  pelo próprio Router.

## Decisões a confirmar no Plan Mode (perguntar ao humano)
1. **Como chamar o Router:**
   - (a) CPI manual com discriminador `verify` `85a18d3078c65896`, Borsh de
     `Seal{selector[4], proof{pi_a[64], pi_b[128], pi_c[64]}}`, `image_id[32]`
     e `journal_digest[32]`, e contas `[router PDA, verifier_entry,
     verifier_program, system_program]`. Sem dependência nova no programa;
     compatibilidade provada pelos testes contra o `.so` real.
     **Recomendado.**
   - (b) Dependência git do crate `verifier_router` (feature `cpi`) pinada no
     commit `ee415935…`; exige rede ao GitHub e muda o lock.
   - (c) Vendor do crate no repositório.
2. **Program ID do Router a fixar:** o `declare_id` upstream `6JvFfBrv…` (o
   que o teste usa), registrando que o Router de devnet continua
   `NÃO VALIDADO`; ou uma constante por feature de cluster.
3. **Fixtures de teste no repositório:**
   - Já versionadas no D2c.1 em `anchor/tests-local/fixtures/groth16/`
     (`pass.txt`/`fail.txt` em hex, seal cru com `pi_a` não negado), com teste
     de consistência. Reutilizar; não regenerar.
   - Montar no genesis do teste as contas `VerifierRouter` e `VerifierEntry`
     (dono em memória), para não depender do keypair de teste do D2d.
   - Os `.so` do Router/verificador são reconstruídos do commit pinado por
     comando documentado.
4. **Quem aciona `release`/`refund_on_fail`:** recomendação: permissionless
   (destino fixo), como no timeout.

## Rede e instalação autorizadas (somente estas)
- `index.crates.io`/`static.crates.io` com `--locked` após a resolução
  aprovada.
- GitHub somente se a opção 1(b) for aprovada, e apenas
  `boundless-xyz/risc0-solana` no commit `ee415935…`.
- Nada de Docker: as receipts Groth16 já existem em `d2d/vectors`.
- Qualquer outro destino: parar e perguntar.

## Escopo autorizado
- `anchor/programs/vericode-escrow` (novas instruções, contas, erros),
  `anchor/tests-local` (testes e fixtures), locks de `anchor/` se necessário.
- Criar `docs/d2e-router-settlement-results.md` e
  `docs/handoffs/d2e-to-<próximo>.md`.
- Atualizar `docs/escrow-program.md`, `docs/router-notes.md`,
  `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`,
  `docs/project-context.md`, `docs/architecture.md` e o README, se a
  capacidade passar.

## Fora de escopo / proibido
- Alterar `crates/vericode-core`, `JournalV1`, wire format, `zkvm/`, lock raiz
  ou `docs/context/*`.
- Devnet, deploy, airdrop, RPC remoto; push.
- Aceitar `image_id`, destino ou Program ID do Router vindos do chamador sem
  validação.
- Mock de Router ou verificador, seal sintético como prova, dev mode.
- Alegar "ZK on-chain" em cluster; o máximo é "verificado por CPI ao Router
  em `solana-program-test` local".

## Testes obrigatórios (negativos antes dos positivos)
- `release`:
  - seal adulterado → revert sem movimento;
  - seal válido de **outro journal** (troca PASS↔FAIL) → revert;
  - Program ID de router falso → rejeitado;
  - journal FAIL → `VerdictNotPass`;
  - Job com outro `job_id`/`image_id`/spec/harness → rejeitado pelo core;
  - após o prazo → `DeadlinePassed`;
  - destino ≠ executor ou outro mint → rejeitado;
  - depois o positivo PASS: executor recebe exatamente `amount`; estado
    `Released{artifact_hash}`.
- `refund_on_fail`:
  - seal adulterado;
  - journal PASS → `VerdictNotFail`;
  - destino ≠ buyer;
  - depois o positivo FAIL, antes e depois do prazo: buyer recebe `amount`;
    estado `RefundedOnFail{artifact_hash}`.
- Dupla liquidação: release→release, release→refund, refund→release,
  timeout→release.
- Toda rejeição deixa Job, vault e saldos byte a byte iguais (invariantes 6 e
  10).
- IDL com exatamente 5 instruções, nenhuma administrativa; testes anteriores
  continuam passando; core 36/36.

## Evidências exigidas
- `docs/d2e-router-settlement-results.md` com:
  - decisões, comandos e saídas;
  - SHA-256 do `.so` novo e dos locks;
  - CU de `release`/`refund_on_fail`;
  - matriz de testes × invariantes 3, 4, 6, 8, 9 e 10;
  - riscos e fronteiras.
- Entradas D2e em `docs/decisions.md` e `docs/evidence.md`; `escrow-program.md`
  atualizado.

## Critério de pronto
- Build SBF no Perfil A e todos os testes obrigatórios passam em processo com
  o Router e o verificador reais.
- Nenhum segredo no Git; fixtures sem chaves; `git diff --check` exit `0`;
  diff integral revisado.
- Perfil padrão inalterado, salvo o que já foi registrado.

## Condições de parada
- ABI do `verify` divergente do D1a.3/D2d, ou CPI não confirmável no fonte
  pinado.
- Necessidade de alterar core, journal ou lock raiz.
- Rede fora do autorizado; qualquer resultado obtido com mock ou dev mode.

## Commit
- Commits locais autorizados ao final, somente se o critério de pronto for
  atendido:
  - `anchor: settle escrow through Verifier Router CPI (D2e)`;
  - `docs: record Router settlement (D2e)`.
- Identidade via `git -c`. Push proibido.
- Antes de qualquer gate devnet, recomendar ao humano a revisão adversarial
  separada (Opus 5.5, esforço max, somente leitura) de D2b, D2c e D2e.

## Relatório final
1. arquivos modificados;
2. testes e saídas reais;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de que devnet, deploy e push não foram usados e de que nenhum
   claim ZK on-chain em cluster foi feito;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`.

O próximo gate provável é a **revisão adversarial** seguida do **gate
devnet** (D4 da sequência): Test USDC mock, keypairs de devnet fora do clone,
deploy, Explorer e a decisão sobre o Router em devnet (oficial confirmado ou
próprio). Paralelamente: CLI E2E (D7).
