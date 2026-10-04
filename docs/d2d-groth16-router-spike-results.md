# D2d — spike do caminho forte: Groth16 do VeriCode e Verifier Router em processo

Data: 2026-10-04 · orçamento de 3 h (início 12:53:14, fim técnico 13:34:08,
cerca de 41 min)

## Resultado

**GO para o D2e.**

- **Receipts:** as receipts `Composite` reais do guest VeriCode (PASS e FAIL,
  D1c2b) foram comprimidas localmente para receipts **Groth16 reais** e
  verificadas localmente contra o ImageID `4da06f90…fb1a`.
- **Router:** o Verifier Router e o verificador Groth16 de
  `risc0-solana v3.0.0`, compilados SBF no Perfil A e carregados em
  `solana-program-test 2.3.9`, rejeitaram quatro adulterações por vetor e
  aceitaram as provas válidas de PASS, FAIL e do vetor oficial FIB.

Afirmação máxima permitida: **"a receipt Groth16 do VeriCode foi verificada
pelo programa Verifier Router em `solana-program-test` local"**.

Isto não é verificação ZK on-chain em cluster:
- devnet, deploy e CPI a partir do `vericode_escrow` continuam
  `STATUS: NÃO VALIDADO`;
- o código do spike ficou **fora do clone**, em
  `~/.local/share/vericode-spikes/d2d`;
- o repositório recebeu somente documentação.

## Preflight e checagem anterior

- HEAD `ec980e9` (docs D2c sobre `a10f026`); árvore limpa;
  `git diff --check` exit `0`.
