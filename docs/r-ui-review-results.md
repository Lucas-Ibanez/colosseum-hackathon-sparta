# R-UI — revisão adversarial do worker (D10) e da interface Hive (D11–D12): resultados

> **Nota de registro (2026-10-07):** as decisões da seção 11 foram tomadas pelo humano
> depois desta revisão e estão em `docs/decisions.md` (entrada R-UI):
> - D-EN-1 ratificada; a regra sobre `escrow:6000`–`6003` (RD7-01) foi registrada como
>   regra de operação, fora da lista congelada;
> - D-EN-2: inglês como padrão;
> - D-EN-3: o README inteiro vai para o gate de submissão;
> - D-NEG e D-6014 conforme a recomendação;
> - D-RUI-06 fica fora do D12a;
> - edição autorizada do `HIVE_MVP_UI_ADAPTATION.md` no D12a.
>
> O texto abaixo é o da revisão, sem alteração.

Sessão separada e somente leitura (Claude Code, Opus 5.5, esforço max), 2026-10-07
14:38–14:58 (-03:00). HEAD revisado `faad06c` (`docs: record D11-D12 interface`, sobre
`37d2d63`); worker `bbdf6dbe…`. Nenhum arquivo do repositório criado, alterado ou
apagado; nenhuma escrita em devnet; o worker real (`127.0.0.1:8710`, PID 121845, iniciado
às 13:47:30, depois do commit `faad06c` das 13:41:29) não foi tocado. Artefatos em
`~/.local/share/vericode-spikes/r-ui/` (`0700`, arquivos `0600`).

## Resultado

**APROVADO COM RESSALVAS** para o vídeo definitivo, condicionado ao gate D12a (C-UI-1 a
C-UI-3) e às condições de operação C-UI-4 a C-UI-7 (seção 9).

- Segurança do worker e da interface: sem achado crítico, alto ou médio.
  - HTTP: `Host`, token e ausência de CORS confirmados.
  - CSP: confirmada nas páginas e na API.
  - Estáticos: tabela fixa.
  - Processos: argv fixo, ambiente do zero.
  - DOM: só `textContent`.
  - Token: só na memória da aba.
  - Rótulo: C10-7.
- **RUI-01 (alto):** a interface é só em português, e o vídeo e a submissão serão em
  inglês (decisão humana de 2026-10-07).
- **RUI-02 e RUI-03 (médios):**
  - escrita habilitada em Jobs consumidos;
  - tela contra a cadeia depois de uma reconciliação falha.
- RUI-04 a 06 (baixos) e RUI-07 a 14 (informativos).

## 1. Escopo e método

- Leitura integral:
  - `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`, `docs/handoff-protocol.md`,
    `docs/agent-control.md`;
  - a adaptação, o guia, o `DESIGN.md`, `brand/MANIFEST.md`;
  - as entradas de `docs/decisions.md` do R-D10a ao D11–D12;
  - `docs/r-d10a-review-results.md` (C10-1 a C10-10), `docs/d11-ui-results.md`;
  - `worker/README.md`, `worker/vericode_worker.py`, `worker/tests/` (testes, falsos e
    fixtures);
  - `worker/static/` inteiro e `worker/ui-tools/`;
  - `README.md`, `docs/demo-script.md` (cabeçalho, frases e "Gravação pela interface").
- Workers de revisão: o código do repositório sobre os **executáveis falsos dos testes**,
  numa raiz própria em `r-ui/tmp`:
  - 8712: sequência completa e hostil;
  - 8713: cópia dos estáticos com `LANG="en"`;
  - 8714: reconciliação falha.
- Navegador: Playwright/Chromium headless de `worker/ui-tools/node_modules`, com o Node
  isolado de `ui/env-ui.sh`.
- Devnet: só leitura, pela CLI do D10a (`job show`) e por JSON-RPC direto a
  `api.devnet.solana.com`.

## 2. Preflight e checagem da tarefa anterior

- Git:
  - `pwd` = raiz `/home/lucas/src/vericode`, branch `main`;
  - HEAD `faad06c` sobre `37d2d63` e `1147edd`;
  - `git status --short` vazio; `--ignored` só `worker/ui-tools/node_modules/` e
    `brand/reference/Hive-posicionamento-identidade-2026-10-05.pdf` (decisão D11a);
  - `git diff --check` 0.
- Perfil padrão, iguais no início e no fim:
  - `~/.rustup`, `~/.cache/solana`, `~/.config/solana` e `~/.npm` ausentes;
  - `~/.cargo` `d9e12578`, `~/.avm` `7d29f7f8`, `~/.docker` `6046f67f`.
- Hashes, iguais no início e no fim:
  - worker `bbdf6dbe`;
  - locks: raiz `191802b2`, `zkvm` `f5236689`, guest `1116acef`, `anchor` `19a1db26`,
    `tests-local` `be94760a`, `cli` `4d979577`, `prover` `8b76f1e1`, `ui-tools`
    `0c40c2ab`;
  - guest `e09ba8cf`; shim `2a8f75b8` com modo `0755`;
  - CLI `e6cd4e29` e prover `3f66e1c0`, iguais aos fixados em `d10/worker.json`.
