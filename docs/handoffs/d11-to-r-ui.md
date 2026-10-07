# R-UI — revisão adversarial do worker (D10) e da interface Hive (D11–D12)

## Identificação
- Gate: `R-UI`  ·  Dia da sequência: D11–D12 (revisão; 09/10)  ·  Marcos do guia:
  M5/M6 (fluxo pela interface), M7 (claims iguais ao código); guia de produto §11
  (revisão adversarial)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D11–D12`, commits `37d2d63` (`worker: add Hive interface (D11-D12)`)
  e, por cima dele, `docs: record D11-D12 interface`; relatório
  `docs/d11-ui-results.md`. Inclui o worker do D10 (`6eba814`, `23a91b5`), que ainda
  não teve revisão própria depois de entrar em uso.
- Prazo: revisão 09/10 → vídeo definitivo numa tomada contínua (10/10) → submissão
  (11/10).

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço max**: auditoria adversarial
  final de um marco com superfície HTTP, assinatura de transações em devnet e claims
  públicos (`docs/handoff-protocol.md`).
- **Sessão separada e somente leitura:** nenhum arquivo do repositório é criado,
  alterado ou apagado; nenhuma escrita em devnet; sem commit. A resposta traz o texto
  de registro pronto para uma sessão com escrita.
- Plan Mode: opcional (sem alteração); se quiser executar algo além de leitura
  local, testes offline e RPC só de leitura, pare e peça autorização.
- Checkpoint de UI (bloco `hive-ui-core` do `AGENTS.md`): a primeira linha da resposta
  é `UI checkpoint: li HIVE_MVP_UI_ADAPTATION.md, HIVE_MVP_UI_GUIDE.md e DESIGN.md [e
  brand/MANIFEST.md]. Regras que mais pesam aqui: ...`.

## Leitura obrigatória (integral, antes de revisar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`, `docs/handoff-protocol.md`,
  `docs/agent-control.md`.
- `HIVE_MVP_UI_ADAPTATION.md`, `HIVE_MVP_UI_GUIDE.md`, `DESIGN.md`, `brand/MANIFEST.md`.
- `docs/decisions.md`: "Decisões humanas para o D10", D10, "Vídeo de reserva",
  "Fundação da interface Hive (D11a)" e "D11–D12".
- `docs/r-d10a-review-results.md` (C10-1 a C10-10), `docs/d10-worker-results.md`,
  `docs/d11-ui-results.md`.
- `worker/README.md`, `worker/vericode_worker.py`, `worker/tests/test_worker.py`,
  `worker/tests/test_static.py`, `worker/tests/fakes/`, `worker/tests/fixtures/d10-tx/`.
- `worker/static/` inteiro (`index.html`, `css/app.css`, `css/tokens.css`, `js/*.js`,
  `js/views/*.js`) e `worker/ui-tools/` (`README.md`, `gen-tokens.mjs`,
  `sync-assets.mjs`, `static-assets.json`, `capture.mjs`, `package.json`).
