# D4b — escrow implantado e finalizado em devnet; liquidações PASS, FAIL e timeout com negativos

Data: 2026-10-05 · Executor: Claude Code (Opus 5.5) · Gate anterior: R-D4a
(`561b1b6`, APROVADO COM RESSALVAS, condições CD1 a CD9) · Código implantado:
`fb4bfba` (D4a), sem mudança.

## Resultado

**D4b CONCLUÍDO.** O escrow `vericode_escrow` do D4a está implantado em
Solana devnet e é imutável. Liquidou Jobs reais:
- PASS (Job A), FAIL (Job B) e timeout (Job C);
- antes dos positivos, 8 negativos e o C7, todos rejeitados com o código
  esperado e sem mudança de estado.

| Item | Valor |
| --- | --- |
| Program ID | `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH` |
| ProgramData | `B7s9JJVyD2j8PNgKgjhhB36iSdX9cUpSfZHbbd8nLmbc` |
| Upgrade authority | **`none`** (finalizada no slot 507.798.793) |
| Bytes implantados | 395.064 bytes, SHA-256 `cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133` (dump idêntico ao `.so` do D4a) |
| Verificador chamado por CPI | `THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge` (`risc0-solana v3.0.0`, upgrade authority `None`) |
| Mint Test USDC admitido | `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F`: Tokenkeg, 6 decimais, sem freeze authority |

Claim permitido a partir deste gate (CD7), com os links da seção E7:

> A receipt Groth16 do VeriCode é **verificada em devnet por CPI ao
> verificador Groth16 imutável de risc0-solana v3.0.0**, na mesma
> instrução que libera ou devolve o Test USDC do Job.

Não há Verifier Router no caminho. O smoke (Job S) não é evidência.

## Linha do tempo (2026-10-05, -03:00)

| Hora | Fase |
| --- | --- |
| 13:23–13:27 | preflight Git e perfil padrão; checagem do D4a e do R-D4a; `devnet_state.py` (somente leitura) |
| 13:27–13:36 | E0: raiz `d4b/`, cliente Rust offline, driver RPC, `job_id` do Job C, keypair do buffer; precheck RPC; feature status |
| 13:36 | Plan Mode: plano com os comandos de E1 a E8 aprovado pelo humano |
| 13:36–13:37 | E1: precheck CD2 imediatamente antes da primeira escrita |
| 13:37 | E2: criação do mint |
| 13:37–13:43 | E3, tentativa 1 do deploy (falhou no meio, ver E3) |
| 13:45–13:54 | E3: buffer completado com escritas cadenciadas |
| 13:54–13:55 | E3, tentativa 2 do deploy (sucesso); `program show`; dump |
| 13:55 | E4: SOL, ATAs e estoque de Test USDC |
| 13:55–13:56 | E5: smoke do Job S (**não evidência**) |
| 13:56 | E6: `set-upgrade-authority --final` |
| 13:56–14:02 | E7: Jobs C, A e B; negativos; liquidações; espera do prazo de C; C7 |
| 14:02–14:03 | estado final |

## Preflight e checagem da tarefa anterior

