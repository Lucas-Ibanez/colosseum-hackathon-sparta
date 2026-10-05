# D4a — Router upstream reprovado em devnet; CPI direta ao verificador Groth16 imutável e mint admitido

Data: 2026-10-05 · Executor: Claude Code (Opus 5.5) · Commit do código:
`fb4bfba` · Gate anterior: R-D2e (`2d76441`), com o prompt ajustado em
`cfdd9d7` (`docs/handoffs/r-d2e-to-d4.md`).

## Resultado

**D4a CONCLUÍDO (local); parada para a revisão R-D4a antes de qualquer
escrita em devnet.**

- O reconhecimento somente leitura do devnet (D4-1, caminho (a)) **reprovou**
  o Verifier Router upstream:
  - ele está implantado e imutável, mas não inicializado;
  - o verificador Groth16 upstream, também imutável, nunca poderá ser
    registrado nele.
- **Decisão humana nesta sessão: caminho (b′).** O escrow chama por CPI o
  verificador Groth16 `THq1qFYQ…` direto, sem Router. Junto vieram:
  - o mint Test USDC admitido (erro 6036);
  - o congelamento do `JournalV1` v1, só em documentação;
  - os PoCs 1 a 7 do R-D2e como testes de regressão;
  - receipts Groth16 novas para os Jobs S, A, A′ e B.
- Claim máximo: **"verificado por CPI ao verificador Groth16 de
  `risc0-solana v3.0.0` em `solana-program-test` local, inclusive com os
  bytes do verificador implantado em devnet"**. Deploy do escrow, Test USDC
  e transações em devnet: `STATUS: NÃO VALIDADO` (D4b). Não é "ZK on-chain"
  em cluster.

Números principais:
- `.so` do D4a: 395.064 bytes, `cdf6967f…`; IDL `e8ce2c20…` (6
  instruções, 37 erros).
- Suíte local: 57/57 com o verificador do rebuild (`dab6746d…`) e 57/57
  com os bytes dumpados de devnet (`34ae6e5c…`): escrow 26, settlement 15,
  regressions 7, layout 7, fixtures 2.
- CU: `release` 121.885 e `refund_on_fail` 123.214–126.214, dos quais
  99.541 do verificador.
- Quatro receipts Groth16 novas (S, A, A′ e B), com `job_id` aleatório,
  verificadas localmente e aceitas pelo verificador de devnet em simulação.
- Core, `zkvm/`, guest, `JournalV1`, `Cargo.toml` e locks inalterados.
- Nenhuma escrita em devnet.

## Linha do tempo (-03:00)

| Hora | Fase |
| --- | --- |
| 07:26 | F0 somente leitura (Git, perfil, locks, artefatos, imagem) |
| 07:27–07:40 | F1 somente leitura em devnet; pergunta ao humano; plano aprovado |
| 07:44 | E0: raiz `d4`, helpers e shim do Docker |
| 07:47–07:58 | rechecagem: core A/B; build SBF de baseline; suíte de baseline e build do harness **em paralelo → exit 137** |
| 08:01 | build do harness repetido sozinho; dump do verificador; log do F1; keypairs (E1) |
| 08:01–08:27 | edição do programa e dos testes, sem compilar; build do harness (26 min 40 s) |
| 08:08 | `job_id`s de S, A e B |
| 08:28–08:29 | build SBF do D4a (target limpo, 1 min 11 s) |
| 08:29–08:30 | suíte de baseline (HEAD) e IDL de baseline |
| 08:30–08:33 | suíte D4a com o verificador local e com os bytes de devnet; checagem de vermelho; IDL D4a |
| 08:33–08:39 | provas S, A, A′ e B, uma de cada vez |
| 08:40 | simulação dos seals novos em devnet |

## F0 — preflight e checagem do R-D2e

