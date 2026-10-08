# Hive MVP — Adaptação da interface ao MVP construído

> O `HIVE_MVP_UI_GUIDE.md` e o `DESIGN.md` foram escritos antes de o MVP existir, a partir
> dos documentos de contexto iniciais. Este arquivo reconcilia o guia com o que foi de
> fato construído e verificado (programa, CLI, prover e worker), sem mudar nada no
> backend. Decisão humana de 2026-10-07: respeitar o que foi construído, manter a maior
> fidelidade possível ao front-end planejado e respeitar **integralmente** a identidade.

## 0. Como ler

**Precedência** (só sobre fatos do MVP e sobre o que um elemento significa ou promete):

1. Evidência executada e o código do MVP (programa, CLI, prover, worker), conforme a
   ordem de `docs/project-context.md`.
2. **Este arquivo.**
3. `HIVE_MVP_UI_GUIDE.md`, em tudo o que este arquivo não contraria.
4. `DESIGN.md`, para a aparência. **Este arquivo não altera nenhum valor de
   identidade**: paleta, tipografia, escala, espaçamento, raios, componentes,
   movimento e acessibilidade seguem o `DESIGN.md` como estão.

Regras:
- A interface se adapta ao backend, nunca o contrário. Se o guia descreve X de forma
  conceitual e o MVP faz Y, a tela mostra Y.
- Se algo aqui contrariar a evidência ou o código, vale o código: pare e reporte.
- Nomes de estados, instruções, erros, programas e comandos vêm do código real.

## 1. O MVP real (fatos que a tela não pode contrariar)

- **Fluxo:**
  1. o comprador cria e financia o Job numa só transação (`create_job` + `fund`);
  2. o executor prova localmente (`Composite`, depois `Groth16` num container Docker
     local, sem rede);
  3. entrega e liquida numa só transação: `deliver` + `release` (veredito `PASS`) ou
     `deliver` + `refund_on_fail` (veredito `FAIL`);
  4. depois do prazo, qualquer um pede `refund_on_timeout`.
- **Verificação on-chain:** o escrow chama **direto, por CPI**, o verificador Groth16
  imutável de risc0-solana v3.0.0 (`THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge`), na
  mesma instrução que libera ou devolve o Test USDC.
  - **Não existe Verifier Router no caminho.** O Router upstream de devnet nunca foi
    inicializado, e o D4a decidiu pela CPI direta.
  - **Não existe fallback atestado.**
- **Programas imutáveis:** escrow `vericode_escrow` (`GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`)
  e verificador, ambos com upgrade authority `none`. Não há admin nem e-stop.
- **Termos fixos da v1:**
  - spec, harness e `image_id` admitidos são constantes do programa (erros 6028–6030
    se diferirem);
  - mint admitido: Test USDC `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F` (6
    decimais, sem freeze, erro 6036 se outro);
  - no worker, o valor é fixo em 1 Test USDC e o prazo vai de 1.560 a 9.000 slots.
- **Regra v1 trivial:** saída = 2 × entrada (frase congelada 3).
- **Assinatura:** pela CLI, através do worker local (decisão P3). Não há carteira no
  navegador; as chaves de devnet do projeto ficam no worker; o mesmo operador opera
  comprador e executor.
- **Fonte de dados:** o worker HTTP local (`http://127.0.0.1:8710`, `worker/README.md`).
  O estado on-chain vem de `vericode job show` (RPC de devnet), com o slot da leitura.
- **Claims:** só as 8 frases congeladas do `README.md`, com a troca de marca da seção 6.

## 2. Decisões em aberto do guia (§14), já resolvidas