| Checagem | Resultado real |
| --- | --- |
| Git | `/home/lucas/src/vericode`, `main`, HEAD `561b1b6` (`docs: record R-D4a delta review`) sobre `24364ae`; `status --short` e `--ignored` vazios; `diff --check` 0 |
| Perfil padrão | `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes; `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| `.so` do D4a | `d4/out/d4a-final/vericode_escrow.so`: 395.064 bytes, `cdf6967f…` |
| Receipts | `sha256sum -c ../logs/receipts-out.sha256` → 28/28 OK; os journals de S, A, A′ e B têm os `job_id`s e vereditos esperados, e o digest bate com o journal |
| Locks | raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…` |
| CLIs (homes do Perfil A) | `solana-cli 2.3.9 (src:47647df7; feat:2142755730, client:Agave)`; `spl-token-cli 5.3.0` |
| `devnet_state.py` | deployer `617ogw9T…` com 5.000.000.000 lamports; buyer, executor, `9TE2V…` e `GZqb…` inexistentes. Rent: 82 B → 1.066.800; 165 B → 1.488.440; 284 B → 2.092.960; ProgramData de 395.109 B → 2.007.803.960 |
| `solana feature status` (devnet) | SBPFv1 e SBPFv2 ativos; "Disables execution of SBPFv0 programs" inativo |

## E0 — ferramentas fora do clone

Raiz `~/.local/share/vericode-spikes/d4b/` (`0700`; logs `0600`):
- `bin/env.sh` (`6fdabb91…`): cópia do helper do D4 com `D=d4b`; `sol()`
  roda a CLI do Agave 2.3.9 com `HOME` isolada;
- `bin/cli.sh` (`6619930e…`): grava a saída das CLIs só em log, mostra linhas
  filtradas e para se aparecer texto de mnemônico.

**Cliente Rust** `client/tests-local`:
- cópia de `anchor/tests-local` com caminhos absolutos e o mesmo lock
  `be94760a…`;
- `examples/d4b.rs` (`9feb47fc…`) inclui `tests/common/mod.rs` e usa os
  builders da suíte (`create_job_ix`, `fund_ix`, `deliver_ix`, `release_ix`,
  `refund_on_fail_ix`, `refund_ix`, `negate_g1`, `ata_address`, `job_pda`,
  `vault_pda`);
- só monta e assina offline e imprime pagador, assinatura, tamanho e tx em
  base64; nunca imprime chave;
- build `cargo +1.89.0 build --locked --offline --example d4b`: exit 0,
  2 min 17 s, 1,1 GB de RSS, sem warnings do exemplo.

**Driver RPC** `bin/devnet.py` (`aed55649…`, só stdlib Python):
- sequência `getLatestBlockhash` → `simulateTransaction` (`sigVerify`) →
  `sendTransaction` → `getSignatureStatuses` → `getTransaction`;
- grava tudo em `logs/tx.jsonl`;
- nos negativos, exige:
  - a linha `Program <programa> failed: custom program error: 0x…` na
    simulação e na tx enviada;
  - `InstructionError(0, Custom(código))`;
  - Job, vault e ATAs iguais antes e depois (`getMultipleAccounts` com
    `minContextSlot` = slot da tx).

**Endereços derivados:**

| Conta | Endereço |
| --- | --- |
| ATA do buyer | `61tkoEv4YSYgXRyuxqmF3K5vPDNZgyPMCDG9Xg75FZuw` |
| ATA do executor | `HpZkHZ59uB2hfxPGuPXWd2sWJMbm6MNyb8PwJK1PPJpG` |
| Job S / vault | `2DAGGWJhDWh5EVpSVpRLRJMm5nebLd3zAti8xc6G2Xb2` / `BeRn6VTq7xfzk7CVTKd1nZC8yqqkDc9mB9VsG8sVzvDy` |
| Job A / vault | `7G8qwpivJcGn2KV6ffeJWA9Sz3uHURdBcSe3QnmACUDX` / `Bgj3b573AdxbPBXgeXK7hJjsUW4ohGbERT7qXkfkRb6m` |
| Job B / vault | `EtzHRTqJyh2Cy8j7gwHcsbaG5EzV4sJ1jLVj9jcUUnwC` / `HjLUAoHTdhxWRDWZChyAwfyf847ZhTzFLoexvUtXB4NN` |
| Job C / vault | `FUA8FGFjdsLnDEto9VKFJbLtc3g3Tgcph1yHwWD6HBNu` / `8d13nGHaxEtwHuSTAx28otMVRFKGWuct8E1tRdaHQ26G` |

**`job_id`s:**
- S, A e B: os do D4a (`fe6d25fe…`, `3f0dd1c8…`, `5a25ae48…`);
- C: `96598a41d8c251e71e346054ff6f8d033dfa36c9936db71b5c7feb3fa5dee6fd`,
  32 bytes de `/dev/urandom`, ≠ `0x11…` e distinto dos outros.

**Keypair do buffer (CD4):**
- arquivo `d4/keys/escrow-buffer.json` (`0600`), criado com `solana-keygen
  new --silent --no-bip39-passphrase`, stdout descartado e stderr vazio;
- pubkey `3CVLyXpMTozH2eiUDfkd8h56qzV5Wcd65HfAzgDpou1j`.

Nenhum outro keypair foi criado. Os keypairs de deployer, buyer, executor e
mint (`d4/keys`) e o do programa (`d2c/keys`) só foram usados para assinar.
Nenhum conteúdo foi exibido.

**Dry-build offline** de todas as transações planejadas: release e
`refund_on_fail` com 838 B; deliver+release com 883 B; create+fund com 577 B;
timeout com 342 B; C7 com 238 B. Todas cabem no limite de 1.232 B.

## E1 — precheck imediatamente antes das escritas (CD2)

`devnet.py precheck` às 13:36:29 (slot confirmado 507.793.873). Estavam
inexistentes e sem assinaturas:
- `GZqb…` e `9TE2V…`;
- as duas ATAs;
- os Jobs e vaults de S, A, B e C.

Saldos: deployer 5 SOL; buyer e executor 0. O mint não estava
pré-financiado, então o fallback transfer + allocate + assign não foi
necessário.

## E2 — mint Test USDC (CD3)

```text
sol spl-token -u https://api.devnet.solana.com --fee-payer d4/keys/deployer.json \
  -p TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA create-token --decimals 6 \
  --mint-authority 617ogw9Tbem67ZwWDokEAkLV6JMq5avreijFMPrG75nd d4/keys/test-usdc-mint.json