| Checagem | Resultado real |
| --- | --- |
| Git | `/home/lucas/src/vericode`, `main`, HEAD `cfdd9d7` sobre `2d76441`; árvore limpa; `git diff --check` 0 |
| Perfil padrão | `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes; `find . -printf '%p %s %T@\n' \| sort \| sha256sum`: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| Locks | raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…` |
| Artefatos D2d (`sha256sum -c`) | 6/6 OK: ELF `63fac491…`, `vericode-guest.bin` `e09ba8cf…`, `recursion_zkr.zip` `744b999f…` e as três receipts `Composite`; vetores 17/17 OK |
| Imagem do prover | `risczero/risc0-groth16-prover@sha256:7f173963…e331` presente (5.209.789.112 bytes), tag local `v2025-04-03.1`; nenhum pull |
| Core lane-a `1.85.0` / lane-b `1.89.0` `test --locked --offline` | 42 passed cada, 0 warnings |
| `sbf_build baseline` (target limpo) | exit 0, 4 min 12 s; `vericode_escrow.so` 398.504 bytes, `6457aecf471e6d2cb38796fd7dc4442572001b3025fdf8330cde29b93ca9ca96` (igual) |
| Router e verificador do D2e | `1b26b017…` e `dab6746d…`, copiados para o out-dir de baseline |

| `anchor/tests-local` de baseline (`git archive HEAD anchor crates` fora do clone, lock `be94760a…`, `.so` `6457aecf…`) `--no-fail-fast` | exit 0; escrow 25, settlement 16, layout 6, fixtures 2 |
| `anchor-0.31.1 idl build` do baseline | exit 0; 21.779 bytes, `37a3028a…` (igual); 6 instruções, 36 erros |

A primeira tentativa da suíte de baseline (07:58) e o build do harness de
receipts, que rodavam **em paralelo**, terminaram com exit 137 (SIGKILL).
- O WSL não reiniciou (uptime contínuo desde 07:23).
- A causa provável é falta de memória: 7,6 GiB para dois builds pesados. O
  `dmesg` não estava acessível para confirmar.
- Os dois foram repetidos **em sequência** e passaram. Desde então, nenhum
  build, teste ou prova pesada roda em paralelo.
- Como a árvore já tinha os testes novos, a suíte de baseline rodou a partir
  de um `git archive HEAD`, sem `stash` nem checkout.

Raiz de trabalho fora do clone: `~/.local/share/vericode-spikes/d4/`, com:
- `bin/env.sh`: cópia do helper do R-D2e com `D=d4`, mais `sol` (CLI do
  Agave 2.3.9 com `HOME` isolada) e `zk` (raia zkVM do D2d com homes
  copiadas, `RISC0_PROVER=local` e `TMPDIR` em `d4`);
- `bin/docker-shim/docker`: em `docker run`, injeta `--pull=never
  --network=none` e troca a tag do prover pelo digest `7f173963…`.

## F1 — reconhecimento somente leitura do Router upstream em devnet

Somente `https://api.devnet.solana.com`, com `getAccountInfo`,
`getSignaturesForAddress`, `getTransaction` e `simulateTransaction`
(`sigVerify=false`, `replaceRecentBlockhash=true`).
- Nenhuma assinatura, transação, airdrop ou custo.
- O pagador simulado é a conta upstream `7uAQqk…` (conta de sistema com
  saldo); nada foi assinado.
- Script: `d4/bin/f1_recon.py`; saída integral em `d4/logs/f1-recon.log`
  (execução das 08:02:12).

### Critério 1 — programas e autoridades: ok

| Programa | Conta | ProgramData | Upgrade authority | Histórico do loader |
| --- | --- | --- | --- | --- |
| Verifier Router | `6JvFfBrvCcWgANKh1Eae9xDq4RC6cfJuBcf71rp2k9Y7`, executável | `HbJ7h1mNJgGqZrT6rhNKoV76tu3jgjEkUDopWdUTJiP1`, 321.248 bytes de ELF, `0179edf3…` | **`None`** | `DeployWithMaxDataLen` no slot 416.492.066 e `SetAuthority` sem nova autoridade no slot 416.492.069 (23/10/2025), pela chave `7uAQqkPQMV8b7yD2sbBVk3RDWjwhLVF9FUcerDNVtSui` |
| Verificador Groth16 | `THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge`, executável | `ENdLkqHpzKcrQy4XzFUhkN7H3C3Mz1cugjJBpiNEDxpn`, 199.256 bytes, `34ae6e5c…` | **`None`** | deploy no slot 416.492.112, `Upgrade` no 416.495.876 e `SetAuthority` final no 416.495.880, pela mesma chave |

