# D11–D12 — interface Hive: Jobs, Novo job e Detalhe do job, servidos pelo worker

## Identificação
- Gate: `D11–D12`  ·  Dia da sequência: D11–D12  ·  Marcos do guia: M5/M6
  (fluxo pela interface fina), M7 (claims iguais ao código); guia de produto §4
  (`frontend`), §7, §10, §12
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D11a` (fundação da interface Hive), commitado pelo humano sobre
  `23a91b5`. Decisão "Fundação da interface Hive (D11a)" em `docs/decisions.md`.
- Este prompt **substitui** `docs/handoffs/d10-to-d11.md` (que previa telas
  "Buyer/Submit/Result" sem a identidade Hive).
- Prazo do MVP: 11/10. Ordem: telas (até 09/10) → revisão do worker e das telas
  (09/10) → vídeo definitivo numa tomada contínua (10/10) → submissão (11/10).
  Registrar timestamps por fase.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh**. É interface pública
  com claims, superfície HTTP nova no worker e escritas em devnet pela interface.
- **Plan Mode obrigatório** antes de alterar `worker/` e antes da primeira escrita
  em devnet. O plano traz:
  - as rotas estáticas e os cabeçalhos;
  - cada tela, bloco a bloco, com a rota e o campo da API de onde vem cada dado
    (`HIVE_MVP_UI_ADAPTATION.md`, seção 4);
  - as mudanças do worker para as lacunas 1 a 6 (seção 5 da adaptação);
  - a lista de tokens e componentes;
  - as frases usadas, todas da lista congelada;
  - os testes;
  - as escritas U1 a U10 com os passos na interface.
- **Checkpoint de UI** (bloco `hive-ui-core` do `AGENTS.md`): a primeira linha de
  cada resposta de UI/UX é `UI checkpoint: li HIVE_MVP_UI_ADAPTATION.md,
  HIVE_MVP_UI_GUIDE.md e DESIGN.md [e brand/MANIFEST.md]. Regras que mais pesam
  aqui: ...`.
- **Skill:** use a skill `frontend-design` (instalada no escopo de usuário) para a
  execução. O `DESIGN.md` é o briefing e vence qualquer escolha estética dela.

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md` (inclusive o bloco `hive-ui-core`), `CLAUDE.md`,
  `docs/project-context.md`, `docs/handoff-protocol.md`, `docs/agent-control.md`.
- **`HIVE_MVP_UI_ADAPTATION.md`** (prevalece sobre o guia em fatos do MVP),
  **`HIVE_MVP_UI_GUIDE.md`**, **`DESIGN.md`** e **`brand/MANIFEST.md`**,
  inteiros. Os três primeiros já vêm importados pelo `CLAUDE.md`.
- `docs/decisions.md`: "Decisões humanas para o D10", D10, "Vídeo de reserva" e
  "Fundação da interface Hive (D11a)".
- `docs/r-d10a-review-results.md` (C10-1 a C10-10; sobretudo C10-6, C10-7 e
  C10-9).
- `worker/README.md`, `worker/vericode_worker.py` (rotas, `state`/`state_scope`,
  `rejection`, `job_view`, `op_view`, `parse_show`, `public_tx`, `prove_steps`),
  `worker/tests/test_worker.py`.
