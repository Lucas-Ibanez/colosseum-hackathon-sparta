# D10a — endurecimento da CLI e do prover (RD7-01, 02, 03, 04 e 07)

Data: 2026-10-06 (-03:00) · Executor: Claude Code (Opus 5.5) · Commits:
`50dede0` (cli), `7fe9c3b` (prover) e `docs: record D10a hardening` · Gate
anterior:
D9 (`39a87f7`, `docs: record D9 clean-environment demo and freeze claims`,
sobre `9d3efa7`) · Prompt: versão revisada de `docs/handoffs/d9-to-d10a.md`,
com as decisões de "Decisões humanas para o D10a" (`docs/decisions.md`).

## Resultado

**D10a CONCLUÍDO.** Os cinco achados baixos do R-D7 estão corrigidos na CLI e
no prover. Cada um tem teste que falha com a lógica do D9 e passa com a nova
(seção F2).

| Achado | Correção | Teste "antes" (D9) | Teste "depois" |
| --- | --- | --- | --- |
| RD7-01 | `--expect-error PROGRAMA:N` só casa quando esse programa é a **falha mais interna**; para `escrow`, nenhum outro programa pode ter falhado | `escrow:6000`/`6003` sobre falha do verificador → PASS, transação enviada | recusado na simulação, nada enviado; `verifier:6003` aceito |
| RD7-02 | shim do Docker = **allowlist exata** de 3 argv, com o comando remontado do zero | 19 de 22 argv hostis chegam ao `docker` | todos recusados (exit 2) antes do `docker` |
| RD7-03 | `LocalProver` direto; recusa de `RISC0_PROVER` ≠ `local`, `BONSAI_*` e `RISC0_DEV_MODE` antes de qualquer trabalho | `GET /images/upload/4da06f90…` no listener local (Bonsai); `ipc` tenta um `r0vm` externo; dev mode → panic | recusa limpa, 0 conexões, nada gravado |
| RD7-04 | testes do casamento do `--expect-error` (inclusive a sobreposição 6000–6003) e do `--tamper-seal` | o casamento falha nos casos de sobreposição; o tamper passa (era lacuna de teste, não defeito) | 30/30 na CLI |
| RD7-07 | "already processed" resolvido pelo status da assinatura; leituras depois da transação com `minContextSlot` = slot da transação | erro falso (exit 1) numa transação que aterrissou, nos dois casos | PASS, com o estado depois da transação |

Também entraram os opcionais aprovados no Plan Mode:
- **RD7-06:** keypair com mais de um hard link é recusado; um `--log` que já
  existe passa a `0600`.
- **RD7-09:**
  - mensagem "past the deadline" para Job vencido;
  - a liquidação positiva (`Expect::Verified`) exige o verificador: nada é
    enviado sem ele e nunca sai PASS antes do erro.
- **RD7-10:**
  - `--rpc-url` só `https://`, ou `http://` em loopback;
  - proxies ignorados;
  - o `check` confere os bytes do verificador (`34ae6e5c…`).

Fica aberto o RD7-08 (informativo).

Fronteiras:
- **Nada mudou no programa, no core, em `zkvm/`, no guest, no `JournalV1` nem
  em lock algum.** Não houve dependência nem feature nova.
- **Nenhuma escrita em devnet.**
- O schema mudou só na errata da linha 86, aprovada em Plan Mode.

O claim continua o do CD7. A lista de frases permitidas não mudou.

## Linha do tempo (2026-10-06, -03:00)

| Hora | Fase |
| --- | --- |
| 12:42–12:52 | F0: preflight Git, perfil, locks, guest; devnet só leitura; escritas do humano |
| 12:51–12:58 | F0: cópia das homes de `d9/homes` (417 s) |
| 12:53–12:55 | Plan Mode; plano aprovado pelo humano às 12:55 |
| 12:56–13:30 | F1: prover, shim, CLI, testes e textos |
| 13:00–13:32 | build release do prover (31 min 39 s; RSS 2,1 GB) |
| 13:06–13:11 | árvore "antes"; CLI do D9 contra o RPC falso; prover do D9 com ambiente Bonsai |
| 13:32–13:33 | testes do prover, `check`; prover novo com ambiente Bonsai |
| 13:33–13:40 | CLI: build, testes 30/30, RPC falso com o binário novo, árvore "antes"; shim do D9; core ×2; devnet só leitura |
| 13:40–13:46 | suíte `anchor/tests-local` 61/61 |
| 13:46–13:51 | prova e compressão com o binário errado (incidente 1); desconexão da sessão (incidente 2) |
| 13:52–13:56 | diagnóstico; binário restaurado; prova, compressão e `verify` com o binário novo |
| 13:56– | F6: fronteiras, varredura de segredos, registros, revisão do diff, commits |

