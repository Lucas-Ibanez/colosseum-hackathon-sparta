# `worker/` — worker local do VeriCode (D10) e interface Hive (D11–D12)

Servidor HTTP mínimo, só em `127.0.0.1`, que executa o fluxo do MVP chamando
apenas os binários do D10a: `vericode` (cliente de devnet, [`cli/`](../cli/README.md))
e `vericode-prover` (prova local, [`prover/`](../prover/README.md)). Desde o D11–D12,
também serve a interface Hive (`static/`, em `http://127.0.0.1:<porta>/ui/`). Python
3.12, só a biblioteca padrão; a interface é HTML, CSS e JavaScript sem dependência.

O worker não decide nada econômico. Cada ação é uma lista de argv fixa, montada no
servidor, sem shell, com ambiente construído do zero, uma por vez. Toda regra continua
no programa em devnet e nas conferências da CLI. O estado mostrado vem de
`vericode job show` e do `--log` JSONL da CLI, nunca só do código de saída.

Decisões e condições: "Decisões humanas para o D10" (P1–P6) e o veredito do R-D10a
(C10-1 a C10-10) em [`docs/decisions.md`](../docs/decisions.md) e
[`docs/r-d10a-review-results.md`](../docs/r-d10a-review-results.md).

## Limitações (declaradas)

- **Sem carteira no navegador (P3).** As chaves de devnet do projeto (buyer e
  executor) ficam num worker local, por caminho, e o mesmo operador local opera os
  dois papéis. A chave do deployer (mint authority do Test USDC) nunca entra no worker.