- `worker/ui-tools/README.md`, `brand/fonts/README.md`.
- `README.md` ("Frases permitidas (congeladas no D9)", "Não dizer",
  "Limitações"), `docs/demo-script.md`, `docs/escrow-program.md` (tabela de erros
  6000–6036 e do verificador), `docs/manifest-schema.md` (layout do `JournalV1`).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3`. O topo esperado é o commit do
  D11a sobre `23a91b5`. Se não for, parar.
- `git status --short` e `--ignored` vazios, exceto `worker/ui-tools/node_modules/`
  (ignorado); `git diff --check` 0.
- Perfil padrão, no início e no fim:
  - `~/.rustup`, `~/.cache/solana`, `~/.config/solana` e `~/.npm` ausentes;
  - `cd <dir> && find . -printf '%p %s %T@\n' | sort | sha256sum`:
    `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…`.
- Ferramentas de UI:
  - `source ~/.local/share/vericode-spikes/ui/env-ui.sh` → `node --version`
    `v24.21.0`;
  - `npm ci --ignore-scripts --prefix worker/ui-tools` (lock com 96 pacotes);
  - `npm --prefix worker/ui-tools run lint:design` → 0 erros, 0 avisos.
- Raiz própria `~/.local/share/vericode-spikes/d11/` (`0700`, logs `0600`), com
  `logs/`, `tmp/` (TMPDIR) e `shots/` (capturas).
- **Não alterar** `d9/`, `d10a/`, `rd10a/`, `rec-*` nem `d10/logs`. O worker
  continua com `d10/worker.json` e `d10/data` (preserva o Job `bc334093…` e a
  receipt dele, usada no U4).
- Scripts com nomes prefixados; não carregar o `env.sh` herdado nem usar `R`, `B`,
  `D` ou `VC`. O `env-ui.sh` só define `VU_*`, `PATH`, `npm_config_*`,
  `PLAYWRIGHT_BROWSERS_PATH` e `LD_LIBRARY_PATH`.
- **Memória:** 7,6 GiB de RAM e 8 GiB de swap. Um processo pesado por vez. O
  worker só comprime com MemAvailable ≥ 2,5 GiB. Feche o Chromium do Playwright
  antes de uma prova.

## Checagem da tarefa anterior
- `TMPDIR=~/.local/share/vericode-spikes/d11/tmp python3 -B -m unittest discover
  -s worker/tests -v` → **23/23**; `git status --ignored` sem `__pycache__`.
- `worker/vericode_worker.py` `e276e588…`.
- Locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/`
  `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover`
  `8b76f1e1…`, `worker/ui-tools` (`package-lock.json`) `0c40c2ab…`; guest
  180.300 B `e09ba8cf…`.
- Binários do D10a: CLI `e6cd4e29…`, prover `3f66e1c0…`; shim
  `prover/docker-shim/docker` `2a8f75b8…`, modo `0755`.
- Fontes: cada arquivo de `brand/fonts/` confere com a tabela de
  `brand/fonts/README.md` (tamanho e SHA-256).
- **Partida do worker:**
  - comando: `env -i HOME=~/.local/share/vericode-spikes/d10/home
    PATH=/usr/bin:/bin LANG=C.UTF-8 python3 -B worker/vericode_worker.py --config
    ~/.local/share/vericode-spikes/d10/worker.json`, destacado, saída em
    `d11/logs` (`0600`);
  - esperado: hashes, pubkeys de buyer e executor, `check=ok` com o verificador
    `34ae6e5c…`, `prover=LocalProver env=ok`,
    `worker.url=http://127.0.0.1:8710`.
- Devnet, só leitura, pela API (`POST /api/jobs/<id>/show`): `bc334093…`
  `Released { 225384a7… }`; `8ab4ee8d…` e `8bce67f2…` `RefundedOnTimeout`.
- Saldos esperados (fim do D10): SOL do deployer 2.805.194.240, do buyer
  99.750.400 e do executor 29.910.000 lamports; ATA do buyer 999.993.000.000;
  ATA do executor 7.000.000.
- Se divergir: comparar `getSignaturesForAddress` com novas pastas `rec-*` do
  humano (só campos públicos). Escrita não explicada: parar e reportar.

## Objetivo
Entregar a interface Hive servida pelo worker (`worker/static/`), com as telas
**Jobs**, **Novo job** e **Detalhe do job** (blocos A a F). Ela:
- segue o `HIVE_MVP_UI_GUIDE.md` adaptado pelo `HIVE_MVP_UI_ADAPTATION.md` e o
  `DESIGN.md` integralmente;
- funciona contra devnet com Jobs reais;
- fica pronta para o vídeo definitivo numa tomada contínua;
- é ensaiada pelas escritas U1 a U10.

## Decisões já tomadas
- **Tudo da decisão "Fundação da interface Hive (D11a)":**
  - adaptação ao MVP com precedência sobre o guia;
  - frase 1 com "da Hive" só na interface;
  - fontes auto-hospedadas;
  - cópia verificada de assets;
  - `worker/static/`;
  - execução sem dependência;
  - ferramentas npm só de desenvolvimento;
  - português como idioma padrão, com o dicionário pronto para inglês.
- **P1 a P6** ("Decisões humanas para o D10"):
  - P3: sem carteira no navegador; limitação declarada na página e no README;
  - P4: rótulo do programa que rejeitou só com `rejection` (C10-7); `UNEXPECTED`
    aparece como tal.
- **C10-1 a C10-10** continuam valendo; a interface não as enfraquece.
- Claim único (CD7) e as 8 frases congeladas, com a lista "Não dizer". Nenhuma
  frase nova sem decisão registrada e revisão.
- **Não há Verifier Router nem fallback no MVP.** A camada on-chain é a CPI direta
  ao verificador Groth16 `THq1q…`; sem liquidação, "Ainda não verificada
  on-chain", nunca "fallback".
