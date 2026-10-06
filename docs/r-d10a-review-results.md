# R-D10a — revisão delta curta do endurecimento da CLI e do prover (D10a)

Data: 2026-10-06 (15:01–16:06 -03:00) · Revisor: Claude Code (Opus 5.5,
esforço max), sessão separada e somente leitura · HEAD revisado: `2a2e3c5`
(sobre `7fe9c3b` e `50dede0`) · Base: `39a87f7` (D9); delta
`39a87f7..2a2e3c5`.

> Registro feito por outra sessão, com permissão de escrita, a partir da
> resposta da revisão, que não podia editar arquivos.

## Resultado

**APROVADO COM RESSALVAS para o D10**, com as condições C10-1 a C10-10
(seção 5).
- Nenhum achado crítico, alto ou médio; 1 baixo (RD10A-01) e 6 informativos
  (RD10A-02 a 07).
- RD7-01, 02, 03, 04 e 07 fechados; opcionais RD7-06, 09 e 10 fechados;
  RD7-05 fechado no D9; RD7-08 aberto.
- Nenhum achado permite mover fundos para destino escolhido pelo chamador,
  reportar sucesso falso ou provar fora da máquina. A solidez não depende do
  shim: a Groth16 é verificada localmente pelo prover e em devnet pelo
  verificador.
- Toda a checagem do D10a reproduziu a partir de um clone. A CLI do D10a
  (`e6cd4e29…`) e a da revisão (`8c709feb…`) se comportam igual em 39
  cenários; os hashes diferem só pelos caminhos embutidos (`CARGO_HOME`,
  fontes).
- Nenhum arquivo do repositório foi alterado; nenhuma escrita em devnet.
  Um incidente: uma chamada real, só de leitura, `docker image inspect`
  (seção "Incidentes").

## Linha do tempo (2026-10-06, -03:00)

| Hora | Fase |
| --- | --- |
| 15:01–15:06 | preflight Git, perfil, locks, guest, snapshots de `d9/`, `d10a/`, `rd7/`, `rec-*`; leitura obrigatória |
| 15:06–15:11 | cópia das homes de `d10a/homes` (334 s); clone de `2a2e3c5` (`--no-hardlinks`) |
| 15:09–15:17 | CLI: build release e testes |
| 15:13–15:24 | `getTransaction` de 24 transações (devnet, só leitura); CLI D9/D10a/HEAD contra o RPC falso |
| 15:26–15:56 | build release do prover (26 min 48 s); em paralelo, só tarefas leves: devnet só leitura, shim com `docker` falso, segredos, URLs/proxy, docs |
| 15:56–15:58 | testes e `check` do prover; PoC Bonsai (HEAD, D9; incidente às 15:57); caminho do shim; prova de controle |
| 15:58–16:04 | core ×2 e suíte (opcionais) |
| 16:04–16:06 | fronteiras, snapshots finais, varredura de segredos dos artefatos da revisão |

## 1. Preflight e comandos executados

Raiz própria `~/.local/share/vericode-spikes/rd10a/` (`0700`, logs `0600`),
`bin/envr.sh` `1a73e9e1…` (nomes `RV_`/`rv_`; sem o `env.sh` herdado e sem
`R`, `B`, `D` ou `VC`). Um `CARGO_TARGET_DIR` por árvore
(`rd10a/targets/{cli,prover,core-*,tests}`). Um processo pesado por vez,
destacado (`setsid nohup`, `nice`, `CARGO_BUILD_JOBS=2`; prover com 4);
`free -m` antes de cada um.