- `README.md` (frases congeladas, "Não dizer", "Limitações", tabela de devnet),
  `docs/demo-script.md` (sobretudo "Gravação pela interface"),
  `docs/escrow-program.md` (erros), `docs/manifest-schema.md` (`JournalV1`).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3` (topo = `docs: record D11-D12
  interface`, sobre `37d2d63`); `git status --short` vazio e `--ignored` só
  `worker/ui-tools/node_modules/` e `brand/reference/*.pdf`; `git diff --check` 0.
- Perfil padrão no início e no fim: `~/.rustup`, `~/.cache/solana`,
  `~/.config/solana`, `~/.npm` ausentes; `cd <dir> && find . -printf '%p %s %T@\n' |
  sort | sha256sum`: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker`
  `6046f67f…`.
- Raiz própria `~/.local/share/vericode-spikes/r-ui/` (`0700`, logs `0600`, TMPDIR
  próprio). Não alterar `d9/`, `d10a/`, `rd10a/`, `rec-*`, `d10/`, `d11/`. Nomes de
  script prefixados; nunca `R`, `B`, `D`, `VC`; não usar `pkill -f` (casou com o
  próprio shell no D11–D12): parar processo só por PID exato.
- Memória: 7,6 GiB de RAM; um processo pesado por vez.

## Checagem da tarefa anterior
- `TMPDIR=<r-ui/tmp> python3 -B -m unittest discover -s worker/tests -v` → **53/53**,
  sem `__pycache__` depois.
- `source ~/.local/share/vericode-spikes/ui/env-ui.sh`; `npm ci --ignore-scripts
  --prefix worker/ui-tools` (96 pacotes); `npm --prefix worker/ui-tools run
  lint:design` (0/0), `check:tokens` e `check:assets` (43) sem deriva.
- Hashes: `worker/vericode_worker.py` `bbdf6dbe…`; locks raiz `191802b2…`, `zkvm`
  `f5236689…`, guest `1116acef…`, `anchor` `19a1db26…`, `tests-local` `be94760a…`,
  `cli` `4d979577…`, `prover` `8b76f1e1…`, `worker/ui-tools` `0c40c2ab…`; guest
  `e09ba8cf…`; CLI do D10a `e6cd4e29…`, prover `3f66e1c0…`; shim `2a8f75b8…` `0755`.
- Devnet, só leitura: `vericode job show` (CLI do D10a, por fora do worker, ou
  `POST /api/jobs/<id>/show` de um worker iniciado pela revisão) de T₃
  `2c38ca665ec4e8fbfa4674ad3cd744bdc1d52e9ebbe429df2e3ffb37f8c1015b`
  (`RefundedOnTimeout`), P₃
  `26df9ef48e5773d4b48dbb2aff1a3419763efcc5e09982900c1df61ab3ff3618` (`Released`) e F₃
  `7395a76d83379e93b30982d5e2a684451cf868e32424123da9cc0759dbede571`
  (`RefundedOnFail`); `getTransaction` das 10 assinaturas de U1–U10
  (`docs/d11-ui-results.md`, seção 4).
- Saldos esperados (fim do D11–D12, slot 508.515.193): SOL deployer 2.805.194.240,
  buyer 88.981.200, executor 29.885.000 lamports; ATA do buyer 999.992.000.000; ATA do
  executor 8.000.000. Divergência: `getSignaturesForAddress` desde `5oTbFiTX…` (buyer),
  `4jcugMyX…` (executor) e o último do deployer; escrita não explicada → parar e
  reportar.

## Objetivo
Revisar, de forma adversarial e somente leitura, o worker (D10) e a interface Hive
(D11–D12) juntos, e entregar o veredito para o vídeo definitivo, com o texto de
registro pronto e o roteiro final do vídeo.

## Decisões já tomadas
- P1–P6 do D10, C10-1 a C10-10 do R-D10a — `docs/decisions.md`,
  `docs/r-d10a-review-results.md`.
- Fundação da interface Hive (D11a): adaptação com precedência sobre o guia; frase 1
  com "da Hive" só na interface; fontes auto-hospedadas; cópia verificada de assets;
  `worker/static/`; execução sem dependência; ferramentas npm só de desenvolvimento;
  português padrão — `docs/decisions.md`.
- D11–D12: `/ui/` em vez de `/`; token só na memória da aba; lacunas 1–6 como visões
  só de leitura; tokens em quatro camadas; ícones inseridos como SVG — `docs/decisions.md`.
- Sem Verifier Router nem fallback no MVP; claim único (CD7) e as 8 frases
  congeladas; Jobs consumidos nunca reutilizados (inclusive T₃, P₃, F₃).

## Decisões a confirmar ou pendentes
- Nenhuma escrita em devnet nesta revisão. Se uma PoC exigir escrita, pare:
  `AGUARDANDO_AUTORIZAÇÃO`.
- Idioma do vídeo: português (padrão). Inglês exigiria tradução registrada das frases
  congeladas — decisão humana, fora desta revisão.

## Escopo autorizado
- Leitura do repositório e de `~/.local/share/vericode-spikes/d10/` e `d11/` (sem
  alterar). Testes offline. Um worker de revisão em outra porta (≥ 1024, por exemplo
  8712) sobre os **falsos** dos testes, numa raiz própria, para PoCs HTTP e de
  navegador; o worker real (`d10/worker.json`) só se necessário, só leitura (`GET`,
  `POST …/show`, `POST /api/check`).
- Playwright de `worker/ui-tools/node_modules` (Chromium headless) para checar CSP,
  DOM, teclado e capturas; fora do clone.
- Rede: `api.devnet.solana.com` só leitura; `127.0.0.1`.

## Fora de escopo / proibido
- Editar, criar ou apagar arquivo do repositório; commit; push.
- Escrita, airdrop ou deploy em devnet; mainnet; Docker pull; `compress` real.
- Abrir arquivo de chave (só `d4/keys/pubkeys.txt`); imprimir token, keypair, seed ou
  caminho de chave.
- `npm install` novo; rebuild de binários.

## Implementação esperada
Não se aplica (revisão somente leitura). Checklist adversarial mínimo:
- **HTTP do worker:** bind e `Host` (DNS rebinding), token em toda rota `/api/*`,
  CORS ausente, CSP e cabeçalhos das páginas e da API, rotas estáticas (tabela,
  `..`, `%2e%2e`, `//`, maiúsculas, query, métodos, caminho reescrito pela stdlib),
  limites de corpo, 409, `reconcile_pending`.
- **Assinatura e envio:** argv fixo por ação, ambiente do zero, shim conferido antes
  do `compress` e a linha `docker_run` depois, uma operação por vez, reconciliação, nenhum
  reenvio automático; o cliente HTTP nunca fornece caminho, flag, programa ou URL.
- **Interface:** DOM só com `textContent` (inclusive `rich`, `hashField`, diálogos,
  `DOMParser` dos ícones), links só para o Explorer de devnet, token só em memória e no
  header, nenhum dado sem fonte, nenhum botão que decida veredito ou destino de token,
  o 6014 sempre identificando a receipt, `rejection` como única fonte do programa que
  rejeitou (C10-7) e `UNEXPECTED` nunca como rejeição, `Proving` rotulado como local.
- **Visões novas:** `decode_journal`, `invocations_of`, `balances_of`, `v1_facts`,
  `slot_clock`, `read_app_commit` (entradas hostis, logs truncados ou forjados).
- **Chaves:** nenhuma resposta, log, página ou arquivo versionado com caminho ou
  conteúdo de chave; fixtures `d10-tx` só com dados públicos.
- **Claims e frases:** as 8 frases (frase 1 com "da Hive"), "Não dizer", microcopy da
  interface que possa dizer mais que as frases, limitação P3 visível, "Verifier
  Router" e "fallback" ausentes.
- **Identidade:** fidelidade ao `DESIGN.md` (tokens, âmbar nunca como resultado, estado
  nunca só por cor, foco, contraste no claro, PartyMark idêntico, sem halo/selo/favos/
  mascote) e à adaptação (estados, compromissos × journal, verificação, anatomia,
  cenários, faixa de ambiente).
- **Roteiro "Gravação pela interface":** factível numa tomada contínua; nenhuma fala
  além das frases; riscos de operador (a seção adversarial aceita Jobs antigos;
  `app_commit` exige reiniciar o worker depois do commit; token fora da câmera).

## Testes obrigatórios
- Os 53 `unittest` e as checagens de `worker/ui-tools`.
- PoCs adversariais contra o worker de revisão sobre os falsos (HTTP e navegador),
  registrando comando e saída.
- Devnet só leitura (seção "Checagem").
- Comandos: com TMPDIR e homes em `r-ui/`, sem tocar o perfil padrão.

## Evidências exigidas
- Resposta com: achados (crítico/alto/médio/baixo/info) com PoC e saída real;
  veredito para o vídeo (APROVADO, APROVADO COM RESSALVAS ou REPROVADO) e as
  condições; o **texto de registro pronto** (`docs/r-ui-review-results.md` e as
  entradas de `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`,
  `docs/project-context.md`); o **roteiro final do vídeo** (cenas, cliques, falas só
  com frases congeladas, tempos, plano B).
- Artefatos fora do clone, em `r-ui/`.

## Critério de pronto
- Checklist percorrido com evidência; veredito e condições claras; roteiro final.
- Nenhum arquivo do repositório alterado; `git status` limpo no fim; perfil padrão e
  locks inalterados; nenhuma escrita em devnet; varredura de segredos limpa nos
  artefatos da revisão.

## Condições de parada
- Hash de binário, shim, lock ou worker divergente; preflight falhou → `BLOQUEADO`.
- Saldo ou estado divergente sem explicação → parar e reportar.
- Necessidade de escrita, rebuild, instalação ou edição → `AGUARDANDO_AUTORIZAÇÃO`.
- Conteúdo de chave numa saída → parar e registrar o incidente.

## Commit
- Não autorizado (revisão somente leitura). O registro é feito depois, por sessão com
  escrita, a partir da resposta. Push sempre proibido salvo autorização separada.

## Relatório final
1. arquivos modificados (esperado: nenhum); 2. testes, PoCs e saídas reais;
3. invariantes; 4. decisões pendentes; 5. riscos; 6. confirmação de fronteiras;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`: registro da revisão e,
   se aprovado, gravação do vídeo definitivo (10/10) pela seção "Gravação pela
   interface", e submissão (11/10).
