# D13 — README em inglês, texto de submissão, roteiro do pitch e cortes da demo técnica: resultados

Sessão de 2026-10-07 23:10 a 2026-10-08 ~07:55 (-03:00), com pausa durante a noite, Claude
Code (Opus 5.5), conforme `docs/handoffs/d12a-to-d13.md`, com o plano aprovado em Plan Mode.
HEAD de partida `7ada476` (`docs: record D12a`, sobre `ba79080` e `9fa4b3b`). Nenhuma escrita
em devnet; a única leitura de rede foi um `getSignatureStatuses` (só leitura). Artefatos
fora do clone em `~/.local/share/vericode-spikes/d13/` (`0700`).

## Resultado

- **`README.md` inteiro em inglês**, com a marca Hive e os identificadores legados exatos:
  o que está demonstrado (f1 com o link da liquidação PASS de P₃ e a mesma tabela de casos
  em devnet), o que a prova não diz, "Frozen phrases (English, ratified R-UI)" (título, 8
  frases e "Do not say" byte a byte iguais), vídeos (`TODO(video)`), como funciona, escopo,
  versões, reprodução (comandos iguais aos de antes), a interface Hive, limitações,
  estrutura, evidências e licença.
- **`docs/README.pt-BR.md`**: o README português anterior, com aviso no topo, f1 "da Hive"
  (citação e item 1), nome do produto Hive na prosa, links relativos corrigidos e a cópia
  da seção inglesa trocada por um ponteiro para o `README.md`.
- **`docs/submission.md`**: seções genéricas até o `TODO(form)`; cada claim técnico com link
  de devnet ou relatório.
- **`docs/pitch-script.md`**: 8 partes (alvo 2:55), 7 slides, só frases ratificadas como
  claims, visão rotulada "Roadmap (not built yet)".
- **`docs/demo-script.md`**: f1 portuguesa "da Hive", link para `README.pt-BR.md` e a seção
  nova "Demo técnica (corte de 2–3 min)" (regras, 13 cortes com rótulos, narração com
  fontes).
- **Teste:** o PT vem de `docs/README.pt-BR.md` e o EN do `README.md`; igualdade direta da
  f1 "da Hive". `unittest` 58/58.

## Decisões humanas desta sessão (Plan Mode)

- Autorizada a correção de **uma linha** de `worker/README.md` (link das frases
  portuguesas), fora do escopo original.