→ Creating token 9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  Decimals:  6
  Signature: 2uQf8ZuDkHe3qA8grLJv3XBcC2otbMFgHH1jvbXN8WwCNb28ij98k1pgC1bpkAZC95idbZtGSWfy6EkPqF7CVy4i
devnet.py mint (getAccountInfo)
→ owner=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA data_len=82 lamports=1066800
  mint_authority=617ogw9T… supply=0 decimals=6 is_initialized=1 freeze_authority=None
```

Sem `--enable-freeze`. A freeze authority ficou `None`, e o C7 (E7) mostra
que ela não pode ser adicionada.

## E3 — deploy (CD1, CD4)

**Hash antes de cada tentativa:** `sha256sum` do `.so` =
`cdf6967f…`, 395.064 bytes.

**Tentativa 1** (13:37:14–13:43:01), o comando do plano:

```text
sol solana -u https://api.devnet.solana.com -k d4/keys/deployer.json program deploy --use-rpc \
  --program-id d2c/keys/vericode_escrow-keypair.json --buffer d4/keys/escrow-buffer.json \
  --upgrade-authority d4/keys/deployer.json d4/out/d4a-final/vericode_escrow.so
→ Error: Data writes to account failed: Custom error: Max retries exceeded   (exit 1)
```

- O buffer foi criado (assinatura `5fb4TRa6…`, slot 507.794.066, rent de
  2.007.803.960), mas só 32 escritas aterrissaram nas 5 rodadas de
  assinatura da CLI.
- Durante o deploy, uma leitura RPC nossa recebeu HTTP 429. O RPC público
  de devnet limita o envio em rajada da CLI com `--use-rpc`.
- Como o buffer era um keypair em arquivo, a CLI **não** imprimiu
  mnemônico: 0 ocorrências no log (RD4A-01).
- `--use-rpc` mantém todo o tráfego em `api.devnet.solana.com`. Por isso
  não se usou o cliente TPU, que fala com os validadores.

**Retomada cadenciada do buffer** (13:45:30–13:54:29), desvio de método
registrado:
- Repetir o mesmo comando exigiria cerca de 10 tentativas.
- `bin/buffer_fill.py` (`6e87bafd…`) envia, a cerca de 3 tx/s, escritas
  `Write { offset, bytes }` do loader v3 (chunks de 1.012 B, tx de
  1.232 B), assinadas pelo deployer, que é a authority do buffer.
- As escritas são montadas por `d4b buffer-write`, que lê os bytes só do
  `.so` e confere o hash `cdf6967f…` antes.
- Só os chunks diferentes do `.so` foram escritos, e o buffer foi relido a
  cada rodada:

| Rodada | Chunks diferentes | Aterrissados | Erros |
| --- | ---: | ---: | ---: |
| 1 | 361 | 183 | 0 |
| 2 | 178 | 94 | 0 |
| 3 | 84 | 42 | 0 |
| 4 | 42 | 42 | 0 |
| 5 | **0** | buffer = `.so` | — |

  Foram 665 envios, 361 aterrissados, 0 erros e 0 HTTP 429 nesse ritmo. As
  escritas não aterrissadas foram reenviadas na rodada seguinte.

**Tentativa 2** (13:54:45–13:54:48), **o mesmo comando do plano**. A CLI
compara cada chunk do buffer com o `.so` e reescreveria os diferentes antes
de implantar:

```text
→ Program Id: GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH
  Signature: 3qtNbuNpfsWdmiChtM8Q3XDXnDWLiim5V2jALti6aoSdteJ8hwSJYoCpT6NWwAw4ATNMitGHnAH5Tb6Ls9cfr2Jb   (exit 0)
```

A transação `3qtNbuNp…` (slot 507.798.457) contém `CreateAccount` do
programa e `DeployWithMaxDataLen`, com o log `Deployed program GZqbL2Tb…`.
O histórico do buffer tem 395 assinaturas: criação, 32 + 361 escritas e o
deploy.

**`program show` e `program dump` (CD1), antes de qualquer outro passo:**

```text
sol solana -u … -k d4/keys/deployer.json program show GZqbL2Tb…
→ Program Id: GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH
  Owner: BPFLoaderUpgradeab1e11111111111111111111111
  ProgramData Address: B7s9JJVyD2j8PNgKgjhhB36iSdX9cUpSfZHbbd8nLmbc
  Authority: 617ogw9Tbem67ZwWDokEAkLV6JMq5avreijFMPrG75nd
  Last Deployed In Slot: 507798457
  Data Length: 395064 (0x60738) bytes
  Balance: 2.00780396 SOL
