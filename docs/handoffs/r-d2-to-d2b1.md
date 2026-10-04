# D2b.1 — vincular a liquidação ao artefato entregue e aos termos admitidos da v1

## Identificação
- Gate: `D2b.1`  ·  Dia da sequência: D3 (vínculo job–journal)  ·  Marcos do
  guia: M3 e M4 (local)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `R-D2` (revisão adversarial somente leitura, **REPROVADO**
  para o D2e), registrada no commit `docs: record R-D2 adversarial review`
  sobre `42b4f58`; relatório `docs/r-d2-adversarial-review-results.md`

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh** — política
  econômica, estado on-chain, contas e autoridade.
- **Plan Mode obrigatório** (`CLAUDE.md`: `escrow.rs`,
  `docs/escrow-state-machine.md`, `docs/manifest-schema.md`, programa
  Anchor). As decisões de produto já estão tomadas (abaixo); o Plan Mode
  serve para apresentar o desenho técnico e pedir aprovação, não para
  reabrir as decisões.

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§2, §3, §5, §7, §9, §11)
- `docs/r-d2-adversarial-review-results.md` (todos os achados; foco em F-01,
  F-02, F-07, F-08, F-10, F-11)
- `docs/escrow-state-machine.md`, `docs/escrow-program.md`,
  `docs/manifest-schema.md`, `docs/architecture.md`,
  `docs/mvp-agent-operating-guide.md`
- `docs/decisions.md`: D2a, D2a.1, D2b, D2c, D2c.1, R-D2 e
  **"Decisões humanas para o D2b.1"**
- `crates/vericode-core/src/escrow.rs`
- Somente leitura: `crates/vericode-core/src/lib.rs`,
  `zkvm/methods/guest/src/main.rs`
- `anchor/programs/vericode-escrow/src/lib.rs`, `anchor/tests-local/tests/*.rs`,
  `anchor/tests-local/fixtures/groth16/README.md`

## Preflight
- `pwd`; raiz Git; branch; HEAD (esperado: commit
  `docs: record R-D2 adversarial review` sobre `42b4f58`); `git status
  --short` (vazio); `git diff --check`.
- Confirmar que `docs/r-d2-adversarial-review-results.md` e a entrada
  "Decisões humanas para o D2b.1" existem. Se faltarem: parar.