- `docs/README.pt-BR.md`: f1 "da Hive" e nome do produto Hive também na prosa (título e "Num
  Job da Hive"); identificadores como no código.

## 1. Preflight e checagem da tarefa anterior

- Git: raiz `/home/lucas/src/vericode`, branch `main`, HEAD `7ada476` → `ba79080` →
  `9fa4b3b`; `status --short` vazio; `--ignored` só
  `brand/reference/Hive-posicionamento-identidade-2026-10-05.pdf` e
  `worker/ui-tools/node_modules/`; `diff --check` 0. `origin/main` igual ao HEAD na última
  busca local (sem `gh` para conferir a visibilidade do repositório).
- Perfil padrão, igual no início e no fim: `~/.rustup`, `~/.cache/solana`,
  `~/.config/solana` e `~/.npm` ausentes; `~/.cargo` `d9e12578`, `~/.avm` `7d29f7f8`,
  `~/.docker` `6046f67f`.
- Hashes, iguais no início e no fim: worker `bbdf6dbe`; locks raiz `191802b2`, `zkvm`
  `f5236689`, guest `1116acef`, `anchor` `19a1db26`, `tests-local` `be94760a`, `cli`
  `4d979577`, `prover` `8b76f1e1`, `ui-tools` `0c40c2ab`; guest `e09ba8cf`.
- Código: `LANG = "en"` em `worker/static/js/i18n.js`; `claim.f1` EN ("Hive's…") igual à
  seção inglesa do README.
- Antes de editar: `TMPDIR=d13/tmp python3 -B -m unittest discover -s worker/tests -v`
  **58/58** (35,9 s), sem `__pycache__` (`d13/logs/unittest-before.log`); `lint:design` 0
  erros, 0 avisos; `tokens ok`; `43 assets match the table`
  (`d13/logs/ui-checks-before.log`).
- Referência externa conferida (só leitura): o guia da Colosseum pede pitch ≤ 3 min (time,
  problema, público, validação, visão) e demo técnica de 2–3 min (funcionalidades, stack,
  decisões, integração com Solana).

## 2. Diff por arquivo

| Arquivo | Linhas (+/−) | O quê |
| --- | ---: | --- |
| `README.md` | 134 / 113 | reescrito em inglês (seções acima) |
| `docs/README.pt-BR.md` | novo, 189 | README português anterior, com as mudanças da decisão |
| `docs/submission.md` | novo, 144 | texto de submissão |
| `docs/pitch-script.md` | novo, 119 | roteiro do pitch e slides |
| `docs/demo-script.md` | 70 / 2 | f1 "da Hive", link PT e a seção "Demo técnica (corte de 2–3 min)" |
| `worker/tests/test_static.py` | 10 / 11 | `README_SECTIONS` por arquivo; f1 PT por igualdade direta |
| `worker/README.md` | 1 / 1 | link das frases para `docs/README.pt-BR.md` (autorizado) |
| `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`, `docs/project-context.md` | registros | entrada D13, linha de evidência, controle e estado |
| `docs/handoffs/d13-to-d13b.md` | novo | prompt do próximo gate |

`git diff --quiet HEAD -- worker/static worker/vericode_worker.py cli prover anchor crates
zkvm DESIGN.md HIVE_MVP_UI_GUIDE.md HIVE_MVP_UI_ADAPTATION.md brand Cargo.lock`: exit 0.

## 3. Testes

```text
$ TMPDIR=…/d13/tmp python3 -B -m unittest discover -s worker/tests -v
test_english_claims_are_the_ratified_phrases … ok
test_portuguese_claims_are_the_frozen_phrases … ok
Ran 58 tests in 38.108s
OK
$ npm --prefix worker/ui-tools run lint:design     "errors": 0, "warnings": 0
$ npm --prefix worker/ui-tools run check:tokens    tokens ok: worker/static/css/tokens.css equals the DESIGN.md output
$ npm --prefix worker/ui-tools run check:assets    43 assets match the table
```

Sem `__pycache__` (`d13/logs/unittest-final.log`, `ui-checks-final.log`).

- **Teste novo contra `HEAD`** (`git archive HEAD` em `d13/head`, com o `test_static.py`
  novo): `test_portuguese_claims_are_the_frozen_phrases` dá **ERROR** (o arquivo PT não
  existe). Também falha `test_the_app_commit_is_read_from_git_files`, porque a cópia do
  `git archive` não tem `.git`: artefato do método, não do D13
  (`d13/logs/unittest-new-test-on-head.log`).
- **Mutações** (`d13/bin/mut13.sh`, na cópia, com os arquivos atuais;
  `d13/logs/test-mutations.log`):

  | Mutação | Resultado |
  | --- | --- |
  | nenhuma | OK |
  | título "Frozen phrases (English, ratified R-UI)" removido do `README.md` | FAIL: `README.md must have exactly one '## Frozen phrases (English, ratified R-UI)'` |
  | título "Frases permitidas (congeladas no D9)" removido do PT | FAIL: `docs/README.pt-BR.md must have exactly one …` |
  | item 1 do PT de volta a "do VeriCode" | FAIL em `test_portuguese_claims_are_the_frozen_phrases` |

- **Frases EN:** linhas das 8 frases e do "Do not say" comparadas com a seção anterior
  (`d13/tmp/en-section-old.txt`, `a80be3a0…`): `IDENTICAL`.

## 4. Varreduras (`d13/bin/scan13.py`, `d13/logs/scan-2.log` e `scan-final.log`)

Arquivos: `README.md`, `docs/README.pt-BR.md`, `docs/pitch-script.md`, `docs/submission.md`
e a seção nova do roteiro; links relativos também em `docs/demo-script.md` inteiro,
`worker/README.md` e nos registros deste gate.

```text
== 1. terms: 59 hits, 0 violations
== 2. explorer links: 75, distinct signatures: 29; other URLs: ['http://127.0.0.1:8710/ui/', 'https://github.com/Lucas-Ibanez/colosseum-hackathon-sparta']
== 3. renamed identifiers: none found unless listed below; README legacy identifiers missing: []
== 4. relative links checked: 137
== 5. README commands: 29; equal to HEAD README (comments aside): True; pt-BR equal to HEAD: True; added: ['python3 -B worker/vericode_worker.py --config <worker.json outside the clone>']
== 6. getSignatureStatuses at slot 508664367: 29/29 found
== problems: 0
```

- **Termos:** lista do prompt, "Do not say"/"Não dizer" (EN e PT) e o guia §8.3, mais
  "verified on-chain", "live"/"ao vivo" e "mascot". Os 59 casos permitidos são só: as listas
  "Do not say"/"Não dizer" e a regra de termos do pitch (inclusive as linhas que continuam
  o item); negações ("There is no Verifier Router in the path", "not through the Verifier
  Router", "It does not prove that any code is correct", "no mainnet and no real money");
  a F5 (também quebrada em duas linhas no pitch); a F7 ("not a new machine"); "No mascot";
  e as regras do corte ("Nenhum rótulo diz "live""). Primeira passada: 17 achados, todos
  do próprio script (negação quebrada em linhas, "recertified" casando com "certified",
  `HIVE_MVP_*` como identificador); o script foi corrigido, o texto não precisou mudar.
- **"mainnet":** só na F5 e nas limitações que a negam. **"Verifier Router":** só em
  negação. **"verified on-chain":** em nenhuma fala; só na regra que a proíbe.
- **Explorer:** 75 links, todos `?cluster=devnet`; as 29 assinaturas estão nos
  relatórios `docs/*-results.md`; em devnet, 29/29 `finalized`, com `err=null` nas
  liquidações e nos reembolsos e os códigos esperados nos negativos (6021, 6014, 6003,
  6007, 6008).
- **Identificadores:** nenhuma forma renomeada (`hive-core`, `hive_escrow`,
  `HiveEscrowError`, `X-Hive-Token`, `HIVE_*`, `` `hive job` ``…); o `README.md` cita
  `vericode-core`, `vericode-prover`, `vericode_escrow`, `VericodeEscrowError`,
  `X-VeriCode-Token`, `worker/vericode_worker.py`, `prover/artifacts/vericode-guest.bin` e
  o program ID.
- **Links relativos:** 137, todos com arquivo e âncora existentes (slug no estilo GitHub).
- **Comandos:** os 28 comandos de antes estão iguais no `README.md` e no PT; o único novo é
  a partida do worker, igual à do `worker/README.md`. As opções citadas conferem com
  `cli/README.md`, `prover/README.md` e `worker/README.md`.
- **Segredos no diff** (`d13/logs/secrets.log`): 667 linhas acrescentadas; 0 arrays de 64
  números; 29 strings base58 de 80–90 caracteres, todas assinaturas presentes nos
  relatórios; nenhum marcador (`PRIVATE KEY`, `seed phrase:`, `worker.token=`…).
- `git diff --check`: exit 0.

## 5. Invariantes dos textos públicos

- Só os claims ratificados: F1–F8 citadas como estão; o resto descreve fatos do código e da
  evidência, com a fonte na narração da demo.
- F1 sempre com o link de uma liquidação (README, submissão, slide 4, linha 8 do corte);
  nenhuma fala diz "verified on-chain".
- Receipt do 6014 identificada (Job `bc334093…` em P₃ nas tabelas; Job P da mesma tomada no
  corte). Nenhuma execução anterior apresentada como desta tomada.
- Visão só como "next"/"we plan to", com slide "Roadmap (not built yet)".
- Nada sem fonte: time, validação, modelo de negócio, campos do formulário, slogan e links
  dos vídeos ficam como `TODO(…)`.
- Narração da demo: ~400 palavras em ~2:45 de fala (~2,4 palavras/s); pitch: ~300 palavras
  escritas mais as partes do humano.

## 6. TODO restantes (quem fornece: o humano)

| Marcador | Onde | O que falta |
| --- | --- | --- |
| `TODO(video)` | `README.md` (3), `docs/submission.md` (3), corte da demo (4) | links do pitch, da demo técnica e da tomada contínua; data da tomada |
| `TODO(form)` | `docs/submission.md`, `docs/pitch-script.md` | campos, limites e regras da edição atual |
| `TODO(team)` | `docs/submission.md`, `docs/pitch-script.md` (2) | nomes, papéis e histórico |
| `TODO(validation)` | `docs/submission.md`, `docs/pitch-script.md` (3) | validação real; sem ela, "No user validation yet" |
| `TODO(business)` | `docs/submission.md`, `docs/pitch-script.md` (2) | modelo de negócio, escrito pelo time |
| `TODO(brand)` | `docs/pitch-script.md` (4) | slogan oficial em inglês; SVGs provisórios até o mestre |
| `TODO(repo)` | `docs/submission.md` | confirmar que os jurados abrem o repositório |

## 7. Riscos abertos

- Os comentários de `worker/static/js/i18n.js` (linhas 7 e 14) e a §6 do
  `HIVE_MVP_UI_ADAPTATION.md` ainda descrevem o README português; fora de escopo.
- A seção "Gravação" (vídeo de reserva do D9) mantém a fala "No VeriCode…"; não é usada nos
  vídeos da submissão.
- A lista de cortes é plano: os tempos e o N das acelerações vêm da edição da tomada.
- A visibilidade do repositório não foi conferida (sem `gh`; `TODO(repo)`).
- O worker real parou com o reinício do WSL (seção 8); precisa ser iniciado antes da
  gravação, depois do commit (C-UI-5).
- Os anteriores continuam: P3, RD7-08, sem e-stop, rent preso, mint authority = deployer,
  ImageID não recertificado, regra v1 trivial, orçamento de 8 tomadas, memória do WSL.

## 8. Fronteiras

- Devnet: nenhuma escrita, nenhum airdrop; uma chamada `getSignatureStatuses` (só leitura).
- Worker real (`127.0.0.1:8710`, PID 121845): nenhuma rota chamada e nenhum sinal enviado.
  Durante a pausa da sessão, o WSL reiniciou (`uptime -s`: 2026-10-08 07:43:30) e o worker
  parou com ele; `d10/data` não muda desde 2026-10-07.
- O índice do Git recebeu `git add -N` dos três arquivos novos só para a varredura do diff,
  desfeito em seguida com `git restore --staged` (árvore de trabalho intocada).
- Perfil padrão e locks iguais no início e no fim; nenhum arquivo fora do escopo aprovado
  alterado.
- Sem commit e sem push: aguardam autorização do humano.

## Artefatos fora do clone

`~/.local/share/vericode-spikes/d13/` (`0700`):
- `bin/`: `scan13.py` (varreduras), `mut13.sh` (mutações do teste);
- `logs/`: `timeline.log`, `unittest-{before,after,final}.log`,
  `unittest-new-test-on-head.log`, `test-mutations.log`, `ui-checks-{before,final}.log`,
  `scan-{1,2,final}.log`, `secrets.log`, `diff-work.patch`;
- `tmp/` (TMPDIR, `en-section-old.txt`); `head/` (cópia de `HEAD` para o antes × depois).
