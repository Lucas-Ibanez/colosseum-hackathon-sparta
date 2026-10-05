# D7 — CLI e prover reproduzíveis no repositório; fluxo de ponta a ponta em devnet

Data: 2026-10-05 · Executor: Claude Code (Opus 5.5) · Gate anterior: D4b
(`599837d` sobre `561b1b6`) · Prompt: versão revisada de
`docs/handoffs/d4b-to-d7.md`, com as decisões 1 a 4 ratificadas pelo humano.

## Resultado

**D7 CONCLUÍDO.** O fluxo do MVP em devnet agora sai do repositório:
- `prover/` (`vericode-prover`) prova o guest admitido e comprime para Groth16;
- `cli/` (`vericode`) cria, financia, entrega, liquida e reembolsa Jobs no
  escrow imutável `GZqb…`.

Com essas duas peças, em devnet:
- **Job P (PASS):** `job create` → prova nova `(21, 42)` → `job settle` com
  `deliver`+`release` na mesma transação → `Released`. A receipt foi
  verificada por CPI ao verificador Groth16 imutável (99.541 CU) na mesma
  instrução que pagou o executor:
  [`4oWhwZfU…`](https://explorer.solana.com/tx/4oWhwZfUzZhVhrTydrhdBx1TJxWVH2mdWdKwiUhtHtMnhmeJg3MprEshvp592hYMgsaiwwhyvsDHek1egS9zKM1L?cluster=devnet).
- **Job T (timeout):** refund antes do prazo → 6021; depois do prazo →
  `RefundedOnTimeout`:
  [`3fiNWgTW…`](https://explorer.solana.com/tx/3fiNWgTWdscCB8kRTE4gxacQgNXCZ36NHNhazzgUtRBiF7ub7Zs3NDBBZ7qwEdXGEy2HGUsZyyZX9sv7gxFsVZ7?cluster=devnet).
- **Negativos**, com estado igual antes e depois:
  - journal do Job A no Job P → 6014;
  - **invariante 9 em devnet:** `release` de novo no Job A → 6007;
    `refund_on_fail` de novo no Job B → 6008.

Claim (CD7, inalterado): a receipt Groth16 do VeriCode é **verificada em
devnet por CPI ao verificador Groth16 imutável de risc0-solana v3.0.0**, na
mesma instrução que libera ou devolve o Test USDC do Job. Nunca "Verifier
Router"; nunca mainnet.

Também neste gate:
- RD4A-07 (a), (e) e (f) fechados;
- `README.md` de entrega (M6/M7);
- roteiro da demo em `docs/demo-script.md`.

## Linha do tempo (2026-10-05, -03:00)

| Hora | Fase |
| --- | --- |
| 17:32–17:38 | preflight Git e perfil; raiz `d7/` |
| 17:38–17:44 | checagem do D4b (locks, `program show`/`dump`, estado, guest, Docker); sondagem de dependências offline fora do clone |
| 17:49 | Plan Mode: plano aprovado pelo humano |
| 17:51–18:19 | `prover/`: build release (25 min 25 s), testes 4/4, `check`; commit `a7c6e8a` |
| 18:19–18:23 | `cli/`: build release (29 s), testes 14/14, R0 (`check`, `job show` A/B); commit `deececa` |
| 18:23–18:34 | `anchor/tests-local`: duas interrupções por pressão de memória (ver Desvios); suíte 61/61 com o rebuild |
| 18:34–18:35 | suíte 61/61 com os bytes de devnet; core 42/42 A/B; commit `0ec421b` |
| 18:35–18:36 | W1–W5: SOL, Job T, 6021 em T, Job P |
| 18:36–18:39 | prova e compressão do P |
| 18:39–18:40 | W6–W9: 6014 em P, release de P, 6007 em A, 6008 em B |
| 18:40–18:42 | W10: espera do prazo de T e refund por timeout |
| 18:42 | R1: estado final e saldos |

## Preflight e checagem do D4b

| Checagem | Resultado real |
| --- | --- |
| Git | `/home/lucas/src/vericode`, `main`, HEAD `599837d` sobre `561b1b6`; `status --short` e `--ignored` vazios; `diff --check` 0 |
| Perfil padrão (início) | `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes; `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| Locks | raiz `191802b2…`, zkvm `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…` |
| `program show GZqb…` (CLI Agave 2.3.9, `-k` do deployer só para satisfazer a CLI) | `Authority: none`; Data Length 395064; ProgramData `B7s9JJVy…` |
| `program dump` | 395.064 bytes, SHA-256 `cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133` |
| Estado (`devnet.py state`, cópia em `d7/bin` com logs em `d7`) | S e A `Released`, B `RefundedOnFail`, C `RefundedOnTimeout`; vaults 0; ATA executor 2.000.000; ATA buyer 999.998.000.000; deployer 2.925.249.240, buyer 35.644.400, executor 9.980.000 lamports |
| `vericode-guest.bin` preservado | 180.300 bytes, `e09ba8cf…`; `recursion_zkr.zip` `744b999f…` |
| Imagem Docker | `risczero/risc0-groth16-prover@sha256:7f173963…` presente (5,21 GB) |
| IDL do D4a (`d4/out/idl/d4a-final.json`) | `e8ce2c20…`; discriminadores e ordem de contas conferidos |

**Sondagem antes do plano** (fora do clone, `d7/proto`): com os manifests
propostos, `cargo metadata --offline` e `cargo fetch --locked --offline`
passaram nos dois grafos. O lock da CLI só renomeia o pacote raiz do lock de
`tests-local` (708 pacotes), e o do prover só renomeia o do lock
`ec0dd8d6…` (460 pacotes). O fetch único do crates.io autorizado não foi
usado.

## Ambiente (fora do clone)

Raiz `~/.local/share/vericode-spikes/d7/` (`0700`; logs `0600`):
- `bin/env.sh`: cópia de `d4b/bin/env.sh` com `D=d7` (`zk`, `d2c`,
  `core_lane`, `sol`);
- `bin/cli.sh`: o wrapper de log do D4b, que filtra a saída das CLIs do Agave
  e para se aparecer texto de mnemônico;
- `bin/devnet_d4b.py`: cópia do driver do D4b, usada só para o `state` da
  checagem;
- `bin/d4b_fixtures.py`, `bin/suite.sh`, `bin/prove_p.sh`;
- `homes/zkvm`: cópia de `d4/homes/zkvm`;
- `artifacts/`: cópias de `recursion_zkr.zip` e `vericode-guest.bin`;
- `out/rebuild` e `out/devnet`: escrow (o dump `cdf6967f…`) mais o
  verificador `dab6746d…` ou `34ae6e5c…`;
- `jobs/{P,T}.json`, `receipts/P`, `logs/`.

## Fase 1 — `prover/` (commit `a7c6e8a`)

| Item | Valor |
| --- | --- |
| Manifest | workspace próprio; `bincode =1.3.3`, `risc0-zkvm =3.0.3` (`disable-dev-mode`, `prove`), `vericode-core` por path |
| Lock | `prover/Cargo.lock` `8b76f1e1a8595208c0d6ce901b28e255f1ae0107a5549ffeca0a87f484446814`; o diff para `d4/receipts/Cargo.lock` é só `d4-receipts` → `vericode-prover`; 0 fonte git |
| Guest versionado | `prover/artifacts/vericode-guest.bin`, 180.300 B, `e09ba8cf…`; blob do Git com o mesmo SHA-256; `.gitattributes` `*.bin binary` |
| Shim Docker | `prover/docker-shim/docker` (100755) |
| Build | `zk env CARGO_TARGET_DIR=d7/targets/prover cargo +1.89.0 build --locked --offline --release` → exit 0, 25 min 25 s, RSS máx. 2,1 GB, 0 warnings |
| Testes | `cargo +1.89.0 test --locked --offline --release` → **4/4** (guest admitido; frame de 76 B; execução do guest PASS (7,14) e FAIL (7,15) com journal == core; `0x11` e entrada > 1.000.000 recusados) |
| `check` | `guest.sha256=e09ba8cf…`, `guest.image_id=4da06f90…fb1a`, `guest.admitted=true`, `groth16.selector=73c457ba` |
| `prove 1111…11 7 14` | `Error: "job_id 0x11… is reserved for the versioned test fixtures"` |

## Fase 2 — `cli/` (commit `deececa`)

| Item | Valor |
| --- | --- |
| Manifest | workspace próprio, binário `vericode`; deps `vericode-core`, `solana-{pubkey 2.4.0, instruction 2.3.0, hash 2.3.0, keypair 2.2.3, signer 2.2.1, transaction 2.2.3}`, `reqwest 0.12.28` (blocking, json, rustls/webpki-roots), `serde_json 1.0.145`, `sha2 0.10.9`, `base64 0.22.1`, `getrandom 0.2.16`; dev-deps = as de `tests-local` |
| Lock | `cli/Cargo.lock` `4d979577a6c7e6f60fce119c7dc5ca57b73c8da17a836475e085467fd5304db4`; mesmo conjunto de pacotes de `be94760a…`, só com `vericode-escrow-tests` → `vericode-cli`; 0 fonte git |
| Crates novos | **nenhum**; nenhum fetch do crates.io |
| Build | `d2c env CARGO_TARGET_DIR=d7/targets/cli cargo +1.89.0 build --locked --offline --release` → exit 0, 29,5 s, 0 warnings |
| Testes | `cargo +1.89.0 test --locked --offline` → **14/14**. A primeira compilação falhou com `E0599` (`INIT_SPACE` sem o trait `Space` em escopo), corrigido no teste. Os únicos warnings são os 13 conhecidos do crate do programa compilado no host (`anchor-debug`, `realloc`) |

Os testes (`cli/tests/instructions.rs` inclui
`anchor/tests-local/tests/common/mod.rs`):
- constantes da CLI == `vericode_escrow::*`;
- discriminadores == Anchor == `sha256("global:…")[..8]`;
- PDAs e ATA;
- **as 6 instruções com os mesmos bytes** que `create_job_ix`, `fund_ix`,
  `deliver_ix`, `release_ix`, `refund_on_fail_ix` e `refund_ix`;
- seal e `negate_g1` iguais aos da suíte;
- `CreateIdempotent` executado duas vezes em `solana-program-test`;
- tamanhos 577/883/838/342 B, iguais aos do D4b;
- decoders de `JobAccount` (os 6 estados), token account, mint e ProgramData
  contra Anchor, `spl_token` e o layout do loader;
- receipt com digest ou journal adulterado recusada;
- `--expect-error`;
- chave dentro do clone e modos `0644`/`0640`/`0602` recusados.

**R0 (leitura):** `vericode check` → genesis de devnet; escrow com
authority `none`; ProgramData de 395.064 B `cdf6967f…`; verificador `THq1q…`
(ProgramData `ENdLkqHp…`) com authority `none`; mint com 6 decimais, sem
freeze, supply 10¹²; `check=ok`. `job show` de A e B: `Released`/
`RefundedOnFail`, vaults 0, authority do vault = PDA do Job.

## Fase 3 — `anchor/tests-local` (commit `0ec421b`)

Só testes e fixtures; o programa, o `Cargo.toml` e o lock `be94760a…` não
mudaram.
- `fixtures/groth16/d4b/{S,A,A-fail,B}.txt`, gerados de `d4/receipts-out`
  depois de `sha256sum -c ../logs/receipts-out.sha256` (28/28 OK).
- `tests/d4b_receipts.rs`:
  - vínculo dos 4 vetores aos `job_id`s de devnet;
  - replay em processo do E7 do D4b: 6017, 6014, 6000/6003 no verificador,
    6033, 6019, 6014, 6021 e liquidações;
  - **invariante 9:** A de novo → 6007; B de novo → 6008; S de novo → 6007;
    timeout em A/B/C terminais → 6007/6008/6008.
- `tests/escrow.rs`: variantes no endereço `ADMITTED_MINT`:
  - com freeze → 6024;
  - owner Token-2022 → 3007;
  - **9 decimais → aceito**: documenta RD4A-02; a CLI confere os decimais;
  - ausente → 3012.
- `tests/settlement.rs`: verificador ausente ou conta de dados não executável
  em `THq1q…` → `InstructionError(1, UnsupportedProgramId)`, estado igual.
- RD4A-07 (a): comentários de `verifier_program_is_fixed` e de
  `groth16_fixtures.rs` corrigidos.

| Execução | Resultado |
| --- | --- |
| `SBF_OUT_DIR=d7/out/rebuild` (escrow `cdf6967f…` + verificador `dab6746d…`) | **61/61**: d4b_receipts 2, escrow 27, fixtures 2, layout 7, regressions 7, settlement 16 |
| `SBF_OUT_DIR=d7/out/devnet` (escrow `cdf6967f…` + verificador de devnet `34ae6e5c…`) | **61/61** |
| Core `core_lane lane-a 1.85.0` / `lane-b 1.89.0 test --locked --offline` | 42/42 e 42/42, 0 warnings |

Comando da suíte (`d7/bin/suite.sh`, destacado da árvore do editor):
`d2c env SBF_OUT_DIR=… CARGO_TARGET_DIR=d7/targets/tests CARGO_BUILD_JOBS=2
nice -n 10 cargo +1.89.0 test --locked --no-fail-fast`.

## Fase 4 — devnet

Chaves de `d4/keys` (`0600`), usadas só para assinar; nenhum keypair novo.
CLI: `d7/targets/cli/release/vericode --log d7/logs/cli-tx.jsonl …`; SOL pela
CLI do Agave 2.3.9 via `run_cli` (saída bruta só em log `0600`). Sem compute
budget nem priority fee.

**Jobs novos** (`job_id` de 32 bytes de `getrandom`):

| Job | `job_id` | Job PDA | Vault | Prazo |
| --- | --- | --- | --- | --- |
| T | `05f7493483cba42146cada07822160d2fbec4e67d854fafa32f70ef0bb01d758` | `2U5qA8XdFvfmaShDZzcoS9msfAcqh7JxHTCAhrYnDb9M` | `9yrCiJML5Sp11u3YzkMfxdmMvWtTjr97Ty8ZtzFb8u56` | 507.870.517 (slot 507.868.957 + 1.560) |
| P | `3e4ca0269e4af134738120703ccbfd751ef0d51fa6dcd2d99525d0d3f24c9c57` | `EqrLGmGSvhp8TXY43f7cvsDwp1M9tbygTUYP3suSwNN2` | `MW9jmNCeYDXy4mbhgX9eQtmLN9DbDzJxEWAKntLCq93` | 507.878.067 (slot 507.869.067 + 9.000) |

**Prova do Job P** (`d7/bin/prove_p.sh`, destacado e sozinho):

```text
vericode-prover prove 3e4ca026…9c57 21 42 d7/receipts/P
→ prove.artifact=(21,42) hex=01000000150000002a000000; frame_bytes=76
  prove.seconds=10.7; receipt_type=Composite; local_verify=ok; negative.wrong_image=rejected
  journal_equal_to_core=true; verdict=PASS
  composite_receipt_bytes=221540 sha256=45bcd8cd663c820b227285ed783f06149d64b2069695478566412b6fbe94a98a
vericode-prover compress d7/receipts/P
→ docker_image=risczero/risc0-groth16-prover@sha256:7f173963…; docker_shim=/home/lucas/src/vericode/prover/docker-shim
  compress.seconds=111.2; receipt_type=Groth16; local_verify=ok; negative.wrong_image=rejected
  journal_equal_to_composite=true; verifier_parameters=73c457ba541936f0…eedc
  groth16_receipt_bytes=827 sha256=ad739faf4a74e3afcf5bc10fddb7369deb2a51c041bf8887613af2bc3187bf80
  journal_sha256=journal_digest=e184dc08bda63bb7d4959ea32973a3d552be645b5a7b19aa9617806aa2f18040
  seal_sha256=7ade432da4597e0691038bd7af7f65cdba52a246333d82da0f5e937612e98c06
  docker_run=/usr/bin/docker run --pull=never --network=none --rm -v …/receipts/P/groth16-work:/mnt risczero/risc0-groth16-prover@sha256:7f173963…
vericode-prover verify d7/receipts/P
→ Groth16; local_verify=ok; wrong_image=rejected; selector/seal/image_id/journal/journal_digest=matches; verdict=PASS
```

- Journal do P:
  - `artifact_hash` `225384a7bdc698d8afc3a59ab7ec5f026602a5bce317672b70361b7e5852ea73`
    = `hash_restricted_artifact(21, 42)`;
  - spec, harness e ImageID da v1.
- RSS máximo do prover: 0,6 GB na prova e 1,45 GB na compressão, sem contar
  o container. A memória disponível do sistema caiu a 142 MB no pico.
- `proof.json` fica em `groth16-work/` com dono `root` (Docker rootful).

**Transações** (todas simuladas antes; os negativos foram enviados com
`skipPreflight`, aterrissaram com o erro esperado e deixaram Job, vault e as
duas ATAs byte a byte iguais):

| # | Operação | Pagador | Esperado | Resultado na tx | CU | Tamanho | Fee | Estado igual | Slot | Assinatura |
| --- | --- | --- | --- | --- | ---: | ---: | ---: | --- | ---: | --- |
| W1 | 0,1 SOL → buyer | deployer | ok | ok | — | — | 5.000 | — | — | [`4h9yScpa…`](https://explorer.solana.com/tx/4h9yScpakmZqSKPdfWWp8uoZRrQzHMJrFEGS77b3ypq187G7Wz4kJVsgAWVLCaPrndfdYriKaHoBhwwA8bBpL8gL?cluster=devnet) |
| W2 | 0,02 SOL → executor | deployer | ok | ok | — | — | 5.000 | — | — | [`5WC9qQ8G…`](https://explorer.solana.com/tx/5WC9qQ8G1fzfWDAfGJh36ZRx7XF9aDVYj9YKMFX9zRVqgP9PUSjCybSMMLCWUEhTbhkpZ9zuoCa91YSFV6LRKUX5?cluster=devnet) |
| W3 | T: `create_job`+`fund` | buyer | ok | ok, `Funded` | 48.150 | 577 B | 5.000 | — | 507.868.964 | [`221NrPae…`](https://explorer.solana.com/tx/221NrPaeinZctdvnUS8wiPzfTHmXP8qSTmHsv6r8XTKTuXAoo7mmvSAdnAsyS1HdbGzxgqAcAN9BCXV3gt33nEoZ?cluster=devnet) |
| W4 | T: `refund_on_timeout` antes do prazo | deployer | escrow 6021 | `Custom(6021)` `DeadlineNotReached` | 8.991 | 342 B | 5.000 | sim | 507.869.019 | [`8DmQFdjE…`](https://explorer.solana.com/tx/8DmQFdjEjRsVHTfU1DmgjA6GmXZs52G1fdcuCaKmkSV9qXR6eEKh8MiGpKAKv6xa7LDdNPzK7AS9fbP9DgiyX1S?cluster=devnet) |
| W5 | P: `create_job`+`fund` | buyer | ok | ok, `Funded` | 48.150 | 577 B | 5.000 | — | 507.869.075 | [`4ywPQmTj…`](https://explorer.solana.com/tx/4ywPQmTjecZNYZpSkWeA7FzAdMNTG4gghR3CeQzaDJ3Snd1vh84BrkjXKJrf6RXJw4EXfLMjb1VHCwQbxaMw5eD?cluster=devnet) |
| W6 | P: `deliver(h(7,14))`+`release(journal A, seal A)` | executor | escrow 6014 | `InstructionError(1, Custom(6014))` `JournalJobIdMismatch`; o `deliver` reverteu junto; verificador não chamado | 15.052 | 883 B | 5.000 | sim | 507.869.802 | [`5eNRKgfH…`](https://explorer.solana.com/tx/5eNRKgfHii8gbWvJCNRTUehsT7Z46mXDC4Xorj2mfEfF4QjJ2nvFKwt3wmevJ5L81VXweaYpLzTxk3adKDAcBbJ7?cluster=devnet) |
| **W7** | **P: `deliver(h(21,42))`+`release(P)`** | executor | ok | **`Released { 225384a7… }`; verificador `THq1q…` invocado (99.541 CU); ATA do executor 2.000.000 → 3.000.000; vault 0** | 122.156 | 883 B | 5.000 | — | 507.869.854 | [`4oWhwZfU…`](https://explorer.solana.com/tx/4oWhwZfUzZhVhrTydrhdBx1TJxWVH2mdWdKwiUhtHtMnhmeJg3MprEshvp592hYMgsaiwwhyvsDHek1egS9zKM1L?cluster=devnet) |
| W8 | A: `release` de novo com a receipt A | deployer | escrow 6007 | `Custom(6007)` `AlreadyReleased`; verificador não chamado | 9.989 | 838 B | 5.000 | sim | 507.869.916 | [`61de4rBk…`](https://explorer.solana.com/tx/61de4rBkBSN7UjqFrMed8a4KncQvRiBG4rNHJtqVA46B74tMUBoZ6stXVKx3RH2gMydCbRM4Ki9G7EXemgsqY7na?cluster=devnet) |
| W9 | B: `refund_on_fail` de novo com a receipt B | deployer | escrow 6008 | `Custom(6008)` `AlreadyRefunded`; verificador não chamado | 9.819 | 838 B | 5.000 | sim | 507.869.942 | [`5vZ6TGRo…`](https://explorer.solana.com/tx/5vZ6TGRoF8hnNMCARsw9AdUkrQSQccavrNVxJLuXbiu1VH3rZ8wM1wfuk2DGpFGq1g4YqdHeysBFKdmi4Zag3pjG?cluster=devnet) |
| **W10** | **T: `refund_on_timeout`** (slot 507.870.548 > prazo 507.870.517) | buyer | ok | **`RefundedOnTimeout`; ATA do buyer +1.000.000; vault 0** | 14.605 | 342 B | 5.000 | — | 507.870.556 | [`3fiNWgTW…`](https://explorer.solana.com/tx/3fiNWgTWdscCB8kRTE4gxacQgNXCZ36NHNhazzgUtRBiF7ub7Zs3NDBBZ7qwEdXGEy2HGUsZyyZX9sv7gxFsVZ7?cluster=devnet) |

Assinaturas completas das liquidações do D7:
- W7, release de P:
  `4oWhwZfUzZhVhrTydrhdBx1TJxWVH2mdWdKwiUhtHtMnhmeJg3MprEshvp592hYMgsaiwwhyvsDHek1egS9zKM1L`;
- W10, timeout de T:
  `3fiNWgTWdscCB8kRTE4gxacQgNXCZ36NHNhazzgUtRBiF7ub7Zs3NDBBZ7qwEdXGEy2HGUsZyyZX9sv7gxFsVZ7`.

**CU de W7, pelos logs:**
- `deliver`: 4.947;
- `release`: 117.209, dos quais 99.541 do verificador e 105 do Token;
- total: 122.156.

São os mesmos valores do smoke S do D4b.

**Estado final (R1, `vericode job show`, slots 507.870.629–672):**

| Job | Estado | Vault | Authority do vault |
| --- | --- | ---: | --- |
| P | `Released { 225384a7… }` | 0 | PDA do Job, sem delegate nem close authority |
| T | `RefundedOnTimeout` | 0 | idem |
| A | `Released { d5aa9223… }` (inalterado) | 0 | idem |
| B | `RefundedOnFail { 343ad778… }` (inalterado) | 0 | idem |

**Saldos** (fecham por lamport):

| Conta | Início | Fim | Composição |
| --- | ---: | ---: | --- |
| deployer `617ogw9T…` | 2.925.249.240 | 2.805.224.240 | −120.000.000 transferidos; 5 taxas (W1, W2, W4, W8, W9) |
| buyer `EZgGUg4J…` | 35.644.400 | 128.466.600 | +100.000.000; 2 × (Job 2.092.960 + vault 1.488.440) de rent preso; 3 taxas |
| executor `EdB25bVh…` | 9.980.000 | 29.970.000 | +20.000.000; 2 taxas (W6, W7) |
| ATA buyer `61tkoEv4…` (Test USDC) | 999.998.000.000 | 999.997.000.000 | −1 (T) −1 (P) +1 (refund de T) |
| ATA executor `HpZkHZ59…` (Test USDC) | 2.000.000 | 3.000.000 | +1 (P) |

## Invariantes do guia §7

| Invariante | Onde está exercitada |
| --- | --- |
| 1. vault controlado por PDA | devnet: os vaults de P e T têm authority = PDA do Job, sem delegate nem close authority (`job show`); suíte: `vault_is_controlled_by_the_job_pda_without_private_key` |
| 2. mint, partes, valor e prazo do Job | devnet: termos v1 e mint admitido na criação de P e T; a CLI confere 6 decimais e sem freeze; suíte: variantes do endereço do mint (6024/3007/3012; 9 decimais aceito pelo programa) |
| 3. `Pass` paga só o executor | devnet W7: +1.000.000 na ATA canônica do executor |
| 4. `Fail` devolve só ao buyer | D4b (B) em devnet; suíte e replay `d4b_receipts` |
| 5. timeout só depois do prazo | devnet W4 (6021) e W10 (`RefundedOnTimeout`) |
| 6. estado e transferência atômicos | devnet W4, W6, W8 e W9 com estado igual; W6 mostra o `deliver` revertido junto com o `release` |
| 7. sem admin nem destino livre | devnet: escrow com authority `none` (`check`), verificador imutável, destino = ATA canônica |
| 8. journal de outro Job/spec/harness/ImageID | devnet W6 (6014, journal do Job A no Job P) |
| **9. terminal impede dupla liquidação** | **devnet W8 (6007) e W9 (6008)**; suíte `d4b_receipts` (6007/6008 em release, refund e timeout) |
| 10. falha de verificação reverte | D4b em devnet (6000/6003); suíte: replay com 6000/6003 e verificador ausente (`UnsupportedProgramId`) |

## Desvios e incidentes

1. **Ordem T antes de P e negativo 6014 como `deliver`+`release`**: previstos
   e ratificados no plano. Como P é liquidado com `deliver`+`release` juntos,
   ele estava `Funded` antes do settle; um `release` sozinho daria 6025.
2. **Duas interrupções da suíte por pressão de memória.**
   - Primeira: build com o paralelismo padrão (12 jobs), em
     `solana-runtime`/`solana-program-test`.
   - Segunda: `CARGO_BUILD_JOBS=4`.
   - Nas duas, a pressão de memória (`/proc/pressure/memory` ≈ 30 % "full"
     em 5 min) derrubou o líder da sessão WSL (`UtilAcceptVsock … failed`)
     e, com ele, o servidor do VS Code e os processos filhos, inclusive o
     build (exit 137). O código não falhou.
   - Correção: suíte destacada da árvore do editor (`setsid nohup`), com
     `CARGO_BUILD_JOBS=2` e `nice`. A prova do P rodou do mesmo jeito.
   - Os logs interrompidos ficaram em `d7/logs/b5-suite-rebuild.killed*.log`.
3. **Mensagem cosmética da CLI.** Em W8, a conferência de prazo do `release`
   num Job vencido diz "too close to the deadline", em vez de "past the
   deadline". No modo negativo ela é só informativa e não mudou o resultado.
   Fica para a próxima mudança da CLI.

## Fronteiras

- **Rede:**
  - só `https://api.devnet.solana.com` (RPC da CLI e da CLI do Agave);
  - nenhum fetch do crates.io (locks resolvidos offline);
  - sem airdrop nem pull Docker;
  - o container Groth16 rodou com `--network=none`;
  - links do Explorer gerados, não acessados.
- **Escritas em devnet:** só a lista fechada W1–W10.
- **Segredos:**
  - nenhum keypair novo;
  - as chaves de `d4/keys` só foram lidas para assinar, pela CLI (que recusa
    caminhos dentro do clone e modo diferente de `0600`) e pela CLI do Agave;
  - nenhuma chave foi impressa;
  - a varredura de `d7/logs` por "seed phrase", "solana-keygen recover" e
    "bip39" só achou nomes e descrições de crates em `cargo metadata`;
    nenhum mnemônico.
- **Repositório:**
  - o programa, o core, `JournalV1`, o guest, `zkvm/` e os locks existentes
    não mudaram;
  - mudanças: `prover/`, `cli/`, testes e fixtures de `tests-local`,
    `.gitattributes`, `.env.example` e documentação.

## Riscos abertos

- **Sem e-stop** (RD4A-06): escrow e verificador imutáveis.
- **Rent preso (F-13):** cerca de 0,0036 SOL por Job, sem `close`.
- **Mint authority = deployer:** só o projeto emite Test USDC; um terceiro
  que queira reproduzir as escritas precisa receber Test USDC.
- **`job_id` público (F-09/R-03):** mitigado por `job_id` aleatório por Job.
- **RPC público com limite de taxa:** a CLI cadencia e reenvia.
- **Memória:** a compressão Groth16 e os builds de `solana-program-test`
  esgotam os 7,6 GiB do WSL quando o editor também está aberto. Rode
  destacado e com pouco paralelismo.
- **ImageID não recertificado; spec v1 trivial.**
- `docs/manifest-schema.md` ainda diz "a implantar em devnet depois da R-D4a"
  na tabela "Vínculos da v1". É errata documental; o arquivo exige Plan Mode
  e ficou fora deste gate.

## Artefatos fora do clone

`~/.local/share/vericode-spikes/d7/`:
- `logs/`: `timeline.log`, `cli-tx.jsonl`, logs de build, teste, prova e de
  cada transação, `receipts-P.sha256`, `mem-P.log`;
- `receipts/P` (receipt `Composite` e `Groth16` e vetores);
- `jobs/{P,T}.json`;
- `out/` (dump do escrow e variantes do verificador);
- `targets/`, `homes/` e `bin/`.