| Checagem | Resultado real |
| --- | --- |
| Git (início e fim) | `/home/lucas/src/vericode`, `main`; `2a2e3c5 → 7fe9c3b → 50dede0 → 39a87f7`; `status --short` e `--ignored` vazios; `diff --check` 0 |
| Perfil padrão (início e fim) | `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes; `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| Locks | raiz `191802b2…`, zkvm `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover` `8b76f1e1…` |
| Delta | `git diff --stat 39a87f7..2a2e3c5 -- '*.lock' '*Cargo.toml' anchor crates zkvm prover/artifacts` vazio; código só em `cli/src`, `cli/tests`, `prover/src`, `prover/tests` e `prover/docker-shim/docker` |
| Guest | 180.300 B `e09ba8cf…` |
| Clone | `rd10a/clone`, árvore `dba01c12…` = HEAD |
| CLI | `build --release` 1 min 45 s (wall 1 min 55 s), 0 warnings, 5.224.800 B `8c709feb…`; `test --no-fail-fast` **30/30** (guards 5, instructions 15, negative_runs 10), 5 min 16 s, RSS 2,1 GB; só os 13 warnings conhecidos do crate do programa |
| Prover | `build --release` 26 min 48 s (wall 29 min 16 s), RSS 2,1 GB, 0 warnings, 92.545.056 B `8240541e…`; `test --release` **5/5** + `docker_shim` **2/2**; `check`: `e09ba8cf…`, `4da06f90…fb1a`, `admitted=true`, `selector=73c457ba`, `prover=LocalProver env=ok` |
| Opcionais | core `+1.85.0` **42/42** e `+1.89.0` **42/42**; suíte com `SBF_OUT_DIR=rd10a/out/devnet` (`cdf6967f…`/`34ae6e5c…`, 199.256 B) **61/61** (d4b_receipts 2, escrow 27, fixtures 2, layout 7, regressions 7, settlement 16), 5 min 18 s; o log mostra os `.so` desse diretório e 28 CPIs ao verificador |
| `strings` dos binários | prover do D10a `3f66e1c0…` e da revisão: 0 ocorrências de `/images/upload`, `BONSAI_TIMEOUT_MS`, `x-api-key` e `RISC0_SERVER_PATH` (o do D9 `79b83528…` tem todas). Shim embutido: D10a → `/home/lucas/src/vericode/prover` + `docker-shim`; D9 → `d9/clone/prover`; revisão → `rd10a/clone/prover` |
| Devnet, só leitura (CLI da revisão) | `check=ok`: escrow 395.064 B `cdf6967f…`, authority `none`; `check.verifier=THq1q… program_data=ENdLkqHp… upgrade_authority=none bytes=199256 sha256=34ae6e5c…`; mint 6 decimais, sem freeze, supply 1.000.000.000.000 |
| `job show` | P′ `91ea6fcd…` `Released 225384a7…`, vault 0; T′ `ec9afb74…` `RefundedOnTimeout`, vault 0; `8ab4ee8d…` e `8bce67f2…` **`Funded`**, vault 1.000.000, prazos 508.078.115 e 508.089.477 vencidos (slot ~508.183.000); authority do vault = PDA do Job em todos |
| Saldos (slot 508.183.097) | deployer 2.805.194.240; buyer 103.351.800; executor 29.925.000 lamports; ATA do buyer 999.992.000.000; ATA do executor 6.000.000. Assinaturas mais novas: deployer `5CR2s5Jb…` (508.105.045), buyer `4W3Gn3vc…` (508.105.371), executor `24oguMEB…` (508.104.968), todas das gravações. Nenhuma escrita nova |

### Antes × depois: CLI binária contra RPC falso em 127.0.0.1

`rd10a/poc/rpc/rv_fake.py` (`6a949ac5…`) e cópia de `fake10.py` (`d9279207…`
= D10a); `accounts.json` `35df1a29…` (= D10a). Os cenários B, C e D usam os
`err` e `logMessages` **reais** de devnet (`rv_gettx.py`, só leitura, 24
transações: 8 do D9 e 16 das gravações). Assinatura com
`d4/keys/executor.json` só por caminho; blockhash falso; nada foi a devnet.
D9 = `d9/targets/cli/release/vericode` (`7e7a9260…`); D10a =
`d10a/targets/cli/release/vericode` (`e6cd4e29…`); HEAD = build da revisão
(`8c709feb…`).