### Critério 2 — estado do Router: falhou

| Conta | Endereço | Resultado |
| --- | --- | --- |
| PDA `["router"]` | `4Sh5ofCzLmmCg1zQraXRoPL2oB88bDZJEEbsfjXaauZT` | **não existe** |
| Entrada `["verifier", 73c457ba]` | `4Z7ok78xEh7vYmtHSnzF1Tobtm4sozZfHoQfUBgfG8pi` | **não existe** |

Pelo fonte pinado (`ee415935`, `verifier_router/src/router/mod.rs`):
- `initialize` exige `authority == INITIAL_OWNER`, a chave embutida no build
  upstream; o projeto não a controla;
- `add_verifier` exige `verifier_program_data.upgrade_authority_address ==
  Some(PDA do Router)`. Com a autoridade de `THq1q…` em `None`, esse
  verificador **nunca** poderá ser registrado neste Router, mesmo que o dono
  upstream o inicialize.

### Critério 3 — `Router.verify` por simulação: falhou

Dados: discriminador `85a18d30…` ‖ selector ‖ prova (`pi_a` negado) ‖
ImageID ‖ digest. Contas: router PDA, entrada, verificador, system program.

| Caso | Resultado |
| --- | --- |
| vetor oficial FIB | `Custom(3012)` `AccountNotInitialized` (conta `router`), 3.545 CU |
| fixture PASS `0x11` | idem |
| fixture FAIL `0x11` | idem |
| seal adulterado | idem |
| ImageID errado | idem |

### Dump do verificador (registro; não reprova sozinho)

```text
sol solana -u https://api.devnet.solana.com program dump THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge \
  d4/out/devnet-dump/groth_16_verifier.devnet.so
→ Wrote program …; 199.256 bytes,
  SHA-256 34ae6e5c9d63dfe67c48fa04cad04e9752ad9f1cfbc8b4d66e99941df7666cd1
primeiros 199.248 bytes → 638db2b93d4ab07f15183b520202ef764014966a3a18f4340e345a66ba5630dc
rebuild local (D2d/D2e/R-D2e) → dab6746d4a24f1d263f303ef91103166e97d03c5ae0e37893ed517b831e01e0c
```

Os bytes diferem. A equivalência funcional vem das simulações abaixo e da
suíte local rodada com o dump (seção E3).

### Informativo: o verificador chamado direto, por simulação

Dados: discriminador ‖ prova (`pi_a` negado) ‖ ImageID ‖ digest (328 bytes).
Conta: system program.

| Caso | Resultado |
| --- | --- |
| FIB oficial | **sucesso**, 99.541 CU |
| PASS `0x11` | **sucesso**, 99.541 CU |
| FAIL `0x11` | **sucesso**, 99.541 CU |
| seal adulterado (1 bit em `pi_c`) | `Custom(6003)` `PairingError`, 100.528 CU |
| ImageID errado | `Custom(6000)` `VerificationError`, 101.157 CU |

O consumo de 99.541 CU é igual ao do verificador local dentro do Router no
D2e.

### Conclusão do F1

O caminho (a) está reprovado; não há nenhum caminho com o Router upstream.
A regra D4-1 levava ao caminho (b), Router próprio. O agente apresentou ao
humano a alternativa (b′), verificador direto, com os custos de cada uma,
porque ela não estava coberta pelas decisões registradas. **O humano
escolheu (b′)** e pediu as receipts novas já nesta sessão.

## E1 — keypairs efêmeros de devnet

Criados com `solana-keygen new --silent --no-bip39-passphrase` (Agave
2.3.9), com stdout descartado, em `d4/keys` (`0700`, arquivos `0600`). Só as
pubkeys:

| Papel | Pubkey |
| --- | --- |
| deployer/payer | `617ogw9Tbem67ZwWDokEAkLV6JMq5avreijFMPrG75nd` |
| buyer | `EZgGUg4JhEAhzMPd4jBuKWFXdDNpFATvCjkj7mLAdxU6` |
| executor | `EdB25bVhdj6rm5b7FbVHe5A3zx2hAhPpK4YAzrLaDs6U` |
| mint do Test USDC (`ADMITTED_MINT`) | `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F` |