- Jobs consumidos, nunca reutilizar: S, A, B, C, P, T, P′, T′, `8ab4ee8d`,
  `8bce67f2`, `cd77e7bd`, `3e115ca8`, `104f9a21` e `bc334093`.
- Escrow `GZqb…` imutável; verificador `THq1q…`; mint `9TE2V…`; `JournalV1` v1
  congelado.

## Decisões a confirmar no Plan Mode
- **Escritas em devnet, lista fechada, todas pela interface** (ensaio do vídeo;
  confirmar no Plan Mode):
  - **U1:** create do Job T₃ (offset 1.560);
  - **U2:** `escrow-6021` em T₃, logo depois (margem ≥ 300);
  - **U3:** create do Job P₃ (offset 9.000);
  - prova `(21,42)` e `compress` de P₃ (local, não é escrita);
  - **U4:** `escrow-6014` em P₃ com a receipt do Job `bc334093…`, identificada na
    tela como receipt de outro Job (CR6);
  - **U5:** `verifier-6003` em P₃;
  - **U6:** settle de P₃ → `Released`, com o verificador invocado;
  - **U7:** `escrow-6007` em P₃;
  - **U8:** create do Job F₃ (offset 9.000);
  - prova `(7,15)` de F₃ → veredito `FAIL` (local);
  - **U9:** settle de F₃ → `RefundedOnFail`, com o verificador invocado (cenário
    "FAIL válido");
  - **U10:** `refund-timeout` de T₃ depois do prazo → `RefundedOnTimeout`.

  Nada além disso. Sem airdrop nem transferência de SOL; o buyer tem SOL para
  cerca de 25 Jobs.
- **Idioma:** se o humano pedir inglês como padrão no Plan Mode, as frases
  congeladas em inglês exigem tradução fiel, registrada em `docs/decisions.md`,
  antes de aparecer na tela.

## Escopo autorizado
- **`worker/static/`:**
  - HTML, CSS e JavaScript (módulos ES), sem framework, bundler, CDN ou
    dependência;
  - `tokens.css` gerado do `DESIGN.md`;
  - `fonts/`, `brand/` e `icons/` copiados por tabela fixa.
- **`worker/vericode_worker.py`:**
  - rotas estáticas fixas fora de `/api`;
  - as lacunas 1 a 6 da adaptação (campos do journal pelos offsets congelados,
    invocações da liquidação, saldos antes e depois, termos v1 e slot antes do
    create, commit opcional);
  - sem mudar argv, ambiente, lock de operação, reconciliação, shim nem
    `rejection`.
- **`worker/tests/`:** testes novos (estáticos, cabeçalhos, cópias, lacunas).
  Os 23 existentes continuam verdes.
- **`worker/ui-tools/`:** scripts de desenvolvimento, sem dependência nova; o lock
  não muda:
  - gerar `tokens.css` a partir de `design.md export` (`dtcg` e/ou `css-vars
    --prefix hive`), com a camada de papéis claro/escuro do "Mapa de papéis";
  - sincronizar `brand/fonts`, o logotipo e os ícones Lucide escolhidos;
  - capturas com Playwright;
  - checar deriva (o gerado é igual ao versionado).
- **Docs:**
  - `worker/README.md`, `worker/ui-tools/README.md`;
  - `README.md` ("Limitações" e tabela de devnet, sem mudar as frases
    congeladas);
  - `docs/demo-script.md`: nova seção "Gravação pela interface", com a tomada
    contínua U1–U10, preparação (worker, navegador, token colado **antes** de
    gravar), cenas, cliques, saída esperada, tempos e falas, só com frases
    congeladas;
  - `docs/architecture.md` (`frontend`);
  - `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`,
    `docs/project-context.md`;
  - `docs/d11-ui-results.md`, `docs/handoffs/d11-to-r-ui.md`.
- **Rede:** `api.devnet.solana.com` (pela CLI, via worker) e `127.0.0.1`. Sem npm
  install novo.
- **Docker:** só via `vericode-prover compress` (shim), imagem já presente.

## Fora de escopo / proibido
- Editar `DESIGN.md`, `HIVE_MVP_UI_GUIDE.md`, `HIVE_MVP_UI_ADAPTATION.md` ou
  `brand/**`, salvo pedido explícito. Se a adaptação estiver errada, pare e
  reporte.
- Mudar `cli/`, `prover/` (inclusive o shim), programa, core, `zkvm/`, guest,
  `JournalV1`, schema ou locks; rebuild dos binários; dependência npm nova.
