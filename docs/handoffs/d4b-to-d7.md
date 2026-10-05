# D7 — CLI de ponta a ponta em devnet, README de entrega e roteiro da demo

## Identificação
- Gate: `D7`  ·  Dias da sequência: D7 e D9 (roteiro)  ·  Marcos do guia: M6
  e M7
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D4b`, commit `docs: record devnet deploy, finalization and
  settlements (D4b)` sobre `561b1b6`; relatório `docs/d4b-devnet-results.md`.
  O escrow está implantado e finalizado em devnet.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh**. O gate envolve
  cliente de devnet, contas, assinaturas e claims públicos.
- **Plan Mode obrigatório** antes de:
  - criar o crate ou lock da CLI;
  - qualquer escrita em devnet;
  - qualquer prova nova (Docker);
  - mudar `anchor/tests-local` ou `.env.example`.
- Prazo do MVP: **11/10**. Cortar primeiro interface e extras, nunca
  receipt real, vínculo Job–journal, cenário negativo ou explicação de
  limites (guia §3).

## Leitura obrigatória (integral, antes de agir)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/d4b-devnet-results.md`, inclusive "Desvios" e "Riscos abertos"
- `docs/decisions.md`: "D4a…", "R-D4a…" e "D4b…"
- `docs/r-d4a-review-results.md`: RD4A-07 (a), (e) e (f)
- `docs/escrow-program.md` (constantes, contas, erros, medições em devnet),
  `docs/router-notes.md`, `docs/manifest-schema.md`, `README.md`
- `docs/context/guia-mvp-agentes-de-codigo.md`: §3, §7, §9 e §12 (M6/M7)
- `docs/context/sequencia-mvp.md`: D7 e D9
- `anchor/programs/vericode-escrow/src/lib.rs`;
  `anchor/tests-local/tests/common/mod.rs`
- Fora do clone, somente leitura:
  - `~/.local/share/vericode-spikes/d4b/client/tests-local/examples/d4b.rs`
    (cliente offline do D4b);
  - `~/.local/share/vericode-spikes/d4b/bin/devnet.py` (driver RPC);
  - `~/.local/share/vericode-spikes/d4b/bin/buffer_fill.py` (envio
    cadenciado);
  - `~/.local/share/vericode-spikes/d4/receipts/` (harness `prove` e
    `compress` do D4a).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3`. O topo esperado é o commit
  do D4b sobre `561b1b6`. Se não for, parar e reportar.
- `git status --short` e `--ignored` vazios; `git diff --check` 0.
- Perfil padrão, no início e no fim:
  - `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes;
  - snapshots com `cd <dir> && find . -printf '%p %s %T@\n' | sort |
    sha256sum`: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker`
    `6046f67f…`.
- Raiz nova `~/.local/share/vericode-spikes/d7/`, com o helper copiado de
  `d4b/bin/env.sh` e `D` trocado.
- Nunca usar `/tmp`. RAM de 7,6 GiB: um build pesado ou prova por vez.
- Sem reset, checkout destrutivo, clean ou stash.

## Checagem da tarefa anterior (somente leitura)
- Locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/`
  `19a1db26…`, `tests-local` `be94760a…`.
- Escrow em devnet:
  - `solana program show GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`
    (CLI do Perfil A, com `-k` do deployer porque a CLI exige signer
    padrão) → `Authority: none`, Data Length 395064;
  - `program dump` → SHA-256
    `cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133`.
- Estado (`d4b/bin/devnet.py state`, somente leitura):

  | Job | Estado |
  | --- | --- |
  | S | `Released` |
  | A | `Released` |
  | B | `RefundedOnFail` |
  | C | `RefundedOnTimeout` |

  Vaults 0; ATA do executor 2.000.000; ATA do buyer 999.998.000.000.
- Saldos esperados (podem ter mudado por terceiros): deployer ≈ 2,925 SOL,
  buyer ≈ 0,0356 SOL, executor ≈ 0,00998 SOL.
- Se divergir: parar e reportar.

