# Controle autônomo — D13 concluído; próximo D13b (fechamento da submissão)

## Objetivo atual

D13b, conforme `docs/handoffs/d13-to-d13b.md`: preencher os `TODO(…)` com os dados do
humano (vídeos, formulário, time, validação, modelo de negócio, slogan, acesso ao
repositório), push autorizado e conferência final dos links, para a submissão de 11/10.

Em paralelo, o humano grava a tomada contínua em inglês (10/10), pela seção "Gravação pela
interface" de `docs/demo-script.md`, depois do reinício do worker (C-UI-5), e monta a demo
técnica pela seção "Demo técnica (corte de 2–3 min)" e o pitch por
`docs/pitch-script.md`.

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
- D11a (fundação da interface Hive): `DESIGN.md`, `HIVE_MVP_UI_GUIDE.md`,
  `HIVE_MVP_UI_ADAPTATION.md`, `brand/` (com `brand/fonts/`), bloco
  `hive-ui-core` no `AGENTS.md`, `worker/ui-tools/` (ferramentas de
  desenvolvimento) e o Node 24 isolado em `~/.local/share/vericode-spikes/ui/`.
- D10: worker local e W1–W7 pela API:
  - `6eba814` (worker);
  - `23a91b5` (`docs: record D10 worker`).
- D11–D12: interface Hive servida pelo worker e U1–U10 pela interface:
  - `37d2d63` (worker);
  - `faad06c` (`docs: record D11-D12 interface`).
- R-UI (`docs: record R-UI review`): **APROVADO COM RESSALVAS** para o vídeo
  (RUI-01 a RUI-14; condições C-UI-1 a C-UI-7).
- D12a (`docs/d12a-results.md`): interface em inglês por padrão, trava D-NEG,
  liquidação aterrissada com reconciliação falha, recomendados RUI-04/05/08/09/11,
  ensaio C-UI-4 sobre os falsos e roteiro final em inglês; commits `ba79080`
  (`worker: English interface and R-UI fixes (D12a)`) e `docs: record D12a`.