## F0 — preflight e checagem do D9

| Checagem | Resultado real |
| --- | --- |
| Git | `/home/lucas/src/vericode`, `main`, HEAD `39a87f7` sobre `9d3efa7`; `status --short` e `--ignored` vazios; `diff --check` 0 |
| Perfil padrão (início e fim) | `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes; `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| Locks (início e fim) | raiz `191802b2…`, zkvm `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover` `8b76f1e1…` |
| Guest | 180.300 B, `e09ba8cf…` |
| Devnet (só leitura, CLI do D9 `7e7a9260…`) | `check=ok` (escrow `cdf6967f…`, authority `none`; verificador `ENdLkqHp…` authority `none`; mint 6 decimais, sem freeze); P′ `Released { 225384a7… }`, T′ `RefundedOnTimeout`, vaults 0 |

### Escritas do humano (gravações `rec-*`)

Só campos públicos: `job_id` de `jobs/*.json` e assinatura, rótulo, resultado
e slot de `logs/cli-tx.jsonl`.

| Pasta | Job | Transações enviadas |
| --- | --- | --- |
| `rec-1006-0802` | — | nenhuma (pasta vazia) |
| `rec-1006-0804` | T `8ab4ee8d…` | `create+fund` `KHrMJitn…`; 6021 `35668bHU…` |
| `rec-1006-0838` | P `8bce67f2…` | `create+fund` `5qJvodqq…`; 6014 `3esTjmwx…`; o `verifier:6003` não passou da simulação, e nada foi enviado |
| `rec-1006-0933` | P `cd77e7bd…` | `create+fund` `5BB6wVZD…`; 6003 do verificador `5Cb4MJjL…`; `Released` `57DpkE3b…`; 6007 `3z9YCshN…` |
| `rec-1006-1011` | T `104f9a21…`, P `3e115ca8…` | T: `create+fund` `26nYjwz1…`, 6021 `2uAyJwHq…`, `RefundedOnTimeout` `4W3Gn3vc…`; P: `create+fund` `4ZdZFFEJ…`, 6014 `5pyEKyHb…`, 6003 `3UACKnby…`, `Released` `24oguMEB…`, 6007 `5CR2s5Jb…` |

`getSignaturesForAddress` desde a última assinatura do D9 de cada carteira:
- buyer 6, executor 6 e deployer 4: as **16 assinaturas** são exatamente as
  enviadas nas gravações;
- nenhuma outra escrita.

Saldos (slot 508.142.476), fechando por lamport e por unidade com essas 16
transações:

| Conta | Fim do D9 | Agora | Composição |
| --- | ---: | ---: | --- |
| deployer | 2.805.214.240 | 2.805.194.240 | 4 taxas (2 × 6021, 2 × 6007) |
| buyer | 121.288.800 | 103.351.800 | 5 × (Job 2.092.960 + vault 1.488.440 + taxa 5.000) + 1 taxa |
| executor | 29.955.000 | 29.925.000 | 6 taxas |
| ATA do buyer (Test USDC) | 999.996.000.000 | 999.992.000.000 | −5 (cinco Jobs) +1 (refund de `104f9a21`) |
| ATA do executor | 4.000.000 | 6.000.000 | +2 (dois `Released`) |

Estado dos Jobs (`job show`, só leitura):
- `cd77e7bd` e `3e115ca8`: `Released { 225384a7… }`;
- `104f9a21`: `RefundedOnTimeout`;
- `8ab4ee8d` e `8bce67f2`: continuam **`Funded`**, com 1 Test USDC cada no
  vault e o prazo vencido. Qualquer um pode pedir o reembolso por timeout;
  isso é escrita e não entra no D10a.