- `TMPDIR=r-ui/tmp python3 -B -m unittest discover -s worker/tests -v`: **53/53**
  (35,5 s); nenhum `__pycache__`.
- Ferramentas da interface:
  - `npm ci --ignore-scripts --offline` numa cópia do `package.json` e do lock fora do
    clone: 96 pacotes, 0 vulnerabilidades; o `node_modules` do clone não foi tocado, e a
    cópia foi apagada no fim;
  - `lint:design`: 0 erros, 0 avisos (1 info);
  - `check:tokens`: `tokens ok`;
  - `check:assets`: `43 assets match the table`.

## 3. Devnet (só leitura)

- `vericode job show` (CLI do D10a, `env -i`, HOME e TMPDIR em `r-ui/`):
  - T₃ `2c38ca66…`: `RefundedOnTimeout`, vault 0;
  - P₃ `26df9ef4…`: `Released artifact_hash=225384a7…`;
  - F₃ `7395a76d…`: `RefundedOnFail artifact_hash=343ad778…`.
- `getTransaction` das 10 assinaturas de U1–U10, iguais ao `docs/d11-ui-results.md`:

  | Transação | Slot | Erro | CU | Movimento |
  | --- | ---: | --- | ---: | --- |
  | U1 `8AJzbbKf` | 508.512.386 | — | 45.150 | buyer −1 / vault +1 |
  | U2 `4jDL97DG` | 508.512.520 | `DeadlineNotReached` 6021 | 8.991 | — |
  | U3 `5vWoxx2m` | 508.512.702 | — | 43.650 | buyer −1 / vault +1 |
  | U4 `5CBFajsa` | 508.513.784 | `JournalJobIdMismatch` 6014 | 15.052 | — |
  | U5 `kTQdVojx` | 508.513.912 | `PairingError` 6003 (verificador, 100.528 CU) | 119.238 | — |
  | U6 `2gB9djwM` | 508.514.034 | — (verificador 99.541 CU) | 122.156 | vault −1 / executor +1 |
  | U7 `j2pmqFWu` | 508.514.166 | `AlreadyReleased` 6007 | 9.989 | — |
  | U8 `2jjiYDAo` | 508.514.282 | — | 43.650 | buyer −1 / vault +1 |
  | U9 `4jcugMyX` | 508.514.963 | — (verificador 99.541 CU) | 121.985 | vault −1 / buyer +1 |
  | U10 `5oTbFiTX` | 508.515.140 | — | 14.605 | vault −1 / buyer +1 |

  Todas com taxa de 5.000 lamports.
- Saldos nos slots 508.534.089 (início) e 508.537.364 (fim), iguais ao fim do D11–D12:
  - SOL: deployer 2.805.194.240, buyer 88.981.200, executor 29.885.000 lamports;
  - Test USDC: ATA do buyer 999.992.000.000, ATA do executor 8.000.000.
- `getSignaturesForAddress`:
  - buyer desde `5oTbFiTX`: 0;
  - executor desde `4jcugMyX`: 0;
  - última do deployer `5CR2s5Jb`, no slot 508.105.045, anterior ao D10.
- Ritmo medido: (508.537.364 − 508.515.193) slots entre 13:29:08 e 14:57 ≈ 0,24 s por
  slot.
- Worker real (só leitura dos arquivos de `d10/data`):
  - 6 Jobs: `bc334093` `Released`, P₃ `Released`, F₃ `RefundedOnFail`, T₃
    `RefundedOnTimeout`, `8ab4ee8d` e `8bce67f2` externos `RefundedOnTimeout`;
  - 26 operações, nenhuma reconciliação pendente.

## 4. PoCs HTTP (worker de revisão 8712, sockets crus)

`r-ui/poc/rui_http.py`, saída em `r-ui/logs/poc-http.log`. Toda resposta veio com
`nosniff`, CSP e nenhum `Access-Control-*`.

| Requisição | Resposta |
| --- | --- |
| `Host` `localhost:8712`, `127.0.0.1`, `[::1]:8712`, `127.0.0.1:08712`, `127.0.0.1.:8712`, `rebind.example:8712`, valor com espaço final | 421 |
| dois `Host` | 400 `bad_host` |
| sem token / token errado / dois tokens / nome do header em minúsculas | 401 / 403 / 403 / 200 |
| `OPTIONS` com `Origin` e preflight | 405, sem CORS |
| POST cross-origin `text/plain` sem token | 401 |
| `/ui/` | 200 com a CSP de página |
| `/ui/index.html`, `..`, `%2e%2e`, `/ui/./js/main.js`, `;x`, `%00`, `/UI/`, forma absoluta | 404 |
| `//ui/`, `///api/health`, tab ou espaço duplo na linha de requisição | 400 `bad_path` |
| `?`, `#` | 400 `query_not_allowed` |
| `HEAD`, `TRACE` / `POST /ui/` / método `FOO` | 405 / 405 / 501 |
| HTTP/0.9 | recusado (`bad_host`) |
| `/api/jobs/../health`, barra final, `op_id` maiúsculo | 404 / 404 / 400 |
| `Transfer-Encoding: chunked`, dois `Content-Length`, `-1` | 400 |
| 4.097 bytes | 413 |
| `Content-Length: ²` (U+00B2 em latin-1) | **500 `internal`** (RUI-06) |
| `Content-Length: ١` (UTF-8) | 400 |
| JSON com chave duplicada, `9e3`, `NaN`, inteiro enorme, BOM | 400 |
| `negative/..%2fescrow-6007` | 400 `bad_kind` |
| 6014 com `job_id` maiúsculo ou o da fixture | recusado |

