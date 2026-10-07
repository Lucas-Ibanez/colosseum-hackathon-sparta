# D11–D12 — interface Hive servida pelo worker: resultados

Commits: `37d2d63` (`worker: add Hive interface (D11-D12)`) e `docs: record D11-D12
interface`.

## Resultado

- **Interface Hive** em `worker/static/` (HTML, CSS e módulos ES, sem framework nem
  dependência), servida pelo worker em `http://127.0.0.1:8710/ui/`: telas **Jobs**,
  **Novo job** e **Detalhe do job** (cabeçalho, A linha do tempo, B compromissos ×
  journal, C verificação local × on-chain, D anatomia e saldos, cenários adversariais,
  E limites, F CLI), com a identidade do `DESIGN.md` só por tokens gerados dele.
- **Worker:** rotas estáticas fixas em `/ui/` e as lacunas 1 a 6 da adaptação como
  visões só de leitura; argv, ambiente, lock, reconciliação, shim e `rejection`
  inalterados.
- **Testes:** `unittest` **53/53** (os 23 do D10 inalterados + 30 novos); lint do
  `DESIGN.md` 0 erros, 0 avisos; `check:tokens` e `check:assets` sem deriva.
- **Devnet, U1–U10, todas pela interface** (cliques num Chromium headless dirigido por
  script, como o operador faria): T₃ `RefundedOnTimeout`, P₃ `Released` com o
  verificador invocado (99.541 CU), F₃ `RefundedOnFail` com o verificador invocado
  (99.541 CU), negativos 6021, 6014 e 6007 pelo escrow e 6003 pelo verificador.
  Exatamente 10 transações novas (5 do buyer, 5 do executor, 0 do deployer); saldos
  fecham por lamport e por unidade.
- Seção "Gravação pela interface" em `docs/demo-script.md`, com os tempos medidos.

## Linha do tempo (2026-10-07, -03:00)

| Hora | Fase |
| --- | --- |
| 09:19–09:22 | preflight e checagem do D10 (só leitura) |
| 09:37 | plano aprovado em Plan Mode |
| 09:37–13:16 | ferramentas, worker, interface, testes, ensaio U1–U10 sobre os executáveis falsos |
| 13:17 | worker reiniciado com o código final; saldos U0 |
| 13:17:52–13:29:08 | **U1–U10 em devnet pela interface** (11 min 16 s) |
| 13:29–13:40 | conferência independente, capturas finais, varreduras |

## 1. Preflight e checagem da tarefa anterior

- `pwd` = raiz Git `/home/lucas/src/vericode`, branch `main`, HEAD `1147edd` (D11a)
  sobre `23a91b5`; `git status --short` vazio; `--ignored` só
  `worker/ui-tools/node_modules/` e o PDF de identidade (decisão D11a);
  `git diff --check` 0.
- Perfil padrão: `~/.rustup`, `~/.cache/solana`, `~/.config/solana`, `~/.npm`
  ausentes; `~/.cargo` `d9e12578`, `~/.avm` `7d29f7f8`, `~/.docker` `6046f67f`
  (iguais no início e no fim).
- Locks: raiz `191802b2`, `zkvm` `f5236689`, guest `1116acef`, `anchor` `19a1db26`,
  `tests-local` `be94760a`, `cli` `4d979577`, `prover` `8b76f1e1`, `ui-tools`
  `0c40c2ab`; guest 180.300 B `e09ba8cf`; binários do D10a `e6cd4e29`/`3f66e1c0`;
  shim `2a8f75b8`, modo `0755`; worker do D10 `e276e588`; 8/8 arquivos de
  `brand/fonts/` conferem com o README (tamanho e SHA-256).
- Ferramentas: Node `v24.21.0`; `npm ci --ignore-scripts` (96 pacotes);
  `lint:design` 0 erros, 0 avisos.
- `unittest` do D10: 23/23; nenhum `__pycache__`.
- Partida do worker (código do D10): hashes, `worker.buyer=EZgG…`,
  `worker.executor=EdB2…`, `check=ok` com o verificador `34ae6e5c…`,
  `prover=LocalProver env=ok`, `worker.url=http://127.0.0.1:8710`.
- Devnet só leitura pela API: `bc334093…` `Released 225384a7…`; `8ab4ee8d…` e
  `8bce67f2…` `RefundedOnTimeout`. Saldos iguais ao fim do D10 (deployer
  2.805.194.240, buyer 99.750.400, executor 29.910.000 lamports; ATAs 999.993.000.000
  e 7.000.000).