Esses 5 `job_id` passam à lista de consumidos.

## F1 — mudanças por achado

### RD7-01 (`cli/src/tx.rs`)
- `failing_program` lê só as linhas de runtime `Program <pubkey> failed: …`.
  O token precisa ser uma pubkey, então `Program log: …` e `Program data: …`
  nunca contam.
- `innermost_failure` devolve a primeira dessas linhas, porque uma CPI que
  falha registra a própria linha antes da linha de quem a chamou.
- `expected_failure` (agora `pub`), com `Failure { program, code }`, exige:
  - `Custom(code)`;
  - a falha mais interna igual a `Program <program> failed: custom program
    error: 0x…`;
  - para o escrow, nenhuma linha `failed` de outro programa.
- A regra vale para a simulação e para a transação aterrissada. As mensagens
  mostram a falha mais interna.

### RD7-04 (`cli/src/escrow.rs`, `main.rs`, testes)
- `Groth16Seal::tampered()` (`pi_c[10] ^= 1`) substitui a expressão solta
  em `main.rs`.
- Teste em `cli/tests/instructions.rs`:
  - o seal difere só em `pi_c[10]`;
  - o `release_ix` é igual ao builder da suíte com a mesma mutação de
    `d4b_receipts.rs`;
  - diante da instrução honesta, os dados diferem num único byte, no offset
    383.
- Matriz de `expected_failure` em `cli/tests/negative_runs.rs`.

### RD7-07 (`cli/src/rpc.rs`, `tx.rs`, `main.rs`)
- `Rpc::send` devolve `Sent::Signature | Sent::AlreadyProcessed`. O segundo
  caso é o erro −32002 com `data.err = "AlreadyProcessed"` ou a mensagem
  "already been processed" (texto de `solana-transaction-error 2.2.1`).
  - `tx::run` segue com a assinatura local para o mesmo laço de status; o
    resultado vem do `getTransaction`.
  - A comparação da assinatura devolvida pelo RPC continua.
- `read_after` em `main.rs` lê Job, vault e destino num único
  `getMultipleAccounts` com `minContextSlot` = slot da transação. Recusa
  resposta de slot anterior.
  - Usado em `job create`, `deliver`, `settle` e `refund-timeout`.
  - Imprime `after.read_slot=… transaction_slot=…`.

### RD7-02 (`prover/docker-shim/docker`)
Chamadores levantados no fonte pinado:
- `risc0-groth16 3.0.2`, `src/prove/docker.rs:52-58,74-79`: `--version` e
  `run --rm -v <work>:/mnt <tag>`;
- o prover, `lib.rs`: `image inspect --format {{.Id}} <digest>`;
- `risc0-build` (o outro crate com `docker`) não roda: o guest é embutido.

O shim aceita só esses três argv e remonta:
- `docker --version`;
- `docker --context default image inspect --format {{.Id}} <digest>`;
- `docker --context default run --pull=never --network=none --rm -v
  <work>:/mnt <digest>`.

Regras:
- `<work>` precisa ser absoluto, canônico (`realpath -e`), um diretório
  existente, diferente de `/` e sem `:`, `,` ou quebra de linha.
- Recusa um `DOCKER_HOST` que não seja `unix://`.
- Todo o resto sai com exit 2 e linha `REFUSED` no log.

Cabeçalho do shim e `prover/README.md` corrigidos. O prover passou a mostrar
o stderr do `image inspect`, em vez de dizer "imagem ausente" quando o shim
recusa.

### RD7-03 (`prover/src/lib.rs`)
- `LocalProver::new("local")` com `Prover::prove`/`compress`, e
  `local_executor()` no `execute`.
- `default_prover`/`default_executor` saíram. Os três itens já eram
  exportados por `risc0-zkvm 3.0.3` com a feature `prove`, que já estava
  ativa.
- `local_prover_env` roda no início de `check`, `prove` e `compress`. Recusa:
  - `RISC0_PROVER` diferente de vazio ou `local`;
  - qualquer `BONSAI_*` (o valor nunca é impresso);
  - qualquer `RISC0_DEV_MODE`.
