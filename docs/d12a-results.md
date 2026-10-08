# D12a — correções da R-UI para o vídeo em inglês: resultados

Sessão de 2026-10-07, 21:00–22:00 (-03:00), Claude Code (Opus 5.5), conforme
`docs/handoffs/r-ui-to-d12a.md`, com o plano aprovado em Plan Mode. HEAD de partida
`9fa4b3b` (`docs: record R-UI review`, sobre `faad06c`). Nenhuma escrita em devnet e
nenhuma leitura RPC; o worker real (`127.0.0.1:8710`, PID 121845) não foi tocado.
Artefatos fora do clone em `~/.local/share/vericode-spikes/d12a/` (`0700`, tokens e
scripts `0600`).

## Resultado

- **C-UI-1 (RUI-01):** interface em inglês por padrão (`LANG = "en"`, `LOCALE =
  "en-US"`, `<html lang="en">`, `noscript` em inglês); dicionário `EN` completo, com as
  mesmas 371 chaves do `PT`; frases f1–f5 e f8 iguais às ratificadas (D-EN-1, f1 com
  "Hive's"); rótulos da tabela "Labels and vocabulary" do `DESIGN.md` iguais nas duas
  colunas. Seção "Frozen phrases (English, ratified R-UI)" no `README.md` (D-EN-3).
- **C-UI-2 (RUI-02, D-NEG, D-6014):** os quatro cenários adversariais só habilitam em Jobs
  com `first_seen` ≥ `startup.started_at` da partida atual do worker, com o motivo "Available
  only for jobs created since the worker's current start."; o seletor do 6014 lista do Job
  mais recente para o mais antigo.
- **C-UI-3 (RUI-03):** a liquidação exibida é a aterrissada (`outcome=PASS`, operação
  não `running`), também com a reconciliação falha; bloco D continua exigindo estado
  terminal da cadeia, e o bloco C, `verifier_invoked=true`; nota "The reconciliation read
  of this operation failed; the state came from the chain reread."
- **Recomendados:** RUI-04, 05, 08, 09 e 11 feitos (seção 3).
- **Ensaio C-UI-4** completo sobre os falsos, em inglês, com Job antigo e reinício: 52
  capturas, 0 palavras portuguesas, 0 violações de CSP, 0 erros de console.
- **Roteiro final** em `docs/demo-script.md` ("Gravação pela interface"), conferido contra
  as telas.
- **Desvio de escopo (registrado e aceito pelo humano):** uma regra de CSS fora do
  RUI-11 (seção 4).

## 1. Preflight e checagem da tarefa anterior

- Git: raiz `/home/lucas/src/vericode`, branch `main`, HEAD `9fa4b3b` sobre `faad06c`;
  `git status --short` vazio; `--ignored` só `brand/reference/Hive-posicionamento-identidade-2026-10-05.pdf`
  e `worker/ui-tools/node_modules/`; `git diff --check` 0.
- Perfil padrão, iguais no início e no fim: `~/.rustup`, `~/.cache/solana`,
  `~/.config/solana` e `~/.npm` ausentes; `~/.cargo` `d9e12578`, `~/.avm` `7d29f7f8`,
  `~/.docker` `6046f67f`.
- Hashes, iguais no início e no fim: worker `bbdf6dbe`; locks raiz `191802b2`, `zkvm`
  `f5236689`, guest `1116acef`, `anchor` `19a1db26`, `tests-local` `be94760a`, `cli`
  `4d979577`, `prover` `8b76f1e1`, `ui-tools` `0c40c2ab`; guest `e09ba8cf`; proposta EN
  `e5c1ecbfbcf8f0585fca386c0b586edb825bb26ede29164fba35fa872cb03c64`.
- Antes de editar: `TMPDIR=d12a/tmp python3 -B -m unittest discover -s worker/tests -v`
  **53/53** (35,1 s), sem `__pycache__`; `lint:design` 0 erros, 0 avisos; `tokens ok`;
  `43 assets match the table`.
- Premissas da R-UI conferidas no código: `LANG`/`LOCALE` `pt-BR`; `settlementOp()` com
  `status === "ok"`; `first_seen` em `GET /api/jobs` e `startup.started_at` em
  `GET /api/health` (ambos `now_iso()`, segundos com fuso); o 6014 exige só `Funded` e um
  candidato.

## 2. Diff por arquivo

