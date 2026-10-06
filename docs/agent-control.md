# Controle autônomo — D10 concluído (worker local); próximo D11–D12

## Objetivo atual

D11–D12, conforme `docs/handoffs/d10-to-d11.md`: telas finas Buyer, Submit e
Result em HTML/JS estático servido pelo worker do D10, sem carteira no
navegador (P3), só com frases congeladas.

Depois: revisão curta da interface (claims e chaves), vídeo definitivo numa
tomada contínua e submissão até 11/10.

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
  - `2a2e3c5` (docs).
- R-D10a `544b685`: **APROVADO COM RESSALVAS** para o D10 (C10-1 a C10-10).
- D10: worker local e W1–W7 pela API:
  - `6eba814` (worker);
  - `docs: record D10 worker`.

## Baseline (D10)

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record D10 worker`.
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
    (`RefundedOnTimeout` desde o W1/W2 do D10), `cd77e7bd` e `3e115ca8`
    (`Released`), `104f9a21` (`RefundedOnTimeout`);
  - **do D10: `bc334093…` `Released`** (W3–W7, pela API do worker).
- **Saldos no fim do D10** (slot 508.249.020):
  - SOL: deployer 2.805.194.240; buyer 99.750.400; executor 29.910.000
    lamports;
  - Test USDC: ATA do buyer 999.993.000.000; ATA do executor 7.000.000.
- **Worker (D10):** `worker/vericode_worker.py`, `unittest` 23/23;
  configuração `d10/worker.json` (`0600`), dados em `d10/data` (3 Jobs, 10
  operações, receipt de `bc334093…`), porta 8710; iniciado por
  `env -i HOME=… PATH=/usr/bin:/bin python3 -B worker/vericode_worker.py
  --config …/d10/worker.json` (o token sai só no terminal do operador).
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
    `X` e `X-wrong-binary`, logs;
  - `d10/`: configuração do worker, `bin/` (`w10_api.sh`, `w10_rpc.py`,
    `w10_sum.py`, `w10_secret.py`), `data/` do worker e logs (W1–W7,
    saldos, memória, conferências).

## Gate atual

`D10` (concluído) → próximo `D11–D12`.

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
  `JournalV1` e locks inalterados.
- R-D10a (`docs/r-d10a-review-results.md`): correções confirmadas com os
  binários do D9, do D10a e um rebuild contra logs reais de 24 transações;
  sem achado crítico, alto ou médio; condições C10-1 a C10-10 para o D10.
- D10 (`docs/d10-worker-results.md`): worker local em `127.0.0.1` (Python,
  só biblioteca padrão) sobre os binários do D10a, com C10-1 a C10-10 no
  código e em 23 testes; W1–W7 em devnet pela API: reembolso dos dois Jobs das
  gravações, Job `bc334093…` `Released` com o verificador invocado (99.541
  CU), negativos 6021 (escrow), 6003 (verificador) e 6007 (escrow) com o
  programa identificado só pela saída da CLI. Saldos fecham; nenhuma chave
  nem caminho de chave saiu do worker.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1",
  D2b.1, D2e, R-D2e, "Decisões humanas para o D4".
- "Prazo de 11/10, congelamento do JournalV1 v1 e nomes de gate".
- D4a, R-D4a, D4b, D7, R-D7, "Decisões humanas para o D9" (ratificadas
  com o prompt e o Plan Mode do D9) e D9 (`docs/decisions.md`).
- "Decisões humanas para o D10a" (ratificadas com o prompt e o Plan Mode do
  D10a) e D10a.
- R-D10a, "Decisões humanas para o D10" (ratificadas com o prompt e o Plan
  Mode do D10, 19:18), "Vídeo de reserva" e D10.

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
- RD7-01, 02, 03, 04, 05 e 07: corrigidos (05 no D9; os demais no D10a)
  e confirmados pela R-D10a.
- RD10A-01 (baixo): o prover executa o shim da árvore de fontes sem
  conferir hash; desde o D10, o worker confere hash `2a8f75b8…` e modo
  `0755` antes de cada `compress` e exige a linha `docker_run` depois (C10-2).
  Fora do worker, a CLI e o prover continuam como no D10a.
- RD10A-02 a 07 (info): ambiente ainda escolhe `RISC0_WORK_DIR`,
  `VERICODE_REAL_DOCKER`, `DOCKER_HOST=unix://…`; log do shim forjável;
  exit 1 depois de transação aterrissada; logs sintéticos nos testes; slice
  sem checagem no `check` do escrow; "daemon local" nos READMEs. As erratas de
  `docs/manifest-schema.md:86` e de `docs/d7-cli-results.md:128` (esta no
  relatório do D10a) estão feitas.
- RD7-06, 09 e 10: corrigidos no D10a (opcionais). **RD7-08** continua
  aberto na CLI: um negativo `escrow:6021` perto do prazo pode virar
  reembolso real (a CLI reporta `UNEXPECTED`). O worker só envia o 6021 com
  `deadline_slot − slot ≥ 300` num `job show` imediatamente anterior (C10-8).
- A seção "Gravação" do roteiro usa os binários do D9, anteriores às
  correções; os negativos dela continuam os da CR2.
- Jobs `8ab4ee8d` e `8bce67f2` (gravações do humano): reembolsados no D10
  (W1, W2).
- **Worker (D10):**
  - P3: chaves de devnet do projeto num worker local, por caminho; o mesmo
    operador opera buyer e executor; sem carteira no navegador;
  - `http.server` não é endurecido para rede: só em `127.0.0.1`, com C10-6;
  - o token da execução fica na captura do terminal do operador (`0600`);
  - a rota `escrow-6014` foi testada só offline;
  - se o `compress` estourar o tempo, um container já iniciado termina
    sozinho (`--rm`) e a receipt não é usada.
- Memória no `compress` do D10: 167 MB disponíveis e 4,2 GB de swap no pico,
  sem exit 137.
- Vídeo de reserva: 17 clipes em `C:\Users\lucas\Videos\` que misturam
  Jobs; uso só com legendas por Job. O vídeo definitivo deve ser uma tomada
  contínua ("Vídeo de reserva" em `docs/decisions.md`).
- Memória: swap do WSL aumentado para 8 GiB em 2026-10-06
  (`C:\Users\lucas\.wslconfig`); RAM continua 7,6 GiB.
- Memória do WSL: compressão a 208 MB livres no D10a. Uma compressão com 80
  MB livres e swap cheio derrubou a sessão do Claude Code (incidente do
  D10a).
- Builds de árvores diferentes no mesmo `CARGO_TARGET_DIR` sobrescrevem os
  binários de mesmo nome (incidente do D10a): usar targets separados.
- O `env.sh` herdado define `R`, `B`, `D` e `VC`; não reutilizar esses nomes
  (incidente do R-D7).
- **Calendário:** 2026-10-06; prazo de entrega 11/10.

## Próxima transição permitida

`D11–D12`, conforme `docs/handoffs/d10-to-d11.md`, em sessão nova (Opus
5.5, xhigh, Plan Mode antes de alterar `worker/` e antes da primeira escrita
em devnet).