| ID | Cenário | `--expect-error` | D9 | D10a | HEAD |
| --- | --- | --- | --- | --- | --- |
| A1 | `overlap6003` (sintético do D10a) | `escrow:6003` | **PASS, enviado** (exit 0) | recusado na simulação, nada enviado (exit 1) | igual ao D10a |
| A2 | `overlap6000` | `escrow:6000` | **PASS, enviado** | recusado, nada enviado | igual |
| A3 | `overlap6003` | `verifier:6003` | PASS | PASS | PASS |
| A4 | `already` (refund positivo) | — | erro falso no envio (exit 1) | "already processed" → status → PASS (exit 0) | igual |
| A5 | `lag` | — | PASS da tx, depois "state differs" (exit 1) | PASS (exit 0) | igual |
| A6 | `noverifier` (settle positivo) | — | PASS da tx, depois "without verifier" (exit 1) | nada enviado (exit 1) | igual |
| A7 | `ok6021` | `escrow:6021` | PASS | PASS | PASS |
| A8 | `flip` | `escrow:6021` | `UNEXPECTED` (exit 1) | `UNEXPECTED` | `UNEXPECTED` |
| B1 | real W5 `HXgnGUhh…` | `escrow:6003` | **PASS, enviado** | recusado, nada enviado | igual |
| B2 | real W5 | `verifier:6003` | PASS | PASS | PASS |
| B3 | real `5Cb4MJjL…` (gravação) | `escrow:6003` | **PASS, enviado** | recusado, nada enviado | igual |
| B4 | real `5Cb4MJjL…` | `verifier:6003` | PASS | PASS | PASS |
| B5 | real `3UACKnby…` (gravação) | `verifier:6003` | PASS | PASS | PASS |
| B6 | real 6014 `8Kk5UkX5…` | `escrow:6014` | PASS | PASS | PASS |
| B7 | real 6014 | `verifier:6014` | nada enviado | nada enviado | nada enviado |
| B8 | real 6021 `3BKnczeK…` | `escrow:6021` | PASS | PASS | PASS |
| B9 | real 6007 `5T9XE5Y3…` | `escrow:6007` | PASS | PASS | PASS |
| C1 | W5 + `Program log: Program GZqb… failed: …0x1773` forjado | `escrow:6003` | **PASS, enviado** | recusado | igual |
| C2 | W5 + quebra de linha dentro de um `Program log:` | `escrow:6003` | **PASS, enviado** | recusado | igual |
| C3 | W5 + `Program data:`/`Program return: <escrow> failed:` | `escrow:6003` | **PASS, enviado** | recusado | igual |
| C4 | idem | `verifier:6003` | PASS | PASS | PASS |
| C5 | W5 truncado (`Log truncated`) | `escrow:6003` | nada enviado | nada enviado | nada enviado |
| C6 | idem | `verifier:6003` | nada enviado | nada enviado | nada enviado |
| C7 | verificador falha sem `Custom` (`ProgramFailedToComplete`) | `verifier:6003` | nada enviado | nada enviado | nada enviado |
| C8 | simulação: escrow 6003; aterrissada: W5 real | `escrow:6003` | **PASS** | `UNEXPECTED` (exit 1) | igual |
| D1 | real `5tjezXYh…` (`Released`) | — (settle positivo) | PASS | PASS | PASS |
| D2 | + `already` | — | erro falso no envio | PASS | PASS |
| D3 | + `lag` (um −32016) | — | PASS da tx + "state differs" (exit 1) | PASS (`after.read_slot` ≥ slot da tx) | igual |
| D4 | + nó que responde com slot anterior | — | PASS da tx + "state differs" | PASS da tx + "RPC answered at slot … before the transaction slot" (exit 1) | igual |
| D5 | simulação com verificador; aterrissada sem | — | PASS da tx + "without verifier" (exit 1) | `UNEXPECTED` (exit 1) | igual |
| D6 | refund real `33ezPvow…` | — | PASS | PASS | PASS |
| D7 | + `already` | — | erro falso no envio | PASS | PASS |
| D8 | + `lag` | — | PASS da tx + "state differs" | PASS | PASS |
| D9 | + slot anterior | — | PASS da tx + "state differs" | PASS da tx + erro de slot (exit 1) | igual |
| D10 | `already` + aterrissada com 6021 | — | erro no envio | `UNEXPECTED` (exit 1) | igual |
| D11 | `already` + nunca aterrissa | — | erro no envio | "did not land before its blockhash expired" (exit 1) | igual |
| D12 | status só `processed` até expirar | — | não aterrissou (exit 1) | igual | igual |
| D13 | `job deliver` + `lag` | — | PASS da tx, depois erro (exit 1) | PASS | PASS |
| D14 | refund + −32016 permanente | — | PASS da tx + "state differs" | PASS da tx + "−32016 after 8 attempts" (exit 1) | igual |

Nos cenários recusados (A1, A2, B1, B3, C1–C3, C5–C7), o log de chamadas do
RPC falso tem 0 `sendTransaction`. D4, D9 e D14 são a base do RD10A-04.

### Shim (`rd10a/poc/shim/rv_shim.sh`, `docker` falso que só registra argv)

Shim do clone = do repositório (`2a8f75b8…`).

| Resultado | Casos |
| --- | --- |
| Aceito e remontado (exit 0) | argv do chamador; diretório com espaço; diretório com `*` literal; `/etc` e `$HOME` como work dir (RD10A-02); `DOCKER_HOST=unix:///home/lucas/evil.sock` e `unix://relative.sock` (RD10A-02); com `DOCKER_CONTEXT`, `DOCKER_CONFIG` ou `DOCKER_TLS_VERIFY`/`DOCKER_CERT_PATH` presentes, o argv executado tem `--context default` (o efeito no Docker real não foi testado); `--version` com `DOCKER_CONTEXT` |
| Recusado antes do `docker` (exit 2) | quebra de linha no diretório; vírgula; barra final; relativo; symlink para `/`; symlink para o work dir; `//`; `:/mnt:/mnt`; `:ro`; origem vazia; tag@digest; quebra de linha num argv; `DOCKER_HOST=ssh://…`; `UNIX://` maiúsculo; `tcp://` também no `--version` |
| Log | uma recusa com quebra de linha no argv grava uma segunda linha idêntica à de um `run` executado (RD10A-03) |
| Descartado | `DOCKER_HOST= tcp://…` (com espaço) deu exit 127 por divisão de palavras no próprio script da revisão; pela leitura do `case`, é recusado |

### Prover com ambiente hostil (`rd10a/poc/prover/rv_bonsai.sh`, listener em 127.0.0.1)

`BONSAI_API_KEY` fictícia e aleatória; o valor não aparece em nenhuma saída
(0 ocorrências nos dois logs).

| Caso | D9 (`79b83528…`) | HEAD (`8240541e…`) |
| --- | --- | --- |
| `RISC0_PROVER=bonsai` + URL/KEY (prove) | **1 conexão `GET /images/upload/4da06f90…`**, exit 1 | recusa, 0 conexões, nada gravado |
| só URL/KEY | **1 conexão** | recusa, 0 conexões |
| `RISC0_PROVER=LOCAL` + URL/KEY | prova (o upstream minusculiza) | recusa |
| `RISC0_PROVER=local` + só URL | prova | recusa |
| `RISC0_PROVER=actor` | panic, exit 101 | recusa |
| `RISC0_DEV_MODE=0` | prova | recusa |
| `compress` com URL/KEY | erro do `BonsaiProver`, **depois de um `docker image inspect` real** (incidente) | recusa antes do shim, 0 conexões |
| `check` com `RISC0_PROVER=bonsai` + KEY | exit 0 | recusa, exit 1 |