| # | Decisão do guia | Resolução no MVP |
| --- | --- | --- |
| 1 | Assinatura A (carteira) ou B (CLI) | **B**: CLI via worker local (P3, "Decisões humanas para o D10") |
| 2 | Idioma padrão | **Inglês** como padrão, porque o vídeo e a submissão são em inglês (decisão D-EN-2, entrada R-UI de `docs/decisions.md`); o português continua completo no dicionário, sem seletor. As frases congeladas em inglês foram ratificadas (D-EN-1, mesma entrada) e estão no `README.md`, seção "Frozen phrases (English, ratified R-UI)"; os rótulos EN são os do `DESIGN.md` |
| 3 | Tema com acabamento total | **Claro**, canônico pelo `DESIGN.md`; o escuro fica preparado por tokens |
| 4 | Jobs reais da demonstração | Só Jobs reais, criados pelo worker em devnet: um que passa, os negativos e um timeout. Nenhum dado fictício |
| 5 | Origem dos dados de cada bloco | Worker (seção 4); o estado on-chain é o `job show` do worker, com o slot |
| 6 | Router no caminho reproduzível | **Não.** Verificação por CPI direta ao verificador Groth16; não há fallback ativo |

## 3. O guia seção por seção

Legenda: **Manter** (vale como está), **Adaptar** (vale com a mudança descrita),
**Fora** (não se aplica a este MVP).

| Seção do guia | Decisão | O que muda |
| --- | --- | --- |
| §0.2 Inspecionar, adaptar, não inventar | Manter | É exatamente o método deste arquivo |
| §0.3 Tokens provisórios `TODO(identity)` | Adaptar | A identidade já existe: tokens vêm do `DESIGN.md` com os valores reais. `TODO(identity)` só onde o próprio `DESIGN.md` marcar pendência |
| §1 Princípio central | Adaptar | Onde diz "a proof é verificada pelo Verifier Router", leia "pelo verificador Groth16 imutável, por CPI direta do escrow". O resto, inclusive o slogan, vale |
| §1.1 Regras invioláveis | Manter, com a regra 6 adaptada | Não há fallback. A tela só diz "verificada on-chain" junto de uma transação de liquidação com o verificador invocado (`verifier_invoked=true`) e o link do Explorer |
| §2 Leitores | Manter | — |
| §3 Escopo, fora do escopo e ordem de corte | Manter | Todos os itens "dentro do escopo" têm dado real (seção 4) |
| §4.1 Mapa de telas (Jobs, Novo job, Detalhe do job) | Manter | Substitui o plano anterior "Buyer/Submit/Result". As ações do executor ficam no Detalhe do job |
| §4.2 Jobs | Adaptar | Lista = Jobs conhecidos pelo worker (`GET /api/jobs`). "Tarefa" = "Regra v1: saída = 2 × entrada". Valor e prazo vêm do último `job show` |
| §4.3 Novo job | Adaptar (ver 3.1) | Compromissos fixos e somente leitura; o único campo é o prazo |
| §4.4 A, linha do tempo | Adaptar | Estados reais na seção 3.2 |
| §4.4 B, compromissos × observado | Adaptar | O `artifact_hash` **é comprometido** pelo executor no `deliver` (D2b.1): a linha compara entrega × journal e não é "não se aplica". Ver 3.3 |
| §4.4 C, verificação | Adaptar | Camada on-chain = CPI ao verificador Groth16. Sem variante fallback. Ver 3.4 |
| §4.4 D, liquidação | Adaptar | Instruções reais em ordem. Ver 3.5 |
| §4.4 E, limites da prova | Manter | Texto pelas frases congeladas 2 e 3. Ver seção 6 |
| §4.4 F, reproduzir | Manter | Comando real de cada operação. Ver 3.6 |
| §4.5 Proving | Manter, com fonte definida | Tempo decorrido a partir de `started_at` do worker (relógio do worker); etapas `prove` e `compress`; tempo final = `prove.seconds` + `compress.seconds`. Rótulo: etapa local do executor, fora da cadeia |
| §4.6 Cenários negativos | Adaptar | Mapa real na seção 3.7 |
| §4.7 Faixa de ambiente | Adaptar | "Versão do Verifier Router" vira o verificador Groth16. Ver 3.8 |
| §5 Fonte de verdade e proveniência | Adaptar | A UI não fala com o RPC direto (CSP `'self'`): "On-chain" = `job show` pelo worker, "lido da cadeia no slot N". "Calculado localmente" não ocorre no navegador; a variante cobre a prova local do executor |
| §6.1 O que destacar de Solana | Adaptar | Item 3: "CPI ao verificador Groth16 imutável" no lugar de "CPI ao Verifier Router". Prioridades mantidas |
| §6.2 Onde aparece | Adaptar | Novo job: vault PDA só depois do create (ver 3.1). Verificação: program ID do verificador, resultado, CU do verificador |
| §6.3 Regras de uso | Manter | Os links do Explorer já vêm do worker, com `cluster=devnet` |
| §6.4 Anatomia da transação | Adaptar | Instruções reais em 3.5 |
| §6.5 Carteira | Resolvida | B (seção 2) |
| §7 Identidade por papel | Manter integralmente | Valores do `DESIGN.md` |
| §8 Vocabulário | Adaptar | Trocas na seção 6 |
| §9 Estados vazio, carregando e erro | Manter | — |
| §10 Interação e acessibilidade | Manter | Confirmação antes de: financiar, enviar entrega e prova, negativos de demonstração e refund |
| §11 Componentes | Manter os nomes | `VerificationPanel` sem variante fallback ativa; `ScenarioOutcome` com os cenários reais |
| §12 Anti-padrões | Manter | O item 8 passa a ser: "liquidação sem verificador invocado exibida como verificada" |
| §13 Critérios de aceitação | Adaptar | Itens 4, 14 e 16: "Router" vira o verificador Groth16, e não há fallback. Item 13: lido da cadeia pelo worker, com o slot |
| §14 Decisões em aberto | Resolvidas | Seção 2 |
| §15 Ordem de trabalho | Adaptar | Passo 2: tokens com valores reais do `DESIGN.md`. Passo 11: não se aplica |