sol solana -u … program dump GZqbL2Tb… d4b/out/escrow.devnet.so
→ 395064 bytes; SHA-256 cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133
```

- O dump é **idêntico** ao `.so`, sem zeros finais (`--max-len` padrão).
- A primeira execução de `program show` sem `-k` falhou com `No default
  signer found`; a CLI exige um signer padrão até para leitura. O `-k` só
  satisfaz a CLI e nada é assinado.

## E4 — SOL, ATAs e estoque

| Ação | Assinatura |
| --- | --- |
| 0,05 SOL deployer → buyer | `2SWxnGv8MS31VuLqD2RSEZfWn8LXaz234Sz7B37VfFPMtGR5QBnrjvBha8GbSxM5mC8HgXQTSGSmAA3wMZ9aU1Hc` |
| 0,01 SOL deployer → executor | `3fC25jQ2x4146fCSpWcRGGGviT6ZEgpcPMpWxDne4c1kbV7n8mDSP51e23K8cbmnz4zcDJbtVjmULWZXt244X1Vo` |
| ATA do buyer `61tkoEv4…` (`spl-token create-account --owner`) | `UwifuF37aPJsmmTC55Dzock2H3vWokdu4ALhL36NhJDrSKwTDdokn9fPYDxeeXiYQGdPTp6c7yHnsCsTNnxjq9H` |
| ATA do executor `HpZkHZ59…` | `3zvWV3iasqMJagSf15VbKsUNHyQtZSeUj14FaN5aYf1MFj8DJCGhwR1j5hGvoE5jBhtBCjQ9HdoLrHxEvUhF9LJZ` |
| `mint` de 1.000.000 Test USDC na ATA do buyer | `4Q62yw95Dc2pPWi9RGmfF45bxdxL9psJ94rH4HNf97P8JPjiaJPL6cXv1GpuxkWkq9Gzk814uuZHgPSKZ8XTbz3c` |

Estado: ATA do buyer com 1.000.000.000.000 unidades; supply igual; buyer com
0,05 SOL e executor com 0,01 SOL.

## E5 — smoke do Job S (**não evidência**)

O smoke serviu só para decidir a finalização, como mandam D4-2 e CD5.
- `create_job`+`fund` atômicos: prazo = slot 507.798.691 + 3.000.
  Assinatura `5Sw4v2MC…`, 48.150 CU.
- `deliver`+`release` com a receipt S: assinatura `5CrchWBe…`, 122.156 CU,
  dos quais 99.541 do verificador por CPI.
- Estado final: `Released { d5aa9223… }`, vault 0, ATA do executor
  +1.000.000.

Os links estão na tabela do E7, rotulados `S-…`.

## E6 — finalização

```text
sol solana -u … -k d4/keys/deployer.json program set-upgrade-authority GZqbL2Tb… --final \
  --upgrade-authority d4/keys/deployer.json
→ Account Type: Program
  Authority: none