Controle positivo (`r10`): `RISC0_PROVER=local RISC0_EXECUTOR=ipc
RISC0_SERVER_PATH=/nonexistent/r0vm`, `job_id` aleatório (nunca vai a
devnet): `prove.prover=LocalProver`, 8,3 s, `Composite`, `local_verify=ok`,
`journal_equal_to_core=true`, RSS 0,6 GB. O ambiente não escolhe o prover nem
o executor.

### Caminho do shim (`rd10a/poc/prover/rv_shimpath.sh`)

`PATH=<fakebin>:/usr/bin:/bin`, `docker` falso que sai com 1; composite do
D10a `X` copiada; o shim do clone volta ao modo original no fim.

| Caso | O que o `docker` falso recebeu | `docker-shim.log` |
| --- | --- | --- |
| shim executável (`VERICODE_REAL_DOCKER` = falso) | `--context default image inspect --format {{.Id}} <digest>` | vazio (o `image inspect` não é registrado) |
| shim em modo 644 | **`image inspect --format {{.Id}} <digest>`, sem passar pelo shim** | vazio |

### Rede (RD7-10)

- `--rpc-url` (`r6`):
  - aceitos e normalizados para loopback: `HTTP://LOCALHOST`, `127.1`, `2130706433`, `0x7f.1`, `[0:0:0:0:0:0:0:1]`, `user:pw@127.0.0.1`, `127.0.0.1#@evil.com`;
  - recusados: `localhost.`, `127.0.0.2`, `[::ffff:127.0.0.1]`, `localhost.localdomain`, `file://`, `evil.com#@127.0.0.1`, `127.0.0.1@evil.com`.
- Proxy (`r7`), com `HTTP(S)_PROXY`/`ALL_PROXY` apontando para um listener local:
  - D9: 4 requisições `POST http://127.0.0.1:…/` foram pelo proxy;
  - HEAD: 0, com `job.status=Funded` lido direto.

## 2. Achados

| ID | Sev. | Componente | Local | Resumo | Evidência | Recomendação | Bloqueia D10 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RD10A-01 | Baixo | prover / shim | `prover/src/lib.rs:282-300`, `371-375` | O binário executa o shim de `env!("CARGO_MANIFEST_DIR")/docker-shim` (árvore de fontes do build), conferido só com `is_file()`, sem SHA-256 nem bit de execução; o `compress` aceita a Groth16 sem exigir que o shim tenha registrado o `run`. Solidez intacta | `strings`: o binário do D10a usa `/home/lucas/src/vericode/prover/docker-shim` (árvore viva); incidente 1 do D10a (binário da árvore "antes" com o shim "antes"); PoC: shim em modo 644 → `image inspect` cru no `docker` seguinte do `PATH`, log vazio. Com `/usr/bin/docker` em seguida, o `run` do `risc0-groth16` iria por tag e com rede | D10: C10-2. Código (gate futuro): embutir o shim e conferir hash e modo, ou gravá-lo em diretório `0700` do run; falhar o `compress` sem a linha `run` desta execução | Não, com C10-2 |
| RD10A-02 | Info | prover / shim, ambiente | `prover/src/lib.rs:319-323`; `prover/docker-shim/docker:28,42-45` | O ambiente ainda escolhe `RISC0_WORK_DIR` (qualquer diretório canônico ≠ `/` vira `/mnt` com escrita do container root), `VERICODE_REAL_DOCKER`, `DOCKER_HOST=unix://<qualquer caminho>` e `DOCKER_CONFIG`. Nada disso escolhe o prover | PoC de shim: `/etc`, `$HOME`, `unix:///home/lucas/evil.sock`, `unix://relative.sock`, `DOCKER_CONFIG` aceitos | D10: C10-3. Futuro: recusar `RISC0_WORK_DIR` externo e `VERICODE_REAL_DOCKER` fora dos testes | Não, com C10-3 |
| RD10A-03 | Info | log do shim | `prover/docker-shim/docker:30-39,74`; `prover/src/lib.rs:371-375` | `REFUSED` grava `$*` cru: um argv com quebra de linha forja uma linha igual à de um `run` executado, que o `compress` reimprime como `compress.docker_run=`. Só alcançável por quem já executa o shim com argv arbitrário | PoC `log-injection` | `printf %q` no log; aceitar só a linha exata (C10-2) | Não |
| RD10A-04 | Info | CLI, semântica da saída | `cli/src/main.rs:249-268` (usos em 440, 498, 641, 701); `651`, `706` | Depois de uma transação aterrissada e impressa como `[label] PASS`, a CLI sai com exit 1 se a leitura posterior falhar (−32016 por 8 tentativas, ~63 s; slot anterior; erro de RPC) ou se o saldo do destino mudar por terceiros ou concorrência. Nunca sucesso falso | RPC falso D4, D9, D14 (D10a e HEAD) | D10: C10-4 e C10-5 | Não, com C10-5 |
| RD10A-05 | Info | testes | `cli/tests/fake_rpc/mod.rs:132-134`; `cli/tests/negative_runs.rs:44-56` | As linhas `failed` dos testes usam a mesma string de formato do código, e os logs são sintéticos; `read_after`/`minContextSlot` não tem teste de cargo | Replay dos logs reais pela revisão (B1–B9, C, D1) confirma o formato | Versionar os `logMessages` públicos (W5, gravações, 6014, 6021, 6007, `Released`) como fixtures (futuro) | Não |
| RD10A-06 | Info | CLI `check` | `cli/src/main.rs:326-327` | O slice do ProgramData do escrow não confere o tamanho (o do verificador, novo, confere em 342-344): RPC hostil que passe a guarda de genesis → panic (exit 101). Pré-existente | Leitura | Mesma checagem do verificador | Não |
| RD10A-07 | Info | docs | `README.md:151`; `prover/README.md:112-117` | "no daemon local" e "roda, portanto, só a imagem local… sem rede" valem quando o `docker` executado é o shim íntegro (RD10A-01); "daemon local" = qualquer socket `unix://` (RD10A-02) | Leitura; PoCs | Qualificar na próxima edição de docs; a frase 8 do README só vale para receipt com a linha do C10-2 | Não |