## Objetivo
Entregar no repositório uma CLI reproduzível que execute o fluxo do MVP em
devnet: criar e financiar o Job, entregar, liquidar com receipt e reembolsar
por timeout, sempre sobre o escrow finalizado. Junto, o README de entrega
(M6/M7), o roteiro da demo e as pendências RD4A-07 (a), (e) e (f).

## Decisões já tomadas
- Escrow `GZqb…` imutável; verificador `THq1q…` por CPI direta; mint
  `9TE2V…`; `JournalV1` v1 congelado (D4a, R-D4a, D4b).
- Claim único de verificação: "verificada em devnet por CPI ao verificador
  Groth16 imutável de risc0-solana v3.0.0", com links. Nunca "Verifier
  Router" nem mainnet (CD7).
- `create_job`+`fund` na mesma transação; `deliver`+`release` juntos quando
  couber (D4-5).
- `job_id` novo e aleatório por Job. S, A, B e C estão consumidos (F-09).
- Keypairs de devnet em `d4/keys` (`0600`), só pubkeys publicadas (D4-6).
- Gate seguinte ao D4 é o D7; worker e telas só se houver tempo.

## Decisões a confirmar ou pendentes (no Plan Mode, com AskUserQuestion)
Sem confirmação: `AGUARDANDO_AUTORIZAÇÃO`.

1. **Linguagem e local da CLI.** Recomendação: crate Rust separado
   (`cli/`, workspace e lock próprios, como `anchor/tests-local`).
   - Monta as instruções a partir das contas e dos discriminadores da IDL
     do D4a (`e8ce2c20…`), reutilizando `vericode-core` para hashes e
     journal.
   - Não muda os locks existentes.
2. **crates.io para o lock novo da CLI** (por exemplo, `solana-rpc-client`
   2.3.x), ou RPC mínimo próprio sem dependências novas. Recomendação:
   lock novo semeado de `anchor/tests-local/Cargo.lock`, com fetch único
   autorizado.
3. **Prova de Jobs novos:**
   - (a) trazer o harness `prove`/`compress` do D4a para o repositório
     (exige Plan Mode em `zkvm/`);
   - (b) mantê-lo fora do clone, chamado pela CLI como passo documentado.

   Ambos exigem Docker (`--pull=never --network=none`, imagem por digest) e
   um cenário por vez.
4. **Escritas em devnet no D7:**
   - quantos Jobs novos para validar a CLI e para a demo;
   - SOL ao buyer: cerca de 0,0036 por Job; o buyer tem cerca de 0,0356.

## Escopo autorizado (depois das confirmações)
- Criar a CLI (`cli/` ou o local decidido), com README próprio e testes
  offline.
- `anchor/tests-local`, só testes e fixtures (RD4A-07 f):
  - receipts do D4b (journal, seal, selector) como fixtures versionadas;
  - variantes do mint admitido;
  - verificador ausente.

  Não mudar o lock `be94760a…` nem o programa.
- RD4A-07 (a): corrigir o comentário de `verifier_program_is_fixed` em
  `anchor/tests-local/tests/settlement.rs`.
- RD4A-07 (e): `.env.example`, trocando `RISC0_VERIFIER_ROUTER_PROGRAM_ID`
  por `RISC0_GROTH16_VERIFIER_PROGRAM_ID` vazio; só nomes, sem valores
  reais.
- `README.md`: versões, hashes, Program IDs, links do Explorer, comandos da
  CLI e limitações.
- Criar `docs/demo-script.md` (roteiro da demo, conteúdo do D9).
- `docs/architecture.md`: seção "Estado D4b/D7". O D4b não podia editar
  esse arquivo, e a última seção ainda é a do D4a.
- Criar `docs/d7-cli-results.md`; atualizar `docs/decisions.md`,
  `docs/evidence.md`, `docs/agent-control.md` e `docs/project-context.md`.
- Criar `docs/handoffs/d7-to-<próximo>.md`.

## Fora de escopo / proibido
- Mudar o programa, o core, `JournalV1`, o guest ou os locks existentes.
- Novo deploy ou program ID. Um bug no escrow exige decisão e revisão
  novas.
- Rede fora de `api.devnet.solana.com` e do fetch de crates autorizado;
  airdrop.