Nenhum SOL foi pedido; nenhuma conta foi criada em devnet.

## E2 — programa (`anchor/programs/vericode-escrow/src/lib.rs`)

| Mudança | Detalhe |
| --- | --- |
| CPI direta ao verificador | `verify_with_router` → `verify_groth16`. Dados = `VERIFY_DISCRIMINATOR` (`85a18d30…`) ‖ `pi_a` ‖ `pi_b` ‖ `pi_c` ‖ `job.image_id` ‖ `SHA-256(journal)`, 328 bytes. Conta: system program, readonly. `invoke` com o system program e o verificador |
| `SettleWithProof` | 7 contas: job, mint, vault, `recipient_token`, token program, `verifier_program` (`address = GROTH16_VERIFIER_ID`), system program. Saíram `router_program`, `router` e `verifier_entry` |
| Constantes | saíram `VERIFIER_ROUTER_ID`, `ROUTER_PDA` e `GROTH16_VERIFIER_ENTRY`; `ROUTER_VERIFY_DISCRIMINATOR` → `VERIFY_DISCRIMINATOR` (mesmos bytes); nova `ADMITTED_MINT = 9TE2VPFm…` |
| Seal | `RouterSeal` → `Groth16Seal`, com o mesmo layout Borsh (`selector`, `pi_a`, `pi_b`, `pi_c`); selector `73c457ba` ainda exigido antes da CPI (6033) |
| Mint admitido | `CreateJob.mint`: `address = ADMITTED_MINT @ MintNotAdmitted`, depois da constraint de freeze (ordem do `linearize` do anchor-syn 0.31.1: `raw` antes de `address`) |
| Erro novo | 6036 `MintNotAdmitted`, no fim do enum; 6000–6035 inalterados |
| Inalterado | program ID `GZqbL2Tb…`, ordem decode → core → ATA → selector → digest → CPI → transferência → estado, `JobAccount` (276 bytes), tags de `EscrowStatus`, core e `JournalV1` |

## E3 — testes, build e IDL

Arquivos em `anchor/tests-local/tests/`:

| Arquivo | Mudança |
| --- | --- |
| `common/mod.rs` | `start()` injeta no genesis o mint admitido em `ADMITTED_MINT` (6 decimais, sem freeze, autoridade em memória); `verifier_program_test()` (escrow e verificador em `THq1q…`); `settle_accounts` com 7 contas; `groth16_seal`; `E_MINT_NOT_ADMITTED` |
| `escrow.rs` | teste novo `create_job_admits_only_the_test_usdc_mint` (mint sem freeze do próprio buyer → 6036 sem criar contas; mint admitido → aceito); squatter do PoC-9 com o mint admitido; nota de ordem no teste 6024 |
| `settlement.rs` | sem Router no genesis; `reject_before_verifier`; `verifier_program_is_fixed` (SPL Token, system program ou conta qualquer → 2012); o teste de e-stop saiu (não há e-stop); os demais casos ficam |
| `layout.rs` | 6036 literal; `verifier_is_the_pinned_release`; `admitted_mint_is_the_devnet_test_usdc`; layout de `Groth16Seal` |
| `regressions.rs` (novo) | PoCs 1 a 7 do R-D2e como asserções (R-05) |

Regressões contra o R-D2e:

| PoC | Observado no R-D2e | Asserção no D4a |
| --- | --- | --- |
| 1 | troca de owner da ATA → 6010 nos dois lados; com o owner restaurado, paga | igual; executor recebe `AMOUNT` |
| 2 | ATA inexistente → 3012; criada por terceiro → liquida | igual nos dois lados |
| 3 | timeout no slot do prazo → 6021; release e deliver+release aceitos | igual; `Released { h(7,14) }` |
| 4 | fund depois do prazo aceito; deliver → 6022; fund no prazo−1 deixa 2 slots | igual, documentado como **limitação R-02**, não como propriedade |
| 5 | 937/977/1.078/1.118 bytes | **838/878/979/1.019** bytes exatos (−99: 3 chaves e 3 índices a menos); dados do release 437 bytes, 7 contas |
| 6 | 18/18 → 6007/6008/`Custom(0)` | igual, com snapshot inalterado |
| 7 | 3007/2006/3007/3008/2000 | igual |

