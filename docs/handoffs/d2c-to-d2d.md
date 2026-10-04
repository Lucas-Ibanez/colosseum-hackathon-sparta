# D2d — spike do caminho forte: receipt Groth16 e Verifier Router em processo

## Identificação
- Gate: `D2d`  ·  Dias da sequência: D5/D6 (adaptador Router; prova
  incompatível rejeitada antes do happy path)  ·  Marcos do guia: M5
  (preparação)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D2c`, commits `a10f026` (código) e
  `docs: choose Anchor/Agave profile (D2c)`; relatórios
  `docs/d2c-anchor-local-escrow-results.md` e `docs/escrow-program.md`

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh** — Groth16, Router,
  seal, contas de CPI e claims ZK; é o maior risco técnico do guia (§8).
- **Plan Mode obrigatório** (`CLAUDE.md`: integração do Router e `zkvm/`).

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§5, §7, §8, §9, §11 e §12)
- `docs/router-notes.md`, `docs/zkvm-notes.md`, `docs/d1a3-spike-results.md`
  (seções ABI e serialização), `docs/manifest-schema.md`
- `docs/d1c2b3i-local-receipts-results.md`, `docs/d1c2b3j-final-audit.md`,
  `docs/d2c-anchor-local-escrow-results.md`, `docs/escrow-program.md`
- `docs/decisions.md` (D1a.3, D1c2b.3h, D1c2b.3i, D2a.2 e D2c)
- `zkvm/host/src/main.rs`, `anchor/programs/vericode-escrow/src/lib.rs`

## Preflight
- `pwd`; raiz Git; branch; HEAD (esperado: commit docs D2c sobre `a10f026`);
  `git status --short` (vazio); `git diff --check`.
- `~/.rustup` e `~/.cache/solana` ausentes.
- `~/.cargo` e `~/.avm` são preexistentes (D2c): tirar um snapshot no início e
  recomparar no fim.
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou
  stash.

## Checagem da tarefa anterior
- `git log --oneline -8` contém o commit docs D2c, `a10f026`, `58838ae`,
  `402426f`, `0622709` e `4d7e18f`.
- Core nas duas raias (comandos do D2b): `36 passed; 0 failed`.
- Programa no Perfil A, com o ambiente `d2c/` (helper `env -i` descrito no
  relatório D2c):
  - `cargo-build-sbf -- --locked` deve repetir `vericode_escrow.so` SHA-256
    `04cc2a845eea1b2e10e313601d1b51887f3f069f3e80c802651d499ddfd0ae56`;
  - divergência → investigar e registrar antes de seguir;
  - `cargo +1.89.0 test --locked` em `anchor/tests-local` → `10 passed`.
- Locks:
  - `Cargo.lock` `191802b2…3b87`;
  - `zkvm/Cargo.lock` `f5236689…e226`;
  - guest `1116acef…dbfa`;
  - `anchor/Cargo.lock` `19a1db26a33c51bf6e4818a69c9ddf587bdebce72a6d6615fe697d9b9dfa6765`;
  - `anchor/tests-local/Cargo.lock` `be94760a8b49161c24ba630a862e91da1d441afa2d6105cdafc0097b439ea377`.
- Artefatos D1c2b em `/tmp`:
  - `vericode-d1c2b3i-receipts.x2zrL1`;
  - targets `vericode-d1c2b3h1*`;
  - conferir que o método combinado tem SHA-256 `e09ba8cf…78f5`, o ELF
    `63fac491…5408` e o ImageID `4da06f90…fb1a`;
  - copiar para uma raiz persistente do spike antes de usar;
  - se tiverem sumido, parar e pedir decisão (rebuild determinístico custa um
    gate).

## Objetivo
Provar ou refutar localmente, com evidência real, o caminho forte de
verificação:
- receipt **Groth16** do guest VeriCode para PASS e FAIL;
- verificada pelo **Verifier Router** de `risc0-solana v3.0.0`, carregado em
  `solana-program-test`;
- **testes negativos antes do positivo**.

O resultado é GO ou NO-GO para integrar `release`/`refund_on_fail` no gate
seguinte.

## Decisões já tomadas
- Perfil A: Anchor `0.31.1` + Agave `2.3.9` + Rust host `1.89.0` (D2c).
- Caminho forte = CPI ao Router; fallback atestado só se o forte não fechar,
  sempre rotulado como não-ZK (guia §8, `AGENTS.md` §8).
- Journal: `JournalV1` de 165 bytes inalterado. O digest enviado ao Router
  segue o formato confirmado no D1a.3 (`journal_digest = SHA-256(journal)`),
  a reconfirmar no fonte pinado.
- ImageID admitido: o do ELF determinístico D1c2b (`4da06f90…fb1a`), desde
  que os hashes conferem.
- Keypairs de teste somente em memória ou fora do clone (`AGENTS.md` §9).

## Decisões a confirmar no Plan Mode (perguntar ao humano)
1. **Mecanismo Groth16:** confirmar no fonte `risc0-zkvm 3.0.3` e
   `risc0-groth16 3.0.2` como a conversão STARK→SNARK roda localmente (imagem
   Docker do prover, binário ou outro) e quais artefatos exige. Pedir
   autorização explícita para o pull **por digest** exato, se for Docker.
2. **Onde fica o código do spike:** recomendação: harness temporário fora do
   clone, sob `~/.local/share/vericode-spikes/d2d`, sem mudar `zkvm/` nem
   `anchor/`. Alternativa: novo subcomando no host, o que exige Plan Mode e
   testes.
3. **Router em processo:**
   - compilar `verifier_router` e `groth_16_verifier` do checkout
     `ee415935…` no Perfil A;
   - inicializar e registrar o verificador com keypairs em memória;
   - registrar contas, selector e PDAs observados.

## Rede e instalação autorizadas (somente estas)
- `index.crates.io`/`static.crates.io` para os workspaces do
  `risc0-solana v3.0.0` (com `--locked`) e para o harness do spike.
- Imagem ou artefato do prover Groth16 **somente após aprovação no Plan
  Mode**, por digest e com SHA-256 registrado.
- Qualquer outro destino (inclusive scripts automáticos de SDK): parar e
  perguntar.
- Ferramentas reutilizadas: homes `d2c/` (Perfil A) e `d1a3/homes/zkvm`
  (RISC Zero 3.0.3, Docker já instalado). Não reinstalar e não escrever no
  perfil padrão.

## Escopo autorizado
- Ambiente e harness em `~/.local/share/vericode-spikes/d2d`.
- Criar `docs/d2d-groth16-router-spike-results.md` e
  `docs/handoffs/d2d-to-<próximo>.md`.
- Atualizar `docs/router-notes.md`, `docs/zkvm-notes.md`,
  `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md` e
  `docs/project-context.md`.

## Fora de escopo / proibido
- Alterar `crates/vericode-core`, `JournalV1`, wire format, `zkvm/`,
  `anchor/`, locks ou `docs/context/*`, salvo aprovação explícita no Plan
  Mode.
- Devnet, RPC remoto, airdrop, deploy, transação em rede pública.
- Dev mode, receipt `Fake`, mock de Router/verificador ou seal sintético
  apresentado como prova.
- Alegar "ZK on-chain": o máximo permitido é "verificada pelo programa
  Router em `solana-program-test` local".
- Push.

## Implementação esperada
- Inventário e hashes dos artefatos D1c2b reutilizados.
- Receipts Groth16 de PASS e FAIL:
  - `Receipt::verify` com o ImageID;
  - tipo interno `Groth16`, não `Composite` nem `Fake`;
  - journal de 165 bytes igual ao esperado.
- Seal codificado para o Router conforme o fonte (`selector[4]` + proof com
  `pi_a` negado).
- `.so` do Router e do verificador Groth16 compilados no Perfil A, com
  SHA-256.
- Harness `solana-program-test` que:
  1. inicializa o Router e registra o verificador;
  2. **rejeita**: seal adulterado, selector desconhecido, ImageID errado,
     journal digest errado;
  3. **aceita**: PASS e FAIL válidos;
  4. registra as compute units do `verify`.

## Testes obrigatórios
- Groth16 PASS e FAIL verificam localmente; ImageID adulterado rejeitado.
- Router em processo: os 4 negativos acima rejeitados antes do positivo; PASS
  e FAIL aceitos.
- Core 36/36 nas duas raias; programa 10/10; locks inalterados.

## Evidências exigidas
- `docs/d2d-groth16-router-spike-results.md` com:
  - inventário, downloads/imagens (origem, digest, SHA-256);
  - comandos, saídas e timestamps;
  - hashes de receipts, seals, `.so` e journals;
  - compute units;
  - matriz negativos × resultado;
  - decisão GO/NO-GO, riscos e fronteiras.
- Entradas D2d em `docs/decisions.md` e `docs/evidence.md`.
- `docs/router-notes.md` atualizado; manter `STATUS: NÃO VALIDADO` para
  devnet e para CPI a partir do `vericode_escrow`.

## Critério de pronto
- GO: receipts Groth16 reais e a cadeia Router→Groth16 aceitando/rejeitando
  conforme a matriz, tudo em processo e reproduzível.
- Ou NO-GO documentado, com diagnóstico reproduzível e recomendação
  (fallback atestado rotulado ou nova tentativa).
- `git diff --check` exit `0`; diff integral revisado; busca de segredos
  limpa; perfil padrão inalterado.

## Condições de parada
- Artefatos D1c2b ausentes ou com hash divergente.
- Prover Groth16 exige hardware, arquitetura ou artefato não confirmável, ou
  imagem não fixável por digest.
- Router/verificador não compilam no Perfil A com locks preservados.
- API, contas, selector ou formato de seal não confirmáveis no fonte pinado.
- Qualquer rede fora do autorizado.

Registrar timestamps. Se o spike não fechar dentro do orçamento combinado no
Plan Mode, parar com diagnóstico e recomendar ao humano o fallback atestado
rotulado para a demo, preservando o prazo de ~8 out.

## Commit
- Commit documental local autorizado ao final (o spike fica fora do clone):
  `docs: record Groth16 and Router spike (D2d)`. Usar a identidade dos commits
  anteriores via `git -c`. Push proibido.
- Revisões adversariais separadas D2b/D2c continuam pendentes. Recomendar ao
  humano uma sessão somente leitura (Opus 5.5, esforço max) antes de integrar
  `release`.

## Relatório final
1. arquivos modificados;
2. testes e saídas reais;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de que devnet, deploy e push não foram usados e de que nenhum
   claim ZK on-chain foi feito;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`.

O próximo gate provável:
- **GO:** D2e — `release`/`refund_on_fail` no `vericode_escrow` por CPI ao
  Router, testados em processo;
- **NO-GO:** decisão sobre fallback atestado.
