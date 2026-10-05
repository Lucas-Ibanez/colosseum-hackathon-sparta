# Controle autônomo — D4b concluído (escrow implantado e finalizado em devnet); próximo D7

## Objetivo atual

D7, conforme `docs/handoffs/d4b-to-d7.md`:
- CLI de ponta a ponta no repositório para o fluxo devnet, a partir da IDL
  do D4a;
- README com versões, hashes, links e limitações (M6/M7);
- roteiro da demo (conteúdo do D9);
- RD4A-07 (a), (e) e (f).

Worker e telas (D10–D12) só se houver tempo.

## Marcos anteriores

- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c `a10f026`/`ec980e9`, D2d `9a18f71`, D2c.1 `42b4f58`.
- R-D2 `12529b4`: **REPROVADO** para o D2e original.
- D2b.1 `0e9838e` (core), `5736565` (anchor), `75a1971` (docs).
- D2e `2f10a8f` (anchor), `c0aba7d` (docs).
- R-D2e `2d76441`: **APROVADO COM RESSALVAS** para o D4.
- Prazo, congelamento do `JournalV1` e nomes de gate: `cfdd9d7`.
- D4a: `fb4bfba` (anchor) e `24364ae` (docs).
- R-D4a `561b1b6`: **APROVADO COM RESSALVAS** para o D4b.
- D4b: registrado no commit `docs: record devnet deploy, finalization and
  settlements (D4b)`.

## Baseline (D4b)

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  do D4b.
- **Devnet:**
  - escrow `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`: ProgramData
    `B7s9JJVy…`, 395.064 bytes `cdf6967f…`, upgrade authority **`none`**;
  - verificador `THq1q…` imutável;
  - mint Test USDC `9TE2VPFm…`: Tokenkeg, 6 decimais, sem freeze, mint
    authority = deployer, supply 10¹²;
  - ATAs: buyer `61tkoEv4…` com 999.998 Test USDC; executor `HpZkHZ59…` com
    2 Test USDC.
- **Jobs consumidos:** S (`Released`, smoke), A (`Released`), B
  (`RefundedOnFail`) e C (`RefundedOnTimeout`). Nunca reutilizar esses
  `job_id`s.
- **SOL** no fim do D4b: deployer 2,925; buyer 0,0356; executor 0,00998.
- **Testes locais** (inalterados desde o D4a):
  - escrow 26, settlement 15, regressions 7, layout 7, fixtures 2 (57/57);
  - core 42/42 nas duas raias;
  - IDL `e8ce2c20…`.
- **Locks inalterados:** raiz `191802b2…`; host `f5236689…`; guest
  `1116acef…`; `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`.
- **Fora do clone:**
  - `d4/keys`: deployer, buyer, executor, mint e o buffer já consumido
    `3CVLy…`;
  - `d4/receipts-out` (S, A, A′, B, todos consumidos em devnet);
  - `d4b/`, com o cliente offline (`client/tests-local`, exemplo `d4b`), o
    driver RPC (`bin/devnet.py`), o filler cadenciado
    (`bin/buffer_fill.py`), os logs (`tx.jsonl`, `timeline.log`) e o dump.

## Gate atual

`D4b` (concluído) → próximo `D7`.

## Estado

- A receipt é **verificada em devnet por CPI ao verificador Groth16 imutável
  de risc0-solana v3.0.0**:
  - liquidação PASS `4yWq28Gw…` e FAIL `2osG9m8J…`;
  - negativos 6000/6003 dentro do verificador;
  - 6014/6017/6033/6019/6021 antes da CPI ou do prazo;
  - C7 `MintCannotFreeze`.

  Links em `docs/d4b-devnet-results.md`.
- O escrow é imutável. Não há Router, e-stop nem admin.
- O RPC público de devnet limita a taxa de envio. A CLI do Agave com
  `--use-rpc` perdeu a maioria das escritas do deploy, e o buffer foi
  completado com envio cadenciado. Clientes do D7 devem cadenciar e
  reenviar.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1",
  D2b.1, D2e, R-D2e, "Decisões humanas para o D4".
- "Prazo de 11/10, congelamento do JournalV1 v1 e nomes de gate".
- D4a, R-D4a e D4b (`docs/decisions.md`).

## Ações proibidas (permanentes salvo novo objetivo)

- Rede fora do autorizado por gate; instalação no perfil padrão; Docker sem
  gate explícito.
- Alterar locks existentes, `JournalV1` (v1 congelado), wire format ou o
  core sem novo `schema_version` e decisão registrada.
- Keypair fora de gate autorizado, de mainnet, dentro do clone ou com segredo
  exibido.
- Deploy, airdrop ou escrita em devnet sem gate autorizado.
- O escrow `GZqb…` é imutável: um "redeploy" exige novo program ID, mudança
  de código, decisão e revisão.
- Claims:
  - qualquer alegação de verificação on-chain além de "verificada em devnet
    por CPI ao verificador Groth16 imutável de risc0-solana v3.0.0", com os
    links;
  - chamar a verificação de "Verifier Router";
  - claim de mainnet.
- Mock, dev mode ou receipt `Fake` como sucesso.
- Builds, testes ou provas pesadas em paralelo (7,6 GiB de RAM).
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- **Sem e-stop:** escrow e verificador imutáveis. Um bug de soundness
  exigiria um novo program ID (RD4A-06).
- **Rent preso (F-13):** cerca de 0,0036 SOL por Job; não há `close`.
- **Mint authority** = deployer de devnet (pode emitir mais; não afeta a
  custódia).
- **`job_id`s públicos (F-09/RD4A-03):** squatting de um `job_id` antes da
  criação inutiliza a receipt. O D7 deve gerar `job_id` novo por Job.
- **R-02:** `fund` fora da janela, mitigado por `create_job`+`fund`
  atômicos. **R-03:** replay entre implantações, mitigado por `job_id`
  aleatório.
- **RPC público de devnet com limite de taxa.**
- RD4A-07 (a), (e), (f): comentário em `settlement.rs`, `.env.example` e
  testes novos (receipts do D4b, variantes do mint, verificador ausente),
  para o D7.
- F-14; ImageID não recertificado; spec v1 trivial.
- **Margem de RAM do prover:** cerca de 160 MiB no pico.
- O WSL reinicia e limpa `/tmp`.
- **Calendário:** 2026-10-05; prazo de entrega 11/10.

## Próxima transição permitida

`D7`, conforme `docs/handoffs/d4b-to-d7.md`, em sessão nova (Opus 5.5,
xhigh). Plan Mode antes de qualquer escrita em devnet ou mudança fora de
documentação e CLI.