Comandos e saídas:

```text
sbf_build d4a   (CARGO_TARGET_DIR novo; Perfil A; -- --locked)
→ exit 0, 1 min 11 s; 15 warnings de macro do Anchor (os mesmos do D2e)
→ vericode_escrow.so 395.064 bytes,
  SHA-256 cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133

prog_tests d4a --no-fail-fast   (SBF_OUT_DIR: escrow cdf6967f + verificador dab6746d)
→ exit 0; escrow 26, fixtures 2, layout 7, regressions 7, settlement 15 (57/57)
  create_job 36.302 CU; deliver 4.949 CU; release 121.885 CU; refund_on_fail 126.214 CU

prog_tests d4a-devnet-verifier --no-fail-fast   (escrow cdf6967f + bytes de devnet 34ae6e5c)
→ exit 0; 57/57; release 121.885 CU; refund_on_fail 123.214 CU

Logs das duas execuções (iguais nas duas):
  21 CPIs ao THq1q… em profundidade 2;
  17 × 99.541 CU (sucesso), 2 × 6000 (seal de outro journal), 2 × 6003 (seal adulterado)

checagem de vermelho: suíte nova contra o .so do D2e 6457aecf (com o verificador)
→ exit 101; escrow 25/26 (só create_job_admits_only_the_test_usdc_mint falha),
  regressions 1/7 (só o PoC-4, que não liquida, passa), settlement 0/15;
  layout 7/7 e fixtures 2/2 (testes de host)

anchor-0.31.1 idl build -p vericode_escrow
→ exit 0; 21.334 bytes,
  SHA-256 e8ce2c20d9ffaa09c72328e9eb865956a573c25fa867235fbee404a60ecb6e42 (não versionada)
  6 instruções (create_job, deliver, fund, refund_on_fail, refund_on_timeout, release);
  37 erros (6000–6036); tipos EscrowStatus, Groth16Seal, JobAccount;
  create_job.mint com endereço 9TE2VPFm…; release/refund_on_fail com 7 contas;
  verifier_program sem endereço na IDL (nome com dígitos, R-D2e R-04d)
```

A CU do `refund_on_fail` varia (123.214 ou 126.214) com a busca do bump da
ATA, que depende das chaves aleatórias. O D2e media 135 k a 145 k com o
Router.

## E4 — receipts novas (D4-4)

**Harness** `d4/receipts`, cópia do `d2d/receipts`:
- `prove <job_id_hex> <entrada> <saída> <dir>`:
  - o frame do host é `job_id ‖ artefato 12 B ‖ ImageID`;
  - prova com `default_prover().prove` sobre `d4/artifacts/vericode-guest.bin`
    (o ELF D1c2b preservado, `e09ba8cf…`), sem reconstruir o guest;
  - checagens: ImageID igual a `4da06f90…fb1a`; tipo `Composite`, nunca
    `Fake`; `verify` ok; journal igual, byte a byte, ao de
    `evaluate_restricted_artifact` do core; ImageID errado rejeitado;
  - `job_id` `0x11` recusado.
- `compress <dir>`: como no D2d.
- Hashes:
  - `main.rs` `2c1d3dd5…`, `Cargo.toml` `0b975c94…`;
  - lock `ec0dd8d6…`: o do D2d, ampliado offline só com `vericode-core` e
    seus crates (`borsh 0.10.4`, `proc-macro-crate 0.1.5`, `toml 0.5.11`,
    `ahash 0.7.8`);
  - binário release `18a5d59a…`.
- O `ahash-0.7.8.crate` foi copiado do cache da raia B do Perfil A, com o
  checksum do lock conferido (`891477e0…`), como no precedente D1c2b.3e.
- Build: `--release --locked --offline`, 26 min 40 s, RSS máximo 2,1 GB, 0
  warnings.