- `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes.
- Snapshots: `~/.cargo` com 5.816 entradas (SHA-256 da listagem `d9e12578…`,
  igual ao D2c) e `~/.avm`.
- Core 36/36 em `+1.85.0`/`+1.89.0`.
- Programa:
  - rebuild SBF em target novo, offline, reproduziu exatamente
    `vericode_escrow.so` `04cc2a845eea1b2e10e313601d1b51887f3f069f3e80c802651d499ddfd0ae56`;
  - testes em processo 10/10.
- Locks do repositório iguais aos esperados.
- Artefatos D1c2b em `/tmp` conferidos e copiados para
  `d2d/artifacts`, com hashes antes e depois:
  - método `e09ba8cf…78f5`; ELF `63fac491…5408`;
  - receipts PASS `5dcf89f5…`, FAIL `3477a592…`, wrong-image `189bd609…`;
  - `recursion_zkr.zip` `744b999f…8849`.

## Fatos confirmados no fonte pinado

- **Compressão:** `risc0-zkvm 3.0.3` `Prover::compress(&ProverOpts::groth16(), …)`
  executa Composite → Succinct → `identity_p254` → `shrink_wrap`.
- **Prover Groth16:** `risc0-groth16 3.0.2` `shrink_wrap` executa
  `docker run --rm -v <work>:/mnt risczero/risc0-groth16-prover:v2025-04-03.1`.
  - Tag fixa no código, sem `--network none`.
  - A alternativa CUDA exige GPU.
  - O Dockerfile oficial embute o proving key; montar a imagem localmente não
    é viável.
- **Recursão:** `risc0-circuit-recursion 4.0.2` baixaria `recursion_zkr.zip`
  do S3; `RECURSION_SRC_PATH` aceita uma cópia local com o SHA-256 exigido.
  - O arquivo no checkout `risc0 v3.0.3` é ponteiro Git LFS (133 bytes).
  - Foi usada a cópia real do `OUT_DIR` do D1c2b.
- **Verifier Router:**
  - PDA `["router"]`; `INITIAL_OWNER` fixado no build;
  - `add_verifier` exige o verificador upgradeable com upgrade authority =
    PDA do Router;
  - entrada PDA `["verifier", selector]`;
  - `verify(Seal{selector, proof}, image_id, journal_digest)` faz CPI para o
    verificador.
- **Seal:** `selector` = 4 primeiros bytes do digest de
  `Groth16ReceiptVerifierParameters::default()`; `pi_a` negado.
- **Claim:** `hash_claim(image_id, SHA-256(journal))`.

## Ambiente do spike

Raiz `~/.local/share/vericode-spikes/d2d`:

| Raia | Homes | Uso |
| --- | --- | --- |
| zkVM | cópias de `d1c2b/cargo` e `d1a3/homes/zkvm/rustup`; binários `d1a3/homes/zkvm/cargo/bin` | Rust `1.89.0`; `CARGO_NET_OFFLINE=true`; `RECURSION_SRC_PATH`; `RISC0_PROVER/EXECUTOR=local`; dev/Bonsai ausentes |
| Solana | ambiente `d2c` lane-b (Perfil A) | `cargo-build-sbf` Agave `2.3.9`; `solana-program-test 2.3.9` |

Keypair de teste do dono do Router:
- `d2d/keys/router-owner.json`, `0600`, criado com `solana-keygen --silent`;
- só o pubkey `8pS2iUcJuSvEBakinzmUcTp4MSBvkqHpauDpvhh8mXak` foi usado em
  `INITIAL_OWNER`.

Os `.so` compilados geraram keypairs de programa no out-dir, todos `0600` e
fora do clone.

## Rede usada (dentro do autorizado)

| Destino | Motivo | Evidência |
| --- | --- | --- |
| `index.crates.io`/`static.crates.io` | resolução/download dos workspaces `risc0-solana` (`--locked`) e do harness do Router | o build offline falhou antes (índice `6f17d22b…` sem `risc0-zkvm`) |
| Docker Hub (metadados) | `docker buildx imagetools inspect` da tag aprovada | índice `sha256:a4f80ce2e0b8e2bb7637a93c37136a6776ac00ec843a3fdf1c67b1d5ffea64ee`; amd64 `sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331`; config `sha256:f6f756b0…24cd`; 9 camadas, 2.399.158.367 bytes comprimidos |
| Docker Hub (pull) | pull **por digest amd64** e tag local `v2025-04-03.1` | pull exit `0`; imagem `7f1739631965`, 5,21 GB, `linux/amd64` |

A rede do container do prover não foi isolada: o código upstream não passa
`--network none`. Nenhum outro destino foi acessado.

## Fase 1 — Router com o vetor oficial (sem Docker)

### Build SBF (Perfil A, staging `ee415935…`, lock `54949aa8…`, `--locked`)

| Programa | Exit | Bytes | SHA-256 |
| --- | ---: | ---: | --- |
| `groth_16_verifier` | `0` (1m14s) | 199.248 | `dab6746d4a24f1d263f303ef91103166e97d03c5ae0e37893ed517b831e01e0c` |
| `verifier_router` (`INITIAL_OWNER=8pS2iUcJ…`) | `0` (21s) | 321.256 | `1b26b017b08eb62e79cee1d58dcec429edb2a1076dd5d4dc4b238c3ae79ce1c5` |

O lock upstream ficou inalterado e o staging limpo.

### Vetor oficial FIB

O harness `d2d-receipts fib-vector` desserializou `FIB_RECEIPT`
(`groth_16_verifier/src/v3_test_receipt.rs`):

```text
fib.receipt_type=Groth16
fib.local_verify=ok
fib.negative.wrong_image=rejected
fib.selector=73c457ba
fib.verifier_parameters=73c457ba541936f0d907daf0c7253a39a9c5c427c225ba7709e44702d3c6eedc
fib.image_id=e69b4c5b08ea6d6f09192205ce2fa0cfc2089b00b832d6d46d01c2ca3fe0ec5d
fib.journal_digest=7bcb645a8079c7148be35f1b251215fe5dbcd6d83d8c8302da45e49cf1e0757e
```

O selector `73c457ba` coincide com o digest dos parâmetros padrão do
`risc0-zkvm 3.0.3` e com o `verifier_parameters` da receipt.

## Fase 2 — Groth16 do VeriCode

Comando: `d2d-receipts compress <pass|fail>`, com
`default_prover().compress(&ProverOpts::groth16(), &composite)`.

| Cenário | Exit | Compressão | Wall | Tipo | Verify local | ImageID errado | Journal = Composite | Receipt Groth16 | Seal SHA-256 |
| --- | ---: | ---: | ---: | --- | --- | --- | --- | ---: | --- |
| PASS | `0` | 84,9 s | 1:30,8 | `Groth16` | ok | rejeitado | sim | 827 B (`c3f986b7…`) | `4373517058bdf048484d2f57913740df9c611e113e9a6b212f41d905a9f578c2` |
| FAIL | `0` | 83,2 s | 1:29,2 | `Groth16` | ok | rejeitado | sim | 827 B (`96c2136b…`) | `93a12d12a8ed406a69b54c1b91aa49bd1f67d663723188dd7b03888e2d343ec5` |

Journal digests (SHA-256 do `JournalV1` de 165 bytes):
- PASS `7c3f596eec21cafeef9aa7eab421582f97819ae7961aa1250e6526315b3f7de1`;
- FAIL `29ff43b03973adf789f1426f31b3bbcd095ece94e3d984c92ec53be511127475`.

Selector `73c457ba` nos dois casos. O journal impresso confirma os valores
canônicos da errata D1c2b.3i: harness `01124025…996b50` e artifact PASS
`d5aa9223…224c`.

**Memória:**

| Cenário | Pico do container | RAM do sistema | Swap |
| --- | ---: | --- | ---: |
| PASS | 6,25 GiB | 7.441/7.802 MiB usados no pico | até ~1,8 GB |
| FAIL | 6,21 GiB | pico equivalente | — |

O processo host teve RSS máximo de 1,45 GB. Não houve OOM, mas a margem é
pequena: o worker da demo deve provar um cenário por vez e sem outras cargas
pesadas.

## Router em processo — matriz completa

Comando:

```text
SBF_OUT_DIR=d2d/out/sbf D2D_VECTORS=fib,pass,fail cargo +1.89.0 test --locked -- --nocapture
```

Exit `0`; `1 passed; 0 failed`.

Montagem:
- Router via `add_program`;
- verificador montado manualmente como programa upgradeable (contas
  Program/ProgramData no formato de `solana-program-test`), com upgrade
  authority = PDA do Router;
- `initialize` assinado pelo dono;
- compute budget de 1,4 M por transação.

Resultado de setup:
- `add_verifier` por quem não é dono foi rejeitado com 6000
  (`OwnableError::NotOwner`);
- `add_verifier` pelo dono foi aceito.

| Caso (por vetor, negativos antes do positivo) | FIB | PASS | FAIL | Erro |
| --- | --- | --- | --- | --- |
| proof adulterada (1 bit em `pi_c`) | rejeitado | rejeitado | rejeitado | 6003 `PairingError` (verificador) |
| selector sem entrada (`deadbeef`) | rejeitado | rejeitado | rejeitado | 3012 `AccountNotInitialized` (Anchor) |
| ImageID errado | rejeitado | rejeitado | rejeitado | 6000 `VerificationError` (verificador) |
| journal digest errado | rejeitado | rejeitado | rejeitado | 6000 `VerificationError` (verificador) |
| prova válida | **aceita** (110.851 CU) | **aceita** (110.851 CU) | **aceita** (110.851 CU) | — |

O teste também conferiu, para cada vetor, que `SHA-256(journal) ==
journal_digest`. Para PASS e FAIL, conferiu o tamanho de 165 bytes e o
verdict tag `0`/`1`. O Router aceita FAIL porque verifica a validade da
execução; o veredito econômico continua sendo decidido pelo core.

## Hashes do spike (fora do clone)

| Arquivo | SHA-256 |
| --- | --- |
| `receipts/Cargo.lock` (semeado do `zkvm/Cargo.lock`) | `24023c6d11761459c8e1cc79dd4e67b1856ad7d0c70c4f76246f94ef1c58e8f4` |
| `receipts/src/main.rs` | `312815540aaeeca979acded437ead159eb7b900816a5cd20e9085c8faafeac7b` |
| `router-test/Cargo.lock` (semeado do `anchor/tests-local/Cargo.lock`) | `5e6f5974ea013d7ab13389ccfff848f4ae1c6ffdad806d77ae12751577452225` |
| `router-test/tests/router.rs` | `3de432117bbf5abb0cb859803606b32cd6eab23e93ddaf8b05c6c0e7fedd4c90` |
| binário `d2d-receipts` (build release, 29 min, offline) | `756f5932eb27d991e8d6255f5516648750ab1aa110edd4dce39b1f3332beba63` |

Os vetores (selector, seal, image_id, journal, digest) e as receipts Groth16
estão em `d2d/vectors`, com `sha256sum` em `d2d/logs/vectors.sha256`.

## Fronteiras e higiene

- Repositório: nenhuma alteração de código; locks inalterados
  (`191802b2…`, `f5236689…`, `1116acef…`, `19a1db26…`, `be94760a…`).
- Checkouts D1a.3 e staging D2c limpos; locks upstream inalterados.
- `~/.cargo` e `~/.avm` idênticos aos snapshots; `~/.rustup`,
  `~/.cache/solana` e `~/.config/solana` ausentes; `~/.risc0`, preexistente
  de 02/10, intocado.
- **Escrita no perfil padrão causada pelo D2d:** `~/.docker/buildx/current`.
  - 62 bytes: `{"Key":"unix:///var/run/docker.sock","Name":"","Global":false}`.
  - Criado às 13:08:29 por `docker buildx imagetools inspect`; `~/.docker` já
    existia desde 29/09.
  - Sem segredo; não removido.
- `~/.codex/*` também mudou no período, mas pertence a outra ferramenta em
  execução, não aos comandos deste gate.
- Imagens Docker locais: builder D1c2b e o prover Groth16; nenhum container
  remanescente (`--rm`).
- Sem dev mode, receipt `Fake`, mock de Router, devnet, deploy ou push.

## Decisão GO e implicações para o D2e

GO: a cadeia Groth16 → Router → verificador funciona localmente com receipts
VeriCode reais e rejeita adulterações.

Implicações:
- `release`/`refund_on_fail` no `vericode_escrow`, a cada chamada:
  1. recebem o journal de 165 bytes e o seal;
  2. validam o journal contra o Job pelo core;
  3. calculam `SHA-256(journal)` on-chain;
  4. fazem CPI `verify(seal, job.image_id, digest)`;
  5. só então transferem.
- Cada `verify` consome cerca de 111 k CU; a transação precisa de compute
  budget acima de 200 k.
- A dependência do `verifier_router` (git pinado, vendor ou CPI manual por
  discriminador/ABI do D1a.3) é decisão do D2e.
- O Program ID do Router em devnet continua não confirmado.

## Riscos abertos

- **Margem de memória:** o prover Groth16 usou cerca de 6,25 GiB de 7,62 GiB
  disponíveis.
- **Imagem do prover:** fixada localmente por digest, mas referenciada por tag
  no código; sem isolamento de rede do container.
- **Router em teste:** `INITIAL_OWNER` e dono de teste; em devnet, o Router
  real, seu dono e o Program ID ainda não foram confirmados
  (`STATUS: NÃO VALIDADO`).
- **Artefatos:** os vetores e receipts Groth16 existem só fora do clone; o
  ImageID não foi recertificado após as mudanças do core no D2a/D2b.
- **Revisões adversariais** D2b/D2c continuam pendentes.