### 3.1 Novo job

- **Compromissos fixos, somente leitura, exibidos antes do envio:**
  - especificação "Regra v1: saída = 2 × entrada" com `spec_hash`;
  - `harness_hash`;
  - `image_id` admitido;
  - destinatário = executor configurado no worker;
  - valor 1 Test USDC;
  - mint `9TE2V…` identificado como token de teste.

  Os hashes são as constantes da v1, impressas por `vericode check` (`check.terms_v1`)
  e fixadas no programa. **Não são calculados no navegador.** O "hash nascendo na
  tela" do guia vira "hash fixado pelo programa", com a proveniência dita.
- **Único campo editável:** prazo, como deslocamento em slots de 1.560 a 9.000, com o
  slot atual ao lado e a tradução aproximada ("~", "estimativa").
- **Vault PDA e `job_id`:** a CLI gera o `job_id` aleatório no envio, e o vault
  depende dele. Por isso não existem antes do clique. A confirmação diz isso, e a tela
  os mostra assim que a CLI os imprime (`create.job_id`, `create.vault`).
- **"Financiar job"** cria e financia numa só transação. O compromisso fica imutável
  a partir daí; nenhum campo volta a ser editável.

### 3.2 Estados (linha do tempo e `StatusLabel`)

| Estado | Escopo | De onde vem |
| --- | --- | --- |
| `Draft` | interface | antes de `POST /api/jobs` |
| `Funded` | cadeia | `job show` |
| `Proving` | local (executor, fora da cadeia) | worker, `state_scope: local` |
| `Submitted` | worker (transação enviada, não reconciliada) | worker, `state_scope: worker` |
| `Delivered` | cadeia | `job show`. No fluxo do worker, `deliver` e a liquidação saem na mesma transação, então `Delivered` raramente aparece sozinho |
| `Released` | cadeia | `job show` (com `artifact_hash`) |
| `RefundedOnFail` | cadeia | `job show` (com `artifact_hash`) |
| `RefundedOnTimeout` | cadeia | `job show` |
| `Failed` | worker (erro operacional, sem veredito) | worker, `state_scope: worker`. Ramo separado, nunca na linha dos `Refunded*` |

- `Created` existe no programa, mas a CLI cria e financia atomicamente: nunca fica
  visível.