**Ambiente:**
- `RISC0_PROVER=local` e `RISC0_EXECUTOR=local`; sem dev mode nem Bonsai
  (feature `disable-dev-mode`);
- `RECURSION_SRC_PATH` aponta para a cópia de `recursion_zkr.zip`
  (`744b999f…`);
- `RISC0_WORK_DIR` e `TMPDIR` dentro de `d4`.
- O prover Docker rodou pelo shim, um cenário por vez. As 4 invocações
  registradas em `d4/logs/docker-shim.log` foram:

```text
/usr/bin/docker run --pull=never --network=none --rm -v …/d4/work/<cenário>:/mnt \
  risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331
```

O prover funcionou **sem rede** (`--network=none`), o que o D2d não tinha
isolado.

**`job_id`s:** 32 bytes de `/dev/urandom`, conferidos ≠ `0x11…`:

| Job | `job_id` |
| --- | --- |
| S (smoke, D4b) | `fe6d25fe77784e80d64ddd99b95fc0095011355cafbf93beac6cea678ce0959b` |
| A (PASS, D4b) | `3f0dd1c833714e193744c5b95879662c1d8b440cdd3c6467a1585ecc60baec8a` |
| B (FAIL, D4b) | `5a25ae4808b6da928f57c8787c4fbb173791239d18342b540d4c485647fbc309` |

**Receipts** (todas: `Composite` verificada, journal igual ao core e ImageID
errado rejeitado; depois `Groth16` de 827 bytes verificada, journal igual à
Composite, selector `73c457ba` e ImageID errado rejeitado):

| Cenário | Job | Artefato | Veredito | Prova | Compressão | Pico de RAM (sistema) | Composite SHA-256 | Groth16 SHA-256 | Journal SHA-256 (= digest) | Seal SHA-256 |
| --- | --- | --- | --- | ---: | ---: | --- | --- | --- | --- | --- |
| S | S | `(7,14)` | PASS | 8,1 s | 74,7 s | 7.644 MiB usados, 158 MiB livres | `836683d5…` | `65fb0289…` | `20b3353cf3ba872e6bdb7c62e4685545f3784dd76c396ee3431bc6176e26c0b1` | `4f1131c29b091faf285bb95720d42a477565eb6bc9d1156c66309d0157de98ef` |
| A | A | `(7,14)` | PASS | 8,5 s | 78,9 s | 7.621 / 181 | `dcbf14de…` | `cea94533…` | `a1060efe594b409e934d338987adcbc8fe2b864b49715fe1b0f33ae7a9dc5f47` | `610704b1bb2be5d49f5626718990b73393e9bb85ded982533dcbaa8b02da8cf0` |
| A′ | A | `(7,15)` | FAIL | 8,3 s | 77,5 s | 7.621 / 180 | `5e9889d8…` | `5861fc75…` | `e04f62d92a2a6c40e6f3da7ab3f1ad3257523175959393313069a117402c163c` | `7c5962252392c005093dbbcb8cfe91000864413afeed20ec979adeb361c85036` |
| B | B | `(7,15)` | FAIL | 8,4 s | 76,9 s | 7.565 / 236 | `7f72e8ee…` | `c8aebc99…` | `e931596b57fec1f96512d19a9f5c5d5baf214243484e8b20315d9d5245ec7741` | `462c4855b78897b44e0f3fccd1f2bff35a68ac1c83a5430c44ca5dbfc395d63f` |

Arquivos em `d4/receipts-out/<cenário>/`. A lista completa de SHA-256 está
em `d4/logs/receipts-out.sha256`. A margem de memória é pequena (cerca de 160
MiB livres no pico): provar só um cenário por vez, sem outra carga pesada.

**Simulação somente leitura em devnet** (`d4/bin/sim_vectors.py`,
`THq1q….verify` direto):

| Caso | Resultado |
| --- | --- |
| S, A, A′ e B | **sucesso**, 99.541 CU cada |
| journal A com o seal de S | `Custom(6000)` `VerificationError` |

O A′ é um FAIL verificável **do Job A**. No D4b ele sustenta o negativo
`refund_on_fail` de artefato não entregue → 6017, antes da CPI.

## Invariantes