## 5. PoCs de navegador e das visões

**Fluxo pela API (8712, falsos).** Sequência executada:
- create OLD, prova, settle;
- create T, 6021;
- create P, prova, 6014 com a receipt de OLD, 6003, settle, 6007;
- create F, prova `(7,15)` `FAIL`, settle;
- releitura de T e refund.

Todas as operações terminaram `ok`. `rejection` veio só quando a saída da CLI mostrou
tudo: `escrow:6021`, `escrow:6014`, `verifier:6003`, `escrow:6007`.

**Auditoria do DOM** (`rui_ui.mjs audit`): Jobs, Novo job e os Jobs OLD, P, F e T.

| Verificação | Resultado |
| --- | --- |
| Links fora de `#/…` ou do Explorer de devnet | 0 |
| `target=_blank` sem `noopener` | 0 |
| Imagens | só `/ui/brand/hive-horizontal-branco.svg` |
| Scripts | só `/ui/js/main.js` |
| Atributos `on*` | 0 |
| Token no DOM ou na URL | não |
| `localStorage`/`sessionStorage`/cookie | vazios |
| `StatusLabel` sem ícone ou texto | 0 |
| Âmbar | só no botão primário e nas barras de seleção |
| Violações de CSP; erros de console | 0; 0 |
| Teclado | 25 paradas em ordem visual, contorno sólido de 2 px `rgb(57, 45, 64)` |
| Recarga | volta à tela do token, armazenamento 0 |

**Saída hostil.** O falso devolveu, no create e no `stderr` de um 6021,
`<img src=x onerror=…>` e `<script>…</script>`. A tela mostrou o texto literal, não
inseriu `img` nem `script`, e o título não mudou.

**`UNEXPECTED`.** Um 6021 com `outcome=UNEXPECTED` ficou com `rejection: null`, operação
`failed` e "Sem rejeição confirmada". Nunca aparece como rejeição.

**6007 em Job antigo (RUI-02).** No Job OLD (`Released`, criado antes):
- 6021, 6014 e 6003 desabilitados com motivo; "Liquidar de novo" **habilitado**;
- o diálogo não diz que o Job é antigo;
- ao confirmar, a operação foi enviada: "Transação rejeitada: job já liquidado".

**Reconciliação falha (RUI-03), worker 8714.**
- `settle`: `status=failed`, `outcome=PASS`, `verifier_invoked=true`, 99.541 CU,
  `reconciled=false`.
- Releitura: `Released`.
- Tela:
  - cabeçalho e banner: `Released`, mas o banner fica sem link do Explorer;
  - bloco C: "Ainda não verificada on-chain";
  - bloco D: "Ainda não liquidado";
  - frase 1 ausente.

**Inglês (RUI-01), worker 8713.** Cópia dos estáticos com `LANG="en"`/`LOCALE="en-US"`:
82 ocorrências de palavras em português na tela do Job P, contra 90 em português.
Continuam em português, entre outros, "Verificação", "Proof verificada on-chain…",
"Programa", "Compute units do verificador" e as frases f1 e f8.

**Visões novas** (`r-ui/logs/poc-views.log`):

| Função | Entrada | Resultado |
| --- | --- | --- |
| `invocations_of` | log truncado | invocação do verificador com `result: null` |
| `invocations_of` | log fora de ordem | só o `success` do topo conta |
| `invocations_of` | 100 `invoke` | limite de 64 |
| `decode_journal` | `schema_version` 0 ou 2, tag 2, hex maiúsculo, quebra de linha | recusado |
| `balances_of` | `accountIndex` booleano, dono não base58, valor negativo | campos `null` |
| `v1_facts` | linhas do check forjadas | `cluster: null` |

## 6. Achados

- **RUI-01 (alto, bloqueia o vídeo em inglês).**
  - `worker/static/js/i18n.js` fixa `LANG = "pt-BR"`; o dicionário `EN` tem 33 das 366
    chaves.
  - Ficam fora do dicionário: `index.html` (`lang="pt-BR"` e o `noscript`) e o
    `"Job "` do cabeçalho em `views/job.js`.
  - As frases congeladas não têm tradução registrada.
  - Com a decisão humana de 2026-10-07 (vídeo e submissão em inglês), a tomada mostraria
    a interface em português.
  - Correção: seção 9, C-UI-1.
- **RUI-02 (médio).**
  - `adversarial()` avalia só o estado do Job: com o Job liquidado e uma receipt
    utilizável, "Liquidar de novo" fica habilitado também em P₃, F₃ e `bc334093`, que
    são Jobs consumidos (C10-10).
  - Sem risco econômico: o `--expect-error` só envia se a simulação falhar com 6007, e o
    Job é terminal. Mas uma tomada pode gravar uma escrita fora do roteiro num Job
    antigo, paga pelo executor.
  - O seletor do 6014 começa pelo candidato mais antigo (`bc334093`).