## 3. Situação de RD7-01 a RD7-10

| Item | Situação | Evidência |
| --- | --- | --- |
| RD7-01 | Fechado | `tx.rs:116-151`: só linhas de runtime contam, a mais interna decide e, para o escrow, nenhuma outra linha `failed`; D9 aceitava `escrow:6003` sobre o W5 real (B1, B3) e na divergência C8; D10a/HEAD recusam sem enviar; `verifier:6003` aceito com os logs reais (B2, B4, B5); forjados, truncados e não-custom não enganam (C1–C7) |
| RD7-02 | Fechado quanto ao argv; resíduos RD10A-01/02/03 | allowlist exata de 3 argv; lista de chamadores completa (460 pacotes do lock: só `risc0-groth16 3.0.2` `docker.rs:52-60,74-79` em tempo de execução, mais o `image inspect` do prover; `risc0-build` só em build scripts, e o prover não tem `build.rs`); 28 casos hostis |
| RD7-03 | Fechado | `LocalProver::new("local")` e `local_executor()`; `get_prover_server` só desvia para o dev mode, recusado antes; 0 strings do cliente Bonsai no binário; 8 casos recusados com 0 conexões (D9: 1); controle positivo |
| RD7-04 | Fechado; RD10A-05 informativo | tamper = builder da suíte com a mutação de `d4b_receipts.rs:149`/`settlement.rs:238` (byte 383); casamento com logs sintéticos, formato confirmado pelo replay real |
| RD7-05 | Fechado no D9 (CR1); sem regressão | o delta só fez a errata de `manifest-schema.md:86` |
| RD7-06 | Fechado (opcional) | `keys.rs:30-35` antes da checagem de work tree (as duas recusam; a ordem só muda a mensagem); `rpc.rs:119-120`; testes verdes |
| RD7-07 | Fechado; resíduo RD10A-04 | `rpc.rs:58-63,284-298`; `read_after` em create, deliver, settle e refund-timeout; A4, A5, D2, D3, D7, D8, D13 → PASS; D10, D11 nunca sucesso |
| RD7-08 | Aberto (info), sem mudança | condição C10-8 |
| RD7-09 | Fechado (opcional) | `deadline_margin_problem`; `Expect::Verified` (A6, D5) |
| RD7-10 | Fechado (opcional) | `check_url` (15 URLs); `no_proxy` (D9 4 × HEAD 0); `check` com `ENdLkqHp…`, 199.256 B, `34ae6e5c…` = `.so` de devnet = cadeia |

## 4. Checklist adversarial

1. **RD7-01: sem caso residual.**
   - `failing_program` exige que o token depois de `Program ` seja uma pubkey e o resto comece com `failed: `. `Program log:`, `Program data:`, `Program return:` e `Program consumption:` nunca contam; uma quebra de linha dentro de um elemento de log não cria linha nova.
   - A truncagem do Agave é monotônica: se a linha do verificador cai, a do escrow também cai, e nada casa.
   - Com `CreateIdempotent` antes do escrow, a falha dele é atribuída ao programa da ATA.
   - Erro de Token dentro do escrow casa só com `token:N`.
   - Nenhum caso em que `escrow:N` case com a falha de outro programa, nem em que `verifier:N` deixe de casar com o W5 real.
   - O índice da instrução não é conferido. Isso não muda o rótulo do programa, porque o código é único no escrow.