- Carteira no navegador; instruções, hashing ou regras refeitos em JavaScript; RPC
  direto do navegador.
- Enfraquecer C10-1 a C10-10 (bind, `Host`, token, CORS, argv fixo, ambiente,
  reconciliação, shim, rótulo, margem do 6021).
- **No JavaScript:** `innerHTML`, `outerHTML`, `insertAdjacentHTML`, `eval`,
  `new Function` ou `document.write`; `localStorage`, `sessionStorage` ou
  `document.cookie`.
- Token em URL, log ou armazenamento persistente. Caminho ou conteúdo de chave na
  página.
- Cor, fonte ou medida literal em componente (só tokens); `≈` ou `→` como texto;
  âmbar como cor de resultado; estado só por cor; mascote, halo, selo ou favos;
  logotipo redesenhado ou redigitado.
- Frase fora da lista congelada; "Verifier Router" como caminho atual; "fallback";
  mainnet; "trustless"; "ZK on-chain" sem a frase 1; "auditado"; "privado";
  "máquina nova".
- Escrita em devnet fora de U1 a U10; airdrop; deploy; mainnet; Docker pull;
  `/tmp`.
- Push.

## Implementação esperada
- **Estrutura** (`DESIGN.md`, Layout):
  - navegação lateral berinjela (`brand/logo/hive-horizontal-branco.svg`, itens
    Jobs e Novo job);
  - barra superior (trilha, chip "Devnet", "Token de teste", estado do signatário
    "Assinatura pela CLI no worker local");
  - barra de status fixa: cluster, Test USDC `9TE2V…`, `image_id`, verificador
    `THq1q…` (v3.0.0, imutável), escrow `GZqb…` (upgrade authority `none`),
    commit ou "Indisponível".
- **Token:** campo para colar o token do terminal, guardado só na memória da aba;
  todas as chamadas `/api/*` usam o header `X-VeriCode-Token`.
- **Jobs:** tabela (identificador mono, tarefa "Regra v1: saída = 2 × entrada",
  `StatusLabel`, valor tabular, prazo); linha inteira clicável; vazio honesto.
- **Novo job** (adaptação 3.1):
  - compromissos fixos e somente leitura (spec, harness, `image_id`, executor,
    valor e mint, com proveniência);
  - um campo de prazo (1.560 a 9.000 slots) com o slot atual e "~ min
    (estimativa)";
  - confirmação com o modelo de argv;
  - "Financiar job".
  - `job_id` e vault só depois que a CLI os imprime.
- **Detalhe do job**, blocos A a F na ordem do `DESIGN.md`:
  - A: `StateTimeline` com os estados reais (adaptação 3.2) e links;
  - B: `CommitmentVsObserved` (adaptação 3.3), dizendo que quem barra é o
    programa;
  - C: `VerificationPanel` local × on-chain (adaptação 3.4);
  - D: `TransactionAnatomy` e `BalanceDelta` (adaptação 3.5);
  - E: `LimitsOfProof` (frases 2 e 3);
  - F: `CliEquivalent` (argv real).

  Ações do executor no próprio detalhe: "Gerar prova" (entrada, saída alegada) com
  `ProvingProgress` (tempo real do worker, etapas, sem barra falsa) e "Enviar
  entrega e prova". "Solicitar refund" só quando `past_deadline`; desabilitado com
  o motivo. Os negativos ficam agrupados como "Cenários adversariais
  (demonstração)", com confirmação.
- **Componentes:** `HashField` (truncamento 8+8 ou 4+4, valor completo por clique
  e teclado, copiar com "Copiado"), `ProvenanceBadge` ("Lido da cadeia no slot N",
  "Worker", "Prova local"), `StatusLabel`/`StatusBanner`, `ScenarioOutcome` (um
  por cenário da adaptação 3.7), `EnvironmentBar`.
- **Strings e números:**
  - strings num dicionário (português padrão, inglês pronto);
  - números com `Intl.NumberFormat`, tabulares, com `−` (U+2212);
  - datas absolutas UTC.
- **Atualização:** polling de `/api/ops/<id>` e `/api/jobs/<id>`; falha de leitura
  dita como falha, com o último valor marcado como desatualizado.
- **Verificação visual:** capturas Playwright de cada tela e estado-chave (Jobs,
  Novo job, Detalhe em `Funded`, `Proving`, `Released`, `RefundedOnFail`,
  `RefundedOnTimeout` e cada negativo), lidas e criticadas contra o `DESIGN.md` e
  os critérios do guia adaptados (adaptação, seção 3, §13). Teclado e
  `prefers-reduced-motion`.