sol solana -u … -k … program show GZqbL2Tb…
→ Authority: none   (ProgramData B7s9JJVy…, Data Length 395064, Last Deployed In Slot 507798457)
```

- A transação `4AsofYxry7vjdpo2CQxSKSg9MdczuCB4GnRCLzz6ZeVFffPkWdedFBuBQDw58audRHJLcAQ1sGrN5G3t2CoW7eYH`
  (slot 507.798.793) é um `SetAuthority` do loader (tag 4) com só
  `[ProgramData, authority atual]`, ou seja, sem nova autoridade.
- O ProgramData tem só duas assinaturas: o deploy e essa finalização.
- Link: <https://explorer.solana.com/tx/4AsofYxry7vjdpo2CQxSKSg9MdczuCB4GnRCLzz6ZeVFffPkWdedFBuBQDw58audRHJLcAQ1sGrN5G3t2CoW7eYH?cluster=devnet>

## E7 — evidência (CD6), depois da finalização

Slot de referência: 507.798.833. Prazos:
- Job C: 507.800.393 (+1.560: a janela mínima de 1.500 mais 60 de margem
  para a aterrissagem);
- Jobs A e B: 507.807.833 (+9.000).

**Pagadores:**
- buyer: `create+fund` e os refunds;
- executor: `deliver` e `release`;
- deployer: os negativos e o C7, como terceiro, porque a liquidação é
  permissionless.

**Envio:**
- Toda transação foi simulada antes do envio (`sigVerify`).
- Os negativos foram enviados com `skipPreflight` e **aterrissaram** na
  rede como transações com erro.
- O estado de Job, vault, ATA do buyer e ATA do executor (no C7, o mint) foi
  lido antes e depois, com o mesmo resultado em todos.
- Sem compute budget nem priority fee.

| Rótulo | Esperado | Resultado na tx enviada | Verificador invocado | CU | Tamanho | Fee (lamports) | Estado inalterado | Slot | Assinatura |
| --- | --- | --- | --- | ---: | ---: | ---: | --- | ---: | --- |
| S-create-fund (smoke) | `ok` | sucesso | não | 48.150 | 577 B | 5.000 | — | 507.798.700 | [`5Sw4v2MC…`](https://explorer.solana.com/tx/5Sw4v2MCSubkuF98o5qARK7J1kT5K9NtZjwQwaFWq1bS1zuQW4dJq3KcvFpbiYHZQ7QHh7GyjbZ9hfEbTHfBuh9T?cluster=devnet) |
| S-deliver-release (smoke) | `ok` | sucesso | sim | 122.156 | 883 B | 5.000 | — | 507.798.723 | [`5CrchWBe…`](https://explorer.solana.com/tx/5CrchWBe9qVxDUozjyfvu9Ffzn8GfhdVNk1xCF6gLUzQ5h1e8xUx9QesTymjN1jYTWcFgwPQHGjsvBL4CiGuatZU?cluster=devnet) |
| C-create-fund | `ok` | sucesso | não | 43.650 | 577 B | 5.000 | — | 507.798.843 | [`5cEFDTR9…`](https://explorer.solana.com/tx/5cEFDTR9r9pPBFW6o2vSuwWZiDuEGweu2dLH9NcvgntqSLVFmzTuQ1XnfrgozPENcWc6YMRDWjuFgrAJvhykUvAi?cluster=devnet) |
| A-create-fund | `ok` | sucesso | não | 43.650 | 577 B | 5.000 | — | 507.798.866 | [`2DwGw99A…`](https://explorer.solana.com/tx/2DwGw99AAvJAzvjSxB4wAZa8vWkxTVPSGWzWWnm6dbvrMFr5DYRU192YHemroaiMDLsKNp1suTKfwpLAgoaBBXzJ?cluster=devnet) |
| B-create-fund | `ok` | sucesso | não | 43.650 | 577 B | 5.000 | — | 507.798.899 | [`TDFzsjcF…`](https://explorer.solana.com/tx/TDFzsjcFeDiSZtNhL8DtTXof8ZNE9tNgXgL8hDFtW3t67B98rkMwXWEPusMjciMtTwFJtqjgTEyygnLsmpRqBQE?cluster=devnet) |
| A-deliver `(7,14)` | `ok` | sucesso | não | 4.947 | 243 B | 5.000 | — | 507.798.920 | [`g5GBo7HE…`](https://explorer.solana.com/tx/g5GBo7HEuk9fUGMWwtNeWrxmz9jRRmvC4KcekantmuLgRCDVi9VzEvhXm3Rj4a3eDYdiqkvcfmCMaXYE7QJR56d?cluster=devnet) |
| A: `refund_on_fail` com A′ (FAIL de `(7,15)`, não entregue) | escrow 6017 | `Custom(6017)` `JournalArtifactHashMismatch` | não | 10.027 | 838 B | 5.000 | sim | 507.798.967 | [`4QimpRfa…`](https://explorer.solana.com/tx/4QimpRfatTMAiBeA2DV6pgnWMFLMJMY5MVertCucKQJtwCWVGaGiJcMCp5hBGzCFHdVbs7nzSQNnAhDitdEv8QSN?cluster=devnet) |
| A: `release` com journal+seal do Job S | escrow 6014 | `Custom(6014)` `JournalJobIdMismatch` | não | 10.105 | 838 B | 5.000 | sim | 507.798.989 | [`3mxKVPiH…`](https://explorer.solana.com/tx/3mxKVPiHq5FeyxeWwbY4g6aGP6fjcYE2W1VbnRKXvvhrPEQPthAfmRHiGivhim1s1BkV7N3twhiccgLwGjAhrN6o?cluster=devnet) |
| A: `release` com journal A + seal S | verificador 6000 | `Custom(6000)` `VerificationError` em `THq1q…` | sim (101.157 CU) | 114.920 | 838 B | 5.000 | sim | 507.799.012 | [`2i8GGo2S…`](https://explorer.solana.com/tx/2i8GGo2SxKFMgGPGDW6Ni3sq9cMpMPv6vdkHG89VkxyiHDi386sTDxNmUUfeURp2Vd5WmoBhmi6aRtb31APJuXfC?cluster=devnet) |
| A: `release` com seal adulterado (`pi_c[10] ^= 1`) | verificador 6003 | `Custom(6003)` `PairingError` em `THq1q…` | sim (100.528 CU) | 114.291 | 838 B | 5.000 | sim | 507.799.046 | [`5UwKqSJs…`](https://explorer.solana.com/tx/5UwKqSJsku7rvYCXWz96BYNDNo3DjA8Yn7YXC4eMj1SYMhiUUvDLBvTLwWQEeabffLe8GtVFLC6x8Z1KH1PvCsUU?cluster=devnet) |
| A: `release` com selector `00000000` | escrow 6033 | `Custom(6033)` `UnexpectedSelector` | não | 14.004 | 838 B | 5.000 | sim | 507.799.068 | [`2W8CCHq3…`](https://explorer.solana.com/tx/2W8CCHq3CKzoxQ9nXvahqGUoRoPpVFzP5jKjMdUSNFbFoixTisVstbdByTcTwRUZ1LATQQmV4iaNx4jnweT9GNbG?cluster=devnet) |
| **A-release (PASS)** | `ok` | sucesso: `Released { d5aa9223… }` | **sim (99.541 CU)** | 117.209 | 838 B | 5.000 | — | 507.799.117 | [`4yWq28Gw…`](https://explorer.solana.com/tx/4yWq28GwkT9uLbG8haXbQYQyMqNWd6Qc29hWrez9cT4tGu36mS1fY8w6ocu75d5JxfQSLzTKKbMPc131fbL7chhR?cluster=devnet) |
| B-deliver `(7,15)` | `ok` | sucesso | não | 4.947 | 243 B | 5.000 | — | 507.799.139 | [`653cAiZ1…`](https://explorer.solana.com/tx/653cAiZ1y1VdMgR5wMr1i2xbtMSXT9bAkdyqV1hfJ9GRnreFKsL6ECGjDFYds46w1dsyTrnLQyAPPbMtb4NV3Yjx?cluster=devnet) |
| B: `release` com a receipt FAIL de B | escrow 6019 | `Custom(6019)` `VerdictNotPass` | não | 10.206 | 838 B | 5.000 | sim | 507.799.183 | [`4ahaVPg9…`](https://explorer.solana.com/tx/4ahaVPg9HDce1qm6JNzNKmEyYM2UrNMtVXSoJMBeLGq8Fo4jJ5xUmZzus4pX85aD3dGcSHqneCLsTRRtwJ5vnihM?cluster=devnet) |
| B: `refund_on_fail` com A′ (FAIL do Job A) | escrow 6014 | `Custom(6014)` `JournalJobIdMismatch` | não | 9.929 | 838 B | 5.000 | sim | 507.799.203 | [`5WpJriku…`](https://explorer.solana.com/tx/5WpJrikuWzrfPdo1BnBuu3z9mCiu4bfKjdZ6X3ZN3dg8U7YdWxxZzZGCPKdyWFP9y2fmm8uNoGpAQfFpCSce4iX9?cluster=devnet) |
| B: `refund_on_timeout` antes do prazo | escrow 6021 | `Custom(6021)` `DeadlineNotReached` | não | 9.022 | 342 B | 5.000 | sim | 507.799.225 | [`4Pk5hL44…`](https://explorer.solana.com/tx/4Pk5hL44fD9WhoU8KDgtFaeM1AFB5V78wHoyUNAcT2z1aoptBvnUDmwigUdxcR7txzwJ8WSbVbi1coP5QDg7LPju?cluster=devnet) |
| **B-refund_on_fail (FAIL)** | `ok` | sucesso: `RefundedOnFail { 343ad778… }` | **sim (99.541 CU)** | 117.038 | 838 B | 5.000 | — | 507.799.247 | [`2osG9m8J…`](https://explorer.solana.com/tx/2osG9m8JcE1CpAKt8JribtJCw8ffrdhBh6hYm5xKeLPptM9YLsugouMRH2KnmRXAymwAMvBABUmuZY6tkwHoVbP4?cluster=devnet) |
| **C-refund_on_timeout** (slot 507.800.412 > prazo 507.800.393) | `ok` | sucesso: `RefundedOnTimeout` | não | 14.605 | 342 B | 5.000 | — | 507.800.412 | [`5sHg855Q…`](https://explorer.solana.com/tx/5sHg855QNbKoXqnveLG6aUqvLcvtVkkUThN7hcKcuktNjMwh5EJ5yYJEy8QvgZjFfFFi144yKz4VYnAcBP8qdSTV?cluster=devnet) |
| C7: `SetAuthority(FreezeAccount)` no mint admitido, assinado pela mint authority | Token 16 | `Custom(16)` `MintCannotFreeze` ("This token mint cannot freeze accounts") | não | 228 | 238 B | 5.000 | sim (mint) | 507.800.470 | [`4obmfJ8b…`](https://explorer.solana.com/tx/4obmfJ8bH7VU6FuKnNbt364GmGAQ4yjXuNoGPAJpgCbAMqaXd5pnwFJW81CKCFAnJ5L5b7aTr4QXdKhMXrGMRqCr?cluster=devnet) |

Assinaturas completas das três liquidações de evidência:
- A-release:
  `4yWq28GwkT9uLbG8haXbQYQyMqNWd6Qc29hWrez9cT4tGu36mS1fY8w6ocu75d5JxfQSLzTKKbMPc131fbL7chhR`;
- B-refund_on_fail:
  `2osG9m8JcE1CpAKt8JribtJCw8ffrdhBh6hYm5xKeLPptM9YLsugouMRH2KnmRXAymwAMvBABUmuZY6tkwHoVbP4`;
- C-refund_on_timeout:
  `5sHg855QNbKoXqnveLG6aUqvLcvtVkkUThN7hcKcuktNjMwh5EJ5yYJEy8QvgZjFfFFi144yKz4VYnAcBP8qdSTV`.

Os logs completos de simulação e de cada tx enviada estão em
`d4b/logs/tx.jsonl`, fora do clone.

**CU por programa, dos logs.** Escrow/release:
- `release` (A): o escrow consome 117.209 no total, dos quais 99.541 do
  verificador e 105 do Token;
- `refund_on_fail` (B): 117.038;
- `deliver`: 4.947;
- `create_job`: 32.159 a 36.659; `fund`: 11.491.

O D4a media em processo 121.885 e 123.214–126.214. A diferença vem da busca
do bump da ATA, que depende das chaves.

**Tamanhos:** iguais ao dry-build e ao PoC-5 do R-D2e (`release` 838 B).

**Estado final** (`devnet.py state`, slot 507.800.504):

| Job | Estado | `deadline_slot` | Vault |
| --- | --- | ---: | ---: |
| S | `Released { d5aa9223… }` | 507.801.691 | 0 |
| A | `Released { d5aa9223… }` | 507.807.833 | 0 |
| B | `RefundedOnFail { 343ad778… }` | 507.807.833 | 0 |
| C | `RefundedOnTimeout` | 507.800.393 | 0 |

- **Vaults:** os quatro têm owner Tokenkeg, mint `9TE2V…`, **authority =
  PDA do Job**, sem delegate nem close authority, e saldo 0.
- **ATAs:** executor 2.000.000 (S e A); buyer 999.998.000.000, isto é, o
  estoque menos S e A. Os refunds de B e C voltaram ao buyer.
- **Mint:** supply 10¹², decimals 6, freeze `None`.

**SOL:** os saldos fecham por lamport.

| Conta | Início | Fim | Gasto | Composição |
| --- | ---: | ---: | ---: | --- |
| deployer | 5.000.000.000 | 2.925.249.240 | 2.074.750.760 | ProgramData 2.007.803.960 (buffer drenado no deploy); conta do programa 833.120; mint 1.066.800; 2 ATAs 2.976.880; 0,06 SOL transferido; taxas 2.070.000 (393 escritas do buffer, das quais 32 da CLI e 361 cadenciadas, mais mint, buffer, deploy, transferências, ATAs, `mint_to`, finalização, 8 negativos e C7) |
| buyer | 50.000.000 | 35.644.400 | 14.355.600 | 4 × (Job 2.092.960 + vault 1.488.440) de rent, preso (F-13), e 6 taxas |
| executor | 10.000.000 | 9.980.000 | 20.000 | 4 taxas |

## Invariantes do guia §7 em devnet

| Invariante | Evidência em devnet |
| --- | --- |
| 1. vault controlado por PDA | os 4 vaults têm authority = PDA do Job; sem delegate nem close authority |
| 2. termos do Job | mint admitido `9TE2V…` (6 decimais, sem freeze); termos da v1 aceitos na criação |
| 3. `Pass` paga só o executor | A-release: +1.000.000 na ATA canônica do executor |
| 4. `Fail` devolve só ao buyer | B-refund_on_fail: +1.000.000 na ATA do buyer |
| 5. timeout só depois do prazo | B antes do prazo → 6021; C depois do prazo → `RefundedOnTimeout` |
| 6. estado e transferência atômicos | 8 negativos e C7 aterrissaram com erro e estado igual antes e depois |
| 7. sem admin nem destino livre | **upgrade authority `none`**; verificador imutável; destino = ATA canônica |
| 8. journal de outro Job ou artefato | 6014 (Job S no A; A′ no B) e 6017 (A′ no A), sem CPI |
| 9. terminal impede dupla liquidação | não exercitado em devnet; testado em processo (D2e/D4a) |
| 10. falha de verificação reverte | 6000 e 6003 dentro do verificador revertem tudo; estado igual |

## Desvios do plano aprovado

1. **Deploy em duas tentativas, com retomada cadenciada do buffer.**
   - A tentativa 1 da CLI falhou com 32 escritas; o RPC público limitou a
     rajada.
   - Em vez de repetir o comando cerca de 10 vezes, o buffer foi completado
     por `buffer_fill.py`, com bytes só do `.so` conferido.
   - A tentativa 2 foi o comando exato do plano.
   - O resultado foi conferido pelo dump: `cdf6967f…`, byte a byte. O plano
     previa até 3 tentativas da CLI; foram 2.
2. **`program show` com `-k deployer`:** a CLI 2.3.9 exige um signer padrão
   até para leitura. Nada é assinado.

Nenhum outro desvio. Os prazos, pagadores, códigos e a ordem CD5 foram os do
plano.

## Fronteiras

- **Rede:**
  - só `https://api.devnet.solana.com` (JSON-RPC);
  - o deploy usou `--use-rpc`, sem TPU;
  - sem airdrop nem crates.io: o cliente compilou `--offline` com o lock
    `be94760a…`;
  - sem Docker nem prova nova;
  - links do Explorer gerados, não acessados.