- Efeito colateral: sem o caminho Bonsai, o binário caiu de 100.371.472 B
  (D9) para 92.544.872 B.

### Opcionais
- **RD7-06:** `keys::load` recusa `nlink != 1`, antes da checagem de work
  tree (as chaves de `d4/keys` têm `nlink=1`); `Rpc::new` aplica `0600` ao
  `--log`.
- **RD7-09:**
  - `escrow::deadline_margin_problem` diferencia "past the deadline" de "too
    close";
  - nova variante `Expect::Verified`, usada no `settle` positivo. A checagem
    "settlement landed without invoking the Groth16 verifier" saiu de
    `main.rs`, porque agora é feita em `tx::run`.
- **RD7-10:**
  - `rpc::check_url`, por `reqwest::Url`, já presente;
  - `.no_proxy()`;
  - `check` com `VERIFIER_PROGRAM_DATA_ID` (`ENdLkqHp…`),
    `VERIFIER_PROGRAM_LEN` (199.256) e `VERIFIER_PROGRAM_SHA256`
    (`34ae6e5c…`).

### Testes novos
- `cli/tests/negative_runs.rs` (10). Usa o RPC falso em processo,
  `cli/tests/fake_rpc/mod.rs` (127.0.0.1, só std), e keypairs em memória,
  sem arquivo.
- `cli/tests/guards.rs` (5).
- `cli/tests/instructions.rs` (+1).
- `prover/tests/docker_shim.rs` (2, com `docker` falso).
- Teste unitário `only_a_local_prover_environment_is_accepted` (+1).

## F2 — testes que falham no D9 e passam agora

**Árvore "antes":** `d10a/before` é um clone local de `39a87f7`, só com:
- os testes novos que compilam contra a API do D9 (`instructions.rs`,
  `negative_runs.rs`, `fake_rpc/`, `prover/tests/docker_shim.rs`);
- `pub` em `expected_failure`;
- `tampered()` copiado literalmente da expressão do D9.

O diff está em `d10a/logs/before-src.diff`. `guards.rs` e o teste unitário de
ambiente usam API nova e não rodam no D9: a evidência "antes" deles é o R-D7
(PoCs, log do W8) e o binário do D9 abaixo.

### CLI (`cargo +1.89.0 test --locked --offline --no-fail-fast`)

| Teste | D9 (`c3`) | D10a (`c2`) |
| --- | --- | --- |
| `an_escrow_code_matches_only_an_innermost_escrow_failure` | **FAILED**: "escrow:6000 on a verifier failure" | ok |
| `an_overlapping_escrow_code_is_refused_before_sending` | **FAILED**: `unwrap_err()` sobre `Ok(Outcome { … verifier_invoked: true })`, transação enviada | ok |
| `already_processed_is_resolved_by_the_signature_status` | **FAILED**: `rpc sendTransaction: {"code":-32002,…"AlreadyProcessed"…}` | ok |
| `already_processed_never_turns_a_failed_transaction_into_a_success` | **FAILED** (erro do envio, não do resultado) | ok |
| `already_processed_without_a_landed_transaction_is_not_landed` | **FAILED** (idem) | ok |
| `an_existing_log_file_is_set_to_0600` | **FAILED**: `left: 420` (0644) × `right: 384` (0600) | ok |
| `a_hard_linked_keypair_file_is_refused` | **FAILED**: "not a readable keypair file", ou seja, o arquivo com hard link chegou à leitura | ok |
| `codes_lines_and_successes_must_all_agree` | ok | ok |
| `a_signature_other_than_ours_is_refused` | ok | ok |
| `tamper_seal_needs_expect_error_and_is_refused_before_any_rpc` | ok | ok |
| `tampered_seal_flips_only_pi_c_10_and_keeps_the_suite_bytes` | ok: o D9 já fazia a mutação certa (RD7-04 era lacuna de teste) | ok |
| `guards.rs` (5) | não compila no D9 (API nova) | ok |