| Arquivo | Linhas (+/−) | O quê |
| --- | ---: | --- |
| `worker/static/js/i18n.js` | 399 / 26 | `LANG`/`LOCALE` em inglês; bloco `EN` da proposta `e5c1ecbf…`; chaves novas nos dois dicionários: `adv.why.thisRun`, `job.title`, `settle.external`, `settle.reconcileFailed`, `banner.FailedCreate.detail`; RUI-09 no PT ("impressas"); `limit.proving` EN polido ("The proof (`Proving`) is…") |
| `worker/static/js/views/job.js` | 63 / 23 | C-UI-2, C-UI-3, RUI-04, 05, 08, 11 e `job.title` |
| `worker/static/js/views/new-job.js`, `main.js`, `components.js` | 9 / 9 | RUI-08: todo `"?"` de reserva vira "Unavailable" |
| `worker/static/index.html` | 2 / 2 | `lang="en"`, `noscript` em inglês |
| `worker/static/css/app.css` | 11 / 0 | RUI-11 (cor dos nomes de campo na tabela do 6014) e a quebra de rótulo dos botões (seção 4) |
| `worker/tests/test_static.py` | 115 / 8 | leitura do README por seção e dos dicionários por bloco; 5 testes novos e 3 ajustados |
| `README.md` | 15 / 0 | só a seção "Frozen phrases (English, ratified R-UI)" |
| `docs/demo-script.md` | 223 / 147 | frases em inglês; "Gravação pela interface" substituída pelo roteiro final; numeração das cenas no Plano B |
| `HIVE_MVP_UI_ADAPTATION.md` | 7 / 2 | só §2 (decisão 2) e §6 (frase 1 em inglês e referência à lista) |

Nenhum arquivo novo em `worker/static/` (a tabela `STATIC_FILES` não mudou);
`worker/vericode_worker.py`, `worker/README.md`, `DESIGN.md`, `HIVE_MVP_UI_GUIDE.md`,
`tokens.css`, `brand/`, `worker/ui-tools/`, `cli/`, `prover/`, `anchor/`, `crates/`,
`zkvm/` e os locks: inalterados (`git diff --quiet HEAD -- …` exit 0).

## 3. Correções

- **C-UI-2** (`views/job.js`): `load()` guarda `st.firstSeen` do Job a partir da lista que
  já lia e ordena `st.candidates` por `first_seen` decrescente. `adversarial()` calcula
  `thisRun = started !== null && seen !== null && seen >= started`; sem os dois tempos,
  `thisRun = false`. Fora da partida atual, os quatro motivos viram `adv.why.thisRun`
  (inclusive o 6007 em Job terminal); dentro, os motivos de antes.
- **C-UI-3**: `settlementOp()` filtra `(kind === "settle" || kind === "refund-timeout") &&
  outcome === "PASS" && status !== "running"`. `reconcileNote()` mostra
  `settle.reconcileFailed` em C e D quando a operação ficou `failed` e a cadeia já é
  terminal.
- **RUI-04**: Job terminal sem operação de liquidação deste worker mostra em C (só
  `Released`/`RefundedOnFail`) e em D "Settlement read from the chain; the transaction was
  not recorded by this worker.", nunca "Not yet…" nem a f1.