- Guia §7:
  - 1 a 6, 8 e 9 inalteradas e testadas (57/57);
  - 2 reforçada pelo mint admitido;
  - 7: nenhuma instrução administrativa; o verificador é imutável em
    devnet; a upgrade authority do escrow fica para o D4b;
  - 10: a falha de verificação reverte, testada com os bytes de devnet.
- Princípio 5 (`AGENTS.md`): o `JournalV1` v1 está congelado sem mudança de
  byte.
- Princípio 8: nenhuma alegação de verificação on-chain; só simulação e
  `solana-program-test`.
- `Verdict::Fail` continua saída normal: A′ e B são receipts FAIL válidas.

## Fronteiras

- **Rede:** só `api.devnet.solana.com`, somente leitura (`getAccountInfo`,
  `getSignaturesForAddress`, `getTransaction`, `simulateTransaction` com
  `sigVerify=false`, `program dump`).
  - Nenhum airdrop, deploy, assinatura ou transação.
  - crates.io não foi usado; nenhum pull Docker.
- **Docker:** só a imagem local por digest, com `--pull=never
  --network=none` e `HOME` isolada; nenhum container remanescente.
- **Keypairs:**
  - só em `d4/keys` (`0600`), sem exibição de segredo; só pubkeys
    publicadas;
  - o keypair do escrow foi copiado sem leitura para os out-dirs (`0600`).
- **Repositório:**
  - mudaram o programa, os testes, as fixtures README e os docs;
  - core, `zkvm/`, guest, `Cargo.toml` e os cinco locks ficaram inalterados;
  - nada ignorado nem gerado dentro do clone.
- **Perfil padrão** no fim: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`,
  `~/.docker` `6046f67f…` (iguais); `~/.rustup`, `~/.cache/solana` e
  `~/.config/solana` ausentes.
- **Persistência:** tudo fora do clone ficou em
  `~/.local/share/vericode-spikes/d4/`, persistente, nunca em `/tmp`.

## Riscos abertos

- **Sem e-stop:** um bug de soundness do verificador Groth16 de
  `risc0-zkvm 3.0` não pode ser pausado. Isso foi aceito para devnet com
  Test USDC.
- **Bytes do verificador de devnet ≠ rebuild local:** a equivalência é
  funcional (simulações e suíte com o dump), não byte a byte.
- **Upgrade authority do escrow:** a finalizar no D4b, depois do smoke run.
  Até lá, o escrow é upgradeable. Ele também ainda não foi implantado.
- **R-02:** `fund` fora da janela. A mitigação é `create_job`+`fund` na mesma
  transação (D4b).
- **R-03:** replay entre implantações. A mitigação são os `job_id`s
  aleatórios e as receipts novas.
- **SOL de devnet:**
  - o escrow (395.064 bytes) exige cerca de 2,75 SOL de rent mais taxas;
  - o deployer `617ogw9T…` ainda não tem saldo;
  - um faucet limitado pode bloquear o D4b.
- **Mint admitido:** a autoridade de mint será o deployer, que pode emitir
  Test USDC à vontade. Isso não afeta a custódia.
- **Endereço do mint pré-financiado:** como a pubkey `9TE2VPFm…` é pública,
  um terceiro pode enviar lamports para ela antes do D4b. Nesse caso, o
  `SystemProgram::CreateAccount` falha (`AccountAlreadyInUse`).
  - O cliente do D4b deve conferir o saldo e, se houver lamports, criar a
    conta com transfer + allocate + assign, assinados pelo keypair do mint.
  - Ninguém consegue criar o mint nesse endereço sem esse keypair.
- **Herdados:** ImageID não recertificado; spec v1 trivial; F-09, F-13 e F-14.
- **Margem de RAM** do prover, de cerca de 160 MiB.

## Próximo gate

`R-D4a`: revisão delta somente leitura do D4a (Opus 5.5, max), conforme
[`docs/handoffs/d4a-to-r-d4a.md`](handoffs/d4a-to-r-d4a.md). Se aprovada,
vem o `D4b`: deploy em devnet, smoke, finalização, Jobs S, A, B e C,
negativos no Explorer e C7.