- **D9:** `negative_runs` 3 passed, 7 failed; `instructions` 15/15; exit 101.
- **D10a:** `guards` 5, `instructions` 15, `negative_runs` 10. **30/30**, exit
  0, 5 min 12 s, RSS 2,1 GB. Só os 13 warnings conhecidos do crate do
  programa compilado no host.

### Shim (`cargo +1.89.0 test --release --test docker_shim`)

| | D9 (`p4`) | D10a (`p2`) |
| --- | --- | --- |
| `the_caller_argv_lists_are_rebuilt_exactly` | **FAILED**: `inspect`, `S1` e `S1-local-unix-socket` sem `--context default` | ok |
| `every_other_argv_is_refused_before_docker_runs` | **FAILED**: 19 de 22 chegam ao `docker` (S2 `--network=host --pull=always --privileged -v /:/host`, S4 `container run`, S5 `--context`, S6 `pull`, S7, flag a mais, caminho relativo, `/`, inexistente, não canônico, com `:`, `-v …:/host`, `DOCKER_HOST=tcp://…`, `inspect` de outra imagem, por tag ou com flag, `version`, `--version --format`, argv vazio); só S3, digest no lugar da tag e outra imagem foram recusados | ok |

### CLI binária contra RPC falso em 127.0.0.1

Roteiro `d10a/poc/rpc/run10.sh` + `fake10.py`, cópia estendida do PoC do R-D7.
As transações foram assinadas com `d4/keys/executor.json`, só por caminho
(aprovado no Plan Mode), e têm blockhash falso; nada foi a devnet.
- D9 = `d9/targets/cli/release/vericode` (`7e7a9260…`);
- D10a = `d10a/targets/cli/release/vericode` (`e6cd4e29…`).

| Cenário | D9 | D10a |
| --- | --- | --- |
| `ok6021` (`escrow:6021`) | PASS, exit 0 | PASS, exit 0 |
| `flip` (tx com sucesso) | `UNEXPECTED`, exit 1 | `UNEXPECTED`, exit 1 |
| `changed` (estado alterado) | `UNEXPECTED`, exit 1 | `UNEXPECTED`, exit 1 |
| `wrongsig` | erro "RPC returned signature", exit 1 | idem |
| `simsuccess` | nada enviado, exit 1 | idem |
| `overlap6003` + `escrow:6003` (`--tamper-seal`) | **PASS, exit 0**, enviado | **recusado na simulação**, falha mais interna = verificador, nada enviado, exit 1 |
| `overlap6000` + `escrow:6000` | **PASS, exit 0**, enviado | **recusado na simulação**, nada enviado, exit 1 |
| `overlap6003` + `verifier:6003` | PASS, exit 0 | PASS, exit 0 |
| `noverifier` (settle positivo sem verificador) | **PASS impresso**, enviado, depois erro, exit 1 | "did not match Verified (the Groth16 verifier was not invoked); nothing was sent", exit 1 |
| `mainnet` (`check` e `job show`) | recusado pelo genesis, exit 1 | idem |
| `none` (`--tamper-seal` sem `--expect-error`) | recusado antes de qualquer RPC | idem |
| `notx` | erro após 30 `getTransaction`, exit 1 | idem |
| `refundok` (controle positivo) | PASS, exit 0 | PASS, exit 0 |
| `already` (−32002 AlreadyProcessed no envio) | **erro falso**: `rpc sendTransaction: {…AlreadyProcessed…}`, exit 1 | "already processed; resolving … by its status" → PASS; `RefundedOnTimeout`, vault 0, buyer +1.000.000; exit 0 |
| `lag` (nó atrasado) | **erro falso**: lê `Funded`, vault 1.000.000 → "refund state or balances differ", exit 1 | um −32016 com retry, depois `after.read_slot=600000010` → `RefundedOnTimeout`, vault 0, buyer +1.000.000; exit 0 |

Logs: `d10a/logs/fake-d9.log`, `fake-new.log`; `fake-{d9,new}.jsonl` com
`0600`.

### Prover binário com ambiente Bonsai (RD7-03)

Roteiro `d10a/poc/prover/bonsai10.sh`, com listener em 127.0.0.1 que registra
método e caminho. `BONSAI_API_KEY` é fictícia.
- D9 = `d9/targets/prover/release/vericode-prover` (`79b83528…`);
- D10a = `3f66e1c0…`.