- Raiz `~/.local/share/vericode-spikes/d11/` (`0700`; logs `0600`): `logs/`, `tmp/`
  (TMPDIR), `shots/`, `bin/` (scripts `u11_*`; o `env.sh` herdado não foi usado).

## 2. O que mudou

### Worker (`worker/vericode_worker.py`, `e276e588` → `bbdf6dbe`)

- **Rotas estáticas (lacuna 6):** tabela literal `STATIC_FILES` (56 arquivos) servida em
  `/ui/` por igualdade exata, só GET, sem token; `Host` conferido; CSP `default-src
  'none'; script-src 'self'; style-src 'self'; font-src 'self'; img-src 'self';
  connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'`,
  `nosniff`, `no-referrer`, `no-store`, `DENY`; nenhum `Access-Control-*`. O prefixo
  `/ui/` (e não `/`) mantém verde o teste do D10 que exige 404 em `/`.
- **Caminho reescrito → 400:** o `http.server` do Python 3.12 colapsa `//` inicial em
  `/`; o teste novo mostrou `//ui/` servido como `/ui/`. Agora qualquer caminho que a
  stdlib reescreveu (comparado com a linha de requisição) recebe 400, também em
  `//api/…`.
- **Lacuna 1:** `decode_journal` (165 bytes, `schema_version` 1, verdict 0/1, offsets
  congelados); a receipt do Job traz `journal` (de `compress.journal_hex`, exigido igual
  a `prove.journal_hex`) e `local` (linhas reais do prove/compress).
- **Lacunas 2 e 3:** `GET /api/ops/<id>` traz `anatomy` (invocações dos logs públicos
  da transação e `pre/postTokenBalances` com o papel do dono).
- **Lacuna 4:** `health.v1` (termos, escrow, verificador, mint, pubkeys, valor e
  janela) das linhas do `check` da partida; `health.slot_clock` (último slot lido e
  segundos por slot medidos entre leituras).
- **Lacuna 5:** `health.app_commit` lido de `.git/HEAD`, sem processo.
- **Extras de visão:** `list_jobs` com valor, prazo e slot da leitura; `running_step`
  (etapa em curso); `create_live` (`create.job`/`vault`/`slot` assim que a CLI imprime);
  `worker.ui=` na partida.

### Interface (`worker/static/`)

- `index.html`, `css/tokens.css` (gerado), `css/app.css`, `js/` (`main`, `api`,
  `i18n`, `format`, `dom`, `icons`, `components`, `views/jobs`, `views/new-job`,
  `views/job`), `fonts/`, `brand/`, `icons/`.
- SPA por hash; token só na memória do módulo `api.js`, no header
  `X-VeriCode-Token`; DOM só por `createElement`/`textContent`; ícones Lucide
  importados de SVG por `DOMParser` (XML) para herdar a cor e ter traço de 1,5 px.
- Componentes: `BrandMark`, `HashField`, `ProvenanceBadge`, `StatusLabel`,
  `StatusBanner`, `ScenarioOutcome`, `StateTimeline`, `CommitmentVsObserved`,
  `VerificationPanel`, `TransactionAnatomy`, `BalanceDelta`, `PartyMark`,
  `ProvingProgress`, `LimitsOfProof`, `CliEquivalent`, `EnvironmentBar`,
  `EnvironmentChip`, `ConfirmDialog`, `Toast`, `TokenGate`.
- Frases: `claim.f1` ("da Hive"), `claim.f2`, `f3`, `f4`, `f5`, `f8`, iguais ao
  `README.md`; F6 e F7 não são usadas. Limitação P3 do README visível no Novo job e no
  Detalhe. Português padrão; o dicionário inglês cobre os rótulos do `DESIGN.md` e cai
  no português no resto (as frases congeladas não têm tradução registrada).

### Ferramentas (`worker/ui-tools/`, lock inalterado)

- `gen-tokens.mjs` (4 camadas do `DESIGN.md`; `--check` byte a byte),
  `sync-assets.mjs` + `static-assets.json` (43 cópias com SHA-256), `capture.mjs`
  (capturas); scripts `gen:tokens`, `check:tokens`, `sync:assets`, `check:assets`,
  `capture` no `package.json`.

### Testes (`worker/tests/test_static.py`, `worker/tests/fixtures/d10-tx/`)