- Famílias e ícones seguem o `DESIGN.md`. `Delivered` usa a família `pending` com um
  ícone próprio, distinto de `Submitted`. `RefundedOnFail` e `RefundedOnTimeout` usam
  a família de `Refunded`, com o rótulo que diz qual.

### 3.3 Compromissos × observado

| Campo | Comprometido | Publicado no journal |
| --- | --- | --- |
| `schema_version` | não se aplica | journal, offset 0 (`u32`, sempre 1) |
| `job_id` | `job show` | offset 4 |
| `spec_hash` | `job show` | offset 36 |
| `harness_hash` | `job show` | offset 68 |
| `artifact_hash` | entrega do executor (`deliver`); aparece no `job show` depois da liquidação | offset 100 |
| `image_id` | `job show` | offset 132 |
| `verdict` | não se aplica | offset 164 (`PASS`=0, `FAIL`=1) |

- O journal é o `JournalV1` v1 congelado (165 bytes, `docs/manifest-schema.md`). O
  prover imprime o journal inteiro (`prove.journal_hex`).
- O "vínculo" (confere ou diverge) é uma comparação de exibição. **Quem barra é o
  programa**: 6014 (`job_id`), 6015 (spec), 6016 (harness), 6017 (artefato) e 6018
  (`image_id`). A tela diz isso.

### 3.4 Verificação

- **Local:**
  - `prove.receipt_type=Composite`, `prove.local_verify=ok`,
    `prove.journal_equal_to_core=true`;
  - `compress.receipt_type=Groth16`, `compress.local_verify=ok`;
  - `image_id` admitido; selector `73c457ba`;
  - a linha `docker_run` (imagem por digest, `--pull=never --network=none`);
  - tempos.
- **On-chain:**
  - a transação de liquidação com o verificador invocado;
  - programa `THq1q…` (verificador Groth16 de risc0-solana v3.0.0, imutável);
  - CU do verificador (cerca de 99.541);
  - resultado, assinatura e Explorer.
- **Sem liquidação ainda:** a camada on-chain mostra "Ainda não verificada on-chain"
  (família `pending`), **nunca** "fallback". O rótulo `verify.fallback` do `DESIGN.md`
  não é usado neste MVP.

### 3.5 Liquidação e anatomia

- **Instruções reais, em ordem:**
  - de topo, quando presentes: compute budget e `CreateIdempotent` da ATA;
  - `deliver` (escrow);
  - `release` ou `refund_on_fail` (escrow), com a CPI `THq1q…` (verificação Groth16) e
    a CPI do Token (`TransferChecked`).

  A atualização de estado acontece dentro de `release`/`refund_on_fail`, não como
  instrução separada. Os nomes vêm dos logs públicos da transação.
- **Motivo:**
  - "veredito PASS verificado" (`Released`);
  - "veredito FAIL verificado" (`RefundedOnFail`);
  - "prazo expirado sem liquidação" (`RefundedOnTimeout`).
- **Saldos antes e depois:** do vault e da ATA paga.

### 3.6 Reproduzir na CLI

- O comando é o argv real de cada operação (`GET /api/ops/<op_id>`, `steps[].argv`).
  Os binários aparecem pelo nome (`vericode`, `vericode-prover`), não pelo caminho
  local.
- As chaves aparecem como `<buyer-keypair>`/`<executor-keypair>`, como o worker já
  devolve.
- Antes de uma operação, a tela mostra o modelo de argv da tabela de
  `worker/README.md`, e depois o argv real.

### 3.7 Cenários negativos