| Caso | D9 | D10a |
| --- | --- | --- |
| `RISC0_PROVER=bonsai` + `BONSAI_API_URL/KEY` | **1 conexão: `GET /images/upload/4da06f90…`**, "server error", exit 1 | "refusing to prove: BONSAI_API_KEY is set, BONSAI_API_URL is set, RISC0_PROVER=bonsai", **0 conexões**, exit 1 |
| só `BONSAI_API_URL/KEY` | **1 conexão**, idem | recusa, 0 conexões |
| `RISC0_PROVER=ipc` | tenta um `r0vm` externo ("No such file or directory") | recusa |
| `RISC0_DEV_MODE=1` | panic do `disable-dev-mode`, exit 101 | recusa limpa, exit 1 |

Nos oito casos nada foi gravado no diretório de saída. O valor da chave
fictícia não aparece em nenhuma saída.

## F3 — regressão (`--locked --offline`, homes copiadas, um processo por vez)

| Passo | Comando | Resultado |
| --- | --- | --- |
| `k1` | core, `cargo +1.85.0 test` (core-a) | **42/42** |
| `k2` | core, `cargo +1.89.0 test` (core-b) | **42/42** |
| `c1` | `cargo +1.89.0 build --release` (cli) | exit 0, 1 min 02 s, 0 warnings; binário 5.224.584 B `e6cd4e29…` |
| `c2` | `cargo +1.89.0 test --no-fail-fast` (cli) | **30/30** (14 do D7 + 16 novos) |
| `s1` | `anchor/tests-local`, `SBF_OUT_DIR` com os `.so` de devnet (`cdf6967f…`/`34ae6e5c…`) | **61/61**: d4b_receipts 2, escrow 27, fixtures 2, layout 7, regressions 7, settlement 16; 5 min 51 s |
| `p1` | `cargo +1.89.0 build --release` (prover, `JOBS=4`) | exit 0, 31 min 39 s, RSS 2,1 GB, 0 warnings; binário 92.544.872 B `3f66e1c0…` |
| `p2` | `cargo +1.89.0 test --release` (prover) | **5/5** unitários (os 4 do D7 + ambiente local) e **2/2** `docker_shim` |
| `p3` | `vericode-prover check` | `guest.sha256=e09ba8cf…`, `image_id=4da06f90…fb1a`, `admitted=true`, `selector=73c457ba`, `prover=LocalProver env=ok` |

## F4 — prova local e compressão pelo shim novo (sem devnet)

Execução `X` (`d10a/bin/q3_prove.sh`), destacada e sozinha:
- `job_id` aleatório `303b5b91…9309`, que nunca vai a devnet;
- binário `3f66e1c0…`; shim do repositório `2a8f75b8…`;
- ambiente (`env -i`): `RISC0_PROVER=local`, `RISC0_EXECUTOR=local`, 0
  `BONSAI_*`, 0 `RISC0_DEV_MODE`.

```text
prove → prove.prover=LocalProver; prove.seconds=7.9; receipt_type=Composite; local_verify=ok
        negative.wrong_image=rejected; journal_equal_to_core=true; verdict=PASS
        composite_receipt_bytes=221540 sha256=ec3a5bcf…68ca
compress → docker_shim=/home/lucas/src/vericode/prover/docker-shim; compress.prover=LocalProver
           compress.seconds=131.2; receipt_type=Groth16; local_verify=ok; wrong_image=rejected
           journal_equal_to_composite=true; verifier_parameters=73c457ba541936f0…eedc
           groth16_receipt_bytes=827 sha256=7a2318d2…8932; journal_sha256=journal_digest=62444adc…da03
           seal_sha256=ddca5f26…414d; verdict=PASS
verify → Groth16; local_verify=ok; wrong_image=rejected; selector/seal/image_id/journal/journal_digest=matches
         artifact_hash=225384a7…ea73; verdict=PASS
```

`docker-shim.log`, escrito pelo shim novo com o argv real do `risc0-groth16
3.0.2`:

```text
2026-10-06T13:54:27-03:00 /usr/bin/docker --context default run --pull=never --network=none --rm -v /home/lucas/.local/share/vericode-spikes/d10a/receipts/X/groth16-work:/mnt risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331
```

Memória e arquivos:
- RSS do prover: 0,6 GB na prova e 1,45 GB na compressão.
- **No pico, 208 MB disponíveis e 1,4 GB de swap.** Não houve exit 137.
- Sem pull: a imagem já presente por digest.
- `proof.json` pertence a `root`, como no D9.

## F5 — devnet, só leitura, com o binário novo

`vericode check`:

```text
check.escrow=GZqbL2Tb… upgrade_authority=none deployed_slot=507798457
check.escrow_program_data bytes=395064 sha256=cdf6967f…8133
check.verifier=THq1q… program_data=ENdLkqHp… upgrade_authority=none bytes=199256 sha256=34ae6e5c…6cd1
check.mint=9TE2V… decimals=6 freeze_authority=none supply=1000000000000
check=ok
```

- `job show`: P′ `Released { 225384a7… }` e T′ `RefundedOnTimeout`, vaults 0,
  authority = PDA do Job.
- Saldos no slot 508.156.224 iguais aos do F0: nenhuma escrita.

## Errata de `docs/d7-cli-results.md:128` (decisão 4 do D10a)

O relatório do D7 lista "`--expect-error`" entre os testes de `cli/`. No D7,
só o parse era testado (`expected_errors_name_the_failing_program`); o
casamento com a transação e o `--tamper-seal` não tinham teste (RD7-04).

Desde o D10a, os dois têm teste:
- `negative_runs.rs`: matriz de `expected_failure` e `tx::run` contra RPC
  falso;
- `instructions.rs`: `tampered_seal_flips_only_pi_c_10_and_keeps_the_suite_bytes`.

O arquivo do D7 não foi editado, por ser relatório histórico.

## Desvios e incidentes

1. **Binário do prover sobrescrito pela árvore "antes"** (13:39).
   - O que aconteceu:
     - o passo `p4` (`cargo test --release --test docker_shim` em
       `d10a/before`) usou o mesmo `CARGO_TARGET_DIR` do repositório;
     - ao compilar o binário `vericode-prover` do D9 para o teste, o cargo
       substituiu o link `targets/prover/release/vericode-prover` por ele
       (`b7d7d618…`, 100.372.520 B, sem as strings novas).
   - Consequência: a prova e a compressão das 13:46–13:51 rodaram com o
     binário do D9 e o shim da árvore "antes" (log sem `--context default`).
   - Os passos anteriores ao `p4` usaram o binário novo `3f66e1c0…` e
     continuam válidos: testes, `check` e Bonsai das 13:32–13:33.
   - Correção:
     - a execução errada foi preservada como `receipts/X-wrong-binary` e
       `logs/wrongbin-*`;
     - `cargo build --release` no repositório religou o artefato sem
       recompilar (1,3 s), de volta a `3f66e1c0…`;
     - a execução `X` foi repetida, registrando o hash do binário e do shim
       (F4).
   - A CLI não foi afetada: o release `e6cd4e29…` é de 13:33, e o `c2` rodou
     antes do `c3`.
   - Lição para o D10: não compartilhar target entre árvores com pacotes de
     mesmo nome.
2. **Desconexão da sessão do Claude Code** durante a compressão errada.
   - No pico, a memória disponível caiu a 80 MB e o swap de 2 GB encheu
     (13:47:49, `wrongbin-mem-X.log`).
   - As filas destacadas (`setsid nohup`) continuaram e terminaram.
   - Ao retomar, o estado foi reconferido: filas, logs, `git status` e o
     `negative_runs.rs`, idêntico à cópia testada.
   - A causa provável é a pressão de memória, como no D7.
3. **Teste do shim antes do build:** o shim novo foi exercitado em bash, com
   o `docker` falso, antes do build (`d10a/poc/shim/quick.log`).

## Invariantes do guia §7

