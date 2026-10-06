# Controle autônomo — R-D7 registrado (APROVADO COM RESSALVAS); próximo D9

## Objetivo atual

D9, conforme `docs/handoffs/r-d7-to-d9.md`:
- demo em ambiente limpo, com as escritas W1 a W8 em devnet;
- congelamento dos claims e erratas da CR1;
- roteiro de gravação para o vídeo de reserva, gravado pelo humano.

Depois: D10a (RD7-01, 02, 03, 04, 07) com revisão delta curta, D10 (worker),
D11–D12 (telas finas, CR8), revisão curta da interface, vídeo definitivo e
submissão até 11/10.

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
- R-D7: **APROVADO COM RESSALVAS** para o D9 e o D10–D12 (CR1 a CR8);
  registrado no commit `docs: record R-D7 final review`.

## Baseline (D7)

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record R-D7 final review`.
- **Devnet:**
  - escrow `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`, 395.064 bytes
    `cdf6967f…`, upgrade authority **`none`**;
  - verificador `THq1q…` imutável;
  - mint Test USDC `9TE2V…` (6 decimais, sem freeze).
- **Jobs consumidos** (nunca reutilizar os `job_id`s):
  - S, A `Released`; B `RefundedOnFail`; C `RefundedOnTimeout` (D4b);
  - **P `Released`** (`3e4ca026…`) e **T `RefundedOnTimeout`**
    (`05f74934…`), do D7.
- **Saldos no fim do D7:**
  - SOL: deployer 2.805.224.240; buyer 128.466.600; executor 29.970.000
    lamports;
  - Test USDC: ATA do buyer 999.997.000.000; ATA do executor 3.000.000.
- **Testes:**
  - `anchor/tests-local` 61/61 com o rebuild e com o dump de devnet:
    d4b_receipts 2, escrow 27, fixtures 2, layout 7, regressions 7,
    settlement 16;
  - `cli` 14/14; `prover` 4/4; core 42/42 A/B; IDL `e8ce2c20…`.
- **Locks:**
  - inalterados: raiz `191802b2…`; host `f5236689…`; guest `1116acef…`;
    `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`;
  - novos: `cli` `4d979577…`; `prover` `8b76f1e1…`.
- **Fora do clone:**
  - `d4/keys`: deployer, buyer, executor e mint;
  - `d4/receipts-out` (S, A, A′, B);
  - `d7/`: homes, targets, receipts do P, jobs P/T e logs
    (`cli-tx.jsonl`, `timeline.log`).

## Gate atual

`R-D7` (registrado) → próximo `D9`.

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

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1",
  D2b.1, D2e, R-D2e, "Decisões humanas para o D4".
- "Prazo de 11/10, congelamento do JournalV1 v1 e nomes de gate".
- D4a, R-D4a, D4b, D7, R-D7 e "Decisões humanas para o D9"
  (`docs/decisions.md`).

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
- RD7-05: erratas de docs e reprodutibilidade (CR1, no D9), inclusive
  `docs/manifest-schema.md:70`, que exige Plan Mode.
- RD7-06 a 10: hardlink e modo do `--log`; leituras sem `minContextSlot`
  ("already processed"); negativo que vira liquidação; mensagens
  cosméticas; `--rpc-url`/proxy.
- O `env.sh` herdado define `R`, `B`, `D` e `VC`; não reutilizar esses nomes
  (incidente do R-D7).
- **Calendário:** 2026-10-05; prazo de entrega 11/10.

## Próxima transição permitida

`D9`, conforme `docs/handoffs/r-d7-to-d9.md`, em sessão nova (Opus 5.5,
xhigh, Plan Mode antes das escritas em devnet e da errata em
`docs/manifest-schema.md`).