| Cenário do guia | No MVP | Como acontece | Erro exibido |
| --- | --- | --- | --- |
| 1. FAIL válido | `RefundedOnFail`, com o verificador invocado | prova com saída ≠ 2 × entrada, depois liquidar | não é erro; veredito `FAIL` verificado |
| 2. Vínculo quebrado | receipt de outro Job | negativo `escrow-6014` (a receipt é sempre identificada) | `JournalJobIdMismatch` (6014, escrow) |
| 3. Replay / dupla liquidação | segunda liquidação | negativo `escrow-6007` | `AlreadyReleased` (6007, escrow); `AlreadyRefunded` (6008) só na evidência do D7 |
| 4. Proof incompatível | seal adulterado | negativo `verifier-6003` | `PairingError` (6003, **verificador** Groth16) |
| 5. `Failed` operacional | prova ou compressão falhou | estado `Failed` do worker; receipt nunca usada | sem transação |
| + Timeout antes do prazo | refund cedo | negativo `escrow-6021` (margem ≥ 300 slots) | `DeadlineNotReached` (6021, escrow) |
| + Timeout | refund depois do prazo | `refund-timeout` | não é erro; `RefundedOnTimeout` |

- **6015 a 6018** existem no programa, mas não há rota no worker para provocá-los.
  Só aparecem se ocorrerem.
- **Nomes dos erros:** os do escrow vêm da tabela de `docs/escrow-program.md`
  (6000–6036). Os do verificador são `VerificationError` (6000), `InvalidPublicInput`
  (6001), `ArithmeticError` (6002) e `PairingError` (6003).
- **6000 a 6003 colidem entre escrow e verificador.** O programa que rejeitou vem
  sempre de `rejection.program` (C10-7), nunca só do código. Sem `rejection`, a tela
  não diz "rejeitado" (`UNEXPECTED` aparece como tal).
- Os negativos são executados de propósito, para demonstrar que o contrato barra.
  Ficam agrupados como "Cenários adversariais (demonstração)", com confirmação, e
  nunca parecem uma decisão da interface.

### 3.8 Faixa de ambiente

- Cluster: Devnet.
- Token de teste: Test USDC, mint `9TE2V…`.
- `image_id` ativo: `4da06f90…fb1a`.
- Verificador: `THq1q…`, risc0-solana v3.0.0, imutável (SHA-256 dos bytes
  `34ae6e5c…`).
- Escrow: `vericode_escrow` `GZqb…`, upgrade authority `none` (`cdf6967f…`).
- Commit da aplicação: "Indisponível" com `TODO(data)` até o worker expor.

## 4. Mapa de dados (bloco → rota do worker)

| Bloco | Fonte |
| --- | --- |
| Jobs | `GET /api/jobs`; detalhes de cada um por `GET /api/jobs/<id>` |
| Novo job | `GET /api/health` (partida: `check`, termos v1, pubkeys); `POST /api/jobs {"deadline_offset": N}`; depois `GET /api/ops/<op_id>` |
| Cabeçalho, A e D | `GET /api/jobs/<id>` (`state`, `state_scope`, `chain`, `ops`); `POST /api/jobs/<id>/show` para reler a cadeia |
| B | `chain` (comprometido) e o journal da receipt (lacuna 1) |
| C | `receipt` do Job (prova local) e a operação de liquidação (`transaction.verifier_invoked`, `verifier_units`) |
| D, anatomia e saldos | operação de liquidação (lacunas 2 e 3) |
| F | `GET /api/ops/<op_id>` (`steps[].argv`) |
| Proving | `GET /api/ops/<op_id>` (`started_at`, `steps`) e `GET /api/jobs/<id>` (`state=Proving`) |
| Faixa | `GET /api/health` (`startup`) |

Atualização: polling de `GET /api/ops/<op_id>` e `GET /api/jobs/<id>`, sem fingir
sincronia. Uma falha de leitura é mostrada como falha, com o último valor marcado como
desatualizado (slot e horário).

## 5. Lacunas do worker (o D11 resolve sem tocar em CLI, prover ou programa)

1. **Campos do journal:** guardar o `prove.journal_hex` e decodificar os campos pelos
   offsets congelados (seção 3.3), com teste contra as fixtures de
   `anchor/tests-local/fixtures/groth16/`. É leitura de um formato congelado, não regra
   nova.
2. **Anatomia:** expor uma lista sanitizada das invocações da transação de
   liquidação (programa, profundidade, instrução, CU, sucesso ou falha), tirada dos
   logs públicos que a CLI já grava no `--log`.