## Testes obrigatórios
- **`unittest` offline**, estendendo `worker/tests/`:
  - rotas estáticas fixas com conteúdo e `Content-Type` certos; nome fora da
    tabela, `..`, `%2e%2e`, barra dupla e query → 404/400;
  - nas páginas: `Host` e CSP `default-src 'none'; script-src 'self'; style-src
    'self'; font-src 'self'; img-src 'self'; connect-src 'self'; base-uri 'none';
    form-action 'none'; frame-ancestors 'none'`, mais `nosniff`,
    `Referrer-Policy: no-referrer` e `Cache-Control: no-store`;
  - nenhum `Access-Control-*`; a API continua exigindo o token;
  - cada arquivo copiado em `worker/static/{fonts,brand,icons}` é idêntico à
    origem (SHA-256);
  - `tokens.css` igual ao que o `DESIGN.md` gera;
  - campos do journal decodificados iguais às fixtures de
    `anchor/tests-local/fixtures/groth16/`;
  - invocações e saldos extraídos de um `--log` real (dados públicos);
  - o token não aparece no HTML/JS servido; nenhum caminho de chave nos
    estáticos;
  - busca textual no JS e no HTML pelas proibições acima e por frases fora da
    lista congelada.
- Regressão: os 23 testes do D10 continuam verdes;
  `npm --prefix worker/ui-tools run lint:design` sem erros.
- Devnet: U1 a U10 pela interface, com assinatura, link, estado e saldos antes e
  depois.

## Evidências exigidas
- `docs/d11-ui-results.md` com:
  - comandos e saídas reais;
  - testes;
  - U1 a U10 com links;
  - saldos que fecham;
  - lista das capturas (fora do clone, em `d11/shots/`) e a crítica de cada
    tela;
  - varredura de frases e de chaves.
- Atualizar `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`,
  `docs/project-context.md` e `docs/architecture.md`.

## Critério de pronto
- As três telas servidas pelo worker, funcionando contra devnet em U1 a U10;
  testes verdes; lint do `DESIGN.md` sem erros.
- Identidade do `DESIGN.md` aplicada só por tokens; frases congeladas; limitações
  (P3) visíveis.
- Seção "Gravação pela interface" pronta.
- `git diff --check` 0; diff integral revisado; busca de segredos limpa; locks e
  perfil padrão inalterados; `d9/`, `d10a/`, `rd10a/`, `rec-*` e `d10/logs`
  intocados.

## Condições de parada
- Hash de binário ou shim divergente; preflight falhou → `BLOQUEADO`.
- Divergência de saldo ou estado não explicada → parar e reportar.
- Escrita fora de U1 a U10 → `AGUARDANDO_AUTORIZAÇÃO`.
- Necessidade de dependência nova, framework, carteira, mudança em `cli/`,
  `prover/` ou nos arquivos de design → `AGUARDANDO_AUTORIZAÇÃO`.
- Conflito entre a adaptação e o código, ou frase nova necessária → parar e
  reportar.
- Mnemônico ou conteúdo de chave numa saída → parar e registrar o incidente.
- `compress` com exit 137 ou MemAvailable < 2,5 GiB → esperar e repetir uma vez;
  se persistir, `BLOQUEADO`.

## Commit
- Commits locais autorizados pelo humano ao enviar este prompt, com identidade via
  `git -c`. Push proibido.
  - `worker: add Hive interface (D11-D12)`;
  - `docs: record D11-D12 interface`.
- Em parada com evidência parcial: commitar só o que estiver testado, com
  evidência real.

## Relatório final
1. arquivos modificados;
2. testes e transações reais, com links;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`, salvo em
   `docs/handoffs/d11-to-r-ui.md`. **R-UI:** revisão adversarial do worker (D10)
   e da interface (D11–D12) juntos, numa sessão separada e somente leitura, com
   Opus 5.5 e esforço max. Escopo:
   - assinatura e envio pelo worker; argv e ambiente; shim;
   - `Host`, token, CORS e CSP; DOM só com `textContent`;
   - rótulo C10-7; chaves;
   - claims e frases (inclusive "da Hive");
   - fidelidade ao `DESIGN.md` e à adaptação;
   - roteiro "Gravação pela interface".

   A resposta traz o texto de registro pronto e o roteiro final do vídeo. Depois:
   vídeo definitivo numa tomada contínua (10/10) e submissão (11/10).
