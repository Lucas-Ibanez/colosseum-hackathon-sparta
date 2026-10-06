# D9 — demo em ambiente limpo, congelamento dos claims e roteiro de gravação

Data: 2026-10-05 (-03:00) · Executor: Claude Code (Opus 5.5) · Gate anterior:
R-D7 (`9d3efa7`, `docs: record R-D7 final review`, sobre `c550a98`) ·
Prompt: versão revisada do item 7 da resposta do R-D7
(`docs/handoffs/r-d7-to-d9.md`), com as decisões humanas de "Decisões humanas
para o D9" (`docs/decisions.md`).

## Resultado

**D9 CONCLUÍDO.** O fluxo do MVP rodou pela CLI e pelo prover do repositório,
num ambiente limpo (CR4, opção (a)), com Jobs novos em devnet:
- **Job P′ (PASS):** `job create` → prova `(21, 42)` nova → `job settle
  --deliver` → `Released`. O verificador Groth16 imutável foi invocado por
  CPI (99.541 CU) na mesma instrução que pagou o executor:
  [`5tjezXYh…`](https://explorer.solana.com/tx/5tjezXYhN361HHUiZcMcJQFXwViWc4cSUfdreHfokdXuB89LrLDt6WPE8KRLhHwNE7rx5nGAgxjdfod6r7pPvDs1?cluster=devnet).
- **Job T′ (timeout):** refund antes do prazo → 6021; depois do prazo →
  `RefundedOnTimeout`:
  [`33ezPvow…`](https://explorer.solana.com/tx/33ezPvow48vSFHYS43i76pDJwkpyKTnDCN3Tdr1p7he8mbHjEpXvsBkt2MrMHKPUNW6AbqWE7oRJ7MriRZNxrjkh?cluster=devnet).
- **Quatro negativos**, todos com Job, vault e as duas ATAs byte a byte
  iguais antes e depois:
  - 6021 em T′;
  - 6014 em P′, com a receipt do Job P do D7, declarada (CR6);
  - **seal adulterado rejeitado pelo verificador** (`verifier:6003`
    `PairingError`), primeira vez pela CLI;
  - 6007 em P′ (invariante 9).

Também neste gate:
- claims congelados: lista de frases permitidas no `README.md` e no roteiro;
- erratas da CR1 (RD7-05 (a)–(d), RD7-04 e RD7-02);
- seção "Gravação" em `docs/demo-script.md`, para o humano gravar o vídeo de
  reserva.

Nenhum código mudou: `cli/`, `prover/`, o programa, o core, `zkvm/`, os locks,
o guest e o `JournalV1` estão como no R-D7.

Claim (CD7, inalterado): a receipt Groth16 do VeriCode é **verificada em
devnet por CPI ao verificador Groth16 imutável de risc0-solana v3.0.0**, na
mesma instrução que libera ou devolve o Test USDC do Job. Nunca "Verifier
Router"; nunca mainnet.

## O que "ambiente limpo" significa aqui (CR4, opção (a))

- **Clone novo** do HEAD em `~/.local/share/vericode-spikes/d9/clone`:
  - `git clone --no-hardlinks` local;
  - HEAD `9d3efa7` e árvore `ee3fcb4f…`, iguais às do repositório;
  - `status --ignored` vazio;
  - os 7 locks e o guest (`e09ba8cf…`) do clone conferem.
- **Homes novas, copiadas** (`cp -a`, offline) das homes isoladas já usadas
  nos gates anteriores, em `d9/homes`:

  | Home do D9 | Origem | Uso |
  | --- | --- | --- |
  | `core-a` | `d1a3/homes/lane-a` (Rust `1.85.0`) | core, lane A |
  | `core-b` | `d1a3/homes/lane-b` (Rust `1.89.0`) | core, lane B |
  | `perfil-a` | `d2c/homes/lane-b` (Rust `1.89.0`, Perfil A) | CLI e `anchor/tests-local` |
  | `zkvm` | `d7/homes/zkvm` + os proxies do rustup de `d1a3/homes/zkvm/cargo/bin` (ver Desvios) | prover |

- **Targets novos** em
  `d9/targets/{core-core-a,core-core-b,cli,tests,prover}`. Todo build e teste
  rodou com `--locked --offline`.
- `recursion_zkr.zip` copiado de `d7/artifacts` (SHA-256 `744b999f…`) e
  apontado por `RECURSION_SRC_PATH`.
- `.so` da suíte lidos de devnet por `getAccountInfo` (só leitura), em
  `d9/out/devnet`:
  - escrow `B7s9JJVy…`: 395.064 B, `cdf6967f…`;
  - verificador `ENdLkqHp…`: 199.256 B, `34ae6e5c…`.
- **Não é uma máquina nova.** As toolchains, o registro do Cargo e a imagem
  Docker já estavam na máquina. Uma máquina nova precisaria de rede: rustup,
  crates.io, S3 e o pull da imagem por digest. O perfil padrão (`~/.cargo`,
  `~/.avm`, `~/.docker`) não foi usado.
- Scripts próprios em `d9/bin`: `env9.sh`, `f1.sh`, `w9.sh`, `prove_pp.sh`
  e `rpc9.py`.
  - Os nomes são `N9_*`/`n9_*`. O `env.sh` herdado não é carregado, e `R`,
    `B`, `D` e `VC` não são definidos (incidente do R-D7).
- Todo processo roda com `env -i`:
  - a CLI vê só `HOME` e `PATH`, sem proxy;
  - o prover vê `RISC0_PROVER=local`, `RISC0_EXECUTOR=local`,
    `RECURSION_SRC_PATH` e um `TMPDIR` próprio.

## Linha do tempo (2026-10-05, -03:00)

| Hora | Fase |
| --- | --- |
| 22:41–22:50 | F0: preflight Git, perfil, locks, guest; devnet só leitura |
| 22:42–22:48 | F1: clone e cópia das homes (311 s) |
| 22:50–23:03 | F1: core ×2, CLI (build e testes) e suíte |
| 22:51–22:56 | F2: erratas sem Plan Mode; Plan Mode (W1–W8 e `manifest-schema.md:70`) aprovado pelo humano às 22:56; errata da linha 70 |
| 23:03–23:36 | F1: build do prover (32 min 32 s), testes, ambiente CR3 e `check`; desvio do `rustdoc` e reexecução (23:36) |
| 23:36–23:37 | F3: checagem com o binário limpo; W1, W2, W3 |
| 23:37–23:40 | F3: prova e compressão de P′ |
| 23:40–23:43 | F3: W4–W7; W8 com espera do prazo |
| 23:43–23:50 | F3: estado final, `getTransaction` das 8 assinaturas, varredura de segredos; F4: roteiro de gravação e ensaio somente leitura do terminal |
| 23:50– | F5: relatório, registros, handoff, revisão do diff, commit |

## F0 — preflight e checagem do R-D7

| Checagem | Resultado real |
| --- | --- |
| Git | `/home/lucas/src/vericode`, `main`, HEAD `9d3efa7` (`docs: record R-D7 final review`) sobre `c550a98`; `status --short` e `--ignored` vazios; `diff --check` 0 |
| Perfil padrão (início) | `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes; `find . -printf '%p %s %T@\n' \| sort \| sha256sum`: `~/.cargo` `d9e12578…` (5.816 entradas), `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| Locks | raiz `191802b2…`, zkvm `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover` `8b76f1e1…` |
| Guest | `prover/artifacts/vericode-guest.bin` 180.300 B, `e09ba8cf…` |
| Receipt P do D7 | `d7/receipts/P`: `sha256sum -c d7/logs/receipts-P.sha256` 7/7 OK |
| Devnet (só leitura, binário da CLI do **D7**, `e6cad858…`) | `check=ok`: genesis de devnet; escrow com authority `none`, 395.064 B `cdf6967f…`; verificador `THq1q…` (ProgramData `ENdLkqHp…`) com authority `none`; mint com 6 decimais, sem freeze, supply 10¹² |
| `job show` P, T, A, B | P `Released { 225384a7… }`, T `RefundedOnTimeout`, A `Released { d5aa9223… }`, B `RefundedOnFail { 343ad778… }`; vaults 0 com authority = PDA do Job |
| Saldos (slot 507.932.655) | SOL: deployer 2.805.224.240, buyer 128.466.600, executor 29.970.000; Test USDC: ATA do buyer 999.997.000.000, ATA do executor 3.000.000. Iguais aos do fim do D7 |

## F1 — build e testes no ambiente limpo

Fila `d9/bin/f1.sh`, destacada (`setsid nohup`), um passo por vez, com
`nice -n 10` e `CARGO_BUILD_JOBS=2` (4 no prover).

| Passo | Comando (no clone, home do D9) | Resultado |
| --- | --- | --- |
| c1 | `cargo +1.85.0 test --locked --offline` (core-a) | **42/42**, exit 0 |
| c2 | `cargo +1.89.0 test --locked --offline` (core-b) | **42/42**, exit 0 |
| c3 | `cargo +1.89.0 build --release --locked --offline --manifest-path cli/Cargo.toml` (perfil-a) | exit 0, 58,9 s, RSS 0,4 GB, 0 warnings; binário 5.206.440 B `7e7a9260…` |
| c4 | `cargo +1.89.0 test --locked --offline --manifest-path cli/Cargo.toml` | **14/14**, 4 min 53 s, RSS 2,1 GB; só os 13 warnings conhecidos do crate do programa compilado no host (`anchor-debug`, `realloc`) |
| c5 | `cd anchor/tests-local && SBF_OUT_DIR=d9/out/devnet cargo +1.89.0 test --locked --offline --no-fail-fast` | **61/61**: d4b_receipts 2, escrow 27, fixtures 2, layout 7, regressions 7, settlement 16; 7 min 09 s, RSS 2,1 GB |
| c6 | `cargo +1.89.0 build --release --locked --offline --manifest-path prover/Cargo.toml` (zkvm) | exit 0, 32 min 32 s, RSS 2,1 GB; binário 100.371.472 B `79b83528…` |
| c7 | `cargo +1.89.0 test --release --locked --offline` (prover) | 4/4 nos testes unitários, mas **exit 101**: o passo de doc-tests não achou `rustdoc` (ver Desvios) |
| c7b | o mesmo, depois de completar os proxies do rustup | **4/4** (guest admitido; frame de 76 B; journal == core; job reservado e entradas inválidas recusados); doc-tests 0/0; exit 0 |
| c8 | ambiente visto pelo prover (CR3) | `RISC0_EXECUTOR=local`, `RISC0_PROVER=local`, `bonsai_vars=0`, `dev_mode_vars=0` |
| c9 | `vericode-prover check` | `guest.sha256=e09ba8cf…`, `guest.image_id=4da06f90…fb1a`, `guest.admitted=true`, `groth16.selector=73c457ba` |

Os binários do D9 diferem dos do D7 (CLI `7e7a9260…` × `e6cad858…`), porque
os caminhos do clone e da home entram no binário. As instruções são as
mesmas: os 14 testes da CLI comparam os bytes com os builders da suíte.

## F2 — erratas e claims congelados (CR1)

| Item | Onde | Correção |
| --- | --- | --- |
| RD7-05 (a) | `README.md` ("Rede no primeiro build"); `prover/README.md` | o primeiro build usa rede para o rustup (toolchain `1.89.0`), o crates.io e o `recursion_zkr.zip` do S3 (`744b999f…`), a menos que `RECURSION_SRC_PATH` aponte para uma cópia local |
| RD7-05 (b) | `README.md` (Requisitos); `prover/README.md` | CLI do Agave `2.3.9` para o dump; imagem Groth16 baixada antes por digest (5,21 GB), porque o prover nunca faz pull |
| RD7-05 (c) | `docs/demo-script.md` (cenas 5a e Gravação) | origem explícita da "receipt de outro Job": um clone limpo não tem nenhuma; no D9, a do Job P do D7; na gravação, a do Job P′ do D9 |
| RD7-05 (d) | `docs/manifest-schema.md:70` (Plan Mode); `docs/escrow-program.md:137` | "a implantar em devnet depois da R-D4a" → "implantado em devnet e finalizado no D4b (upgrade authority `none`)"; "será finalizado" → "foi finalizado (D4b, upgrade authority `none`)" |
| RD7-04 | `docs/decisions.md` (entrada D7); `docs/demo-script.md`; `cli/README.md` | "testado offline" corrigido: `--tamper-seal` e o casamento do `--expect-error` não têm teste unitário (D10a); exercitados em devnet no D9 |
| RD7-02 | `prover/README.md` | o shim troca a tag pelo digest e põe `--pull=never --network=none`, recusa `docker run` sem a tag, mas **não é allowlist de argv**; o único chamador tem argv fixo |
| RD7-03 | `prover/README.md`; `README.md` (Limitações) | o prover obedece `RISC0_PROVER`/`BONSAI_*`; fixar `RISC0_PROVER=local` e conferir |
| RD7-01 | `cli/README.md`; `README.md` (Limitações); roteiro | `escrow:6000` a `6003` podem casar com falha do verificador; os negativos documentados usam só 6014, 6007/6008, 6021 e `verifier:6003` |
| Frases permitidas | `README.md` ("Frases permitidas (congeladas no D9)") e `docs/demo-script.md` | 8 frases e a lista "Não dizer"; mudar exige decisão registrada |

O README ganhou as linhas do D9 na tabela de devnet e os `job show` de P′ e
T′.

## F3 — devnet (W1–W8)

Plano aprovado em Plan Mode às 22:56. Pré-condições cumpridas antes do W1:
- F1 completo;
- `check=ok` e estado/saldos iguais aos do F0, agora com o binário limpo
  (`f3-pre.log`, 23:36);
- `free -m` com 6,2 GB disponíveis e nenhum build rodando.

Chaves de `d4/keys` (`0600`, fora do clone), usadas só por caminho. A CLI
imprimiu só pubkeys. Sem airdrop, sem transferência de SOL, sem compute
budget e sem priority fee. Cada passo: `wrun` (log `0600`, varredura de
mnemônico e de array de 64 bytes) e depois `job show` e `rpc9.py balances`.

**Jobs novos** (`job_id` de 32 bytes de `getrandom`):

| Job | `job_id` | Job PDA | Vault | Prazo |
| --- | --- | --- | --- | --- |
| T′ | `ec9afb74ceb080e09cb3d605d94883ff5ea90cc638c3f086f5e5c6d4cfed5c48` | `CVkF2z3JorinyMfgTvtgircfawf1pJs2sUnayGfxYQh4` | `79xxVtRdbddX435qUMXCWsmRTwJqyYWRkN7pJTV15RSj` | 507.946.068 (slot 507.944.508 + 1.560) |
| P′ | `91ea6fcdd2d0c29942246c664d893634b7c66f6cb2babbe436734a74bef5418e` | `H8BT24fffBjryUiTXKB5HWmCPqtB1QeYo2z3pnzpgKn9` | `2D94VDMu5LRVsmVNXyttaoyPNRFvNj6MMFV1KGZgkQBd` | 507.953.631 (slot 507.944.631 + 9.000) |

**Prova do Job P′** (`d9/bin/prove_pp.sh`, destacada e sozinha; CR3 e CR7):

```text
# d9/logs/cr3-env.log (env -i)
RISC0_EXECUTOR=local
RISC0_PROVER=local
bonsai_vars=0
dev_mode_vars=0
docker_shim_on_path_before_prover=/usr/bin/docker   # o prover põe o shim do clone na frente

vericode-prover prove 91ea6fcd…418e 21 42 d9/receipts/Pp
→ prove.artifact=(21,42) hex=01000000150000002a000000; frame_bytes=76; prove.seconds=9.9
  receipt_type=Composite; local_verify=ok; negative.wrong_image=rejected
  journal_equal_to_core=true; verdict=PASS
  composite_receipt_bytes=221540 sha256=3b5c9d56d8cce97e52b6da1b6100faa9281a44688e05dc5f6c89e72db15846b7
vericode-prover compress d9/receipts/Pp
→ docker_image=risczero/risc0-groth16-prover@sha256:7f173963…
  docker_shim=/home/lucas/.local/share/vericode-spikes/d9/clone/prover/docker-shim
  compress.seconds=106.0; receipt_type=Groth16; local_verify=ok; negative.wrong_image=rejected
  journal_equal_to_composite=true; verifier_parameters=73c457ba541936f0…eedc
  groth16_receipt_bytes=827 sha256=758b1c9bcb74a7fab7ab3b63b6f2393efdf381e12d8dae0a6da5d23d93010496
  journal_sha256=journal_digest=03c6cc05bd2e999c70e97d2fcd0e5b288716aa870975007d803f23cc3d3c27bd
  seal_sha256=7fd2022a207cb8adb17d1bab24d4addf9089234fd8996eee41ba019697b72671
vericode-prover verify d9/receipts/Pp
→ Groth16; local_verify=ok; wrong_image=rejected; selector/seal/image_id/journal/journal_digest=matches
  artifact_hash=225384a7…ea73; verdict=PASS
```

`docker-shim.log` (`d9/receipts/Pp/docker-shim.log`), escrito pelo shim
versionado do clone:

```text
2026-10-05T23:39:08-03:00 /usr/bin/docker run --pull=never --network=none --rm -v /home/lucas/.local/share/vericode-spikes/d9/receipts/Pp/groth16-work:/mnt risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331
```

Memória e arquivos:
- RSS máximo do prover: 0,6 GB na prova e 1,45 GB na compressão, sem
  contar o container.
- **No pico, a memória disponível caiu a 117 MB e o swap de 2 GB encheu**
  (`mem-Pp.log`). Não houve exit 137.
- `proof.json` em `groth16-work/` pertence a `root` (Docker rootful).
- Hashes da receipt em `d9/logs/receipts-Pp.sha256`.

**Transações.** Todas foram simuladas antes. Os negativos foram enviados com
`skipPreflight`, aterrissaram com o erro esperado e deixaram Job, vault e as
duas ATAs byte a byte iguais (`watched accounts unchanged=true`, conferido
com `minContextSlot` = slot da transação).

| # | Operação | Pagador | Esperado | Resultado na tx | CU | Tamanho | Fee | Estado igual | Slot | Assinatura |
| --- | --- | --- | --- | --- | ---: | ---: | ---: | --- | ---: | --- |
| W1 | T′: `create_job`+`fund` | buyer | ok | ok, `Funded`, vault 1.000.000 | 43.650 | 577 B | 5.000 | — | 507.944.524 | [`4eATVTWf…`](https://explorer.solana.com/tx/4eATVTWfUgo6EB7GbaEFttbNhKG7fbR2AtXcS3xxWbF46N8Z4sdCaqSryXbKJrCGXagv7cVVHcahLHekXPCQ9Mms?cluster=devnet) |
| W2 | T′: `refund_on_timeout` antes do prazo (margem 1.493 slots, CR2) | deployer | escrow 6021 | `InstructionError(0, Custom(6021))` `DeadlineNotReached` | 8.991 | 342 B | 5.000 | sim | 507.944.592 | [`3BKnczeK…`](https://explorer.solana.com/tx/3BKnczeKPQd15iCkYfPbnEvDbYayRfbVV9Zokqn3xLyVehJuZSNffgDy1paFTxof8tpYfaN92N789LzB2w4DQTd6?cluster=devnet) |
| W3 | P′: `create_job`+`fund` | buyer | ok | ok, `Funded`, vault 1.000.000 | 45.150 | 577 B | 5.000 | — | 507.944.638 | [`3ME9F9HA…`](https://explorer.solana.com/tx/3ME9F9HAUhutBG8j9GzCXjVSqK9kf2cyitYdRihW8pVPqMZ4YGR81DfahWAtHk9dz1z4X6VeMRPaEg9nqFSGE4Lr?cluster=devnet) |
| W4 | P′: `deliver(h(21,42))`+`release` com a **receipt do Job P do D7** (`job_id 3e4ca026…`, `d7/receipts/P`; CR6) | executor | escrow 6014 | `InstructionError(1, Custom(6014))` `JournalJobIdMismatch`; o `deliver` reverteu junto; verificador não chamado | 15.052 | 883 B | 5.000 | sim | 507.945.288 | [`8Kk5UkX5…`](https://explorer.solana.com/tx/8Kk5UkX5XW8tssPMxNSCEBt71jiv4RriHLDtYqRzb1f1U5WLZfgQmotQTD1ZqecEbyqMmhWSjQqWpXxCJUubJLg?cluster=devnet) |
| W5 | P′: `deliver`+`release` com o seal de P′ adulterado (`--tamper-seal`, `pi_c[10]^=1`) | executor | verifier 6003 | `InstructionError(1, Custom(6003))`; `THq1q… invoke [2]`, `PairingError`, `THq1q… failed: 0x1773` (100.528 CU no verificador) | 119.238 | 883 B | 5.000 | sim | 507.945.343 | [`HXgnGUhh…`](https://explorer.solana.com/tx/HXgnGUhhB4zRgD8Bubs6ULCqpuhoqBSuim3Vbx1kz7V3bK2FFJG3ND2jcTtRFf6QxD9i1bpwKW4z32yFYYEofVr?cluster=devnet) |
| **W6** | **P′: `deliver(h(21,42))`+`release(P′)`** | executor | ok | **`Released { 225384a7… }`; verificador `THq1q…` invocado (99.541 CU) antes da transferência; ATA do executor 3.000.000 → 4.000.000; vault 0** | 122.156 | 883 B | 5.000 | — | 507.945.388 | [`5tjezXYh…`](https://explorer.solana.com/tx/5tjezXYhN361HHUiZcMcJQFXwViWc4cSUfdreHfokdXuB89LrLDt6WPE8KRLhHwNE7rx5nGAgxjdfod6r7pPvDs1?cluster=devnet) |
| W7 | P′: `release` de novo | deployer | escrow 6007 | `InstructionError(0, Custom(6007))` `AlreadyReleased`; verificador não chamado | 9.989 | 838 B | 5.000 | sim | 507.945.434 | [`5T9XE5Y3…`](https://explorer.solana.com/tx/5T9XE5Y3DzqKffoeFFzEhgqxsuzqMstpxtMCQikrZPe1RyP1KWvRmPNs9CFuke1yBgtwg5vVnGZVpwHWZr31Dfrs?cluster=devnet) |
| **W8** | **T′: `refund_on_timeout`** (`--wait`; slot 507.946.080 > prazo 507.946.068) | buyer | ok | **`RefundedOnTimeout`; ATA do buyer +1.000.000; vault 0** | 14.605 | 342 B | 5.000 | — | 507.946.088 | [`33ezPvow…`](https://explorer.solana.com/tx/33ezPvow48vSFHYS43i76pDJwkpyKTnDCN3Tdr1p7he8mbHjEpXvsBkt2MrMHKPUNW6AbqWE7oRJ7MriRZNxrjkh?cluster=devnet) |

Assinaturas completas das liquidações:
- W6, release de P′:
  `5tjezXYhN361HHUiZcMcJQFXwViWc4cSUfdreHfokdXuB89LrLDt6WPE8KRLhHwNE7rx5nGAgxjdfod6r7pPvDs1`;
- W8, timeout de T′:
  `33ezPvow48vSFHYS43i76pDJwkpyKTnDCN3Tdr1p7he8mbHjEpXvsBkt2MrMHKPUNW6AbqWE7oRJ7MriRZNxrjkh`.

Observações:
- **W5 e a RD7-01:** o log tem as duas linhas, `THq1q… failed: … 0x1773` e
  `GZqb… failed: … 0x1773`. Por isso o negativo usou `verifier:6003`, que
  exige a linha do verificador; `escrow:6003` também casaria (RD7-01).
- **CU de W6:**
  - `deliver` 4.947;
  - `release` 117.209, dos quais 99.541 do verificador e 105 do Token;
  - total 122.156, igual ao do Job P do D7 e ao do smoke S do D4b.
- **W1 e W3:** os CU de `create_job` diferem dos do D7 (43.650/45.150 ×
  48.150) por causa da busca dos bumps das PDAs, que depende do `job_id`.
- **Prazo de T′:** 1.560 slots passaram em cerca de 6 min 20 s (~0,24 s por
  slot). O roteiro antigo estimava ~10 min.

**Conferência independente** (`rpc9.py tx` com as 8 assinaturas,
`getTransaction` em `confirmed`, `d9/logs/r1-gettransaction.jsonl`):
- os 8 `err` são os da tabela;
- `verifier_invoked` só em W5 (100.528 CU) e W6 (99.541 CU);
- movimentos de token só em:
  - W1: buyer −1.000.000, vault de T′ +1.000.000;
  - W3: buyer −1.000.000, vault de P′ +1.000.000;
  - W6: vault de P′ −1.000.000, ATA do executor +1.000.000;
  - W8: vault de T′ −1.000.000, ATA do buyer +1.000.000.

**Estado final** (`vericode job show`, slots 507.946.141–163):

| Job | Estado | Vault | Authority do vault |
| --- | --- | ---: | --- |
| P′ | `Released { 225384a7… }` | 0 | PDA do Job, sem delegate nem close authority |
| T′ | `RefundedOnTimeout` | 0 | idem |

**Saldos** (fecham por lamport e por unidade, iguais aos previstos no
plano):

| Conta | Início | Fim | Composição |
| --- | ---: | ---: | --- |
| deployer `617ogw9T…` | 2.805.224.240 | 2.805.214.240 | 2 taxas (W2, W7) |
| buyer `EZgGUg4J…` | 128.466.600 | 121.288.800 | 2 × (Job 2.092.960 + vault 1.488.440 + taxa 5.000) de W1/W3; taxa do W8 |
| executor `EdB25bVh…` | 29.970.000 | 29.955.000 | 3 taxas (W4, W5, W6) |
| ATA buyer `61tkoEv4…` (Test USDC) | 999.997.000.000 | 999.996.000.000 | −1 (T′) −1 (P′) +1 (refund de T′) |
| ATA executor `HpZkHZ59…` (Test USDC) | 3.000.000 | 4.000.000 | +1 (P′) |

## F4 — roteiro de gravação

`docs/demo-script.md` ganhou:
- a lista de frases congelada;
- a seção **"Gravação"**:
  - preparação do terminal com `source …/d9/bin/env9.sh` e as funções `vc`,
    `vp` e `jid`, sem nenhuma leitura de chave;
  - uma pasta nova por tomada;
  - comandos prontos para cada cena, com `job_id` novo, a saída esperada, os
    tempos do D9 e as falas;
  - abas do Explorer a abrir;
  - o que fazer em caso de falha;
- o **plano B** com as 8 transações do D9 rotuladas como "execução
  anterior".

O bloco de preparação foi ensaiado sem escrita: `vc check` (`check=ok`, ~2,2 s),
`vp check` (guest admitido), `n9_zk env` com só `RISC0_PROVER=local` e
`RISC0_EXECUTOR=local`, e `jid` sobre o arquivo do P′. Pasta do ensaio:
`d9/rehearsal`. O vídeo não foi gravado pelo agente (decisão humana 4).

## Invariantes do guia §7

| Invariante | Onde está exercitada no D9 |
| --- | --- |
| 1. vault controlado por PDA | vaults de P′ e T′ com authority = PDA do Job, sem delegate nem close authority (`job show`); suíte 61/61 |
| 2. mint, partes, valor e prazo do Job | termos v1 e mint admitido na criação de P′ e T′ (`terms_v1_admitted=true`); a CLI conferiu 6 decimais e sem freeze |
| 3. `Pass` paga só o executor | W6: +1.000.000 na ATA canônica do executor |
| 4. `Fail` devolve só ao buyer | não exercitada no D9 (fora da lista W1–W8); D4b (B) em devnet e suíte |
| 5. timeout só depois do prazo | W2 (6021) e W8 (`RefundedOnTimeout` no slot 507.946.088 > 507.946.068) |
| 6. estado e transferência atômicos | W2, W4, W5 e W7 com estado igual; W4 e W5 mostram o `deliver` revertido junto |
| 7. sem admin nem destino livre | escrow com authority `none` (`check`); verificador imutável; destino = ATA canônica |
| 8. journal de outro Job/spec/harness/ImageID | W4 (6014, receipt do Job P do D7 em P′) |
| 9. terminal impede dupla liquidação | W7 (6007) |
| 10. falha de verificação reverte | **W5: o verificador rejeita o seal adulterado e nada move** |

## Desvios e incidentes

1. **c7 com exit 101 (ambiente, não código).**
   - Na home `zkvm` copiada, criei só os proxies `cargo` e `rustc` do
     rustup. A original (`d1a3/homes/zkvm/cargo/bin`) também tem `rustdoc`
     e outros proxies.
   - Os 4 testes unitários do prover passaram, mas o passo de doc-tests
     falhou ao executar `rustdoc` ("No such file or directory").
   - Correção: completar os proxies, com o mesmo conjunto da origem menos
     `cargo-risczero` e `rzup`, e repetir só os testes (c7b): 4/4, exit 0.
   - O build (c6) não usa `rustdoc` e não foi refeito.
2. **Checagem de segurança do Claude Code.** O ensaio do terminal foi
   recusado na primeira forma (`bash -c`), por suspeita de `rm`; o script
   não tinha `rm`. O mesmo ensaio rodou direto no shell, sem `bash -c`.
3. **Memória no limite na compressão:** 117 MB disponíveis e swap cheio no
   pico, sem exit 137. A gravação deve rodar sem builds nem editor pesado.

## Fronteiras

- **Rede:**
  - só `https://api.devnet.solana.com`, pela CLI e pelo `rpc9.py`, que é só
    leitura;
  - nenhum fetch do crates.io, rustup ou S3 (tudo `--offline`);
  - sem airdrop nem pull Docker;
  - container Groth16 com `--network=none`;
  - links do Explorer gerados, não acessados.
- **Escritas em devnet:** só W1–W8, na ordem aprovada.
- **Docker:** só o `compress` do prover, pelo shim versionado do clone, com a
  imagem por digest já presente.
- **Segredos:**
  - nenhum keypair novo;
  - as chaves de `d4/keys` só foram lidas pela CLI para assinar;
  - só pubkeys impressas;
  - varredura de `d9/{logs,jobs,bin,receipts}` e da árvore do repositório
    (186 arquivos): 0 arrays de 64 bytes, 0 base58 de 64 bytes cuja metade
    final seja uma pubkey do projeto, 0 texto de mnemônico nos logs.
- **Repositório:**
  - só documentação;
  - `cli/`, `prover/`, o programa, o core, `zkvm/`, os locks, o guest e o
    `JournalV1` não mudaram;
  - nenhuma instalação no perfil padrão;
  - nenhum arquivo do gate em `/tmp`: o prover usou `TMPDIR=d9/tmp`. O
    harness do Claude Code guarda em `/tmp/claude-1000/…` só a saída dos
    seus próprios comandos em segundo plano.

## Riscos abertos

- **Sem e-stop** (RD4A-06): escrow e verificador imutáveis.
- **Rent preso (F-13):** 3.581.400 lamports por Job; agora 8 Jobs (S, A, B,
  C, P, T, P′ e T′).
- **Mint authority = deployer:** só o projeto emite Test USDC.
- **Saldos para a gravação:** o buyer tem 0,121 SOL, cerca de 33 Jobs; o
  executor tem 0,030 SOL.
- **Memória do WSL:** compressão com 117 MB livres no pico.
- **RD7-01, 02, 03, 04 e 07:** abertos até o D10a; RD7-06, 08, 09 e 10 são
  informativos.
- **Errata pendente:** `docs/manifest-schema.md:86` ("como o escrow terá a
  upgrade authority finalizada") ficou fora do escopo autorizado (só a linha
  70) e exige Plan Mode.
- **Comentário do shim:** o cabeçalho de `prover/docker-shim/docker` ainda diz
  "Any other `docker run` is refused". É código, fica para o D10a (RD7-02).
- O relatório do D7 (`docs/d7-cli-results.md:128`) lista `--expect-error`
  entre os testes, mas só o parse é testado (RD7-04). O arquivo ficou fora do
  escopo do D9; a correção está no `cli/README.md` e na errata em
  `docs/decisions.md`.
- ImageID não recertificado; spec v1 trivial.

## Artefatos fora do clone

`~/.local/share/vericode-spikes/d9/` (`0700`; logs `0600`):
- `bin/`: `env9.sh`, `f1.sh`, `w9.sh`, `prove_pp.sh`, `rpc9.py`;
- `logs/`:
  - `timeline.log`, `cli-tx.jsonl`;
  - `c1`–`c9`, `c7b`, `W1`–`W8`;
  - `cr3-env.log`, `prove-`/`compress-`/`verify-Pp.log`, `mem-Pp.log`;
  - `receipts-Pp.sha256`, `r1-gettransaction.jsonl`;
  - `f0-*`, `f3-pre.log`, `post-W7-Pp.log`, `final-*.log`;
- `receipts/Pp` (receipt `Composite` e `Groth16`, vetores e
  `docker-shim.log`);
- `jobs/{Tp,Pp}.json`;
- `clone/`, `homes/`, `targets/`, `out/devnet`, `artifacts/`, `rehearsal/`.