30 testes novos: rotas e cabeçalhos; recusas (`..`, `%2e%2e`, `%2E%2E`, `//`,
diretório, maiúsculas, query, POST, `Host`); tabela = arquivos; sem token nos
estáticos e token exigido na API; token e caminhos de chave ausentes do servido; cópias
= tabela = origem; fontes = README; `tokens.css` = `DESIGN.md` (hash, 38 cores, papéis
nos dois modos); `decode_journal` contra `pass.txt`, `fail.txt` e `d4b/*.txt` e as
recusas; receipt com journal e checagens locais; journal divergente não exibido;
anatomia e saldos contra três registros públicos reais do D10 (`ByGF4BFP…`,
`4cKK83JB…`, `57UYbVX9…`); `health.v1`, `slot_clock` e commit; varredura do JS, HTML e
CSS (APIs proibidas, `.style`, literais de cor/medida/fonte fora do `tokens.css`,
`≈`/`→`/`·`, termos proibidos, "mainnet" só na frase 5, frases = README, "VeriCode" só
no header).

```text
$ TMPDIR=…/d11/tmp python3 -B -m unittest discover -s worker/tests -v
Ran 53 tests in 34.438s
OK
$ npm --prefix worker/ui-tools run lint:design     "errors": 0, "warnings": 0
$ npm --prefix worker/ui-tools run check:tokens    tokens ok: worker/static/css/tokens.css equals the DESIGN.md output
$ npm --prefix worker/ui-tools run check:assets    43 assets match the table
```

## 3. Ensaio sobre os executáveis falsos (sem rede, sem chave)

Antes da primeira escrita, U1–U10 rodaram pela interface contra o worker novo sobre os
**falsos dos testes** (`worker/tests/fakes`), num diretório temporário em `d11/tmp`, na
porta 8711 (`d11/bin/u11_fake.py`). O ensaio pegou uma corrida real da interface (a
rota podia montar uma tela antes de a primeira leitura de `health` terminar; agora a
tela do token só sai depois dela) e confirmou diálogos, polling, estados, cenários e o
6014 com a tabela de vínculo. Capturas em `d11/shots/rehearsal*` (dados falsos,
nunca usados como evidência).

## 4. Escritas em devnet (U1–U10, pela interface)

Driver: `d11/bin/u11_drive.mjs` (só preenche campos e clica em botões; lê o token da
captura do terminal e nunca o imprime). Cada operação é a do worker, com argv fixo.

