# D10 — worker local com prova local forçada (base das telas)

Data: 2026-10-06 (18:57–20:05 -03:00) · Executor: Claude Code (Opus 5.5) · Gate
anterior: R-D10a (`544b685`, `docs: record R-D10a review`, sobre `2a2e3c5`) · Prompt:
`docs/handoffs/r-d10a-to-d10.md`, com as "Decisões humanas para o D10"
(`docs/decisions.md`) · Commits: `6eba814` (`worker: add local proof worker (D10)`) e
`docs: record D10 worker`.

## Resultado

**D10 CONCLUÍDO.** `worker/` tem um worker HTTP local em Python 3.12 (só biblioteca
padrão), em `127.0.0.1`, que só chama os binários do D10a (CLI `e6cd4e29…`, prover
`3f66e1c0…`) por argv fixo e com ambiente construído do zero.
- Testes `unittest` offline: **23/23** (`worker/tests/`), com executáveis falsos
  rotulados.
- Partida real: hashes, sondas de chave sem rede e preflight duplo (`check=ok` com o
  verificador `34ae6e5c…`; `prover=LocalProver env=ok`).
- **W1 a W7 em devnet, todos pela API do worker**, na ordem aprovada; os dois Jobs das
  gravações reembolsados; um Job novo (`bc334093…`) liquidado por PASS com o
  verificador invocado; três negativos com o programa que rejeitou identificado só a
  partir da saída da CLI (C10-7).
- Saldos fecham por lamport e por unidade com o previsto no plano.