2. **RD7-02:** argv e caminhos hostis recusados.
   - `--context default` neutraliza `DOCKER_CONTEXT` e o contexto da configuração. O `DOCKER_HOST` ainda vale no contexto default, por isso a guarda existe; ela aceita qualquer socket `unix://` (RD10A-02).
   - `VERICODE_REAL_DOCKER` e o `PATH` escolhem o executável (RD10A-01/02).
   - A linha `REFUSED` é forjável (RD10A-03).
3. **RD7-03:** nenhum caminho de seleção pelo ambiente.
   - `RISC0_SERVER_PATH` só é lido por `get_r0vm_path` (ipc/actor), inalcançável.
   - `ProverOpts::groth16()` lê só o dev mode, recusado antes.
   - `local_prover_env` roda no início de `check`, `prove` e `compress`; `verify` e `execute` não provam.
   - Os valores de `BONSAI_*` nunca aparecem na saída.
4. **RD7-04:** os testes não são tautológicos (`sendTransaction` contado; tamper contra o builder da suíte), mas o casamento usa o mesmo formato do código (RD10A-05). O RPC falso em processo é fiel ao Agave nos campos lidos: `InstructionError` em JSON, `logMessages`, −32002 com `data.err="AlreadyProcessed"` e o texto de `solana-transaction-error 2.2.1` (`lib.rs:162-163`).
5. **RD7-07:**
   - `AlreadyProcessed` cai no mesmo laço de status, e o resultado vem do `getTransaction` (D10, D11).
   - `read_after` em `minContextSlot` com retry de −32016, nos quatro comandos (`main.rs:440, 498, 641, 701`).
   - As leituras antes da transação (`confirmed`, sem `minContextSlot`) não mudaram e só podem causar erro falso. Um snapshot "antes" desatualizado nunca dá PASS, porque a transação negativa é atômica.
6. **Opcionais:** `nlink` antes da work tree; `0600` mesmo em arquivo existente; `check_url` sem bypass (o `url` normaliza IPv4/IPv6 antes da comparação); `no_proxy` confirmado; constantes do verificador iguais ao `.so` e ao `check` ao vivo.
7. **Docs = código:** "Limitações", `cli/README.md`, `prover/README.md` e roteiro conferem, salvo RD10A-07.
   - Frases permitidas + "Não dizer": README `ed0d1edc…`, roteiro `20e6e009…`, iguais em `39a87f7` e `2a2e3c5`.
   - "Gravação" `12b29a05…`, igual, com os binários do D9 (`d9/bin/env9.sh`).
   - `manifest-schema.md`: um só hunk, na linha 86.
8. **Incidentes do D10a:** relatados com honestidade e sustentados pelos logs.
   - `timeline.log` registra o `p4` às 13:39:49 e o incidente às 13:53.
   - `x-env.log` tem o binário `3f66e1c0…` e o shim `2a8f75b8…`.
   - O `docker-shim.log` de `X` tem uma única linha, com `--context default`; o de `X-wrong-binary` usa o shim "antes".
   - O que veio antes do `p4` (testes, `check`, Bonsai e a CLI) é válido.
9. **Segredos:** blobs novos do delta (23), histórico inteiro (389 blobs), `d10a/{logs,bin,poc}` (76 arquivos) e `rd10a/` (123 arquivos).
   - 0 arrays de 64 bytes.
   - Os base58 de 64 bytes são assinaturas; nenhum tem metade final igual a uma pubkey do projeto (deployer, buyer, executor, mint, escrow, verificador).
   - 0 hex de 128 caracteres com essa metade final.
   - Palavras-chave só em regras dos documentos e nos scanners.
   - Controle positivo detectado. `d4/keys` não foi lido (só `pubkeys.txt`).
10. **CR8 e D10:** o worker usa só os binários do D10a, o preflight duplo e o hash do shim; o rótulo do programa fica liberado (RD7-01 fechado); o resto vai nas condições C10-1 a C10-10.

## 5. Veredito para o D10: APROVADO COM RESSALVAS

**Condições:**
- **C10-1 (binários e preflight):** o worker usa só a CLI `e6cd4e29…` e o
  prover `3f66e1c0…` de `d10a/targets`, com hash conferido na partida; nunca
  os do D9 (`7e7a9260…`/`79b83528…`) nem um rebuild sem decisão. Preflight
  a cada partida:
  - `vericode check` → `check=ok`, com `check.verifier=… bytes=199256
    sha256=34ae6e5c…`;
  - `vericode-prover check` → `prover=LocalProver env=ok`.
- **C10-2 (shim, RD10A-01):**
  - antes de cada `compress`, `sha256` de
    `/home/lucas/src/vericode/prover/docker-shim/docker` = `2a8f75b8…` e modo
    `0755`;
  - depois, exatamente uma linha `compress.docker_run=` desta execução, igual
    a `/usr/bin/docker --context default run --pull=never --network=none --rm
    -v <dir>/groth16-work:/mnt
    risczero/risc0-groth16-prover@sha256:7f173963…`;
  - senão, a receipt não é usada (estado `Failed`);
  - durante D10–D12, não editar, mover, trocar o modo nem fazer checkout de
    `prover/` no repositório;
  - a frase 8 do README só vale para receipts com essa linha.