| # | Clique | Job | Resultado | Transação |
| --- | --- | --- | --- | --- |
| U1 | Novo job, prazo 1.560, "Financiar job" | T₃ `2c38ca665ec4e8fbfa4674ad3cd744bdc1d52e9ebbe429df2e3ffb37f8c1015b` | `create_job`+`fund`, `Funded`, prazo slot 508.513.939 | [`8AJzbbKf…`](https://explorer.solana.com/tx/8AJzbbKfaeKSpjRxzX2wTu1R4DxpujLgT7z2ZRsUHjeUGDMka8bAkyriXZJ7ZuCNTfY3zYzq3rQWr1P4yoABDNo?cluster=devnet) |
| U2 | "Pedir refund antes do prazo" (margem lida: 1.434 slots) | T₃ | `DeadlineNotReached` 6021, **escrow**; contas iguais | [`4jDL97DG…`](https://explorer.solana.com/tx/4jDL97DGJRLQhB2m2NKK9kof85EFHSQMyPDPMmdhQ9mm6nkRKcbFF9HCuXcEiooWKKWomBCuhhTd2KbpYFAGiRHF?cluster=devnet) |
| U3 | Novo job, prazo 9.000, "Financiar job" | P₃ `26df9ef48e5773d4b48dbb2aff1a3419763efcc5e09982900c1df61ab3ff3618` | `Funded`, prazo slot 508.521.695 | [`5vWoxx2m…`](https://explorer.solana.com/tx/5vWoxx2mBra2iP5TfBX9FeYUjt5fujMde7m7saWHpo1kGEksZbrwVy66L9X6DqMLqDDNbk6BR5P6UjXqtZK5gzT6?cluster=devnet) |
| — | "Gerar prova" `(21, 42)` | P₃ | `Composite` → `Groth16`, `PASS`, linha `docker_run`; 7,2 s + 177,9 s (local) | — |
| U4 | "Enviar receipt de outro job" = `bc334093…` | P₃ | `JournalJobIdMismatch` 6014, **escrow**; `deliver` reverteu, sem `artifact_hash` | [`5CBFajsa…`](https://explorer.solana.com/tx/5CBFajsaFvYreCJkQwiPuR8rHWtpBKCULoVAeysUJGRwTDfF2CXYzeFkuit6azjBjqQkoif8TbYwxByv8Ckmo4en?cluster=devnet) |
| U5 | "Enviar prova adulterada" | P₃ | `PairingError` 6003, **verificador** (100.528 CU); contas iguais | [`kTQdVojx…`](https://explorer.solana.com/tx/kTQdVojxjzJULzS97Jo5HBFRcroqnYVMaK9Crtx2LmyAY1pUhnBgh7v1Xs686wY3gvvxGz3aiN9URWhNgepRg39?cluster=devnet) |
| U6 | "Enviar entrega e prova" | P₃ | `deliver`+`release`, **`Released`**, verificador invocado **99.541 CU**, 122.156 CU no total | [`2gB9djwM…`](https://explorer.solana.com/tx/2gB9djwM3WpApHLXYf5riF974mTK5W4UW7HCv1xJamLPu3NEwnPcCY4oT6zAwvbMkaKRcATf1LCQPyXP8rmpj3o?cluster=devnet) |
| U7 | "Liquidar de novo" | P₃ | `AlreadyReleased` 6007, **escrow** | [`j2pmqFWu…`](https://explorer.solana.com/tx/j2pmqFWuTSe4H6isKyEFX3cBYmufEANP9Ue1p74tTUawXbiPSQ4KqckPW6WX3gnoAWsXE5XBdeVHEevpLa852ky?cluster=devnet) |
| U8 | Novo job, prazo 9.000, "Financiar job" | F₃ `7395a76d83379e93b30982d5e2a684451cf868e32424123da9cc0759dbede571` | `Funded`, prazo slot 508.523.275 | [`2jjiYDAo…`](https://explorer.solana.com/tx/2jjiYDAoCQTrF93LvTAFB3RHTB3ab86k2BZmRXnFv3FmgK2ps2Kf1U2vs4MeFMMo7iqDQ77TSqz9hjigaQMrFQbW?cluster=devnet) |
| — | "Gerar prova" `(7, 15)` | F₃ | `Groth16`, **`FAIL`**; 8,6 s + 91,4 s (local) | — |
| U9 | "Enviar entrega e prova" | F₃ | `deliver`+`refund_on_fail`, **`RefundedOnFail`**, verificador invocado **99.541 CU** | [`4jcugMyX…`](https://explorer.solana.com/tx/4jcugMyXqhAttG7cjcjWZnMiyZe3SosHC2cgphQ6rf48n5KfrWZ4TRfzR1ZioP5NwYE2qeo2zUDEcD4rJQDTCmSJ?cluster=devnet) |
| U10 | "Reler a cadeia", "Solicitar refund" (slot 508.515.160 > prazo) | T₃ | `refund_on_timeout`, **`RefundedOnTimeout`**, sem verificador | [`5oTbFiTX…`](https://explorer.solana.com/tx/5oTbFiTXjkv8aBgTQPrKSqTShfNgVqSePZuP5x57GXxfEi6mTPMLfXFCWpnZSQzgcXbyqSjBHSJy65dEy5aAKVzi?cluster=devnet) |

O rótulo de quem rejeitou veio só de `rejection` (C10-7), presente nos quatro negativos
com `outcome=PASS`; nenhuma rota recebeu `UNEXPECTED`. Os nomes dos erros na tela vêm
das linhas `AnchorError` dos logs públicos e só aparecem quando o código bate com o de
`rejection`.

### Saldos (U0 → U10, `d11/bin/u11_rpc.py balances`)

| Conta | Antes (slot 508.512.311) | Depois (slot 508.515.193) | Diferença | Explicação |
| --- | ---: | ---: | ---: | --- |
| SOL deployer | 2.805.194.240 | 2.805.194.240 | 0 | não entra no worker |
| SOL buyer | 99.750.400 | 88.981.200 | −10.769.200 | 3 × 3.581.400 de rent + 5 × 5.000 de taxa (U1, U2, U3, U8, U10) |
| SOL executor | 29.910.000 | 29.885.000 | −25.000 | 5 × 5.000 de taxa (U4, U5, U6, U7, U9) |
| Test USDC, ATA do buyer | 999.993.000.000 | 999.992.000.000 | −1.000.000 | −3 depósitos, +1 (U9), +1 (U10) |
| Test USDC, ATA do executor | 7.000.000 | 8.000.000 | +1.000.000 | U6 |

### Conferência independente (só leitura)

- `getSignaturesForAddress` desde o último registro do D10: buyer 5 (`8AJzbbKf`,
  `4jDL97DG`, `5vWoxx2m`, `2jjiYDAo`, `5oTbFiTX`), executor 5 (`5CBFajsa`, `kTQdVojx`,
  `2gB9djwM`, `j2pmqFWu`, `4jcugMyX`), deployer 0.
- `getTransaction` das 10: erros, taxas, CU, verificador e movimentos de token iguais à
  tabela (`d11/logs/r1-gettransaction.jsonl`).
- Memória: P₃ 4,5 GiB disponíveis antes, pico de 337 MB com swap em uso; F₃ 6,1 GiB
  antes, pico de 355 MB; nenhum exit 137 (`d11/logs/mem-prove-*.log`). O Chromium do
  driver foi fechado logo depois do início de cada prova, antes da compressão.

## 5. Capturas e crítica (fora do clone, `~/.local/share/vericode-spikes/d11/shots/`)

- `U/` (escritas reais): formulário, diálogo de confirmação, envio, resultado e Detalhe
  de cada U; `P3prove-proving.png` e `F3prove-proving.png` (estado `Proving`).
- `final/`: Jobs, Novo job, P₃ (`Released`), F₃ (`RefundedOnFail`), T₃
  (`RefundedOnTimeout`), página inteira e por bloco; Jobs e P₃ em modo escuro;
  `keys-*.png` (teclado); `final-dark/`, `final-reduced/` (1.024 px, movimento
  reduzido), `final-narrow/` (900 px).
- `dev1`–`dev3/`: iterações de desenvolvimento.

Crítica contra o `DESIGN.md` e o §13 do guia adaptado:

- **Jobs:** tabela de largura total, `job_id` em mono 8+8 com cópia, tarefa, estado
  com ícone e rótulo, valor tabular à direita, prazo com o slot da leitura; um único
  botão âmbar. Aceito.
- **Novo job:** compromissos somente leitura com proveniência "Fixado pelo programa";
  único campo é o prazo, com o slot lido, a idade da leitura e `~N min (estimativa)`;
  o modelo do comando antes do envio; `job_id` e vault surgem quando a CLI imprime;
  resumo fixo ao rolar. Aceito.
- **Detalhe `Funded` / `Proving`:** operações numa região, um primário por vez, motivo
  visível em todo botão desabilitado; `Proving` com tempo do relógio do worker, etapa
  `prove`/`compress`, sem barra nem percentual. Aceito.
- **`Released` (P₃):** banner de resultado com o caminho para a evidência; linha do
  tempo com `Delivered` "na mesma transação da liquidação"; B com 7 linhas
  (`artifact_hash` comprometido no `deliver`); C separando local e on-chain, frase 1
  junto do link; D com a anatomia num contêiner único (Instrução 1 `Deliver`;
  Instrução 2 `Release` com as CPIs `Verify` e SPL Token) e os saldos com sinal. Aceito.
- **`RefundedOnFail` (F₃):** família `danger`, "Critérios não atendidos (`FAIL`)" e
  "Resultado verificado, não um erro"; a célula das partes é a mesma de `Released`.
  Aceito.
- **`RefundedOnTimeout` (T₃):** ramo "por prazo" a partir de `Funded`; a verificação
  on-chain diz que essa transação não invocou o verificador. Aceito.
- **Negativos:** quatro apresentações distintas (elo partido, documento com X, setas
  de repetição, ampulheta), com programa, nome e código do erro, "O estado do job não
  foi alterado." só com `watched_unchanged=true`, e o 6014 com a tabela de vínculo e a
  identificação da receipt. Aceito.
- **Escuro:** preparado por tokens e coerente (bordas de 1 px nos banners); não é o
  acabamento canônico. **900 px:** uma coluna, sem recorte. **Movimento reduzido:**
  nenhuma animação. **Teclado:** ordem segue a tela; anel berinjela de 2 px afastado;
  na navegação lateral, o foco usa o âmbar (superfície escura).
- **Pendências visíveis:** o SPL Token não registra o nome da instrução nos logs, e a
  anatomia diz isso ("instrução não registrada no log"); o favicon não existe
  (`TODO(brand)`), e o navegador recebe 404 em `/favicon.ico`.

## 6. Varreduras

- Segredos (`d11/bin/u11_secret.py`, cópia do método do D10): 71 arquivos novos ou
  alterados, **0 ocorrências** (arrays de 64 números, base58 de 64 bytes fora de
  assinaturas conhecidas, marcadores de mnemônico).
- Os 4 tokens de worker do dia (3 execuções reais e o ensaio) não aparecem em nenhum
  arquivo do repositório (240 arquivos).
- Nenhum caminho de chave nos estáticos (teste); `git diff --check` 0; sem espaço no
  fim de linha nos arquivos novos.

## 7. Incidentes e desvios

1. **`pkill -f` casou com o próprio shell** (o padrão estava na linha de comando):
   encerrou o worker do D10 e o comando, antes de qualquer escrita. A partir daí, o
   worker é iniciado por `d11/bin/u11_start.sh` e parado por PID exato.
2. **Colisão de nome no ambiente do ensaio real:** o `U11_URL` exportado para o driver
   sobrescreveu o do helper da API; duas leituras GET de conferência (depois de U1 e U2)
   foram para `/ui//api/…` e receberam 404. Nenhuma escrita afetada; o driver passou a
   usar `U11_UI`, e as leituras foram refeitas.
3. Um `__pycache__/` em `worker/` foi criado por um comando meu sem `-B` (às 09:57) e
   removido; os testes não criam bytecode.
4. O `http.server` colapsa `//` inicial (achado do teste novo); corrigido no worker
   (400 para caminho reescrito).

## 8. Fronteiras

- **Rede:** `api.devnet.solana.com` só pela CLI via worker e pelo `u11_rpc.py` (só
  leitura); `127.0.0.1:8710` e `:8711` (ensaio). Sem npm install novo (`npm ci` do lock
  existente).
- **Devnet:** só U1–U10; nenhum airdrop, deploy ou transferência de SOL.
- **Docker:** só pelo `vericode-prover compress` (shim conferido); sem pull.
- **Repositório:** sem mudança em `cli/`, `prover/` (inclusive o shim), programa,
  core, `zkvm/`, guest, `JournalV1`, locks, `DESIGN.md`, guia, adaptação e `brand/`.
- **Fora do clone:** `d9/`, `d10a/`, `rd10a/`, `rec-*`, `d10/logs`, `d10/bin`, `d4/`,
  `d7/` sem arquivo alterado desde 09:00; `d10/data` e `d10/tmp` mudaram pelo uso
  normal do worker (Jobs T₃, P₃, F₃, leituras). De `d4/keys`, só `pubkeys.txt` foi lido.
- **Perfil padrão** inalterado.

## 9. Riscos abertos

- **P3:** chaves de devnet do projeto no worker local; o mesmo operador opera buyer e
  executor; sem carteira no navegador (declarado na tela e no README).
- A seção adversarial também aceita Jobs antigos já liquidados (por exemplo, o 6007 em
  `bc334093…`): cada clique pede confirmação, mas uma tomada de vídeo pode criar uma
  escrita fora do roteiro se o operador errar o Job.
- `app_commit` mostra o HEAD da **partida** do worker: depois do commit do D11–D12,
  reinicie o worker antes de gravar.
- `slot_clock` é estimativa (~0,24 s por slot medidos); o prazo real é o do programa.
- Token na captura do terminal do operador (`0600`); `http.server` só em loopback.
- O dicionário inglês cobre só os rótulos do `DESIGN.md`; traduzir as frases
  congeladas exige decisão registrada.
- Memória do WSL: a compressão chegou a ~340 MB livres com swap em uso.
- Os de antes continuam: sem e-stop; rent preso (17 Jobs); mint authority = deployer;
  ImageID não recertificado; spec v1 trivial; RD7-08 na CLI (mitigado por C10-8).

## Artefatos fora do clone

`~/.local/share/vericode-spikes/d11/` (`0700`; arquivos `0600`):
- `bin/`: `u11_api.sh`, `u11_rpc.py`, `u11_start.sh`, `u11_drive.mjs`, `u11_fake.py`,
  `u11_keys.mjs`, `u11_secret.py`;
- `logs/`: `timeline.log`, `worker-terminal*.log` (com tokens; não versionar),
  `pre-show-*.json`, `bal-*.json`, `U*-drive.log`, `U*-job.json`, `U*-op.json`,
  `*prove-*.json`, `mem-prove-*.log`, `r1-gettransaction.jsonl`, `r2-sigs-*.jsonl`,
  `t1-unittest.log`, `t2-ui-tools.log`, `f5-secrets.log`;
- `shots/`: capturas (seção 5); `tmp/`: TMPDIR, ensaio.