- Mainnet; keypair dentro do clone; exibir seed phrase, keypair ou conteúdo
  de `d4/keys/*.json`.
- Saída bruta de CLIs com risco de mnemônico no terminal: log `0600` e
  filtro, como no D4b.
- Claim fora de CD7; "ZK on-chain" genérico; mock, dev mode ou receipt
  `Fake` como sucesso.
- Worker e UI, salvo tempo sobrando e decisão explícita. Push.

## Implementação esperada
- CLI com subcomandos mínimos:
  - `job create` (`create_job`+`fund` atômicos; `job_id` aleatório; prazo na
    janela [slot+1.500, slot+1.512.000] com margem);
  - `job deliver`;
  - `job settle` (`release` ou `refund_on_fail` conforme o veredito do
    journal; `deliver`+`release` juntos quando couber);
  - `job refund-timeout`;
  - `job show` (decodifica `JobAccount`, vault e saldos).
- Simulação antes de cada envio; envio cadenciado com reenvio (o RPC público
  devolve HTTP 429 em rajada); confirmação por `getSignatureStatuses`; link
  do Explorer na saída.
- Conferências do cliente antes de cada operação:
  - mint `9TE2V…` com 6 decimais e sem freeze;
  - termos admitidos da v1 (spec `af642b56…`, harness `01124025…`, ImageID
    `4da06f90…`);
  - ATA canônica criada de forma idempotente;
  - selector `73c457ba`;
  - `pi_a` negado;
  - journal de 165 bytes vinculado ao `job_id`.
- Testes offline da CLI: montagem de instruções contra a IDL e as
  constantes, mesmos bytes que os builders de `tests/common/mod.rs`.

## Testes obrigatórios
- Offline:
  - testes da CLI;
  - suíte `anchor/tests-local` com as novas fixtures, nas duas variantes do
    verificador (rebuild `dab6746d…` e dump de devnet `34ae6e5c…`);
  - core 42/42 nas duas raias.
- Devnet, se autorizado:
  - um Job PASS de ponta a ponta pela CLI, com `job_id` novo, prova nova e
    `release`;
  - um negativo pela CLI (por exemplo, journal de outro Job → 6014),
    aterrissado com estado igual;
  - opcional: um Job FAIL ou timeout pela CLI.
- Comandos exatos com homes isoladas (`env.sh`) e `--locked`.

## Evidências exigidas
- `docs/d7-cli-results.md`: comandos, saídas filtradas, assinaturas, links
  do Explorer, CU, tamanhos e saldos.
- README com o claim CD7 e links; `docs/demo-script.md`.
- Atualizar `docs/decisions.md`, `docs/evidence.md`,
  `docs/agent-control.md` e `docs/project-context.md`.

## Critério de pronto
- CLI no repositório, reproduzível, com testes offline passando.
- Se autorizado, Job PASS de ponta a ponta em devnet pela CLI, com link.
- README de entrega e roteiro da demo prontos; RD4A-07 (a), (e) e (f)
  fechados.
- `git diff --check` 0; diff integral revisado; busca de segredos limpa;
  locks existentes e perfil padrão inalterados.

## Condições de parada
- HEAD ou estado de devnet divergente → parar e reportar.
- Decisão 1 a 4 não confirmada → `AGUARDANDO_AUTORIZAÇÃO`.
- `job_id` ocupado ou SOL insuficiente → informar a pubkey e o valor ao
  humano. Nunca airdrop.
- Mnemônico na saída de uma CLI → parar, não transcrever, registrar.
- Prova com falha de memória (exit 137) → repetir sozinha; se persistir,
  `BLOQUEADO`.

## Commit
- Commits locais só com autorização explícita do humano, com identidade via
  `git -c`. Push proibido.
- Mensagens sugeridas:
  - `cli: add devnet end-to-end client (D7)`;
  - `docs: record D7 CLI, README and demo script`.

## Relatório final
1. arquivos modificados;
2. testes e transações reais, com links;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`. Próximo
   provável: revisão adversarial final do MVP (Opus 5.5, max, somente
   leitura) ou D10–D12, se houver tempo.
