# Controle autônomo — D9 concluído; próximo D10a

## Objetivo atual

D10a, conforme `docs/handoffs/d9-to-d10a.md`: endurecimento da CLI e do
prover (RD7-01, 02, 03, 04 e 07), com testes e sem tocar no programa nem nos
locks existentes, seguido de uma revisão delta curta.

Em paralelo, fora do agente: o humano grava o vídeo de reserva seguindo a
seção "Gravação" de `docs/demo-script.md`.

Depois: D10 (worker), D11–D12 (telas finas, CR8), revisão curta da
interface, vídeo definitivo e submissão até 11/10.

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
- D9: demo em ambiente limpo (W1–W8), claims congelados e roteiro de
  gravação; commit `docs: record D9 clean-environment demo and freeze
  claims`.

## Baseline (D9)

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record D9 clean-environment demo and freeze claims`.
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
    (`ec9afb74…`), do D9.
- **Saldos no fim do D9:**
  - SOL: deployer 2.805.214.240; buyer 121.288.800; executor 29.955.000
    lamports;
  - Test USDC: ATA do buyer 999.996.000.000; ATA do executor 4.000.000.
- **Testes:**
  - `anchor/tests-local` 61/61 com o rebuild e com o dump de devnet:
    d4b_receipts 2, escrow 27, fixtures 2, layout 7, regressions 7,
    settlement 16;
  - `cli` 14/14; `prover` 4/4; core 42/42 A/B; IDL `e8ce2c20…`;
  - D9, no ambiente limpo (`d9/clone`, homes copiadas): core 42/42 ×2, CLI
    14/14, suíte 61/61 com os `.so` de devnet, prover 4/4.
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
    `bin/env9.sh` (usado pela seção "Gravação") e logs.

## Gate atual

`D9` (concluído) → próximo `D10a`.

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

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1",
  D2b.1, D2e, R-D2e, "Decisões humanas para o D4".
- "Prazo de 11/10, congelamento do JournalV1 v1 e nomes de gate".
- D4a, R-D4a, D4b, D7, R-D7, "Decisões humanas para o D9" (ratificadas
  com o prompt e o Plan Mode do D9) e D9 (`docs/decisions.md`).

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
- RD7-01: `--expect-error escrow:N` aceita falha do verificador com o mesmo
  N (6000–6003); na demo, só os negativos da CR2.
- RD7-02: o shim do Docker não é allowlist de argv.
- RD7-03: o prover obedece `RISC0_PROVER`/`BONSAI_*`; na demo, a CR3.
- RD7-04: `--tamper-seal` e o casamento do `--expect-error` sem teste.
- RD7-05: corrigido no D9 (CR1). Resta a errata de
  `docs/manifest-schema.md:86` ("terá a upgrade authority finalizada"), que
  exige Plan Mode.
- O comentário de `prover/docker-shim/docker` ainda promete recusar "any
  other docker run" (código; D10a, com a RD7-02).
- `docs/d7-cli-results.md:128` lista `--expect-error` entre os testes, mas só
  o parse é testado (RD7-04).
- Memória do WSL na compressão do D9: 117 MB livres e swap cheio no pico.
- RD7-06 a 10: hardlink e modo do `--log`; leituras sem `minContextSlot`
  ("already processed"); negativo que vira liquidação; mensagens
  cosméticas; `--rpc-url`/proxy.
- O `env.sh` herdado define `R`, `B`, `D` e `VC`; não reutilizar esses nomes
  (incidente do R-D7).
- **Calendário:** 2026-10-05; prazo de entrega 11/10.

## Próxima transição permitida

`D10a`, conforme `docs/handoffs/d9-to-d10a.md`, em sessão nova (Opus 5.5,
high; Plan Mode antes de mudar a CLI ou o prover), seguido de uma revisão
delta curta em sessão separada e somente leitura.
