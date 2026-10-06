# Controle autônomo — D10a concluído; próximo R-D10a

## Objetivo atual

R-D10a, conforme `docs/handoffs/d10a-to-r-d10a.md`: revisão delta curta,
somente leitura e em sessão separada, do diff do D10a contra RD7-01, 02, 03,
04 e 07 (e os opcionais RD7-06, 09, 10).

Depois: D10 (worker com prova local forçada), D11–D12 (telas finas, CR8),
revisão curta da interface, vídeo definitivo e submissão até 11/10.

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
- D4b `599837d`.
- D7:
  - `a7c6e8a` (prover);
  - `deececa` (cli);
  - `0ec421b` (anchor tests-local);
  - `c550a98` (docs).
- R-D7 `9d3efa7`: **APROVADO COM RESSALVAS** para o D9 e o D10–D12 (CR1 a
  CR8).
- D9 `39a87f7`: demo em ambiente limpo (W1–W8), claims congelados e roteiro
  de gravação.
- D10a: endurecimento da CLI e do prover:
  - `50dede0` (cli);
  - `7fe9c3b` (prover);
  - `docs: record D10a hardening`.

## Baseline (D10a)

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record D10a hardening`.
- **Devnet:**
  - escrow `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`, 395.064 bytes
    `cdf6967f…`, upgrade authority **`none`**;
  - verificador `THq1q…` imutável;
  - mint Test USDC `9TE2V…` (6 decimais, sem freeze).
- **Jobs consumidos** (nunca reutilizar os `job_id`s):
  - S, A `Released`; B `RefundedOnFail`; C `RefundedOnTimeout` (D4b);
  - **P `Released`** (`3e4ca026…`) e **T `RefundedOnTimeout`**
    (`05f74934…`), do D7;
  - **P′ `Released`** (`91ea6fcd…`) e **T′ `RefundedOnTimeout`**
    (`ec9afb74…`), do D9;
  - das gravações do humano (2026-10-06, `rec-*`): `8ab4ee8d` e `8bce67f2`
    (`Funded`, prazo vencido), `cd77e7bd` e `3e115ca8` (`Released`),
    `104f9a21` (`RefundedOnTimeout`).
- **Saldos no D10a** (slot 508.156.224, sem escrita do agente):
  - SOL: deployer 2.805.194.240; buyer 103.351.800; executor 29.925.000
    lamports;
  - Test USDC: ATA do buyer 999.992.000.000; ATA do executor 6.000.000.
- **Testes:**
  - `anchor/tests-local` 61/61 com o rebuild e com o dump de devnet:
    d4b_receipts 2, escrow 27, fixtures 2, layout 7, regressions 7,
    settlement 16;
  - `cli` 30/30 e `prover` 5/5 + `docker_shim` 2/2 (D10a); core 42/42 A/B;
    IDL `e8ce2c20…`;
  - D10a (homes copiadas de `d9/homes`, `--locked --offline`): core 42/42
    ×2, CLI 30/30, suíte 61/61 com os `.so` de devnet, prover 5/5 + 2/2,
    `check` com `prover=LocalProver env=ok`.
- **Binários do D10a** (fora do clone): CLI `e6cd4e29…`, prover
  `3f66e1c0…`. Os do D9 (`d9/targets`) continuam os da seção "Gravação".
- **Locks:**
  - inalterados: raiz `191802b2…`; host `f5236689…`; guest `1116acef…`;
    `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`;
  - novos: `cli` `4d979577…`; `prover` `8b76f1e1…`.
- **Fora do clone:**
  - `d4/keys`: deployer, buyer, executor e mint;
  - `d4/receipts-out` (S, A, A′, B);
  - `d7/`: homes, targets, receipts do P, jobs P/T e logs
    (`cli-tx.jsonl`, `timeline.log`);
  - `d9/`: clone, homes copiadas, targets, receipts do P′, jobs P′/T′,
    `bin/env9.sh` (usado pela seção "Gravação") e logs;
  - `rec-*`: gravações do humano;
  - `d10a/`: árvore "antes", homes, targets, RPC falso e listener, receipts
    `X` e `X-wrong-binary`, logs.

## Gate atual

`D10a` (concluído) → próximo `R-D10a`.

## Estado

- O caminho do MVP sai do repositório:
  - `vericode job create` → `vericode-prover prove/compress` →
    `vericode job settle` (com `deliver`+`release`) ou `job refund-timeout`;
  - links em `docs/d7-cli-results.md` e no `README.md`.
- Claim (CD7): a receipt é **verificada em devnet por CPI ao verificador
  Groth16 imutável de risc0-solana v3.0.0**. Nunca "Verifier Router" nem
  mainnet.
- A invariante 9 foi exercitada em devnet: 6007 em A e 6008 em B.
- O escrow é imutável. Não há Router, e-stop nem admin.
- R-D7: build e testes reproduzidos a partir de um clone, 14 transações
  conferidas, nenhum segredo no histórico. Achados RD7-01 a 05 (baixos) e
  06 a 10 (informativos); condições CR1 a CR8.
- D9 (`docs/d9-demo-results.md`): fluxo P′ (PASS) e T′ (timeout) pela CLI
  em ambiente limpo, com os negativos 6021, 6014, `verifier:6003` e 6007.
  CR1 a CR7 cumpridas; a CR8 vale para o D10–D12.
- **Frases permitidas congeladas** no `README.md` e em
  `docs/demo-script.md`; mudar exige decisão registrada.
- D10a (`docs/d10a-hardening-results.md`): RD7-01, 02, 03, 04 e 07
  corrigidos com testes que falham no D9 e passam agora; opcionais RD7-06, 09
  e 10. Nenhuma escrita em devnet. Programa, core, `zkvm/`, guest,
  `JournalV1` e locks inalterados. Aguarda a revisão delta R-D10a.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1",
  D2b.1, D2e, R-D2e, "Decisões humanas para o D4".
- "Prazo de 11/10, congelamento do JournalV1 v1 e nomes de gate".
- D4a, R-D4a, D4b, D7, R-D7, "Decisões humanas para o D9" (ratificadas
  com o prompt e o Plan Mode do D9) e D9 (`docs/decisions.md`).
- "Decisões humanas para o D10a" (ratificadas com o prompt e o Plan Mode do
  D10a) e D10a.

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
  - qualquer alegação além de "verificada em devnet por CPI ao verificador
    Groth16 imutável de risc0-solana v3.0.0", com os links;
  - "Verifier Router"; mainnet.
- Mock, dev mode ou receipt `Fake` como sucesso.
- Builds, testes ou provas pesadas em paralelo. No WSL de 7,6 GiB, com o
  editor aberto, rode builds de `solana-program-test` e a compressão Groth16
  destacados (`setsid nohup`) e com `CARGO_BUILD_JOBS=2`.
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- **Sem e-stop:** escrow e verificador imutáveis (RD4A-06).
- **Rent preso (F-13):** cerca de 0,0036 SOL por Job; não há `close`.
- **Mint authority = deployer:** só o projeto emite Test USDC; um terceiro
  que reproduza as escritas precisa recebê-lo.
- **`job_id`s públicos (F-09/R-03):** mitigado por `job_id` aleatório por
  Job (CLI).
- **R-02** (`fund` fora da janela): mitigado por `create_job`+`fund` na
  mesma transação, que a CLI sempre faz.
- **RPC público com limite de taxa:** a CLI cadencia e reenvia.
- **Memória do WSL:** a compressão Groth16 deixou 142 MB livres no pico; os
  builds de `solana-program-test` derrubaram a sessão do editor duas vezes
  no D7.
- F-14; ImageID não recertificado; spec v1 trivial.
- RD7-01, 02, 03, 04, 05 e 07: corrigidos (05 no D9; os demais no D10a),
  pendentes da revisão delta R-D10a. As erratas de
  `docs/manifest-schema.md:86` e de `docs/d7-cli-results.md:128` (esta no
  relatório do D10a) estão feitas.
- RD7-06, 09 e 10: corrigidos no D10a (opcionais). **RD7-08** continua
  aberto: um negativo `escrow:6021` perto do prazo pode virar reembolso real
  (a CLI reporta `UNEXPECTED`).
- A seção "Gravação" do roteiro usa os binários do D9, anteriores às
  correções; os negativos dela continuam os da CR2.
- Jobs `8ab4ee8d` e `8bce67f2` (gravações do humano): `Funded`, 2 Test USDC
  parados, reembolsáveis por qualquer um depois do prazo.
- Memória do WSL: compressão a 208 MB livres no D10a. Uma compressão com 80
  MB livres e swap cheio derrubou a sessão do Claude Code (incidente do
  D10a).
- Builds de árvores diferentes no mesmo `CARGO_TARGET_DIR` sobrescrevem os
  binários de mesmo nome (incidente do D10a): usar targets separados.
- O `env.sh` herdado define `R`, `B`, `D` e `VC`; não reutilizar esses nomes
  (incidente do R-D7).
- **Calendário:** 2026-10-06; prazo de entrega 11/10.

## Próxima transição permitida

`R-D10a`, conforme `docs/handoffs/d10a-to-r-d10a.md`: revisão delta curta,
somente leitura, em sessão separada (Opus 5.5, max). Depois, o D10 (worker
com prova local forçada), com o prompt que a R-D10a entregar.