| # | Rota | Resultado | Transação |
| --- | --- | --- | --- |
| W1 | `POST /api/jobs/8ab4ee8d…/refund-timeout` | `RefundedOnTimeout`; buyer +1 Test USDC | [`57UYbVX9…`](https://explorer.solana.com/tx/57UYbVX9gEXdp7dproD5mEyxj5UzxuQZdjwhQZe96XKTfCKbrWSxrfhKTmqrTdnAby8PGyJVSa1gDZsdk7P9Xd9v?cluster=devnet) |
| W2 | `POST /api/jobs/8bce67f2…/refund-timeout` | `RefundedOnTimeout`; buyer +1 Test USDC | [`625JvmR8…`](https://explorer.solana.com/tx/625JvmR85QE6LbRZaE49kf3Y2cSFVnbKGaynVBbb8eeqM1Dj6r8UVUZiUbDoA4g5egJfqYUW175pG6d4NHYZWc3y?cluster=devnet) |
| W3 | `POST /api/jobs` `{"deadline_offset": 9000}` | Job `bc334093…` criado e financiado (`Funded`, vault 1.000.000) | [`3XX9PWN2…`](https://explorer.solana.com/tx/3XX9PWN2ZbvB9EntVvAt3S3444Zbo6cvoRyh5vkiQDqqZnDSQYTND11sBYrpJsGRjB7CpTAffZFhUz5ddguMsVpT?cluster=devnet) |
| W4 | `POST /api/jobs/bc334093…/negative/escrow-6021` | margem C10-8 de 8.905 slots; rejeitado pelo escrow (6021), contas iguais | [`3PJfg2jU…`](https://explorer.solana.com/tx/3PJfg2jUmmDVc5RuN31DgF3pJwcYnog8GUCYy9vvyB56n1drScwiY8dTTsXwZWsX59DCuRDn33YhwViff3shxv4L?cluster=devnet) |
| — | `POST /api/jobs/bc334093…/prove` `{"input": 21, "claimed_output": 42}` | prova local `Composite` 7,2 s → `Groth16` 139,6 s; linha `docker_run` do C10-2 | (não é escrita) |
| W6 | `POST /api/jobs/bc334093…/negative/verifier-6003` | seal adulterado rejeitado pelo **verificador** (6003, 100.528 CU), contas iguais | [`4cKK83JB…`](https://explorer.solana.com/tx/4cKK83JBYxRVfrAT5t6visKQdEWim5NANMe3eMH4bHsDmPumzq3WZb83F4eWiuYC694uivPqhsip3D8ySxJBmX2u?cluster=devnet) |
| **W5** | `POST /api/jobs/bc334093…/settle` | **`Released { 225384a7… }`**; verificador invocado por CPI (99.541 CU) na mesma instrução; executor +1 Test USDC | [`ByGF4BFP…`](https://explorer.solana.com/tx/ByGF4BFPD8zkyqc6bg3Hd2FnQ1bF9gjo1EWp4fyVQkcGWfcj53gi99kWPLgyherzmvYp8knoqAzgjLLyhgpLnVR?cluster=devnet) |
| W7 | `POST /api/jobs/bc334093…/negative/escrow-6007` | segunda liquidação rejeitada pelo escrow (6007), contas iguais | [`5ug6Pggx…`](https://explorer.solana.com/tx/5ug6PggxL9qZdgwkCu1Vt3oEYSFiBKXLdCi9bE6W1bewLjfa2yKpYjf8BeHzEjtvdkYDNLHZY4cjjBvkqtF9Bn5w?cluster=devnet) |

Fronteiras:
- Nada mudou em `cli/`, `prover/` (o shim continua `2a8f75b8…`, `0755`), programa, core,
  `zkvm/`, guest, `JournalV1`, schema ou locks. Nenhuma dependência, crate, npm ou lock
  novo; nenhum rebuild.
- Escritas em devnet só W1–W7. Nenhum airdrop, deploy, pull ou keypair novo.
- Nenhum caminho nem conteúdo de chave saiu do worker (F5).

O claim continua o do CD7, e as frases congeladas não mudaram. Este gate não cria frase.

## Linha do tempo (2026-10-06, -03:00)

| Hora | Fase |
| --- | --- |
| 18:57–19:12 | F0: preflight Git, perfil, locks, guest, binários, shim; leitura obrigatória; devnet só leitura; snapshots |
| 19:12–19:18 | Plan Mode; plano aprovado pelo humano às 19:18 (rotas, argv, estados, testes e os comandos HTTP de W1–W7) |
| 19:18–19:41 | F1: `worker/` (código, falsos, testes, README) |
| 19:41–19:42 | F2: `unittest` 23/23 |
| 19:43 | F3: partida real e higiene HTTP |
| 19:43–19:50 | F4: W1, W2, W3, W4, prova, W6, W5, W7 pela API |
| 19:50–19:53 | F5: `getTransaction` das 7 assinaturas, carteiras, estado final, worker encerrado, fronteiras e segredos |
| 19:53– | F6: documentação, revisão do diff, commits |

## F0 — preflight e checagem do R-D10a

| Checagem | Resultado real |
| --- | --- |
| Git | `/home/lucas/src/vericode`, `main`; `544b685 → 2a2e3c5 → 7fe9c3b`; `status --short` e `--ignored` vazios; `diff --check` 0 |
| Perfil (início e fim) | `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes; `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| Locks (início e fim) | raiz `191802b2…`, zkvm `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover` `8b76f1e1…`; guest 180.300 B `e09ba8cf…` |
| Binários (C10-1) | CLI `e6cd4e2914fc…acae`, prover `3f66e1c05a6d…5232` (`d10a/targets/*/release`); o prover contém `/home/lucas/src/vericode/prover` (o diretório do shim embutido); os do D9 (`7e7a9260…`/`79b83528…`) não foram usados |
| Shim (C10-2) | `prover/docker-shim/docker` `2a8f75b87766…d866`, modo `0755`, arquivo regular |
| Preflight duplo | `vericode check` → `check=ok`, `check.verifier=THq1q… program_data=ENdLkqHp… upgrade_authority=none bytes=199256 sha256=34ae6e5c…`; `vericode-prover check` → `prover=LocalProver env=ok` |
| Devnet, só leitura | `8ab4ee8d…` e `8bce67f2…` `Funded`, vault 1.000.000, prazos 508.078.115 e 508.089.477 vencidos (slot 508.237.749); P′ `Released { 225384a7… }`; T′ `RefundedOnTimeout` |
| Saldos (slot 508.237.808) | deployer 2.805.194.240; buyer 103.351.800; executor 29.925.000 lamports; ATA do buyer 999.992.000.000; ATA do executor 6.000.000 — iguais ao fim do R-D10a |
| `getSignaturesForAddress` | desde as últimas assinaturas das gravações (`5CR2s5Jb…`, `4W3Gn3vc…`, `24oguMEB…`): 0 novas nas três carteiras |

Raiz própria `~/.local/share/vericode-spikes/d10/` (`0700`, arquivos `0600`), com
`bin/`, `data/`, `home/`, `logs/` e `tmp/`. Scripts com prefixo `w10_`; o `env.sh`
herdado não foi carregado, e `R`, `B`, `D` e `VC` não foram usados. Os `job_id`s das
gravações vieram só do campo público `job_id` de `rec-*/jobs/*.json`.

## F1 — o worker

Código em `worker/vericode_worker.py`; detalhes em [`worker/README.md`](../worker/README.md).

- **Configuração** fora do clone (`d10/worker.json`, `0600`): caminhos dos binários, do
  shim e das chaves do buyer e do executor, hashes, pubkeys, diretórios e porta 8710.
  `load_config` exige os hashes fixados no código (C10-1/C10-2), recusa os do D9 pelo
  nome, o shim fora de `/home/lucas/src/vericode/prover/docker-shim/docker`, a pubkey do
  deployer, diretórios sem `0700`, em `/tmp` ou dentro de work tree.
- **Partida:** hashes e modo do shim; o binário do prover tem de embutir o diretório do
  shim; **sondas de chave sem rede** (o worker nunca abre um arquivo de chave):
  `job create --buyer-keypair B --executor 1` e
  `job deliver --executor-keypair E --job-id 00` imprimem só a pubkey e falham no
  argumento antes de qualquer RPC; preflight duplo com as linhas exatas; token aleatório
  só no terminal do operador.
- **HTTP (C10-6):** bind fixo em `127.0.0.1`; `Host` exatamente `127.0.0.1:8710`
  (senão 421); `X-VeriCode-Token` em toda rota `/api/*`, inclusive GET
  (`hmac.compare_digest`); nenhum cabeçalho CORS; só GET e POST; rotas fixas; nenhum
  arquivo servido; JSON estrito.
- **Uma operação por vez (C10-4):** lock global não bloqueante, 409 com a operação em
  curso. GETs nunca rodam processo (a interface consulta o estado durante `Proving`).
- **Estado (C10-5):** toda escrita termina com `job show` sob o mesmo lock; o resultado
  vem do `--log` JSONL e do `show`; o `job_id` do create é gravado assim que a CLI o
  imprime; reconciliação falha → 409 em tudo, menos o `show` desse Job.

### Rotas

| Rota | Modo | Ação |
| --- | --- | --- |
| `GET /api/health`, `/api/jobs`, `/api/jobs/<job_id>`, `/api/ops/<op_id>` | imediato | partida, Jobs, visão reconciliada, operação |
| `POST /api/check` | síncrono | os dois `check` |
| `POST /api/jobs` `{"deadline_offset": N}` | 202 | create, N ∈ [1.560, 9.000], 1.000.000 fixo |
| `POST /api/jobs/<id>/show` | síncrono | `job show` |
| `POST /api/jobs/<id>/prove` `{"input", "claimed_output"}` (u32) | 202 | prove + C10-4 + C10-2 + compress |
| `POST /api/jobs/<id>/settle` | 202 | settle positivo |
| `POST /api/jobs/<id>/refund-timeout` | 202 | refund positivo, pago pelo buyer |
| `POST /api/jobs/<id>/negative/<kind>` | 202 | `escrow-6021`, `verifier-6003`, `escrow-6007`, `escrow-6014` (este implementado e testado offline; fora do W1–W7) |

### argv executado em devnet (marcadores no lugar dos caminhos de chave)

```text
partida  vericode job create --buyer-keypair <buyer-keypair> --executor 1
         vericode job deliver --executor-keypair <executor-keypair> --job-id 00
         vericode check ; vericode-prover check
W1/W2    vericode job show --job-id J
         vericode --log data/ops/<op>/cli-tx.jsonl job refund-timeout --job-id J --payer-keypair <buyer-keypair>
         vericode job show --job-id J
W3       vericode --log … job create --buyer-keypair <buyer-keypair> --executor EdB25bVh… --amount 1000000 --deadline-offset 9000
         vericode job show --job-id bc334093…
W4       vericode job show --job-id bc334093…                    (C10-8)
         vericode --log … job refund-timeout --job-id bc334093… --payer-keypair <buyer-keypair> --expect-error escrow:6021
         vericode job show --job-id bc334093…
prova    vericode-prover prove bc334093… 21 42 data/receipts/bc334093…/1
         vericode-prover compress data/receipts/bc334093…/1
W6       vericode --log … job settle --job-id bc334093… --receipt data/receipts/bc334093…/1 --deliver --executor-keypair <executor-keypair> --tamper-seal --expect-error verifier:6003
W5       vericode --log … job settle --job-id bc334093… --receipt data/receipts/bc334093…/1 --deliver --executor-keypair <executor-keypair>
W7       vericode --log … job settle --job-id bc334093… --receipt data/receipts/bc334093…/1 --payer-keypair <executor-keypair> --expect-error escrow:6007
         (cada escrita seguida de vericode job show --job-id …)
```

Ambiente de cada filho, inteiro (registrado em cada passo de `data/ops/<op>/op.json`):
`HOME=d10/home`, `PATH=/usr/bin:/bin`, `TMPDIR=d10/tmp`; o prover também
`RISC0_PROVER=local`. `cwd=d10/home`, `stdin` fechado. O worker foi iniciado com
`env -i HOME=… PATH=/usr/bin:/bin LANG=C.UTF-8`; os testes mostram que nem um ambiente
hostil no worker chega aos filhos.

### Estados

`chain` é o último `job show` (verdade econômica). `state`: `Proving` (local, fora da
cadeia), `Submitted` (assinatura impressa ou resultado ainda não confirmado), `Failed`
(prova inutilizável com o Job `Funded`, ou create sem Job), senão o estado da cadeia.
`Draft` é só da interface. Durante a prova do F4, `GET /api/jobs/bc334093…` respondeu
`Proving` com `state_scope: local`.

## F2 — testes (`unittest`, offline)

```text
$ TMPDIR=~/.local/share/vericode-spikes/d10/tmp python3 -B -X dev -m unittest discover -s worker/tests -v
Ran 23 tests in 28.200s
OK                     (exit 0; RSS 51 MB; sem ResourceWarning; nada criado no clone)
```

`vericode_worker.py` `e276e588…`, `test_worker.py` `9f88d0eb…` (versões testadas antes da
partida real). Os falsos (`tests/fakes/fake-vericode`, `fake-vericode-prover`) são
rotulados "TEST FAKE", registram argv, `cwd` e o ambiente lido de
`/proc/self/environ`, e reproduzem respostas roteirizadas; as chaves dos testes são
arquivos falsos que o worker nunca lê.

| Grupo | Testes | Cobre |
| --- | --- | --- |
| Partida (6) | binário com hash errado (CLI, prover, alterado depois) → recusa, nada roda; `load_config` só aceita os valores fixados (D9 pelo nome, outro hash, outro shim, deployer, porta, campo extra, modo `0644`, arquivo no work tree); shim com modo ou hash errado; prover sem o diretório do shim; sondas (deployer, outra pubkey, exit 0, linha a mais, nenhuma linha); `check` sem a linha exata do verificador, com outro hash, sem `check=ok`, exit 1, e prover sem `prover=LocalProver env=ok` | C10-1, C10-2, C10-9 |
| HTTP (6) | `Host` errado/ausente/duplicado → 421/400; sem token 401, token errado 403 (GET e POST); nenhum `Access-Control-*` (inclusive OPTIONS 405, erros e `Origin` hostil); 23 corpos inválidos → 400, `Content-Type` 415, 5 KB 413, query 400 — sem nenhuma execução; rotas e métodos fixos; segunda operação → 409 com a operação em curso | C10-4, C10-6 |
| Fluxo (11) | fluxo inteiro (W1–W7 simulados) com **argv exato** de 20 execuções e **ambiente exatamente o construído** sob 18 variáveis hostis (`BONSAI_*`, `RISC0_*`, `VERICODE_*`, `DOCKER_*`, proxies, `LD_PRELOAD`…); `UNEXPECTED`, PASS sem a linha `simulation shows`, linha de outro programa e outro código → `rejection: null`; exit 1 depois de `[label] PASS` → estado do `show` e do log; create com exit ≠ 0 → `job_id` gravado durante a execução, `show` falho → 409 em tudo até o `show` do Job; create sem Job → `Failed`; compress sem a linha `docker_run`, com duas, outro diretório, tag, sem `--network=none`, horário antigo, log do shim divergente ou ausente, outro `docker_shim`, exit 137, shim alterado (modo/hash) ou memória baixa → `Failed` e settle/negativos 409; `Proving`/`Submitted` visíveis; C10-8 (299 e −5 não enviam, 300 envia); nenhum caminho ou byte de chave em resposta, log ou arquivo do worker (com uma CLI falsa que ecoa o argv), token em nenhum arquivo; `escrow-6014` usa e identifica a receipt de outro Job; reinício com escrita interrompida exige reconciliação | C10-2, C10-3, C10-5, C10-7, C10-8 |

Regressão sem rebuild: a partida real (F3) rodou os dois `check` com os binários do D10a.

## F3 — partida real e higiene HTTP

```text
worker.cli=…/d10a/targets/cli/release/vericode sha256=e6cd4e2914fcfc1233b876cb6f8a3b752e62e770cc4a309c7ce67e456d59acae
worker.prover=…/d10a/targets/prover/release/vericode-prover sha256=3f66e1c05a6d587cb64d193be33ceda77c85cb6b0a660922f6b4644e24125232
worker.shim=/home/lucas/src/vericode/prover/docker-shim/docker sha256=2a8f75b87766b98fe9759d233b929c215d043f1a97c1434f467cba442ba3d866
worker.buyer=EZgGUg4JhEAhzMPd4jBuKWFXdDNpFATvCjkj7mLAdxU6
worker.executor=EdB25bVhdj6rm5b7FbVHe5A3zx2hAhPpK4YAzrLaDs6U
worker.check.cli: check.escrow=GZqbL2Tb… upgrade_authority=none deployed_slot=507798457
worker.check.cli: check.escrow_program_data bytes=395064 sha256=cdf6967f…8133
worker.check.cli: check.verifier=THq1q… program_data=ENdLkqHp… upgrade_authority=none bytes=199256 sha256=34ae6e5c…6cd1
worker.check.cli: check=ok
worker.check.prover: guest.sha256=e09ba8cf… · guest.admitted=true · groth16.selector=73c457ba · prover=LocalProver env=ok
worker.url=http://127.0.0.1:8710
```

`ss -ltn`: só `127.0.0.1:8710`. Contra o worker real: `Host: localhost:8710` → 421; sem
token → 401; token errado → 403; `OPTIONS` com `Origin` hostil e pedido de preflight →
405 com `Allow: GET, POST` e nenhum `Access-Control-*`; GET autorizado com `Origin` → 0
cabeçalhos `Access-Control-*`. As sondas mostraram as pubkeys certas: as chaves são as do
buyer e do executor, nunca a do deployer.

## F4 — devnet (W1–W7 pela API)

Comandos (funções `w10_get`/`w10_post`/`w10_wait` de `d10/bin/w10_api.sh`, curl com
`--noproxy '*'`, o token lido da captura do terminal, nunca impresso), exatamente os do
plano: `show` + `refund-timeout` de `8ab4ee8d…` (W1) e de `8bce67f2…` (W2);
`POST /api/jobs {"deadline_offset": 9000}` (W3); `negative/escrow-6021` (W4);
`prove {"input": 21, "claimed_output": 42}`; `negative/verifier-6003` (W6); `settle` (W5);
`negative/escrow-6007` (W7). Cada resposta está em `d10/logs/W*-op.json`.

| # | op | Pagador | Esperado | Resultado na tx | CU | Tamanho | Fee | Estado depois | Slot |
| --- | --- | --- | --- | --- | ---: | ---: | ---: | --- | ---: |
| W1 | `775358cf…` | buyer | ok | ok | 14.605 | 342 B | 5.000 | `RefundedOnTimeout`, vault 0 | 508.247.421 |
| W2 | `cc164216…` | buyer | ok | ok | 14.605 | 342 B | 5.000 | `RefundedOnTimeout`, vault 0 | 508.247.532 |
| W3 | `adbff56c…` | buyer | ok | ok | 43.650 | 577 B | 5.000 | `Funded`, vault 1.000.000, prazo 508.256.633 | 508.247.640 |
| W4 | `1e44d1fe…` | buyer | escrow 6021 | `InstructionError(0, Custom(6021))`; contas iguais | 8.991 | 342 B | 5.000 | `Funded` | 508.247.746 |
| W6 | `8c51bd28…` | executor | verifier 6003 | `InstructionError(1, Custom(6003))`; verificador 100.528 CU; contas iguais | 119.238 | 883 B | 5.000 | `Funded` | 508.248.802 |
| **W5** | `19c09a17…` | executor | ok, verificador | **ok; `deliver` 4.947 + `release` 117.209 CU, dos quais 99.541 do verificador e 105 do Token** | 122.156 | 883 B | 5.000 | **`Released { 225384a7… }`**, vault 0 | 508.248.895 |
| W7 | `0fc76a76…` | executor | escrow 6007 | `InstructionError(0, Custom(6007))`; contas iguais | 9.989 | 838 B | 5.000 | `Released` | 508.248.993 |

Assinaturas completas:
- W1 `57UYbVX9gEXdp7dproD5mEyxj5UzxuQZdjwhQZe96XKTfCKbrWSxrfhKTmqrTdnAby8PGyJVSa1gDZsdk7P9Xd9v`
- W2 `625JvmR85QE6LbRZaE49kf3Y2cSFVnbKGaynVBbb8eeqM1Dj6r8UVUZiUbDoA4g5egJfqYUW175pG6d4NHYZWc3y`
- W3 `3XX9PWN2ZbvB9EntVvAt3S3444Zbo6cvoRyh5vkiQDqqZnDSQYTND11sBYrpJsGRjB7CpTAffZFhUz5ddguMsVpT`
- W4 `3PJfg2jUmmDVc5RuN31DgF3pJwcYnog8GUCYy9vvyB56n1drScwiY8dTTsXwZWsX59DCuRDn33YhwViff3shxv4L`
- W6 `4cKK83JBYxRVfrAT5t6visKQdEWim5NANMe3eMH4bHsDmPumzq3WZb83F4eWiuYC694uivPqhsip3D8ySxJBmX2u`
- W5 `ByGF4BFPD8zkyqc6bg3Hd2FnQ1bF9gjo1EWp4fyVQkcGWfcj53gi99kWPLgyherzmvYp8knoqAzgjLLyhgpLnVR`
- W7 `5ug6PggxL9qZdgwkCu1Vt3oEYSFiBKXLdCi9bE6W1bewLjfa2yKpYjf8BeHzEjtvdkYDNLHZY4cjjBvkqtF9Bn5w`

**Job novo:** `bc3340938cb7b6298a657618e972c2ef559f629e8e3d4d37801360c4f160df83`, vault
`CvRooDpUfUTU4aibFDt1nVxNx89az455Q3f5zvqmHjXa` (authority = PDA do Job), prazo 508.256.633
(slot 508.247.633 + 9.000). O `job_id` foi gravado no log do worker às 19:44:47, antes
do envio (assinatura às 19:44:48).

**Rótulos (C10-7).** Nos três negativos, a operação trouxe `rejection` porque a saída da
CLI tinha o `--expect-error` usado, a linha de simulação, `[label] PASS`, o registro
`PASS` e `Custom(code)`:
- W4: ``[refund_on_timeout] simulation shows `Program GZqbL2Tb… failed: custom program error: 0x1785` as the innermost failure``
  → `rejection.program=escrow`, 6021;
- W6: `settle.tampered=pi_c[10]^=1`;
  ``[deliver+release] simulation shows `Program THq1qFYQ… failed: custom program error: 0x1773` as the innermost failure``
  → `rejection.program=verifier`, 6003;
- W7: `precheck.expected_rejection=Job is already Released (6007)`;
  ``[release] simulation shows `Program GZqbL2Tb… failed: custom program error: 0x1777` as the innermost failure``
  → `rejection.program=escrow`, 6007.

**C10-8 (W4):** o `job show` imediatamente anterior deu slot 508.247.728 e prazo
508.256.633: margem de **8.905** slots (exigido ≥ 300).

### Prova local e linha `docker_run` (C10-2, C10-4)

`POST /api/jobs/bc334093…/prove` às 19:45:46; durante a operação,
`GET /api/jobs/bc334093…` → `state=Proving`, `state_scope=local`.

```text
prove    exit 0, 7,2 s: prove.job_id=bc334093… · prove.artifact=(21,42) hex=01000000150000002a000000
         prove.receipt_type=Composite · prove.local_verify=ok · prove.negative.wrong_image=rejected
         prove.journal_equal_to_core=true · prove.verdict=PASS · composite 221540 B b3980e5b…41ba
memória  MemAvailable antes do compress: 5.129.736 kB (≥ 2.621.440 kB); shim 2a8f75b8…, modo 0755
compress exit 0, 139,6 s: compress.docker_shim=/home/lucas/src/vericode/prover/docker-shim
         compress.receipt_type=Groth16 · compress.local_verify=ok · compress.negative.wrong_image=rejected
         compress.journal_equal_to_composite=true · verifier_parameters=73c457ba541936f0…eedc
         groth16 827 B de370735…a110 · journal_sha256=journal_digest=6ee55389…e2f9
         seal_sha256=3f73a90a…a4e0 · compress.verdict=PASS
```

Linha exigida pelo C10-2, única em `compress.docker_run=` e idêntica ao
`docker-shim.log` do diretório da receipt:

```text
2026-10-06T19:46:53-03:00 /usr/bin/docker --context default run --pull=never --network=none --rm -v /home/lucas/.local/share/vericode-spikes/d10/data/receipts/bc3340938cb7b6298a657618e972c2ef559f629e8e3d4d37801360c4f160df83/1/groth16-work:/mnt risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331
```

Memória (`d10/logs/mem-prove.log`, amostra a cada 2 s): **no pico, 167 MB disponíveis
(19:47:05) e 4,2 GB de swap**; nenhum exit 137. Só a prova rodava (editor aberto).

## F5 — fechamento

### Saldos (fecham por lamport e por unidade)

| Conta | Antes (slot 508.247.357) | Depois (slot 508.249.020) | Composição |
| --- | ---: | ---: | --- |
| deployer `617ogw9T…` | 2.805.194.240 | 2.805.194.240 | nenhuma transação |
| buyer `EZgGUg4J…` | 103.351.800 | 99.750.400 | −3.581.400 (Job 2.092.960 + vault 1.488.440, W3) − 4 × 5.000 (W1, W2, W3, W4) |
| executor `EdB25bVh…` | 29.925.000 | 29.910.000 | −3 × 5.000 (W6, W5, W7) |
| ATA do buyer `61tkoEv4…` | 999.992.000.000 | 999.993.000.000 | +1.000.000 (W1) +1.000.000 (W2) −1.000.000 (W3) |
| ATA do executor `HpZkHZ59…` | 6.000.000 | 7.000.000 | +1.000.000 (W5) |

Saldos passo a passo em `d10/logs/bal-{0,W1,W2,W3,W4,W6,W5,W7}.json`, cada um com o
efeito previsto do passo.

### Conferência independente

`w10_rpc.py tx` com as 7 assinaturas (`getTransaction`, `confirmed`,
`d10/logs/r1-gettransaction.jsonl`):
- os `err` são os da tabela; taxa 5.000 em todas; pagadores buyer (W1–W4) e executor
  (W6, W5, W7);
- verificador invocado só em W6 (100.528 CU) e W5 (99.541 CU);
- movimentos de token só em W1 (vault de `8ab4ee8d…` −1.000.000, ATA do buyer +1.000.000),
  W2 (vault de `8bce67f2…` −1.000.000, ATA do buyer +1.000.000), W3 (ATA do buyer
  −1.000.000, vault `CvRooDpU…` +1.000.000) e W5 (vault −1.000.000, ATA do executor
  +1.000.000).

`getSignaturesForAddress` desde as últimas assinaturas das gravações: buyer 4 (W1, W2,
W3, W4), executor 3 (W6, W5, W7), deployer 0 — todas do D10, nenhuma outra escrita.

Estado final pelo worker (`GET /api/jobs`): `8ab4ee8d…` e `8bce67f2…`
`RefundedOnTimeout` (origem externa); `bc334093…` `Released` (origem worker); nenhuma
reconciliação pendente, nenhum incidente, shim íntegro. O worker foi encerrado por
SIGINT às 19:51.

### Chaves, caminhos e token

- O worker nunca abriu um arquivo de chave: as sondas e as escritas passaram as chaves
  só por caminho à CLI.
- `grep` por `/keys/`, `buyer.json`, `executor.json`, `deployer.json` e `d4/keys` em
  `d10/data` (estado, operações, logs da CLI, receipts), nas respostas HTTP guardadas e
  nos logs: **0 ocorrências** em saída do worker. A única ocorrência é o rótulo
  `d4/keys` do snapshot no timeline do agente. `d10/worker.json` contém os caminhos por
  definição (é a configuração, nunca servida).
- O token aparece só na captura do terminal do operador
  (`d10/logs/worker-terminal.log`, `0600`); 0 ocorrências em outros arquivos.
- Varredura de segredos (cópia de `secret10.py`) em 80 arquivos (`d10/{logs,bin,data}`
  e `worker/`): 0 arrays de 64 números; 5 ocorrências explicadas — os padrões de palavra
  do próprio scanner e as três assinaturas públicas das gravações usadas como `until`
  (metade final diferente de qualquer pubkey do projeto).

### Fronteiras

- Perfil padrão, locks, guest e shim iguais ao início.
- Snapshots no fim iguais aos do início: `d9` `05f54f33…`, `d10a` `2de3a614…`,
  `rd10a` `24b6acf5…`, `rec-1006-0802/0804/0838/0933/1011`, `d4/keys` `3432f7e1…`.
- `d10/home` continuou vazio (o Docker não criou configuração nela).
- Nenhum arquivo do gate em `/tmp` (testes e filhos com `TMPDIR=d10/tmp`). As duas
  entradas novas em `/tmp` são do language server Python do editor (pyright).
- Rede: `api.devnet.solana.com` (CLI e leituras de `w10_rpc.py`) e `127.0.0.1`. Docker
  só pelo `compress` do prover, pelo shim, imagem por digest, `--network=none`, sem pull.

## Condições do R-D10a

| Condição | Como foi cumprida |
| --- | --- |
| C10-1 | hashes fixados no código e conferidos na partida e antes de cada execução; D9 recusado; preflight duplo na partida |
| C10-2 | shim conferido (hash, modo, regular) na partida e antes do `compress`; exatamente uma linha `docker_run` desta execução, igual ao log do shim; senão `Failed` e a receipt nunca é usada |
| C10-3 | ambiente construído do zero; argv fixo em lista, sem shell; o cliente HTTP só fornece `job_id` hexadecimal e inteiros validados |
| C10-4 | lock global com 409; `compress` só com MemAvailable ≥ 2,5 GiB; timeouts explícitos |
| C10-5 | estado do `job show` e do `--log`; nenhuma escrita repetida; `job_id` gravado antes do envio; reconciliação obrigatória antes de qualquer outra ação |
| C10-6 | `127.0.0.1`, `Host` conferido, token em header próprio em toda rota, sem CORS, rotas fixas, nenhum caminho ou conteúdo de chave em resposta ou log, saída como texto |
| C10-7 | `rejection` só com todas as linhas da CLI; `UNEXPECTED` nunca vira rejeição; só os negativos da CR2 |
| C10-8 | `show` imediatamente antes do `escrow:6021` e margem ≥ 300 (W4: 8.905) |
| C10-9 | chaves só do buyer e do executor, por caminho, conferidas por sonda; deployer recusado; limitação declarada (P3); frases congeladas intactas; `Proving` rotulado como etapa local |
| C10-10 | só W1–W7, com Job novo; nenhum Job consumido reutilizado; CR5 e CR7 mantidas |

## Invariantes do guia §7

Nenhuma mudou: o programa, o core e o verificador são os mesmos, e o worker não tem
regra econômica (só lê, chama a CLI e mostra). Em devnet, neste gate: vault com
authority = PDA do Job (1); termos e mint do Job (2); PASS pagou só o executor (3, W5);
timeout só depois do prazo (5, W1/W2; W4 antes do prazo → 6021); estado e transferência
atômicos e negativos sem mudança (6, W4/W6/W7); destino = ATA canônica, sem admin (7);
dupla liquidação recusada (9, W7); falha do verificador reverte tudo (10, W6). A 4 (FAIL)
e a 8 (journal de outro Job) não fizeram parte do W1–W7.

## Desvios e incidentes

1. **Linha-marca dos falsos.** Na primeira execução dos testes, a linha
   `fake=test-only` do falso fez a sonda (estrita, como deve ser) recusar a partida
   (7 falhas, 34 erros). O falso deixou de imprimir a marca só nas respostas de sonda;
   o worker não mudou. Depois: 23/23.
2. **Registrador de memória.** Um `pkill -f` do agente casou com o próprio comando e o
   encerrou junto com o registrador às 19:48:41, depois do fim do `compress`
   (19:48:30). Nenhum efeito no worker, na prova ou em devnet; o registro cobre a
   compressão inteira.
3. **Plan Mode único.** O prompt pedia Plan Mode antes de criar `worker/` e antes da
   primeira escrita. O plano aprovado às 19:18 trouxe os comandos HTTP exatos de W1–W7,
   e eles foram executados sem mudança; por isso não houve segundo Plan Mode.

## Riscos abertos

- **P3:** chaves de devnet do projeto num worker local, e o mesmo operador opera buyer e
  executor; sem carteira no navegador. Declarado no README e no `worker/README.md`.
- **`http.server`** não é endurecido para rede: aceitável só em loopback, com C10-6.
- **Token** na captura do terminal do operador (`0600`); vale só para a execução.
- **Memória:** 167 MB disponíveis e 4,2 GB de swap no pico do `compress`.
- **`escrow-6014`** implementado e testado offline, sem execução em devnet no D10.
- **Timeout do `compress`:** o worker mata o grupo do prover, mas um container já
  iniciado termina sozinho (`--rm`); a receipt não é usada.
- RD10A-01 a 07 continuam no código da CLI e do prover (o worker mitiga 01/02/03 com
  C10-2/C10-3 e o 04 com C10-5); RD7-08 continua na CLI e é bloqueado no worker pela
  C10-8.
- Sem e-stop; rent preso (agora 14 Jobs: S, A, B, C, P, T, P′, T′, os 5 das gravações e
  `bc334093…`); mint authority = deployer; ImageID não recertificado; spec v1 trivial.

## Artefatos fora do clone

`~/.local/share/vericode-spikes/d10/` (`0700`; arquivos `0600`):
- `worker.json` (configuração);
- `bin/`: `w10_api.sh`, `w10_rpc.py`, `w10_sum.py`, `w10_secret.py`;
- `data/`: `worker-ops.jsonl` (48 eventos), `jobs/` (3), `ops/` (10 operações, com
  `op.json` e o `cli-tx.jsonl` da CLI), `receipts/bc334093…/1` (receipt `Composite` e
  `Groth16`, vetores e `docker-shim.log`);
- `logs/`: `timeline.log`, `f0-*`, `t1-unittest.log`, `worker-terminal.log`, `f3-*`,
  `W1`–`W7` (`-show`, `-start`, `-op`, `-job`), `P-*`, `mem-prove.log`, `bal-*.json`,
  `d10-signatures.txt`, `r1-gettransaction.jsonl`, `r2-sigs-*.jsonl`, `f5-*`;
- `home/` (vazio) e `tmp/` (vazio).