- **Escritas em devnet:** só as autorizadas: mint, ATAs, `mint_to`,
  transferências do deployer, buffer e deploy, `--final`, instruções do
  escrow e o C7.
- **Segredos:**
  - nenhum seed phrase, keypair ou chave exibido ou versionado;
  - a busca por "seed phrase" e "solana-keygen recover" nos logs de
    `d4b/logs` só encontrou o texto de ajuda das flags
    (`--skip-seed-phrase-validation`, "ASK keyword") nos dois `--help`
    capturados. Nenhum mnemônico;
  - keypair novo: só o do buffer, `0600`, em `d4/keys`.
- **Repositório:**
  - só documentação;
  - programa, core, `zkvm/`, testes, `.env.example`, locks e `JournalV1`
    inalterados.

## Riscos abertos

- **Sem e-stop** (RD4A-06): o escrow é imutável e o verificador também. Um
  bug de soundness exigiria um novo program ID.
- **Rent preso (F-13):** cerca de 0,0036 SOL por Job; não há `close`.
- **Mint authority = deployer:** pode emitir mais Test USDC; isso não afeta
  a custódia.
- **`job_id`s públicos (F-09, RD4A-03):** S, A, B e C já estão consumidos.
  O D7 deve gerar `job_id` novo por Job.