- **RUI-03 (médio).**
  - `settlementOp()` exige `status === "ok"`. Se o `job show` de reconciliação falhar
    depois de uma liquidação aterrissada (RPC público com limite de taxa), a operação
    fica `failed` para sempre.
  - Os blocos C e D e o link do banner passam a contradizer a cadeia, mesmo depois de
    reler. Seria na cena central do vídeo.
- **RUI-04 (baixo).** Um Job terminal sem operação de liquidação deste worker (externo
  lido por `show`, ou 6021 `UNEXPECTED` que virou reembolso pelo RD7-08) mostra "Ainda
  não liquidado" e "Ainda não verificada on-chain".
- **RUI-05 (baixo).** O banner `Failed` diz "A prova ou a compressão falhou… O job
  continua `Funded`" também no caso de um create sem Job na cadeia.
- **RUI-06 (baixo).** `read_body` usa `str.isdigit()`, que aceita `²`; `int()` falha, e a
  resposta é 500. Sem efeito: o token é conferido antes e nada executa. Mudança no
  worker só com autorização separada.
- **RUI-07 (info).** Uma invocação sem resultado (log truncado) aparece como "concluída".
  Na prática só ocorre em transação aterrissada sem erro, cuja atomicidade garante o
  sucesso.
- **RUI-08 (info).**
  - "?" como valor de reserva: modelo do create (`v1.executor || "?"`), diálogos,
    autoridade e "`?` concluída".
  - A frase 1 seria exibida sem link se `tx.explorer` faltasse.
- **RUI-09 (info).** `new.termsHelp` diz "lidas pelo `vericode check`". O check imprime
  as constantes compiladas da CLI (`escrow::admitted_terms()`), iguais às do programa
  fixado (`cdf6967f…`). Usar "impressas", como a adaptação.
- **RUI-10 (info).** O bloco F e a linha `docker_run` mostram caminhos locais de
  `d10/data` (com o usuário), que aparecem no vídeo. Não são caminhos de chave.
- **RUI-11 (info).**
  - Na tabela do 6014 dentro do banner de perigo, os nomes das linhas que conferem herdam
    a cor de perigo; só a linha `job_id` tem o fundo de divergência.
  - Um Job terminal mostra "faltam N slots".
- **RUI-12 (info).** O `DOMParser` dos ícones recusa `script`, `foreignObject` e erro de
  parse, mas não remove `on*`. Mitigado por SHA-256 das cópias e pela CSP.
- **RUI-13 (info).** O `http.server` não limita conexões (slowloris local); HTTP/0.9
  responde sem linha de status e é recusado.
- **RUI-14 (info, operação).** Ver a seção 9.

## 7. Checklist do prompt

| Item | Resultado |
| --- | --- |
| Bind, `Host` (DNS rebinding), token em `/api/*`, sem CORS | ok (seção 4) |
| CSP e cabeçalhos | ok |
| Rotas estáticas | ok |
| Limites de corpo | ok, exceto RUI-06 |
| 409 e `reconcile_pending` | ok |
| argv fixo, ambiente do zero, shim antes e `docker_run` depois | ok (código, testes; o cliente nunca fornece caminho, flag, programa nem URL) |
| Uma operação por vez, reconciliação, sem reenvio | ok (código, testes) |
| DOM só `textContent` (`rich`, `hashField`, diálogos, ícones) | ok (PoC hostil) |
| Links só do Explorer de devnet | ok |
| Token só em memória e no header | ok |
| Nenhum dado sem fonte | ok, exceto RUI-08 |
| Nenhum botão que decida veredito ou destino de token | ok |
| 6014 identifica a receipt | ok |
| `rejection` como única fonte do programa; `UNEXPECTED` nunca como rejeição | ok (PoC) |
| `Proving` rotulado como local | ok |
| Visões novas com entradas hostis | ok (seção 5) |
| Chaves: nada em resposta, log, página ou versionado; fixtures `d10-tx` públicas | ok (testes, varredura) |
| Frase 1 com "da Hive" | ok |
| Frases 2, 3, 4, 5 e 8 = `README.md` | ok |
| "Não dizer", "Verifier Router" e "fallback" ausentes | ok |
| Limitação P3 visível (Novo job e Detalhe) | ok |
| Identidade: tokens, âmbar, estado nunca só por cor, foco, PartyMark idêntico, sem halo, selo, favos ou mascote | ok (RUI-11 cosmético) |
| Adaptação: estados, compromissos × journal, verificação, anatomia, cenários, faixa | ok, exceto RUI-03 e RUI-04 |
| Roteiro factível numa tomada | sim, com C-UI-4 a C-UI-7 e o roteiro da seção 10 |
| Riscos de operador | Jobs antigos: RUI-02; `app_commit` exige reiniciar o worker depois do commit; token fora da câmera |

## 8. Inglês: tradução proposta (ratificação pendente, D-EN-1) e strings sem inglês

**As 8 frases** (fiéis, sem dizer mais; a frase 1 com "Hive" para interface e vídeo):

1. **Claim:** "Hive's Groth16 receipt is verified on devnet via CPI to the immutable
   Groth16 verifier of risc0-solana v3.0.0, in the same instruction that releases or
   refunds the Job's Test USDC." Always with the link of a transaction.