- **C10-3 (ambiente, RD10A-02):** subprocessos com ambiente construído do
  zero (HOME, `PATH=/usr/bin:/bin`, TMPDIR fora de `/tmp`; para o prover,
  opcionalmente `RISC0_PROVER=local`).
  - Nunca herdar o ambiente do worker.
  - Sem `BONSAI_*`, `RISC0_DEV_MODE`, `RISC0_WORK_DIR`, `VERICODE_*`,
    `DOCKER_*` ou proxies.
  - argv fixo por ação, em lista, sem shell; o cliente HTTP nunca fornece
    caminho, flag, programa ou URL.
- **C10-4 (recursos):** uma operação de CLI/prover por vez (lock global; 409
  se ocupado). `compress` sozinho e só com MemAvailable ≥ 2,5 GB (D10a: 208
  MB no pico; a sessão caiu com 80 MB). Timeouts explícitos.
- **C10-5 (estado, RD10A-04):** o estado exibido vem da cadeia (`job show`)
  e do `--log` JSONL (`outcome`, `signature`, `slot`), nunca só do exit code.
  - Nenhuma escrita é repetida automaticamente.
  - `create.job_id=` é registrado antes da transação.
  - Um `create` com exit ≠ 0 é reconciliado por `job show` antes de
    qualquer outra ação.
- **C10-6 (HTTP):** bind só em `127.0.0.1`, com `Host` conferido (anti DNS
  rebinding).
  - Token aleatório por execução, exigido em header próprio em toda
    requisição que escreve ou prova (anti-CSRF; força o preflight de CORS).
  - Sem cabeçalhos CORS; rotas fixas, sem listar diretório.
  - Nenhuma resposta, log do worker ou página contém caminho ou conteúdo de
    chave.
  - Saída da CLI exibida como texto (`textContent`).
- **C10-7 (rótulo, RD7-01):** a UI pode dizer qual programa rejeitou, só a
  partir da saída da CLI do D10a: o `--expect-error` usado, a linha
  `simulation shows … as the innermost failure`, `[label] PASS` e o link do
  Explorer.
  - `UNEXPECTED` nunca vira "rejeitado".
  - Negativos só os da CR2: `escrow:6014`, `escrow:6007/6008`,
    `escrow:6021`, `verifier:6003`.
- **C10-8 (RD7-08):** `escrow:6021` só com `deadline_slot − slot ≥ 300` num
  `job show` imediatamente anterior.
- **C10-9 (claims e chaves):** CD7 e frases congeladas inalteradas.
  - O worker guarda só as chaves do buyer e do executor, por caminho; nunca
    a do deployer (mint authority).
  - Limitação declarada na UI e no README: chaves de devnet do projeto num
    worker local; sem carteira no navegador; o mesmo operador local opera
    buyer e executor.
  - Qualquer frase nova exige decisão registrada e a revisão curta da
    interface.
  - `Proving` é rotulado como etapa local do executor, fora da cadeia.
- **C10-10 (escritas):** só a lista fechada aprovada no Plan Mode do D10, com
  `job_id` novos; nunca reutilizar Jobs consumidos; CR5 e CR7 continuam.

**Decisões pendentes (humanas):** ratificar as propostas P1 a P6 abaixo e a
lista de escritas do D10.

## Propostas para o D10 (avaliação da revisão)

| Proposta | Avaliação | Recomendação final |
| --- | --- | --- |
| P1. Worker HTTP em 127.0.0.1 que só chama os binários do D10a por `subprocess`, sem duplicar regras; chaves só no worker; telas estáticas servidas por ele; estados com Explorer | Concordo. Faltavam: uma operação por vez, reconciliação de estado, anti-CSRF/DNS rebinding, ambiente limpo e checagem do shim | Ratificar, com C10-1 a C10-7. Estados do guia: `Draft` (só UI), `Funded`, `Proving` (local, executor), `Submitted` (tx enviada), `Released`, `Refunded` (`OnFail`/`OnTimeout`), `Failed` (erro operacional); nenhuma transição econômica fora do programa |
| P2. Python (biblioteca padrão) e front-end sem framework nem build | Concordo. Python 3.12.3 presente; `http.server.ThreadingHTTPServer`, `subprocess`, `json`, `secrets`, `hmac`. Alternativas (crate Rust com servidor HTTP, npm) trazem lock novo, revisão e build. `http.server` não é endurecido para rede: aceitável só em loopback, com C10-6 | Ratificar. Testes com `unittest`; executáveis falsos só nos testes, rotulados, e o worker real recusa binário com hash diferente |
| P3. Sem carteira no navegador | Concordo: exigiria refazer as instruções em JS (duplicação vetada pela CR8 e pelo próprio P1) e dependência npm. Diverge do D11 da sequência ("conectar carteira devnet"), que tem a menor precedência | Ratificar e registrar a divergência; limitação declarada (C10-9) |
| P4. Rótulo do programa que rejeitou, só a partir da saída da CLI do D10a | RD7-01 fechado (B1–B9, C1–C8) | Ratificar, com C10-7 |
| P5. Escritas: reembolso de `8ab4ee8d` e `8bce67f2`; um Job de ponta a ponta; um negativo | Concordo com um ajuste: o negativo `escrow:6021` no próprio Job de ponta a ponta, logo após o create, evita um segundo Job parado e outro reembolso | Lista W1–W5 (abaixo); W6/W7 opcionais, por decisão humana |
| P6. D10 → D11–D12 → revisão curta da interface → vídeo → submissão | Concordo | Timebox: D10 até 07/10; telas 08–09/10; revisão curta 09/10; vídeo 10/10; submissão 11/10. O vídeo de reserva (binários do D9) continua como plano B |