Nenhuma mudou: o programa, o core e o verificador são os mesmos, e a suíte
passou 61/61 com os `.so` de devnet. A CLI não ganhou regra econômica: as
mudanças só recusam mais cedo, rotulam melhor ou leem no slot certo. A
invariante 10 (falha de verificação reverte) passa a ser rotulada pelo
programa certo na CLI (`verifier:N`, nunca `escrow:N`).

## Fronteiras

- **Rede:**
  - `https://api.devnet.solana.com`, só leitura (`check`, `job show`,
    `getSignaturesForAddress`, saldos);
  - `127.0.0.1` para os RPCs falsos e o listener Bonsai;
  - nenhum fetch do crates.io, rustup ou S3 (`--offline`; `RECURSION_SRC_PATH`
    local `744b999f…`);
  - container Groth16 com `--network=none`.
- **Devnet:** nenhuma escrita, deploy ou airdrop.
- **Docker:** só o `compress` pelo shim, com a imagem já presente por digest,
  sem pull. A execução errada também passou só pelo shim do D9.
- **Chaves:**
  - nenhum keypair novo;
  - `d4/keys/executor.json` usado só por caminho, contra 127.0.0.1;
  - só pubkeys impressas.
  - Varredura de `d10a/{logs,bin,poc}` e da árvore: 0 arrays de 64 bytes; os
    199 base58 de 64 bytes são assinaturas, nenhuma com metade final igual a
    pubkey do projeto; as palavras "seed phrase"/"bip39" aparecem só nas
    regras dos documentos.
- **Repositório:**
  - só `cli/src`, `cli/tests`, `prover/src`, `prover/tests`, o shim e os
    documentos autorizados;
  - nenhum lock, `Cargo.toml`, programa, core, `zkvm/` ou guest;
  - nenhuma instalação no perfil padrão;
  - `d9/` e `rec-*` intocados (nenhum arquivo novo ou alterado desde as
    12:40);
  - nenhum arquivo do gate em `/tmp` (`TMPDIR=d10a/tmp`).

## Riscos abertos

- **RD7-08 (info):** um negativo `escrow:6021` perto do prazo pode virar
  reembolso real. A CLI reporta `UNEXPECTED`.
- **Gravação:** a seção "Gravação" do roteiro usa os binários do D9,
  anteriores a estas correções. Os negativos dela continuam os da CR2.
- **Jobs `Funded` do humano** (`8ab4ee8d`, `8bce67f2`): 2 Test USDC parados,
  reembolsáveis por qualquer um depois do prazo.
- **Memória do WSL:** compressão a 208 MB livres. Na execução errada, 80 MB
  e o swap cheio derrubaram a sessão.
- **Shim:** depende do Docker CLI aceitar `--context default` (Docker CE
  29.8.1 aceita). `DOCKER_CONFIG`, certificados e o próprio daemon não são
  auditados.
- **"Already processed":** reconhecido pelo texto de `solana-transaction-error
  2.2.1` e pelo `data.err`; uma mudança de formato do RPC cairia no erro
  comum, sem sucesso falso.
- Sem e-stop; rent preso; mint authority = deployer; ImageID não
  recertificado; spec v1 trivial.

## Artefatos fora do clone

`~/.local/share/vericode-spikes/d10a/` (`0700`; logs `0600`):
- `bin/`: `env10.sh`, `f0_copy.sh`, `q1_prover.sh`, `q2_rest.sh`,
  `q3_prove.sh`, `rpc10.py`, `secret10.py`;
- `poc/`:
  - `rpc/` (`fake10.py`, `run10.sh`, `accounts.json`);
  - `prover/` (`listener.py`, `bonsai10.sh`);
  - `shim/` (`docker` falso, teste rápido);
- `logs/`:
  - `timeline.log`;
  - `f0-*`, `p1`–`p5`, `c1`–`c3`, `p4`, `k1`, `k2`, `s1`;
  - `fake-{d9,new}.*`, `bonsai-{d9,new}.log`;
  - `x-*`, `mem-X.log`, `wrongbin-*`, `f5-devnet.log`, `f6-secrets.log`,
    `before-src.diff`;
- `before/` (árvore "antes"), `homes/`, `targets/`, `out/devnet`,
  `artifacts/`, `receipts/{X,X-wrong-binary}`.