- `Proving` é uma etapa local do executor, fora da cadeia.
- Só devnet e Test USDC. O claim continua o do CD7, com as frases congeladas do
  [`docs/README.pt-BR.md`](../docs/README.pt-BR.md#frases-permitidas-congeladas-no-d9); o worker não cria frase.
- `http.server` não é endurecido para rede: por isso o bind é fixo em `127.0.0.1`.
- Se o `compress` estourar o tempo, o worker mata o grupo do prover; um container
  já iniciado termina sozinho (`--rm`) e a receipt não é usada.

## Configuração (fora do clone)

Um arquivo JSON com modo `0600`, fora de qualquer work tree Git, com exatamente estes
campos (exemplo com caminhos fictícios):

```json
{
  "cli": "/caminho/d10a/targets/cli/release/vericode",
  "cli_sha256": "e6cd4e29…",
  "prover": "/caminho/d10a/targets/prover/release/vericode-prover",
  "prover_sha256": "3f66e1c0…",
  "shim": "/home/lucas/src/vericode/prover/docker-shim/docker",
  "shim_sha256": "2a8f75b8…",
  "buyer_keypair": "/caminho/fora/do/clone/buyer.json",
  "buyer_pubkey": "<pubkey do buyer>",
  "executor_keypair": "/caminho/fora/do/clone/executor.json",
  "executor_pubkey": "<pubkey do executor>",
  "data_dir": "/caminho/d10/data",
  "home_dir": "/caminho/d10/home",
  "tmp_dir": "/caminho/d10/tmp",
  "port": 8710
}
```

O programa recusa iniciar se:
- os hashes não forem os fixados no código (CLI `e6cd4e29…`, prover `3f66e1c0…`, shim
  `2a8f75b8…`); os binários do D9 (`7e7a9260…`/`79b83528…`) são recusados pelo nome
  (C10-1);
- o shim não for o do repositório que o prover do D10a executa, com modo `0755`, ou o
  binário do prover não embutir esse diretório (C10-2);
- algum diretório não tiver modo `0700`, estiver em `/tmp` ou dentro de work tree;
- uma pubkey for a do deployer, ou buyer e executor forem a mesma.

## Partida

```bash
python3 -B worker/vericode_worker.py --config /caminho/fora/do/clone/worker.json
```

1. Confere o SHA-256 da CLI, do prover e do shim (e o modo do shim).
2. **Sondas de chave, sem rede.** O worker nunca abre arquivo de chave. Quem lê é a CLI,
   com as guardas dela (`0600`, um hard link, fora de work tree):
   `vericode job create --buyer-keypair <B> --executor 1` e
   `vericode job deliver --executor-keypair <E> --job-id 00`. As duas carregam a chave,
   imprimem só `buyer=<pubkey>`/`executor=<pubkey>` e falham no argumento inválido,
   antes de qualquer RPC. A pubkey impressa tem de ser a configurada, e nunca a do
   deployer.
3. Preflight duplo (C10-1): `vericode check` com `check=ok` e as linhas exatas do
   escrow, do mint e do verificador (`bytes=199256 sha256=34ae6e5c…`);
   `vericode-prover check` com o guest admitido e `prover=LocalProver env=ok`.
4. Carrega o estado de `data_dir`; uma operação interrompida por queda deixa o Job
   pendente de reconciliação.
5. Gera um token aleatório por execução e o imprime **só no terminal do operador**
   (`worker.token=…`), nunca num log do worker.
6. Imprime também `worker.ui=http://127.0.0.1:<porta>/ui/`, o endereço da interface.

## HTTP (C10-6)

- Bind fixo em `127.0.0.1`. `Host` precisa ser exatamente `127.0.0.1:<porta>` (senão
  421; ausente ou repetido, 400).
- Toda rota `/api/*` exige o header `X-VeriCode-Token` (ausente 401, errado 403,
  comparação `hmac.compare_digest`). O header próprio força o preflight de CORS num
  navegador, e o worker não responde CORS: nenhuma resposta tem `Access-Control-*`.
- Só GET e POST; rotas fixas; sem query string. Um caminho que o `http.server`
  reescreveu (por exemplo, `//` inicial colapsado) recebe 400. POST com JSON
  (`Content-Type: application/json`, até 4 KiB, chaves exatas, inteiros JSON).
- Respostas JSON com `Cache-Control: no-store`, `nosniff` e
  `Content-Security-Policy: default-src 'none'`. A saída da CLI vem como texto.
- Uma operação de CLI/prover por vez: a segunda recebe 409 com a operação em curso
  (C10-4). GETs nunca executam processo nem recebem 409.

| Rota | Modo | Ação |
| --- | --- | --- |
| `GET /api/health` | imediato | resultado da partida, hash e modo do shim agora, operação em curso |
| `GET /api/jobs` | imediato | Jobs conhecidos pelo worker |
| `GET /api/jobs/<job_id>` | imediato | visão reconciliada (último `job show` lido) |
| `GET /api/ops/<op_id>` | imediato | registro da operação, com argv, stdout e stderr |
| `POST /api/check` `{}` | síncrono | os dois `check` de novo |
| `POST /api/jobs` `{"deadline_offset": N}` | 202 + `op_id` | create; N ∈ [1.560, 9.000]; valor fixo de 1.000.000 (1 Test USDC); executor da configuração |
| `POST /api/jobs/<job_id>/show` `{}` | síncrono | `job show` de qualquer Job |
| `POST /api/jobs/<job_id>/prove` `{"input": u32, "claimed_output": u32}` | 202 | prova local + compressão Groth16 |
| `POST /api/jobs/<job_id>/settle` `{}` | 202 | `deliver` + liquidação com a receipt do Job |
| `POST /api/jobs/<job_id>/refund-timeout` `{}` | 202 | reembolso por timeout de qualquer Job, pago pelo buyer |
| `POST /api/jobs/<job_id>/negative/<kind>` | 202 | negativo da CR2: `escrow-6021`, `verifier-6003`, `escrow-6007`, `escrow-6014` (`{"receipt_job_id": …}`, a receipt de outro Job deste worker, sempre identificada) |

`prove`, `settle` e os negativos só valem para Jobs criados por este worker.

### Interface Hive (D11–D12): `/ui/`

- Só os arquivos da tabela fixa `STATIC_FILES` do código, por igualdade exata do
  caminho, só GET (outros métodos: 405). `/ui/` é o `index.html`. Fora da tabela,
  `..`, `%2e%2e`, barra dupla ou diretório: 404; query: 400. Nada é listado.
- Os estáticos não pedem token: a página pede o token ao operador, guarda-o só na
  memória da aba e o envia no `X-VeriCode-Token` de cada chamada à API. Toda rota
  `/api/*` continua exigindo o token.
- Cabeçalhos das páginas: `Content-Security-Policy: default-src 'none'; script-src
  'self'; style-src 'self'; font-src 'self'; img-src 'self'; connect-src 'self';
  base-uri 'none'; form-action 'none'; frame-ancestors 'none'`, `nosniff`,
  `Referrer-Policy: no-referrer`, `Cache-Control: no-store`, `X-Frame-Options: DENY`;
  nenhum `Access-Control-*`.
- O JavaScript monta o DOM só com `textContent` (sem `innerHTML`, `eval` nem
  armazenamento do navegador). Fontes, logotipo e ícones são cópias verificadas
  (`ui-tools/static-assets.json`); `css/tokens.css` é gerado do `DESIGN.md`
  ([`ui-tools/README.md`](ui-tools/README.md)).

### Visões só de leitura para a interface (D11–D12)

Derivadas do que o worker já guarda; nenhuma decide nada nem chama processo novo:

| Onde | Campo | Origem |
| --- | --- | --- |
| `GET /api/health` | `v1` | termos v1, escrow, verificador e mint das linhas do `vericode check` da partida; pubkeys das sondas; valor fixo e janela de prazo |
| `GET /api/health` | `slot_clock` | último slot lido por um `job show` (`slot`, `read_at`, `job_id`) e segundos por slot medidos entre a leitura mais antiga e a mais nova (nulo com menos de 10 min entre elas): só estimativa |
| `GET /api/health` | `app_commit` | `.git/HEAD` na partida (leitura de arquivo) |
| `GET /api/jobs` | `amount`, `deadline_slot`, `read_slot`, `read_at`, `past_deadline`, `receipt_status`, `receipt_verdict` | último `job show` e última receipt |
| `GET /api/jobs/<id>` | `receipt.journal` | `compress.journal_hex` (igual a `prove.journal_hex`) decodificado pelos offsets congelados do `JournalV1` |
| `GET /api/jobs/<id>` | `receipt.local` | linhas reais do `prove`/`compress` (`receipt_type`, `local_verify`, `selector`, tempos…) |
| `GET /api/ops/<id>` | `anatomy` | do último registro `tx` do `--log`: `top_level` (`programs`), `invocations` (logs públicos: programa, profundidade, instrução, CU, resultado, erro Anchor) e `balances` (`pre/postTokenBalances`, com o papel do dono) |
| `GET /api/ops/<id>` | `running_step`, `create_live` | etapa em curso (`prove`/`compress`) e `create.job`/`create.vault`/`create.slot` assim que a CLI os imprime |

## argv fixo e ambiente (C10-3)

`LOG` é um arquivo novo por operação em `data_dir/ops/<op_id>/`; `RD` é um diretório
novo por prova em `data_dir/receipts/<job_id>/<n>`.

| Ação | argv |
| --- | --- |
| check | `vericode check` · `vericode-prover check` |
| show | `vericode job show --job-id J` |
| create | `vericode --log LOG job create --buyer-keypair B --executor E --amount 1000000 --deadline-offset N` |
| prove | `vericode-prover prove J input claimed_output RD` → `vericode-prover compress RD` |
| settle | `vericode --log LOG job settle --job-id J --receipt RD --deliver --executor-keypair EK` |
| refund-timeout | `vericode --log LOG job refund-timeout --job-id J --payer-keypair BK` |
| escrow-6021 | `job show` → `vericode --log LOG job refund-timeout --job-id J --payer-keypair BK --expect-error escrow:6021` |
| verifier-6003 | `vericode --log LOG job settle --job-id J --receipt RD --deliver --executor-keypair EK --tamper-seal --expect-error verifier:6003` |
| escrow-6007 | `vericode --log LOG job settle --job-id J --receipt RD --payer-keypair EK --expect-error escrow:6007` |
| escrow-6014 | `vericode --log LOG job settle --job-id J --receipt RD(outro Job) --deliver --executor-keypair EK --expect-error escrow:6014` |

Ambiente dos filhos, inteiro: `HOME`, `PATH=/usr/bin:/bin` e `TMPDIR` da configuração;
o prover recebe também `RISC0_PROVER=local`. Nada é herdado do worker: nem `BONSAI_*`,
`RISC0_*`, `VERICODE_*`, `DOCKER_*` ou proxies. `stdin` fechado, `cwd` = `home_dir` e
timeout explícito por ação. O hash do binário é conferido antes de cada execução.
Nenhuma escrita é repetida automaticamente (C10-5).

Caminhos de chave nunca saem do worker: argv, stdout e stderr são guardados e
devolvidos com `<buyer-keypair>`/`<executor-keypair>` no lugar dos caminhos. Um array
JSON de 64 números numa saída é suprimido e registrado como incidente.

## Estados (P1, C10-5)

A verdade econômica é `chain`, o último `job show` lido (`Funded`, `Delivered`,
`Released`, `RefundedOnFail`, `RefundedOnTimeout` ou ausente). O campo `state`:
- `Proving` (`state_scope: local`): prova em curso, etapa local do executor, fora da
  cadeia;
- `Submitted` (`state_scope: worker`): escrita em curso cuja assinatura a CLI já imprimiu,
  ou escrita cujo resultado ainda não foi confirmado por `job show`;
- `Failed` (`state_scope: worker`): erro operacional — a última prova falhou (a receipt
  nunca é usada) com o Job ainda `Funded`, ou um create sem Job em cadeia. Não é
  `Verdict::Fail`;
- senão, o estado da cadeia (`state_scope: chain`). `Draft` existe só na interface,
  antes do `POST /api/jobs`.

Toda escrita termina com `job show`, sob o mesmo lock. O `job_id` de um create é
registrado assim que a CLI o imprime, antes de simular. Se a leitura de reconciliação
falhar, o Job fica pendente, e o worker recusa tudo (409) menos o `show` desse Job, até
lê-lo.

## Prova e shim (C10-2, C10-4)

1. `prove` num diretório novo; exige `Composite`, `local_verify=ok` e
   `journal_equal_to_core=true`.
2. `compress` só com `MemAvailable` ≥ 2,5 GiB e com o shim conferido logo antes
   (SHA-256 `2a8f75b8…`, modo `0755`, arquivo regular).
3. Depois, exige `Groth16`, o shim do repositório e **exatamente uma** linha
   `compress.docker_run=` desta execução, igual a
   `<horário> /usr/bin/docker --context default run --pull=never --network=none --rm -v <RD>/groth16-work:/mnt risczero/risc0-groth16-prover@sha256:7f173963…`,
   com o mesmo conteúdo em `RD/docker-shim.log`.
4. Faltou qualquer item: estado `Failed`, e `settle` e os negativos recusam com 409.

## Negativos e rótulo (C10-7, C10-8)

A operação traz `rejection = {program, program_id, code, innermost, signature,
explorer}` só quando a saída da CLI mostra tudo: o `--expect-error` usado, a linha
``simulation shows `Program <id> failed: custom program error: 0x…` as the innermost failure``
desse programa e código, `[label] PASS`, o registro `PASS` no log e `Custom(code)` na
transação. `UNEXPECTED`, ou qualquer falta, dá `rejection: null`. O `escrow-6021` lê o
Job logo antes e só envia com `deadline_slot − slot ≥ 300`.

## Testes

```bash
TMPDIR=<diretório fora de /tmp> python3 -B -m unittest discover -s worker/tests -v
```

`test_worker.py` (23 testes do D10) e `test_static.py` (D11–D12). Offline, com os
executáveis falsos rotulados de [`tests/fakes/`](tests/fakes/) (que
registram argv, `cwd` e o ambiente exato) e chaves falsas que o worker nunca lê. Cobrem:
hashes e configuração fixada; sondas de chave; preflight; shim (hash, modo, linha
`docker_run`, memória); ambiente dos filhos sob variáveis hostis; argv exato do fluxo
inteiro; parâmetros inválidos; `Host`, token e ausência de CORS; 409; `UNEXPECTED`
nunca vira rejeição; reconciliação depois de exit ≠ 0, de create falho e de reinício;
C10-8; estados `Proving`/`Submitted`; nenhum caminho ou byte de chave em resposta,
log ou arquivo do worker. Desde o D11–D12 (`test_static.py`): rotas estáticas, cabeçalhos e
recusas; cópias por SHA-256; `tokens.css` contra o `DESIGN.md`; journal contra as
fixtures Groth16; anatomia e saldos contra registros públicos reais do D10
(`tests/fixtures/d10-tx/`); `v1`, `slot_clock` e commit; varredura do JS, HTML e CSS
(APIs proibidas, valores literais, frases congeladas, termos proibidos).