- **RPC público com limite de taxa:** um cliente que envie em rajada perde
  transações. A CLI do D7 deve cadenciar e reenviar.
- **R-02** (`fund` fora da janela) mitigado só por `create+fund` atômicos;
  **R-03** mitigado por `job_id` aleatório.
- ImageID não recertificado; spec v1 trivial.
- **RD4A-07 (a), (e) e (f)** ficam para o D7.

## Artefatos fora do clone (não versionados, persistentes)

`~/.local/share/vericode-spikes/d4b/`:
- `bin/` (`env.sh`, `cli.sh`, `devnet.py`, `buffer_fill.py`,
  `report_table.py`);
- `client/tests-local` (cliente com o lock `be94760a…`);
- `out/escrow.devnet.so` (dump `cdf6967f…`);
- `jobs/job_ids.txt`;
- `logs/`: `timeline.log`, `tx.jsonl`, `e3-buffer-writes.jsonl`, logs das
  CLIs, `devnet-state-{pre,post}.log`.

O keypair do buffer fica em `d4/keys/escrow-buffer.json`; o buffer foi
consumido pelo deploy.

## Próximo gate

`D7`, conforme [`docs/handoffs/d4b-to-d7.md`](handoffs/d4b-to-d7.md):
- CLI de ponta a ponta no repositório, a partir da IDL do D4a;
- README com versões, hashes, links e limitações;
- roteiro da demo;
- RD4A-07 (a), (e) e (f).