- D13 (`docs/d13-results.md`): `README.md` em inglês, `docs/README.pt-BR.md` (f1 "da
  Hive"), `docs/submission.md`, `docs/pitch-script.md` e a seção "Demo técnica (corte de
  2–3 min)" do roteiro; commit pendente de autorização.

## Baseline (D11–D12)

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record D11-D12 interface`.
- **Worker:** `worker/vericode_worker.py` `bbdf6dbe…`; interface em `worker/static/`
  (56 arquivos da tabela `STATIC_FILES`), servida em `http://127.0.0.1:8710/ui/`;
  `unittest` 53/53 (23 do D10 + 30 de `test_static.py`); lint do `DESIGN.md` 0/0;
  `check:tokens` e `check:assets` sem deriva.
- **Jobs consumidos a mais (D11–D12):** T₃ `2c38ca66…` `RefundedOnTimeout`, P₃
  `26df9ef4…` `Released`, F₃ `7395a76d…` `RefundedOnFail`.
- **Saldos no fim do D11–D12** (slot 508.515.193):
  - SOL: deployer 2.805.194.240; buyer 88.981.200; executor 29.885.000 lamports;
  - Test USDC: ATA do buyer 999.992.000.000; ATA do executor 8.000.000.
- **Fora do clone:** `d11/` (`bin/` com `u11_*`, `logs/`, `shots/`, `tmp/`); o worker
  continua com `d10/worker.json` e `d10/data` (6 Jobs, receipts de `bc334093`, P₃ e
  F₃).

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

`D13` (concluído; commit pendente de autorização) → próximo `D13b`; tomada contínua e
montagem dos vídeos pelo humano.

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

- D11–D12 (`docs/d11-ui-results.md`): interface Hive em `worker/static/`, servida
  pelo worker em `/ui/` (tabela fixa, só GET, CSP `'self'`), com as lacunas 1–6 da
  adaptação fechadas como visões só de leitura; ensaio sobre os falsos e U1–U10 em
  devnet por cliques na interface: T₃ `RefundedOnTimeout`, P₃ `Released` e F₃
  `RefundedOnFail` com o verificador invocado (99.541 CU), negativos 6021, 6014, 6007
  (escrow) e 6003 (verificador); 10 transações, saldos fecham; seção "Gravação pela
  interface" no roteiro.
- R-UI (`docs/r-ui-review-results.md`): revisão somente leitura do worker e da
  interface; segurança HTTP, token, CSP, DOM, argv/ambiente e C10-7 sem achado;
  RUI-01 (interface só em português; alto), RUI-02 (escrita adversarial em Jobs
  consumidos) e RUI-03 (C/D contra a cadeia depois de reconciliação falha), ambos
  médios; roteiro final do vídeo em inglês na seção 10.
- D12a (`docs/d12a-results.md`): C-UI-1 a C-UI-3 só em `worker/static/`, nos testes e
  nos documentos autorizados; `unittest` 58/58 (5 testes novos falham na árvore de
  `HEAD`); `lint:design` 0/0, `tokens ok`, 43 assets; ensaio completo em inglês sobre os
  falsos com Job antigo e reinício (0 palavras portuguesas, 0 CSP, 0 erros de console);
  antes × depois de RUI-01/02/03; seção "Frozen phrases (English, ratified R-UI)" no
  README; roteiro final em `docs/demo-script.md`; adaptação editada só em §2 e §6.
  Desvio registrado e aceito pelo humano: regra de CSS de quebra de rótulo dos botões.
  Nenhuma escrita em devnet; worker real intocado.
- D13 (`docs/d13-results.md`): textos públicos em inglês com a marca Hive e os
  identificadores legados exatos; frases EN byte a byte iguais às ratificadas; o PT dos
  testes vem de `docs/README.pt-BR.md`; `unittest` 58/58; varreduras sem achado (termos,
  75 links de devnet, 29/29 assinaturas `finalized`, âncoras, comandos, segredos); uma linha
  de `worker/README.md` autorizada; nenhuma escrita em devnet; worker real intocado.

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
- "Fundação da interface Hive (D11a)": adaptação ao MVP, frase 1 com "da Hive"
  na interface, fontes auto-hospedadas, cópia verificada de assets,
  `worker/static/`, stack (execução sem dependência; ferramentas npm só de
  desenvolvimento, emendando a P2), idioma padrão português.
- D11–D12: plano aprovado em Plan Mode (2026-10-07 09:37), com a lista fechada
  U1–U10 (`docs/decisions.md`).
- R-UI (2026-10-07), em `docs/decisions.md`:
  - vídeo e submissão em inglês;
  - D-EN-1: frases e "Do not say" em inglês ratificados;
  - D-EN-2: inglês como padrão da interface;
  - edição autorizada do `HIVE_MVP_UI_ADAPTATION.md` (§2 decisão 2 e §6) no D12a;
  - D-EN-3: README inteiro no D13;
  - D-NEG por `first_seen`;
  - D-6014: receipt de P em F;
  - D-RUI-06 fora do D12a.
- Depois do D12a (2026-10-07), em `docs/decisions.md`:
  - desvio de CSS aceito;
  - commits do D12a autorizados;
  - README inteiro em inglês e português em `docs/README.pt-BR.md`;
  - frase 1 portuguesa "da Hive";
  - vídeos: pitch de até 3 min e demo técnica de 2–3 min cortada da tomada contínua,
    que fica como evidência completa.
- D13 (2026-10-07, Plan Mode): uma linha de `worker/README.md` autorizada; nome do produto
  Hive também na prosa de `docs/README.pt-BR.md`.

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
- **Interface (D11–D12, D12a):**
  - RUI-01 a 05, 08, 09 e 11 corrigidos no D12a; desvio de CSS (quebra de rótulo dos
    botões) aceito;
  - RUI-06 (`Content-Length: ²` → 500) fica no worker; RUI-07, 10, 12 a 14 abertos
    (informativos);
  - D-NEG depende do relógio do worker (resolução de 1 s);
  - f1 portuguesa "da Hive" em `docs/README.pt-BR.md` e no roteiro desde o D13; os
    comentários de `worker/static/js/i18n.js` e a §6 da adaptação ainda descrevem o README
    português (fora de escopo);
  - `TODO(video)`, `TODO(form)`, `TODO(team)`, `TODO(validation)`, `TODO(business)`,
    `TODO(brand)` e `TODO(repo)` nos textos públicos, até o humano fornecer (D13b);
  - **vídeos da submissão:** pelo guia da Colosseum (edição anterior), pitch de até
    3 min e demo técnica de 2–3 min; o humano confere as regras e os campos da edição
    atual. A tomada de ~13 min não é a demo técnica;
  - o repositório precisa estar acessível aos jurados: o push exige autorização
    separada;
  - orçamento do buyer: no máximo 8 tomadas completas sem novo SOL de devnet
    (C-UI-6);
  - `app_commit` é o HEAD da partida: reiniciar o worker depois do commit;
  - o WSL reiniciou em 2026-10-08 07:43 e o worker real (PID 121845) parou com ele; o
    humano o inicia de novo antes da gravação (C-UI-5);
  - `slot_clock` é estimativa (~0,24 s por slot medidos);
  - o SPL Token não registra o nome da instrução nos logs (a tela diz isso);
  - favicon inexistente (`TODO(brand)`).
- **Calendário:** 2026-10-07; prazo de entrega 11/10.

## Próxima transição permitida

`D13b`, conforme `docs/handoffs/d13-to-d13b.md` (Opus 5.5, high; Plan Mode obrigatório
por mudar claims públicos e pedir push). Em paralelo: a tomada contínua (10/10), cortada
depois para a demo técnica. Submissão: 11/10.