**Lista de escritas recomendada para o D10 (a ratificar):**
- W1: `job refund-timeout --job-id 8ab4ee8d…`, pagador buyer →
  `RefundedOnTimeout`.
- W2: o mesmo para `8bce67f2…`.
- W3: `job create`, pelo worker: buyer, executor `EdB25…`, 1.000.000, offset
  9.000.
- W4: no próprio Job de W3, logo após criar, `job refund-timeout
  --expect-error escrow:6021` com C10-8.
- Prova e compressão locais (não são escritas).
- W5: `job settle --deliver` (executor) → `Released`, com o verificador
  invocado.
- Opcionais: W6 `--tamper-seal --expect-error verifier:6003` antes de W5; W7
  `escrow:6007` depois.

Efeito esperado: ATA do buyer +2.000.000 (W1, W2) −1.000.000 (W3); ATA do
executor +1.000.000 (W5); SOL do buyer −(3.581.400 de rent + taxas).

## Incidentes da revisão

1. **Uma chamada real ao Docker, só de leitura** (15:57). No caso
   `compress-bonsai` da PoC do prover, o binário do D9 (`79b83528…`) passou
   pelo shim do D9 (`d9/clone/prover/docker-shim`). Esse shim repassa
   `image inspect` sem filtro, e o resultado foi um `docker image inspect
   --format {{.Id}} <digest>` real ao daemon local, antes de o
   `BonsaiProver` recusar a compressão.
   - Sem container, pull ou rede.
   - `~/.docker` inalterado (`6046f67f…`); nenhum `.docker` criado na home
     copiada.
   - O caso não foi repetido.
   - O caso equivalente no HEAD foi recusado antes do shim.
   - Lição: PoCs de `compress` com binários antigos precisam de um `docker`
     falso à frente no `PATH`.
2. O caso `DOCKER_HOST= tcp://…` do script do shim deu exit 127 por divisão
   de palavras no próprio script; foi descartado.

## Fronteiras

- **Rede:**
  - `https://api.devnet.solana.com`, só leitura (`getTransaction`,
    `check`, `job show`, saldos, `getSignaturesForAddress`);
  - `127.0.0.1` para os RPCs falsos e os listeners.
  - Sem crates.io, rustup ou S3 (`--offline`; `RECURSION_SRC_PATH`
    `744b999f…`).
- **Devnet:** nenhuma escrita, deploy ou airdrop. As transações assinadas
  com `d4/keys/executor.json` (só por caminho) têm blockhash falso e foram
  só para 127.0.0.1.
- **Docker:** só `docker` falso, exceto o incidente 1. Sem `compress` real.
- **Repositório e arquivos:**
  - nenhum arquivo do repositório criado, alterado ou apagado;
  - `status --short`/`--ignored` vazios no fim;
  - `d9/`, `d10a/`, `rd7/` e `rec-*` iguais ao snapshot do início;
  - `d4/keys`: só `pubkeys.txt` foi lido;
  - perfil padrão inalterado;
  - TMPDIR = `rd10a/tmp`. As esperas em segundo plano da sessão tiveram a
    saída gravada pelo harness do Claude Code em `/tmp/claude-1000/…` (só
    linhas do timeline); nenhum artefato do gate em `/tmp`.

## Artefatos fora do clone

`~/.local/share/vericode-spikes/rd10a/` (`0700`; logs `0600`):
- `bin/`: `envr.sh`, `r0_copy.sh`, `q1_cli.sh`, `q2_prover.sh`, `q3_rest.sh`,
  `rv_gettx.py`, `rv_rpc.py`, `rv_secret.py`;
- `poc/`:
  - `rpc/` (`rv_fake.py`, `rv_run.sh`, cópias de `fake10.py` e
    `accounts.json`);
  - `reallogs/` (24 transações, só campos públicos);
  - `shim/` (`rv_shim.sh`, `docker` falso);
  - `prover/` (`rv_bonsai.sh`, `rv_shimpath.sh`, `listener.py`, `fakebin/`);
  - `net/`;
- `logs/`:
  - `timeline.log`, `snap-{start,end}.txt`;
  - `c1`, `c2`, `p1`–`p3`, `k1`, `k2`, `s1`;
  - `r1`–`r10`;
  - `rv-fake-{d9,d10a,head-*}.log/.jsonl`;
- `clone/`, `homes/`, `targets/`, `out/`, `artifacts/`.