2. **Statement:** "A previously committed deterministic evaluator ran on the artifact
   delivered in the Job and produced the published verdict. The program moves the Test
   USDC only after checking the binding between journal, Job and delivery and verifying
   the proof."
3. **Limit:** "The proof attests to the execution of the fixed rule on this artifact, not
   to the quality of a piece of software. The v1 rule is trivial (output = 2 × input) and
   serves to demonstrate the flow."
4. **No administrator:** "The escrow and the verifier are immutable (upgrade authority
   `none`). No one, not even the project, changes the rules or decides the payment; there
   is also no e-stop."
5. **Network:** "Devnet and Test USDC only; none of this exists on mainnet."
6. **Negatives:** "On devnet, a tampered proof, a journal from another Job, a refund
   before the deadline and a second settlement were rejected without moving funds."
   Always with the links.
7. **Reproduction:** "The flow was reproduced with this repository's CLI and prover in a
   clean environment: a fresh clone and fresh targets, with copied isolated toolchains,
   not a new machine." (D9)
8. **Proof:** "The proof is generated locally and compressed to Groth16 in a local Docker
   container, with no network access."

**Do not say:**
- "the code is correct";
- "trustless" or "nobody needs to trust anybody";
- "any repository";
- "Verifier Router" as the current path;
- "mainnet" or "real money";
- "ZK on-chain" without phrase 1;
- "audited", "private" or "new machine".

Also do not:
- present an earlier run as live;
- show a receipt without saying which Job it belongs to;
- name the rejecting program from an `escrow:6000`–`6003` code (RD7-01).

In the interface, also avoid (guide §8.3): "approved"/"rejected" as a verdict of the
interface, "secure", "safe", "trustworthy", "guaranteed", "certified", "verified agent",
"reputation", "score", "trust level". "Transaction rejected" in the scenario titles names
the program's rejection, not a verdict of the interface.

Nota: o `README.md` mantém o texto em português (frase 1 com "do VeriCode") até a decisão
D-EN-3. A frase 7 descreve o D9 e não se aplica à tomada pela interface.

**Strings da interface sem inglês** (333 chaves de `worker/static/js/i18n.js`). A
proposta completa das 366 chaves está em
`~/.local/share/vericode-spikes/r-ui/i18n-en-proposal.js` (SHA-256
`e5c1ecbfbcf8f0585fca386c0b586edb825bb26ede29164fba35fa872cb03c64`). Ela foi conferida
contra o PT: mesmas chaves, mesmos `{placeholders}`, mesma contagem de crases; nenhum
termo proibido; `mainnet` só em `claim.f5`.

- claim (6): `claim.f1`, `f2`, `f3`, `f4`, `f5`, `f8`
- limit (3): `limit.p3`, `limit.proving`, `limit.title`
- brand, nav, title, crumb, skip, top (12): `brand.name`, `brand.home`, `nav.label`,
  `title.jobs`, `title.new`, `title.job`, `title.gate`, `title.missing`, `crumb.label`,
  `skip`, `top.signer`, `top.running`
- gate (9): `title`, `help`, `field`, `connect`, `connecting`, `empty`, `refused`,
  `unreachable`, `lost`
- env (25): `label`, `details`, `detailsShort`, `imageIdShort`, `verifierShort`,
  `escrowShort`, `immutable`, `authorityShort`, `close`, `cluster`, `genesis`, `mint`,
  `mintDetail`, `imageId`, `verifier`, `verifierNote`, `escrow`, `authority`,
  `programData`, `bytes`, `sha`, `commit`, `commitNote`, `waiting`, `source`
- data, err, prov, chain (17): `data.stale`, `data.loadingJobs`, `data.loadingJob`,
  `data.loadingHealth`, `err.read`, `err.busy`, `err.pending`, `err.http`, `err.detail`,
  `prov.chain`, `prov.tx`, `prov.worker`, `prov.local`, `prov.fixed`, `chain.reread`,
  `chain.reading`, `chain.readDone`
- jobs (11): `title`, `caption`, `col.job`, `col.task`, `col.state`, `col.amount`,
  `col.deadline`, `task`, `external`, `slot`, `readAt`
- state, scope (15): `state.Draft` … `state.Failed`, `state.none`, `scope.chain`,
  `scope.local`, `scope.worker`, `scope.interface`
- new (37): todas as `new.*`, exceto `new.fund` e `new.funded`
- cli, action (11): `cli.lead`, `cli.template`, `cli.copy`, `cli.none`, `cli.op`,
  `action.copyLabel`, `action.expand`, `action.explorerLabel`, `action.cancel`,
  `action.close`, `action.dismiss`
- job (13): todas as `job.*`
- ops (32): todas as `ops.*`, exceto `ops.refund` e `ops.refundDisabled`
- proving (7): `step`, `stepNone`, `scope`, `final`, `finalDetail`, `live`, `liveDone`
- timeline (9): todas as `timeline.*`
- commit, link (10): `commit.field`, `link`, `notDelivered`, `byDeliver`, `noReceipt`,
  `na`, `note`, `source`, `otherSource`, `link.wait`
- verify (18): todas as `verify.*`, exceto `verify.local` e `verify.onchain`
- settle (32): todas as `settle.*`, exceto `settle.released` e `settle.refunded`
- program, party (9): todas as `program.*`, `party.buyer`, `party.executor`
- adv (20), scenario (18), banner (10), kind (7), missing (2): todas