3. **Saldos antes e depois:** estruturar as linhas `settle.*`/`refund.*` e o vault.
4. **Termos v1 e slot atual antes do create:** expor do relatório de partida, ou de
   um `check`, sem RPC no navegador.
5. **Commit da aplicação** (opcional): senão, "Indisponível" com `TODO(data)`.
6. **Rotas estáticas fixas** para `worker/static/` (seção 7).

## 6. Frases, marca e vocabulário

- **Frase 1 na interface:** "A receipt Groth16 da Hive é verificada em devnet por CPI
  ao verificador Groth16 imutável de risc0-solana v3.0.0, na mesma instrução que libera
  ou devolve o Test USDC do Job."
  - Só a palavra de marca muda; o claim é o mesmo e vai sempre junto do link da
    liquidação.
  - **Em inglês** (padrão da interface, D-EN-1 e D-EN-2, entrada R-UI): "Hive's Groth16
    receipt is verified on devnet via CPI to the immutable Groth16 verifier of
    risc0-solana v3.0.0, in the same instruction that releases or refunds the Job's Test
    USDC." Mesmo claim, mesma regra do link.
  - O `README.md` e o roteiro mantêm o texto atual até uma decisão própria sobre a
    documentação.
- As frases 2 a 8 não citam a marca e valem como estão. A lista "Não dizer" do
  `README.md` vale inteira. Em inglês, as 8 frases e o "Do not say" ratificados estão na
  seção "Frozen phrases (English, ratified R-UI)" do `README.md`. "Verifier Router" nunca aparece como caminho atual.
- **Troca de vocabulário do guia:**

  | Guia | Na interface |
  | --- | --- |
  | "Proof verificada no Verifier Router." | "Proof verificada on-chain pelo verificador Groth16 (CPI direta)." |
  | "Fallback: não verificado on-chain." | não usado (não há fallback) |

- **Identificadores técnicos** aparecem como existem: `vericode`, `vericode-prover`,
  `vericode_escrow`, `VericodeEscrowError`, os nomes de erro e os program IDs. O
  header `X-VeriCode-Token` é usado pelo JavaScript com esse nome exato e não é
  exibido.
- `<title>` e textos institucionais: **Hive**. O mascote e o favicon ficam fora: o
  favicon ainda não existe (`brand/MANIFEST.md`, `TODO(brand)`).

## 7. Stack e assets

- **Execução:**
  - HTML, CSS e JavaScript (módulos ES), sem framework, sem bundler e sem dependência,
    servidos pelo próprio worker a partir de `worker/static/`;
  - CSP `'self'`; sem CDN.
  - Mantém a decisão P2 para o que roda.
- **Tokens:**
  - gerados do `DESIGN.md` com `design.md export` e versionados como
    `worker/static/tokens.css` (prefixo `--hive-`);
  - camada de papéis claro/escuro conforme o "Mapa de papéis" do `DESIGN.md`;
  - um teste confere que o arquivo versionado é o que o `DESIGN.md` gera.
- **Fontes:** `brand/fonts/` (Google Fonts, OFL, `brand/fonts/README.md`) copiadas
  para `worker/static/fonts/`.
- **Marca:** `brand/logo/hive-horizontal-branco.svg` na navegação, copiado para
  `worker/static/brand/`.
- **Ícones:** Lucide (ISC), só os SVGs usados, copiados de `lucide-static` para
  `worker/static/icons/`, com a licença.
- **Cópias verificadas:** toda cópia de `brand/` ou de `lucide-static` para
  `worker/static/` vem de uma tabela fixa e é conferida byte a byte (SHA-256) por
  teste. Não há pipeline de build em tempo de execução.
- **Ferramentas de desenvolvimento** (fora do caminho de execução):
  - `worker/ui-tools/` com `@google/design.md` 0.4.0, `lucide-static` 1.52.0 e
    `playwright` 1.63.0;
  - Node 24 isolado em `~/.local/share/vericode-spikes/ui/env-ui.sh`;
  - o Playwright tira as capturas de verificação visual que o `DESIGN.md` pede.
