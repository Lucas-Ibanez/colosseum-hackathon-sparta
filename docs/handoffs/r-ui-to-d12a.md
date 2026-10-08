# D12a — correções da R-UI para o vídeo em inglês

## Identificação
- Gate: `D12a`  ·  Dia da sequência: D12 (08–09/10)  ·  Marcos do guia: M5/M6 (fluxo pela
  interface), M7 (claims iguais ao código)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `R-UI`, commit `docs: record R-UI review` (sobre `faad06c`), relatório
  `docs/r-ui-review-results.md`, decisões em `docs/decisions.md` (entrada "R-UI").
- Prazo: D12a → vídeo definitivo em inglês numa tomada contínua (10/10) → D13 (README e
  texto de submissão em inglês) → submissão (11/10).

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, high**. O gate mexe em interface, em texto
  público em inglês e na lógica de exibição de cenários que escrevem em devnet, mas não
  toca em programa, escrow, worker nem devnet.
- **Plan Mode obrigatório.** O gate muda claims públicos (frases em inglês) e edita o
  `HIVE_MVP_UI_ADAPTATION.md`. O plano lista arquivo por arquivo o que muda e espera
  aprovação.
- Checkpoint de UI na primeira linha da resposta (bloco `hive-ui-core` do `AGENTS.md`):
  `UI checkpoint: li HIVE_MVP_UI_ADAPTATION.md, HIVE_MVP_UI_GUIDE.md e DESIGN.md [e
  brand/MANIFEST.md]. Regras que mais pesam aqui: ...`.