Fora do dicionário:
- `index.html`: `lang` e o texto do `noscript` ("The Hive interface needs JavaScript.");
- `"Job "` no cabeçalho de `views/job.js`;
- `LOCALE` `en-US`, que muda a formatação de números (`1,000.00`).

Strings do roteiro "Gravação pela interface" que precisam de inglês: todos os nomes de
botão e rótulos citados nas cenas, as saídas esperadas e as falas. A seção 10 já traz
tudo em inglês.

## 9. Condições para o vídeo e correções mínimas

- **C-UI-1 (RUI-01).**
  - Dicionário `EN` completo (366 chaves) a partir da proposta, com as frases ratificadas
    em D-EN-1.
  - `LANG = "en"`, `LOCALE = "en-US"`; `index.html` com `lang="en"` e o `noscript` em
    inglês; o "Job " do cabeçalho pelo dicionário. O português continua no dicionário.
  - `test_static.py` atualizado:
    - `lang`;
    - frases EN iguais às registradas;
    - termos proibidos em inglês;
    - `mainnet` só em `claim.f5`;
    - toda chave PT com EN;
    - paridade de `{…}` e de crases.
  - Nenhum arquivo novo na tabela `STATIC_FILES`.
- **C-UI-2 (RUI-02).** Cenários adversariais só para Jobs vistos desde a partida atual
  do worker. O motivo fica visível nos botões desabilitados:

  ```js
  // load(): o Job já está na lista que a tela lê
  st.firstSeen = (list.jobs.find((item) => item.job_id === jobId) || {}).first_seen || null;
  // adversarial():
  const started = parseTime(health().startup && health().startup.started_at);
  const seen = parseTime(st.firstSeen);
  const thisRun = started !== null && seen !== null && seen >= started;
  // cada motivo: !thisRun ? t("adv.why.thisRun") : <motivo atual>
  ```

  - Chave nova `adv.why.thisRun`: "Disponível só para jobs criados desde a partida atual
    do worker." / "Available only for jobs created since the worker's current start."
  - `st.candidates` do mais recente para o mais antigo, para que o 6014 comece pelo Job
    desta tomada.
- **C-UI-3 (RUI-03).** A liquidação passa a ser a operação aterrissada, não a operação
  "ok". Continuam exigidos o estado terminal da cadeia (bloco D) e `verifier_invoked=true`
  (bloco C):

  ```js
  .filter((op) => (op.kind === "settle" || op.kind === "refund-timeout") && op.outcome === "PASS" && op.status !== "running")
  ```

  Opcional: nota "a leitura de reconciliação desta operação falhou; o estado veio da
  releitura".
- **Recomendado no mesmo gate (não bloqueia):**
  - RUI-04: estado terminal sem operação deste worker diz "Liquidação lida da cadeia; a
    transação não foi registrada por este worker", em vez de "Ainda não…";
  - RUI-05: texto próprio para um create sem Job;
  - RUI-08: "?" vira "Indisponível", e a frase 1 só aparece com link;
  - RUI-09: "impressas pelo `vericode check`";
  - RUI-11: a cor dos nomes de campo no 6014.
- **C-UI-4.** Ensaio completo da seção 10 em inglês sobre os falsos, numa porta ≠ 8710:
  - capturas por cena;
  - PoCs desta revisão repetidas: Job antigo com os negativos desabilitados; reconciliação
    falha com C e D coerentes;
  - `unittest`, `lint:design`, `check:tokens` e `check:assets` verdes;
  - 0 violações de CSP;
  - nenhuma escrita em devnet.
- **C-UI-5 (operação, antes de gravar).**
  - Commit do D12a feito e `git status --short` vazio.
  - Worker **reiniciado depois do último commit**, com o "Commit" da barra igual a
    `git rev-parse --short HEAD`.
  - URL exatamente `http://127.0.0.1:8710/ui/` (`localhost` dá 421), aberta no navegador
    da gravação e testada antes.
  - Perfil do navegador limpo, sem extensões; recusar "salvar senha".
  - Token colado fora da câmera; sem recarregar a página.
  - `MemAvailable` ≥ 3 GiB antes da cena 5.
- **C-UI-6 (orçamento).** Cada tomada completa custa ao buyer 10.769.200 lamports
  (3 rents + 5 taxas) e ao executor 25.000. Com 88.981.200, cabem **no máximo 8
  tomadas**. Mais SOL de devnet (airdrop ou transferência do deployer) exige gate.
- **C-UI-7 (falas).** Só as frases ratificadas (D-EN-1), lidas da tela ou indicadas pelo
  número. Dizer de qual Job é a receipt do 6014. Nunca apresentar uma execução anterior
  como ao vivo.

## 10. Roteiro final do vídeo (tomada contínua, interface em inglês, depois do D12a)

Substitui a seção "Gravação pela interface" de `docs/demo-script.md` no D12a.

**Jobs e frases**
- Jobs novos: **T** (prazo de 1.560 slots), **P** (`PASS`) e **F** (`FAIL`). Nunca
  reutilizar um Job consumido.
- Falas F1 a F8 = frases em inglês da seção 8. O resto da narração só lê a tela.
- Tempos tirados do ensaio do D11–D12.