- **RUI-05**: `Failed` com `chain.absent` usa `banner.FailedCreate.detail` ("The create did
  not end with the job on the chain…").
- **RUI-08**: o banner "Proof verified on-chain…" e a f1 só aparecem com o link do
  Explorer válido; "Result" só com o nome da instrução no log, senão "Unavailable".
- **RUI-09**: PT "impressas pelo `vericode check`" (o EN já dizia "printed by").
- **RUI-11**: `.banner .table-wrap { color: var(--hive-text-primary) }`; o cabeçalho não
  mostra "N slots left" em Job terminal.

## 4. Desvio de escopo: quebra de rótulo dos botões

O ensaio mostrou que, em inglês, "Request refund before the deadline" e "Submit another
job's receipt" transbordavam a coluna do grid de 4 colunas e se sobrepunham
(`d12a/shots-run1/01-old-locked.png`); o português cabia. O prompt autorizava no
`app.css` só o RUI-11. Apliquei uma regra mínima, só com tokens, que segue o `DESIGN.md`
("não reduza a fonte: quebre, amplie o componente"):

```css
.op-group .button { height: auto; min-height: var(--hive-comp-button-primary-height);
  padding-block: var(--hive-spacing-sm); white-space: normal; text-align: start; }
```

Depois dela, o ensaio inteiro foi repetido numa raiz nova (`d12a/shots/01-old-locked.png`,
sem sobreposição). Reverter é apagar a regra; a alternativa seria encurtar os rótulos no
dicionário. **Decisão humana (2026-10-07): regra aceita** (`docs/decisions.md`,
"Decisões humanas depois do D12a").

## 5. Testes

- `TMPDIR=d12a/tmp python3 -B -m unittest discover -s worker/tests -v`: **58/58** (35,0 s),
  sem `__pycache__` (`d12a/logs/unittest-after.log`).
- Testes novos ou ajustados em `test_static.py`:
  - `test_portuguese_claims_are_the_frozen_phrases_with_the_brand_change_only` (README PT
    por seção, dicionário PT por bloco);
  - `test_english_claims_are_the_ratified_phrases` (seção EN do README, f1 "Hive's");
  - `test_english_is_the_default_and_both_dictionaries_have_the_same_keys` (`LANG`,
    `LOCALE`, chaves iguais, `{placeholders}` e crases por chave, `adv.why.thisRun`);
  - `test_design_md_labels_in_both_languages` (as duas colunas, com o mapa das chaves que
    o código nomeia diferente; `verify.fallback` ausente);
  - `test_no_forbidden_english_words` ("Do not say" e guia §8.3; "rejected" só em
    `scenario.*`, `adv.expect` e `adv.lead`, que nomeiam a rejeição do programa);
  - `test_r_ui_fixes_stay_in_the_job_view` (filtro do C-UI-3 e trava do C-UI-2);
  - `test_mainnet_appears_only_in_the_frozen_phrase_5` (exatamente 2 linhas, PT e EN);
  - `test_brand_and_legacy_names` (`lang="en"`, `noscript`; "VeriCode" só no header).
- **Antes × depois dos testes:** o `test_static.py` novo contra uma cópia de `HEAD`
  (`git archive`, fora do clone): **5 falhas** (`brand_and_legacy_names`,
  `english_claims…`, `english_is_the_default…`, `mainnet…`, `r_ui_fixes…`); agora 0
  (`d12a/logs/unittest-new-tests-on-head.log`).
- `lint:design` 0 erros, 0 avisos; `tokens ok`; `43 assets match the table`
  (`d12a/logs/ui-checks-after.log`).
- `git diff --check` exit 0.

## 6. Ensaio C-UI-4 (falsos, porta 8715)

`d12a/poc/d12a_fake.py` (cópia adaptada de `r-ui/poc/rui_fake.py`): código do
repositório sobre `worker/tests/fakes`, raiz em `d12a/tmp`, com a sequência de respostas
da tomada e logs no formato real (anatomia `Deliver`/`Release`/`Verify`/`TransferChecked`
e saldos). `--resume` reusa a mesma raiz (dados do worker e contadores dos falsos).

1. Partida (21:20:34); Job OLD criado, provado e liquidado pela API (`d12a_api.py old`):
   `create ok PASS`, `prove ok`, `settle ok PASS`.
2. Worker parado pelo PID exato e reiniciado com `--resume` (partida 21:20:48; OLD visto às
   21:20:38).
3. Tomada inteira por cliques (`d12a_take.mjs`, Playwright de `worker/ui-tools`, Node
   isolado, um Chromium): conexão, cenas 1 a 14 e verificações. `d12a/logs/take-final.log`.

| Verificação | Resultado |
| --- | --- |
| `html lang`; CSP da página | `en`; `default-src 'none'; script-src 'self'; …` |
| OLD depois do reinício | os quatro cenários desabilitados com "Available only for jobs created since the worker's current start." |
| T e P (Jobs novos) | 6021 e 6014 habilitados; 6003 e 6007 com os motivos de antes |
| Seletor do 6014 em F | padrão = `job_id` de P (`29dc8b85…`); ordem P, OLD |
| 6021, 6003, 6007, 6014 | "Transaction rejected: …" com "Rejected by …" e "The job state was not changed." |
| PASS de P | "Settled: Released"; C: "Proof verified on-chain by the Groth16 verifier (direct CPI).", 99,541 CU, "`Verify` succeeded", f1 com "View on Explorer"; D: anatomia e saldos (1.00 → 0.00; +1.00) |
| FAIL de F | "Criteria not met (`FAIL`)", "A verified result, not an error…", f1 com link |
| Refund de T | antes: "Available from slot 2,561. Reread the chain to check the current slot."; depois: "Deadline passed without settlement", C "…did not invoke the verifier (`verifier_invoked=false`)." |
| Palavras portuguesas (Jobs, New job, OLD, T, P, F, Jobs no fim) | **0** em todas (regex de diacríticos e de 40 palavras PT; identificadores e nomes próprios não casam) |
| Violações de CSP; erros de console | 0; 0 |
| Teclado | 30 paradas em ordem visual, contorno sólido de 2 px `rgb(57, 45, 64)` |
| Modo escuro (P, F) | capturas `x-P-dark`, `x-F-dark`; banners com borda da família |
| 900 px (Jobs, New job, P) | estouro horizontal 0 px |
| Recarga | volta à tela do token; armazenamento 0 |

**Capturas** (`d12a/shots/`, 51 PNG + `texts.json` com os textos de cada cena):
`00-gate`, `00-jobs-connected`, `01-old-locked`, `s00-new-job-reread`, `s01-environment`,
`s02-T-{dialog,funded,open}`, `s03-T-6021-{dialog,result,result-full}`,
`s04-P-{dialog,funded,open}`, `s05-P-prove-{dialog,proving}`, `s05-P-local`,
`s06-P-6003-*`, `s07-P-{settle-dialog,released,onchain,anatomy}`, `s08-P-6007-*`,
`s09-F-{dialog,funded}`, `s10-F-6014-*`, `s11-F-prove-*`, `s12-F-{settle-dialog,refunded}`,
`s13-T-{before-reread,refund-dialog,refunded}`, `s14-{limits,jobs}`,
`x-{P,F}-dark`, `x-900-{jobs,new,P}`, `x-P-keyboard`, `old-{before,after}`,
`reconcile-{before,after}`. A primeira passada (antes da correção de CSS) está em
`d12a/shots-run1/`.

## 7. Antes × depois das PoCs da R-UI

Os estáticos de `HEAD` foram copiados com `git archive` para `d12a/poc/before/` e servidos
pelo mesmo worker de ensaio (`--static`), sem mexer no clone.

| PoC | Antes (`HEAD`) | Depois |
| --- | --- | --- |
| RUI-01, página do Job OLD | `lang` `pt-BR`; 266 ocorrências de palavras portuguesas | `lang` `en`; 0 |
| RUI-02, OLD depois do reinício | 6021/6014/6003 desabilitados por estado; **"Liquidar de novo" habilitado** | os quatro desabilitados com o motivo `adv.why.thisRun` |
| RUI-03, reconciliação falha (`d12a_fake_reconcile.py`, 8718 × 8716): `settle` `failed`, `outcome=PASS`, `verifier_invoked=true`, 99.541 CU; releitura `Released` | banner sem link; C "Ainda não verificada on-chain"; D "Ainda não liquidado"; f1 ausente; "faltam 8.500 slots" | banner com "View on Explorer"; C "Proof verified on-chain…" com f1 e link; D com a nota da reconciliação e a anatomia; sem "slots left" |

Logs: `d12a/logs/old-{before,after}-ui.log`, `reconcile-{before,after}-{api,ui}.log`.

## 8. Varreduras

- **Português na interface em inglês:** 0 (seção 6). Comentários de código em português
  (ex.: "Botões") não são exibidos.
- **Segredos:**
  - no diff: as 5 strings base58 de 86–88 caracteres são assinaturas públicas da tabela do
    Plano B (`2gB9djwM`, `2jjiYDAo`, `5CBFajsa`, `j2pmqFWu`, `kTQdVojx`); nenhum array de
    64 números, caminho de chave, `PRIVATE KEY` ou seed;
  - em `d12a/` (332 arquivos): os únicos arrays de 64 números eram as 12 chaves **falsas**
    das raízes do `Fixture` (bytes determinísticos `(i·37+11) mod 256`), conferidas e
    apagadas; as strings base58 longas são assinaturas falsas dos executáveis de teste;
  - tokens dos workers de ensaio só em `d12a/tmp/fake-token-*` (`0600`); o token do worker
    real nunca foi lido.

## 9. Riscos abertos

- O ensaio usa os executáveis falsos: nomes de erro de Anchor (`DeadlineNotReached`,
  `PairingError`, `AlreadyReleased`, `JournalJobIdMismatch`) e os saldos reais vêm do
  D11–D12; nos falsos, a tela mostra "Rejected by …: code N".
- A trava D-NEG depende do relógio do worker: um Job criado no mesmo segundo da partida
  conta como desta partida (comparação ≥, resolução de 1 s).
- Num Job novo, o 6014 continua habilitado com a receipt de um Job antigo, se for a única
  (a escrita é no Job novo; a tomada usa a de P em F).
- A f1 do `README.md` em português ainda diz "do VeriCode" (D13).
- RUI-06, RUI-07, RUI-10, RUI-12 a 14; P3; RD7-08; ritmo de slots; orçamento de SOL do
  buyer (no máximo 8 tomadas); memória do WSL no `compress`; riscos anteriores (sem e-stop,
  rent preso, mint authority = deployer, ImageID não recertificado, spec v1 trivial).

## 10. Fronteiras

- Devnet: nenhuma escrita, nenhuma leitura RPC, nenhum airdrop.
- Worker real em 8710 (PID 121845): não parado, não reiniciado, nenhuma rota chamada.
- Workers de ensaio (8715–8719) parados pelo PID exato; sem `pkill`.
- Perfil padrão e locks iguais no início e no fim.
- Commits autorizados pelo humano depois do relatório: `ba79080` (`worker: English
  interface and R-UI fixes (D12a)`) e `docs: record D12a`. Sem push.