- `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes. Snapshot de
  `~/.cargo` no início e no fim (SHA-256 da listagem
  `find . -printf '%p %s %T@\n' | sort` esperado `d9e12578…`).
- O WSL pode reiniciar e limpar `/tmp`: helpers, logs e artefatos sempre em
  `~/.local/share/vericode-spikes`, nunca só no scratchpad.
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou
  stash.

## Checagem da tarefa anterior
- Core (homes D1a.3, target fora do clone): `cargo +1.85.0` (lane-a) e
  `+1.89.0` (lane-b) `test --locked --offline` → `36 passed` em cada.
- Programa, no Perfil A (ambiente `~/.local/share/vericode-spikes/d2c`
  lane-b; função `d2c` em `docs/handoffs/d2c1-to-review-d2b-d2c.md`):
  - `cargo-build-sbf -- --locked`, copiando antes
    `d2c/keys/vericode_escrow-keypair.json` para o out-dir, sem exibi-lo →
    `vericode_escrow.so`
    `d66ac76bc8ef66488facc6473b7679eac2fa5e605fba409a6cd61acce20baaae`;
  - `anchor/tests-local`, `cargo +1.89.0 test --locked` → escrow 12,
    fixtures 2.
- Locks: raiz `191802b2…`, host `f5236689…`, guest `1116acef…`, `anchor/`
  `19a1db26…`, `tests-local` `be94760a…`.
- Divergência → parar, registrar e reportar.

## Objetivo
Garantir que só o artefato **entregue e assinado pelo executor**, avaliado
pelo guest, spec e harness **admitidos da v1**, possa liquidar o Job.
Nenhum terceiro pode forçar refund ou release com outro artefato ou outro
guest. Os prazos não podem prender fundos indefinidamente.

## Decisões já tomadas (fonte: `docs/decisions.md`, "Decisões humanas para o D2b.1")
1. **F-01 — opção (A), compromisso de entrega assinado pelo executor.**
   - `deliver(artifact_hash)` só a partir de `Funded`, só pelo executor do Job
     (signer), uma única vez, com `current_slot <= deadline_slot`.
   - Só o hash on-chain, com a semântica de `hash_restricted_artifact` (a
     mesma do `artifact_hash` do journal).
   - `release` e `refund_on_fail` exigem `Delivered { h }` e journal com
     `artifact_hash == h`.
   - `refund_on_timeout` aceita `Funded` e `Delivered` após o prazo.
   - Divergência do guia §5 e do D2a.1 já registrada com o motivo (R-D2
     F-01).
2. **F-02 — termos admitidos.** `create_job` mantém os parâmetros (IDL
   estável), mas rejeita:
   - `spec_hash` diferente de `hash_restricted_spec(&RESTRICTED_SPEC_V1)`;
   - `harness_hash` diferente de
     `hash_harness_version(DETERMINISTIC_HARNESS_VERSION)`, ambos calculados
     pelo core em tempo de execução (fonte única);
   - `image_id` diferente da constante `ADMITTED_IMAGE_ID_V1 =
     4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a`.
3. **F-07 — janela de prazo.**
   `Clock.slot + MIN_DEADLINE_WINDOW_SLOTS <= deadline_slot <= Clock.slot +
   MAX_DEADLINE_WINDOW_SLOTS`, com `MIN = 1_500` e `MAX = 1_512_000`. O
   executor deve ser diferente das PDAs do Job e do vault.
4. **F-08 e F-11 incluídos** neste gate.
5. Mantidos:
   - estados D2b; release só com `current_slot <= deadline_slot`; timeout só
     com `current_slot > deadline_slot`; destinos fixos no Job;
   - `job_id` globalmente único (PDA `["job", job_id]`); vault
     `["vault", job]`; mint sem freeze authority (D2c.1);
   - códigos de erro 6000–6024 e tags Borsh de `EscrowStatus` 0–4 estáveis;
     novos entram sempre no fim;
   - `JournalV1`, wire format, `crates/vericode-core/src/lib.rs`, guest e
     locks não mudam.

## O que o Plan Mode deve apresentar (aprovação técnica, não de produto)
- **Onde ficam as regras F-02 e F-07.** Recomendação:
  - regras econômicas (janela de prazo e termos admitidos, com ImageID
    recebido como parâmetro) no core, para manter a regra única e testável
    sem Solana;
  - a constante do ImageID e a leitura do `Clock` no programa.
- **Assinaturas exatas das funções do core** e mapeamento de erros novos
  (core → `VericodeEscrowError` a partir de 6025).
- **Impacto no CU** do cálculo de `hash_restricted_spec`/`hash_harness_version`
  on-chain no `create_job`, a ser medido nos testes.

## Escopo autorizado
- `crates/vericode-core/src/escrow.rs` (política e testes).
- `anchor/programs/vericode-escrow/src/lib.rs`; `anchor/tests-local/tests/`
  (arquivos existentes e novos).
- Docs:
  - `escrow-state-machine.md`, `escrow-program.md`;
  - `manifest-schema.md` (só a seção "Validação pelo contrato");
  - `architecture.md`, `README.md`;
  - `decisions.md`, `evidence.md`, `agent-control.md`, `project-context.md`;
  - criar `docs/d2b1-delivery-binding-results.md` e
    `docs/handoffs/d2b1-to-d2e.md`.
- Corrigir as imprecisões F-10 nesses documentos, incluindo `MintCannotFreeze`
  atribuído ao programa executado `spl_token-3.5.0.so` (o 7.0.0 é o crate
  cliente). Declarar a limitação da spec v1: o executor escolhe a entrada e
  qualquer `(n, 2n)` passa.

## Fora de escopo / proibido
- `crates/vericode-core/src/lib.rs`, `JournalV1`, wire format, `zkvm/`,
  rebuild do guest, qualquer lock. Se precisar de dependência nova: parar.
- `release`/`refund_on_fail` on-chain, Router, CPI, F-03, F-06, F-12: ficam
  para o D2e, especificados no handoff que este gate produz.
- Rede, instalação, Docker, devnet, deploy, keypair novo, push,
  `docs/context/*`.
- Mock, dev mode ou receipt `Fake`.

## Implementação esperada
- Core:
  - `EscrowState::Delivered { artifact_hash }` (não terminal);
  - `JobV1::deliver(state, deliverer: ExecutorId, artifact_hash, current_slot)`;
  - `release`/`refund_on_fail` só a partir de `Delivered`, com a comparação do
    artefato (`Journal(ArtifactHashMismatch)`); a partir de `Funded` →
    `NotDelivered`;
  - `refund_on_timeout` a partir de `Funded` e `Delivered`;
  - `fund` em `Delivered` → `AlreadyFunded`;
  - erros novos no fim: `NotDelivered`, `AlreadyDelivered`,
    `DelivererMismatch` e os de termos/janela (`TermsNotAdmitted` ou
    específicos, `DeadlineOutOfWindow`, `ExecutorIsProgramAccount`; nomes
    finais no Plan Mode).
- Programa:
  - `EscrowStatus::Delivered { artifact_hash }` no **fim** do enum (tag 5);
    `INIT_SPACE` continua 276;
  - instrução `deliver(artifact_hash)` com o executor como `Signer` e
    `Clock::get()`;
  - `create_job` com F-02 e F-07;
  - F-08: `#[account(address = job.mint)]` em `Fund`, `RefundOnTimeout` e
    `Deliver`, quando houver conta mint;
  - erros novos a partir de 6025;
  - IDL com 4 instruções, nenhuma administrativa.

## Testes obrigatórios (negativos antes dos positivos; rejeições com snapshot byte a byte)
- Core:
  - **PoC-1 invertido:** FAIL e PASS de artefato diferente do entregue →
    `Journal(ArtifactHashMismatch)` em `refund_on_fail` e `release`, em todos
    os slots;
  - regras de `deliver`: deliverer, estado, prazo (`== deadline` aceito,
    `+1` rejeitado), repetição;
  - timeout de `Funded` e de `Delivered`;
  - janela de prazo nos limites (`MIN-1`, `MIN`, `MAX`, `MAX+1`) e termos não
    admitidos;
  - matriz com `Delivered` e dois artefatos, contra oráculo independente;
  - propriedade "em cada estado e slot, no máximo um destino é aceito";
  - terminais × 5 operações; remover ou substituir o teste trivial
    `transitions_do_not_change_the_job_terms` (F-11/T7).
- Programa:
  - `deliver` pelo executor; por buyer ou terceiro; antes do funding; duas
    vezes; após o prazo;
  - timeout de `Delivered`; fund após deliver;
  - termos não admitidos (spec, harness, ImageID); prazo fora da janela;
    executor igual a PDA;
  - mint estranho em `fund`/timeout → erro do VeriCode (F-08), não `0x3` do
    SPL Token.
  - F-11:
    - códigos **literais** 6000–60xx (não derivados do enum);
    - ida e volta das 6 variantes de status e `INIT_SPACE == 276`;
    - `Clock.slot == DEADLINE` conferido no teste do prazo;
    - Token-2022 → 3007/3008; vault falso → 2006; Job forjado → 3007;
    - squatting de `job_id` com o erro real (`0x0`);
    - `UnsupportedAccountVersion`.
- Fixtures Groth16: continuam 2/2; o artefato entregue nos testes do programa
  usa o mesmo `artifact_hash` das fixtures (Job `0x11`), para o D2e reutilizar.
- Comandos: os da checagem anterior e
  `anchor-0.31.1 idl build -p vericode_escrow` (target fora do clone).

## Evidências exigidas
- `docs/d2b1-delivery-binding-results.md` com:
  - decisões aplicadas, comandos e saídas reais;
  - SHA-256 do `.so` novo e da IDL;
  - CU do `create_job` antes e depois;
  - matriz invariantes × testes × achados R-D2 (F-01, F-02, F-07, F-08,
    F-10, F-11).
- `docs/handoffs/d2b1-to-d2e.md`: D2e reescrito a partir do histórico
  `docs/handoffs/d2d-to-d2e.md` (marcado como substituído), com:
  - `release`/`refund_on_fail` só a partir de `Delivered`;
  - F-03: `SELECTOR = 73c457ba` e PDA `["verifier", SELECTOR]` do Router
    fixados, com teste de seal de outro selector rejeitado pelo escrow;
  - F-06: ATA canônica do destinatário (sem dependência nova ou com decisão
    explícita);
  - F-08 nas novas contas;
  - F-12: digest sobre os 165 bytes decodificados e `job.image_id`;
  - negativo "FAIL de artefato não entregue" antes dos positivos;
  - uso das fixtures versionadas.
- Entradas D2b.1 em `decisions.md` e `evidence.md`; `agent-control.md` e
  `project-context.md` atualizados.

## Critério de pronto
- Core com 36 ou mais testes nas duas raias, sem warnings.
- Build SBF e testes em processo passando; códigos 6000–6024 inalterados;
  locks inalterados; IDL com 4 instruções.
- `escrow-state-machine.md` só afirma a partição temporal e "um destino por
  estado/slot" com o teste que a prova.
- `git diff --check` exit 0; diff integral revisado; busca de segredos limpa;
  perfil padrão inalterado.
- Risco do ImageID não recertificado registrado (o guest não é reconstruído).

## Condições de parada
- Necessidade de alterar `JournalV1`, `lib.rs` do core, guest ou locks, ou de
  rede.
- Desenho técnico do Plan Mode não aprovado.
- Qualquer regra de produto não coberta pelas decisões acima →
  `AGUARDANDO_AUTORIZAÇÃO`.

## Commit
- Commits locais autorizados ao final, somente se o critério de pronto for
  atendido:
  - `core: bind settlement to the executor delivery (D2b.1)`;
  - `anchor: add deliver and admitted v1 terms (D2b.1)`;
  - `docs: record D2b.1 delivery binding`.
- Identidade dos commits anteriores via
  `git -c user.name=… -c user.email=…`, sem alterar a configuração do Git.
  Push proibido.

## Relatório final
1. arquivos modificados;
2. testes e saídas reais;
3. invariantes e achados R-D2 resolvidos;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras;
7. prompt da próxima fase (`docs/handoffs/d2b1-to-d2e.md`), segundo
   `docs/handoff-protocol.md`. Recomendar:
   - Opus 5.5 xhigh para o D2e;
   - uma revisão adversarial separada (Opus 5.5, max, somente leitura) de
     D2b.1 + D2e antes de qualquer gate devnet.