**Antes de gravar (fora da câmera, ~10 min)**
1. C-UI-5 e C-UI-6 conferidos.
2. Parar o worker em execução pelo PID exato e iniciar com o comando do
   `docs/demo-script.md` (`env -i … python3 -B worker/vericode_worker.py --config …`).
   Esperar `worker.ui=`. O terminal nunca aparece.
3. Abrir `http://127.0.0.1:8710/ui/`, colar o token e clicar em "Connect".
4. Conferir a barra de status: Devnet, Test USDC `9TE2…wV2F`, `image_id`
   `4da06f90…fac0fb1a`, Verifier `THq1…QUge` immutable, Escrow `GZqb…wkCH` authority
   `none`, Commit = HEAD.
5. Em "New job", clicar em "Reread the chain" e anotar `~N min` para 1.560 slots.
   - A ~0,24 s por slot, dá ~6 min.
   - Se der mais de 9 min, fazer a cena 13 depois da 14.
6. Opcional: abas do Explorer do escrow, do verificador e do mint.

**Cenas**

1. **Environment (~30 s).**
   - Jobs → "Details".
   - Mostra: escrow `GZqb…` com upgrade authority `none` e `cdf6967f…`; verificador
     `THq1…` risc0-solana v3.0.0, immutable (`34ae6e5c…`); Test USDC com 6 decimais e
     freeze authority `none`.
   - Fala: **F4**, **F5**. Esc.
2. **Job T (~15 s).**
   - "New job" → deadline `1560` → "Fund job" → ler o diálogo → "Fund job".
   - O resumo mostra `job_id` e o vault; depois "Job funded", a assinatura e "Read from
     chain at slot N".
   - "Open job": `Funded`, 1.00 Test USDC.
   - Mostrar, sem comentar além do texto, "Limitations of this environment" (P3).
3. **Refund before the deadline on T (~20 s)** — logo em seguida: o envio exige pelo
   menos 300 slots de margem.
   - "Adversarial scenarios (demonstration)" → "Request refund before the deadline" →
     confirmar.
   - Resultado: "Transaction rejected: deadline not reached yet", "Rejected by
     `vericode_escrow`: `DeadlineNotReached` (6021)", "The job state was not changed." e o
     link.
4. **Job P (~15 s).**
   - "New job" → `9000` → "Fund job" → confirmar → "Open job".
   - Apontar `spec_hash`, `harness_hash` e `image_id` "Fixed by the program".
5. **Proof of P (~3 min 20 s).**
   - "Input" `21`, "Claimed output" `42` → "Generate proof" → confirmar.
   - Mostra "Generating proof" com o tempo decorrido pelo relógio do worker e a etapa
     `prove` → `compress`; `Proving` (local, off-chain).
   - Fala: **F8**. Se a edição acelerar a espera, rotular "accelerated", sem cortar a
     tomada.
   - Ao fim: "Local verification" (`Composite`, `Groth16`, admitted `image_id`,
     `docker_run`) e "Commitments and observed".
6. **Tampered proof on P (~20 s).**
   - "Submit tampered proof" → confirmar.
   - Resultado: "Transaction rejected: invalid proof", "Rejected by the Groth16 verifier:
     `PairingError` (6003)".
7. **PASS settlement of P (~25 s).**
   - "Submit delivery and proof": o diálogo diz `deliver` + `release` e que o journal diz
     `PASS` → confirmar.
   - Resultado: "Settled: Released".
   - "On-chain verification": `THq1…`, 99,541 CU, `Verify` succeeded, F1 ao lado do link.
   - "Transaction anatomy": Instruction 1 `Deliver`; Instruction 2 `Release` com as CPIs
     `Verify` e SPL Token.
   - "Balances before and after": vault 1.00 → 0.00; executor ATA +1.00.
   - Fala: **F1** apontando o link, depois **F2**.
   - Abrir o Explorer e mostrar `Program THq1… invoke [2]` antes do Token.
8. **Settle P again (~20 s).**
   - "Settle again" → confirmar.
   - Resultado: "Transaction rejected: job already settled", `AlreadyReleased` (6007).
9. **Job F (~15 s).** "New job" → `9000` → "Fund job" → confirmar → "Open job".
10. **Another job's receipt on F (~25 s).**
    - Em "Receipt of job", conferir que o selecionado é o `job_id` de **P** (prefixo
      igual) → "Submit another job's receipt" → confirmar.
    - Fala: diga que a receipt é do Job P, desta mesma gravação.
    - Resultado: "Transaction rejected: binding mismatch", `JournalJobIdMismatch` (6014),
      a tabela com `job_id` em "Mismatch" e "The job state was not changed."
11. **Proof of F (~2 min).**
    - "Input" `7`, "Claimed output" `15` → "Generate proof" → confirmar.
    - Receipt utilizável com `FAIL`.
12. **FAIL settlement of F (~25 s).**
    - "Submit delivery and proof": o diálogo diz `deliver` + `refund_on_fail` e `FAIL` →
      confirmar.
    - Resultado: "Criteria not met (`FAIL`)", "A verified result, not an error";
      verificador invocado, 99,541 CU; 1.00 Test USDC de volta ao comprador.
    - Fala: **F6**, com os resultados dos quatro negativos na tela (links).