- Use a skill `frontend-design` só para crítica visual; o `DESIGN.md` vence.

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`, `docs/handoff-protocol.md`,
  `docs/agent-control.md`.
- `HIVE_MVP_UI_ADAPTATION.md`, `HIVE_MVP_UI_GUIDE.md`, `DESIGN.md` (sobretudo "Labels
  and vocabulary", com a coluna English), `brand/MANIFEST.md`.
- `docs/r-ui-review-results.md` inteiro (achados, seção 8 com as frases ratificadas,
  seção 9 com C-UI-1 a C-UI-7, seção 10 com o roteiro).
- `docs/decisions.md`: entradas "Fundação da interface Hive (D11a)", "D11–D12" e
  "R-UI" (decisões D-EN-1, D-EN-2, D-EN-3, D-NEG, D-6014, D-RUI-06 e a autorização de
  edição da adaptação).
- `docs/d11-ui-results.md`, `worker/README.md`, `README.md` (frases, "Não dizer",
  limitações), `docs/demo-script.md` (frases, "Gravação pela interface", "Plano B").
- `worker/static/` inteiro: `index.html`, `css/app.css`, `js/*.js`, `js/views/*.js`.
  O `css/tokens.css` é gerado; só leitura.
- `worker/tests/test_static.py`, `worker/tests/test_worker.py`, `worker/tests/fakes/`.
- Para ler, não para editar: a proposta
  `~/.local/share/vericode-spikes/r-ui/i18n-en-proposal.js` (SHA-256
  `e5c1ecbfbcf8f0585fca386c0b586edb825bb26ede29164fba35fa872cb03c64`) e os PoCs
  `~/.local/share/vericode-spikes/r-ui/poc/` (`rui_fake.py`, `rui_fake_reconcile.py`,
  `rui_api.py`, `rui_reconcile_drive.py`, `rui_ui.mjs`, `rui_reconcile_ui.mjs`).

## Preflight
- `pwd`; raiz Git; branch.
- `git log --oneline -3`: topo = `docs: record R-UI review`, sobre `faad06c`.
- `git status --short` vazio; `--ignored` só `worker/ui-tools/node_modules/` e
  `brand/reference/*.pdf`; `git diff --check` 0.
- Perfil padrão no início e no fim:
  - `~/.rustup`, `~/.cache/solana`, `~/.config/solana` e `~/.npm` ausentes;
  - snapshots: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…`;
  - método: `cd <dir> && find . -printf '%p %s %T@\n' | sort | sha256sum`.
- Raiz própria `~/.local/share/vericode-spikes/d12a/` (`0700`, logs `0600`, TMPDIR
  próprio).
  - Não alterar `d9/`, `d10a/`, `rd10a/`, `rec-*`, `d10/`, `d11/`, `r-ui/`. Para
    reusar um PoC da R-UI, **copie** para `d12a/poc/`.
  - Scripts com prefixo `d12a_`. Nunca use as variáveis `R`, `B`, `D` ou `VC`.
  - Nunca use `pkill -f`: pare processos só pelo PID exato.
- **Worker real:** `127.0.0.1:8710` (dados em `d10/data`), possivelmente em execução.
  Não pare, não reinicie e não chame rotas de escrita. O reinício faz parte da
  preparação da gravação (C-UI-5), não deste gate.
- Memória: 7,6 GiB de RAM. Nenhuma prova real neste gate; um Chromium por vez.

## Checagem da tarefa anterior
- Reexecutar:
  - `TMPDIR=<d12a/tmp> python3 -B -m unittest discover -s worker/tests -v`: **53/53**,
    sem `__pycache__`;
  - `source ~/.local/share/vericode-spikes/ui/env-ui.sh` e depois
    `npm --prefix worker/ui-tools run lint:design` (0 erros, 0 avisos), `check:tokens`
    (`tokens ok`) e `check:assets` (`43 assets match the table`).
- Hashes esperados:
  - worker `bbdf6dbe…`;
  - locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor`
    `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover` `8b76f1e1…`,
    `ui-tools` `0c40c2ab…`;
  - guest `e09ba8cf…`;
  - proposta EN `e5c1ecbf…`.
- Conferir no código as premissas da R-UI:
  - `LANG`/`LOCALE` = `pt-BR` em `js/i18n.js`;
  - `settlementOp()` filtra `status === "ok"` em `js/views/job.js`;
  - `first_seen` em `GET /api/jobs` e `startup.started_at` em `GET /api/health` no
    worker;
  - o 6014 exige só `Funded` e um candidato.
- Se algo divergir: parar, registrar e reportar, sem corrigir em silêncio.

## Objetivo
Deixar a interface Hive pronta para o vídeo definitivo em inglês: cumprir C-UI-1 a
C-UI-3 da R-UI só na interface, nos testes e nos documentos autorizados, ensaiar o
roteiro inteiro em inglês sobre os executáveis falsos (C-UI-4) e deixar o roteiro final
em `docs/demo-script.md`.

## Decisões já tomadas
Todas em `docs/decisions.md`, entrada "R-UI":
- Vídeo e submissão em inglês.
- **D-EN-1 ratificada.** Texto oficial em inglês: as 8 frases e o "Do not say" da seção
  8 de `docs/r-ui-review-results.md`.
  - Frase 1 com "Hive's".
  - O item "name the rejecting program from an `escrow:6000`–`6003` code (RD7-01)" é
    regra de operação, não item da lista congelada.
- **D-EN-2:** inglês como idioma padrão da interface (`LANG = "en"`, `LOCALE =
  "en-US"`). O português continua no dicionário, sem seletor.
- **Edição autorizada do `HIVE_MVP_UI_ADAPTATION.md`, só em dois pontos:**
  - §2, decisão 2: idioma padrão inglês, português mantido no dicionário, frases em
    inglês ratificadas na entrada R-UI;
  - §6: frase 1 em inglês ao lado da portuguesa, e referência à lista em inglês.
- **D-EN-3:** o D12a acrescenta ao `README.md` **só** uma seção "Frozen phrases
  (English, ratified R-UI)" com as 8 frases e o "Do not say" exatos. A frase 1 entra com
  "Hive's". O resto do README, inclusive a frase 1 portuguesa "do VeriCode", fica para o
  D13.
- **D-NEG:** os cenários adversariais só ficam habilitados para Jobs com `first_seen` ≥
  `startup.started_at` da partida atual do worker, com motivo visível (C-UI-2). Sem
  `data_dir` novo.
- **D-6014:** na tomada, a receipt de P vai para F (cena 10). O seletor do 6014 lista do
  Job mais recente para o mais antigo.
- **D-RUI-06:** fora do D12a. `worker/vericode_worker.py` não muda.

## Decisões a confirmar ou pendentes
- Nenhuma prevista.
- Os seguintes casos ficam em **AGUARDANDO_AUTORIZAÇÃO**, sem contornar:
  - o plano precisa mudar `worker/vericode_worker.py`, a tabela `STATIC_FILES`, o
    `DESIGN.md`, o `tokens.css` ou o guia;
  - alguma frase ratificada precisa mudar uma palavra;
  - um rótulo da coluna English do `DESIGN.md` não serve.

## Escopo autorizado
- `worker/static/js/i18n.js`:
  - dicionário `EN` completo, com as mesmas chaves do `PT` e mais `adv.why.thisRun`;
  - `LANG = "en"`, `LOCALE = "en-US"`.
- `worker/static/js/views/*.js` e `worker/static/js/*.js`:
  - C-UI-2 e C-UI-3;
  - o `"Job "` do cabeçalho passa a vir do dicionário;
  - os recomendados da seção 9 (RUI-04, 05, 08, 09, 11) só por chaves do dicionário e
    tokens.
- `worker/static/index.html`: `lang="en"` e o `noscript` em inglês. Nenhum arquivo novo.
- `worker/static/css/app.css`: só o RUI-11, só com variáveis de `tokens.css`.
- `worker/tests/test_static.py`: testes ajustados e novos (ver abaixo).
- `README.md`: só a seção nova das frases em inglês (D-EN-3).
- `docs/demo-script.md`:
  - frases em inglês ao lado das portuguesas na seção "Frases permitidas";
  - a seção "Gravação pela interface" substituída pelo roteiro final em inglês, a partir
    da seção 10 da R-UI, com os textos **exatos** da interface implementada.
- `HIVE_MVP_UI_ADAPTATION.md`: só os dois pontos autorizados.
- Documentos do gate: `docs/d12a-results.md`, `docs/decisions.md`, `docs/evidence.md`,
  `docs/agent-control.md`, `docs/project-context.md`, `docs/handoffs/d12a-to-d13.md`.
- Fora do clone: `~/.local/share/vericode-spikes/d12a/`.

## Fora de escopo / proibido
- `worker/vericode_worker.py`, `worker/README.md` (salvo uma errata de idioma, se
  necessária e registrada), `cli/`, `prover/`, `anchor/`, `crates/`, `zkvm/`, locks,
  `DESIGN.md`, `HIVE_MVP_UI_GUIDE.md`, `brand/`, `worker/ui-tools/` (exceto rodar os
  scripts), `css/tokens.css`.
- Qualquer escrita em devnet; airdrop; chamada de rota de escrita do worker real (8710);
  abrir arquivos de chave (só `pubkeys.txt` e `worker.json`).
- Mudar a tabela `STATIC_FILES` ou acrescentar arquivo estático.
- Traduzir o README inteiro (é o D13); mudar o sentido de qualquer frase; usar
  "Verifier Router", "fallback", "trustless", "audited", "secure", "safe",
  "guaranteed", "certified", "approved" como veredito.
- Push; reescrita de histórico; stash; reset; `pkill -f`; builds pesados.

## Implementação esperada
1. **C-UI-1 (RUI-01):**
   - Partir da proposta `e5c1ecbf…`. Confira cada chave contra o PT: mesmas chaves,
     mesmos `{placeholders}`, mesma contagem de crases.
   - `claim.f1` a `f8` (as que a interface usa: 1, 2, 3, 4, 5 e 8) **iguais** às frases
     ratificadas, com `Hive's` na f1.
   - As chaves da tabela "Labels and vocabulary" do `DESIGN.md` com o texto exato da
     coluna English.
   - Nomes de estado, instrução, erro, programa e comando sem tradução.
   - `LOCALE = "en-US"`: valores `1.00`, CU `99,541`, slots `508,512,386`. Datas no
     formato absoluto do `DESIGN.md`. O sinal `−` (U+2212) continua.
   - Nenhum `≈`, `→` ou `·`.
2. **C-UI-2 (RUI-02):**
   - Em `views/job.js`, ler `first_seen` do Job na lista que a tela já carrega.
   - Calcular `thisRun = first_seen ≥ health().startup.started_at`. Se algum dos dois
     faltar, `thisRun = false`.
   - Os quatro cenários adversariais desabilitados com o motivo `adv.why.thisRun`
     quando `!thisRun`. Os motivos atuais continuam valendo quando `thisRun`.
   - `st.candidates` do `first_seen` mais recente para o mais antigo; o padrão do
     seletor é o primeiro.
   - Isso vale também para o 6007 em Job terminal.
3. **C-UI-3 (RUI-03):**
   - `settlementOp()` passa a aceitar a operação aterrissada:
     `(kind === "settle" || kind === "refund-timeout") && outcome === "PASS" && status !== "running"`.
   - Continuam exigidos:
     - o estado terminal da cadeia para o bloco D;
     - `verifier_invoked=true` para a frase 1 e a verificação on-chain do bloco C;
     - o link do Explorer sempre ao lado da frase 1.
   - Com `status === "failed"`, mostrar a nota de que a leitura de reconciliação
     falhou e o estado veio da releitura.
4. **Recomendados (fazer na ordem, cortar se faltar tempo):**
   - RUI-04: estado terminal sem operação deste worker vira "Settlement read from the
     chain; the transaction was not recorded by this worker", em vez de "Not yet…";
   - RUI-08: "?" vira "Unavailable"/"Indisponível", e a frase 1 só aparece com link;
   - RUI-09: "printed by `vericode check`";
   - RUI-05: texto próprio para um create sem Job;
   - RUI-11: a cor dos nomes de campo no 6014 e o "N slots left" em Job terminal.
5. **README:**
   - seção nova em inglês com as 8 frases e o "Do not say" ratificados, no mesmo
     formato numerado da seção portuguesa;
   - uma linha dizendo que o texto português continua equivalente e que a tradução do
     README inteiro é o D13.
6. **`docs/demo-script.md`:**
   - frases em inglês;
   - roteiro final da R-UI (seção 10), com cada botão, rótulo e saída esperada igual à
     tela implementada e conferido nas capturas do ensaio;
   - preparação C-UI-5 e C-UI-6 e o plano B.
7. **`HIVE_MVP_UI_ADAPTATION.md`:** os dois pontos autorizados, em português, citando a
   entrada R-UI.

## Testes obrigatórios
- `test_static.py`:
  - **Ajustar os testes que comparam frases.** Hoje,
    `re.findall(r'^(\d)\. \*\*[^*]+\*\* "([^"]+)"', readme, …)` e
    `re.findall(r'"claim\.f(\d)": "([^"]+)"', i18n)` não separam idioma. Uma seção em
    inglês no README e um `claim.f1` em inglês sobrescreveriam as chaves em português.
    Faça a leitura por seção (README) e por dicionário (`PT` e `EN`).
  - PT continua igual ao README português, com "da Hive" na f1.
  - EN igual à seção inglesa do README, com "Hive's" na f1.
  - `index.html` com `lang="en"`.
  - Toda chave do PT existe no EN e vice-versa (fora `adv.why.thisRun`, que está nos
    dois); mesma lista de `{placeholders}` e mesma contagem de crases por chave.
  - Termos proibidos em inglês (lista "Do not say" e guia §8.3) ausentes do dicionário
    EN. "Transaction rejected" nos títulos de cenário é permitido.
  - `mainnet`/"mainnet" só em `claim.f5` (PT e EN): exatamente duas linhas.
  - "VeriCode" só no header `X-VeriCode-Token`.
  - Rótulos EN da tabela do `DESIGN.md` iguais.
  - `adv.why.thisRun` presente.
  - Nenhum arquivo novo em `worker/static/` fora da tabela.
- Suítes:
  - `unittest`: 53 + os novos, todos verdes, sem `__pycache__`;
  - `lint:design` 0/0, `check:tokens` e `check:assets` sem deriva.
- **Ensaio C-UI-4 sobre os falsos**, porta ≠ 8710 (por exemplo 8715), raiz em
  `d12a/tmp`, com o código do repositório e os executáveis de `worker/tests/fakes`:
  - roteiro completo da seção 10 em inglês, com capturas por cena em `d12a/shots/`;
  - varredura de palavras portuguesas nas telas Jobs, New job e nos Jobs T, P e F:
    0, fora identificadores técnicos e nomes próprios; registrar a lista;
  - **Job antigo:** criar OLD, liquidar, parar o worker de ensaio por PID e iniciar de
    novo. Em OLD, os quatro cenários ficam desabilitados com o motivo; num Job novo,
    ficam habilitados. O 6014 de F começa por P;
  - **reconciliação falha** (cópia de `rui_fake_reconcile.py`): blocos C e D coerentes
    com a releitura, frase 1 com link;
  - teclado, recarga (volta ao token), 0 violações de CSP, 0 erros de console, modo
    escuro e 900 px sem recorte.
- Nenhuma escrita em devnet. Leituras RPC são opcionais, só para conferir que os saldos
  continuam iguais ao fim da R-UI.

## Evidências exigidas
- Relatório `docs/d12a-results.md`:
  - comandos e saídas reais;
  - diff resumido por arquivo;
  - antes × depois das PoCs RUI-01, 02 e 03;
  - lista das capturas;
  - varredura de palavras portuguesas e de segredos.
- Atualizar `docs/decisions.md` (entrada D12a), `docs/evidence.md`,
  `docs/agent-control.md` e `docs/project-context.md`.

## Critério de pronto
- Interface em inglês por padrão; PT completo no dicionário; frases EN iguais às
  ratificadas e à seção do README.
- RUI-01, 02 e 03 corrigidos e demonstrados no ensaio, antes × depois.
- Roteiro final em `docs/demo-script.md` igual às telas.
- Adaptação editada só nos dois pontos.
- Testes, lint e checks verdes; nenhum arquivo fora do escopo alterado.
- `git diff --check` 0; diff integral revisado; busca de segredos limpa; perfil padrão
  e locks inalterados.

## Condições de parada
- **BLOQUEADO** se:
  - a proposta EN tiver hash diferente ou chaves faltando sem correção óbvia;
  - os testes do D10/D11 quebrarem por motivo fora do escopo.
- **AGUARDANDO_AUTORIZAÇÃO** se:
  - for preciso tocar `worker/vericode_worker.py`, `STATIC_FILES`, `DESIGN.md`,
    `tokens.css` ou o guia;
  - uma frase ratificada precisar mudar;
  - algum passo exigir escrita em devnet.

## Commit
- Não autorizado por este prompt. Ao fim, peça autorização.
- Mensagens sugeridas:
  - `worker: English interface and R-UI fixes (D12a)`, com `worker/static/`, os testes,
    o `README.md` e o `docs/demo-script.md`;
  - `docs: record D12a`, com a adaptação, os relatórios e o handoff.
- Push sempre proibido salvo autorização separada.

## Relatório final
1. Arquivos modificados.
2. Testes e saídas reais.
3. Invariantes: nenhum botão decide; nada sem fonte aparece como real; "verified
   on-chain" só com `verifier_invoked=true` e link; `UNEXPECTED` nunca é rejeição;
   nenhuma chave sai do worker.
4. Decisões pendentes.
5. Riscos.
6. Confirmação de fronteiras: nenhuma escrita em devnet, worker real intocado, perfil e
   locks iguais.
7. **Checklist da gravação para o humano** (C-UI-5, C-UI-6 e C-UI-7, em passos
   curtos): reiniciar o worker real depois do último commit, URL `127.0.0.1`, perfil
   limpo, token fora da câmera, memória, orçamento de 8 tomadas.
8. Prompt da próxima fase (**D13**: `README.md` inteiro e texto de submissão em inglês,
   marca Hive, identificadores legados exatos, link do vídeo a preencher), salvo em
   `docs/handoffs/d12a-to-d13.md` e reproduzido na resposta, segundo
   `docs/handoff-protocol.md`.