13. **Refund of T by deadline (~40 s).**
    - "Jobs" → T (conferir o prefixo) → "Reread the chain" → "Request refund", habilitado
      quando o último `job show` diz que o prazo passou → confirmar.
    - Resultado: "Deadline passed without settlement", `RefundedOnTimeout`; a "On-chain
      verification" diz que essa transação não invocou o verificador.
    - Antes do prazo: "Available from slot N".
14. **Closing (~40 s).** Rolar até "Limits of this proof", ler **F2** e **F3**; terminar
    com **F5** e a lista de Jobs (T, P, F).

Duração bruta: ~13 min. F7 não se aplica a esta tomada.

**Plano B**

| Sintoma | O que fazer |
| --- | --- |
| "Another operation is running" (409) | Esperar. |
| "The worker is waiting for a chain reread" | "Reread the chain" no Job indicado. Com o C-UI-3, C e D continuam coerentes. |
| "No confirmed rejection" | Parar a tomada; não repetir; registrar. |
| "Operational failure: no verdict" | Fechar outras janelas e tentar uma vez; se falhar, parar. |
| Prazo de T ainda não venceu na cena 13 | Fazer a cena 14 e voltar a T, sem cortar. |
| Tela do token no meio da tomada | Colar o token no campo mascarado, sem mostrar o terminal; se o worker reiniciou, a tomada recomeça com Jobs novos (C-UI-6). |
| RPC de devnet fora do ar | Mostrar as tabelas de "Plano B: evidência já executada" de `docs/demo-script.md`, dizendo que são uma execução anterior (D11–D12 ou D9). |

## 11. Decisões humanas pendentes

| Decisão | Recomendação |
| --- | --- |
| **D-EN-1:** ratificar ou ajustar a tradução das 8 frases e do "Do not say" (seção 8) | Ratificar. |
| **D-EN-2:** inglês como padrão da interface, com o português mantido no dicionário e sem seletor | Inglês como padrão. |
| **D-EN-3:** versão em inglês do `README.md` (frases e restante) e a marca na frase 1 do README | O D12a acrescenta só a seção das frases em inglês, para os testes compararem. O resto do README fica para o gate de submissão. |
| **D-NEG:** trava dos cenários adversariais | Por `first_seen` ≥ partida (C-UI-2), em vez de um `data_dir` novo. |
| **D-6014:** receipt do 6014 na tomada | Receipt de P aplicada em F (seção 10), em vez de `bc334093`. |
| **D-RUI-06:** corrigir o `Content-Length` no worker | Deixar fora do D12a (baixo; o worker não muda). |

## 12. Riscos abertos

- RUI-01 a RUI-05 até o D12a.
- RUI-06 a RUI-14.
- P3: chaves de devnet no worker local, o mesmo operador para os dois papéis.
- RD7-08 na CLI (mitigado por C10-8).
- Ritmo de slots variável.
- Orçamento de SOL do buyer (no máximo 8 tomadas).
- Memória do WSL no `compress`.
- Riscos anteriores: sem e-stop, rent preso (17 Jobs), mint authority = deployer, ImageID
  não recertificado, spec v1 trivial.

## 13. Fronteiras

- **Repositório:** nada alterado; `git status` limpo no fim; nenhum `__pycache__`.
- **Devnet:** só leitura (`job show`, `getTransaction`, `getSignaturesForAddress`,
  `getBalance`, `getTokenAccountBalance`, `getSlot`); nenhuma escrita, airdrop ou
  deploy.
- **Rede:** `api.devnet.solana.com` e `127.0.0.1` (8712–8714 de revisão); `npm ci
  --offline`.
- **Chaves:**
  - nenhuma aberta; só as pubkeys de `d10/worker.json`;
  - o token do worker real nunca foi lido;
  - os tokens dos workers de revisão ficaram em `r-ui/tmp` (`0600`).
- **Processos:** workers de revisão parados por PID exato; sem `pkill`.
- **Varredura de segredos** (8.337 arquivos de `r-ui/`):
  - arrays de 64 números só no código do Playwright e nas chaves **falsas** das fixtures
    (bytes determinísticos `(i·37+11) mod 256` dos testes), conferidas e apagadas;
  - nenhuma string base58 de 64 bytes;
  - nenhum caminho de chave em `data/` ou nos logs.

## Artefatos fora do clone

`~/.local/share/vericode-spikes/r-ui/`:

- `poc/`: `rui_rpc.py`, `rui_fake.py`, `rui_fake_reconcile.py`, `rui_http.py`,
  `rui_api.py`, `rui_reconcile_drive.py`, `rui_ui.mjs`, `rui_crop.mjs`,
  `rui_reconcile_ui.mjs`, `en-static/` (cópia com `LANG="en"`);
- `logs/`: `unittest.log`, `devnet-*.json*`, `devnet-show.log`, `poc-*.log`,
  `i18n-keys.txt`, `fake-*.log`;
- `shots/`: `audit-*`, `en-*`, `old6007-*`, `hostile-create`, `u-unexpected`,
  `reconcile-inconsistent`, `pt-p-verify`;
- `i18n-en-proposal.js` (`e5c1ecbf…`);
- `tmp/`: raízes dos workers de revisão, sem chaves.
