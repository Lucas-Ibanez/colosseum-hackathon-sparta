# `vericode-prover`

Prover local do VeriCode (gate D7). Prova o guest admitido para um Job, com o
prover local do RISC Zero `3.0.3`, e comprime a receipt para Groth16, no
formato que `vericode job settle` (`cli/`) submete ao escrow em devnet.

- O guest **não é recompilado**: o binário versionado em
  [`artifacts/vericode-guest.bin`](artifacts/README.md) é embutido no
  executável. Antes de cada operação, o prover confere o tamanho (180.300 B), o
  SHA-256 (`e09ba8cf…`) e o ImageID (`4da06f90…fb1a`, igual a
  `ADMITTED_IMAGE_ID_V1`). Se algum divergir, aborta.
- Sem dev mode: a feature `disable-dev-mode` faz o `risc0-zkvm` recusar
  `RISC0_DEV_MODE`, e o prover exige `Composite` e depois `Groth16`; nunca
  aceita `Fake`.
- O frame de entrada é o de `zkvm/host`: `job_id ‖ artefato (12 B) ‖
  image_id`. O journal da receipt é comparado, byte a byte, com
  `evaluate_restricted_artifact` do `vericode-core`.

## Requisitos

| Item | Valor |
| --- | --- |
| Rust | `1.89.0` (`cargo +1.89.0 …`), com compilador C/C++ para os crates `risc0-circuit-*-sys` |
| Lock | `prover/Cargo.lock` (workspace próprio; semeado do lock `ec0dd8d6…` do harness do D4a; nenhuma fonte git). Os locks de `zkvm/` não são usados nem alterados |
| Docker (só `compress`) | x86_64, com a imagem `risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331` já presente localmente. O prover **nunca faz pull** |
| Memória | a compressão Groth16 chegou a cerca de 6,3–7,6 GiB (com swap) num WSL de 7,6 GiB. Rode **uma prova por vez** e sem builds em paralelo; exit 137 = falta de memória |
| `recursion_zkr.zip` | o build script de `risc0-circuit-recursion 4.0.2` baixa esse arquivo do S3 da RISC Zero e confere o SHA-256 `744b999f0a35b3c86753311c7efb2a0054be21727095cf105af6ee7d3f4d8849`. Para compilar offline, aponte `RECURSION_SRC_PATH` para uma cópia local com esse hash |

## Comandos (a partir da raiz de um clone)

```bash
export RISC0_PROVER=local RISC0_EXECUTOR=local   # nunca Bonsai
unset RISC0_DEV_MODE BONSAI_API_KEY BONSAI_API_URL
# opcional, para compilar offline: export RECURSION_SRC_PATH=/caminho/recursion_zkr.zip
# opcional, para não usar /tmp: export TMPDIR=/caminho/tmp

cargo +1.89.0 build --locked --release --manifest-path prover/Cargo.toml
cargo +1.89.0 test  --locked --release --manifest-path prover/Cargo.toml

P=prover/target/release/vericode-prover
$P check                                   # guest admitido: SHA-256, ImageID, selector
$P prove <job_id_hex> 21 42 receipts/P     # Composite; journal == core
$P compress receipts/P                     # Groth16 via Docker local
$P verify receipts/P                       # reconfere a receipt e os vetores
```

`<job_id_hex>` é o `job_id` impresso por `vericode job create`. `21 42` é o
artefato `(input, claimed_output)`: `PASS` só quando `claimed_output = 2 ×
input`; outra alegação dá `FAIL` como resultado normal. Entrada acima de
1.000.000 é erro, não `FAIL`. O `job_id` `0x11…` é reservado às fixtures de
teste e é recusado.

## Saída (`<dir>`)

| Arquivo | Conteúdo |
| --- | --- |
| `composite.receipt` | `Receipt` bincode, `Composite` |
| `groth16.receipt` | `Receipt` bincode, `Groth16` |
| `selector` | 4 B, `73c457ba` |
| `seal` | 256 B, seal Groth16 cru (`pi_a` **não** negado; a CLI nega) |
| `image_id` | 32 B |
| `journal` | 165 B, `JournalV1` |
| `journal_digest` | 32 B, SHA-256 do journal |
| `docker-shim.log` | a linha exata do `docker run` executado |
| `groth16-work/` | arquivos de trabalho do prover Groth16 (`RISC0_WORK_DIR`); com Docker rootful, `proof.json` pertence a `root` |

## Docker

O `risc0-groth16 3.0.2` chama `docker run --rm -v <work>:/mnt
risczero/risc0-groth16-prover:v2025-04-03.1`, por tag e com rede. Antes da
compressão, o prover:
1. confere que a imagem existe localmente pelo digest (`docker image
   inspect`);
2. põe [`docker-shim/`](docker-shim/docker) na frente do `PATH` do próprio
   processo.

O shim troca a tag pelo digest e acrescenta `--pull=never --network=none`.
Qualquer outro `docker run` é recusado. `VERICODE_REAL_DOCKER` muda o
executável real (padrão `/usr/bin/docker`).

## O que a receipt prova e o que não prova

A receipt atesta que o guest admitido executou o harness fixo sobre o artefato
`(input, claimed_output)` vinculado ao `job_id` e publicou o journal com o
veredito. Ela não prova que um código está correto: a spec v1 é trivial
(`saída = entrada × 2`), e o executor escolhe a entrada. A verificação
on-chain acontece no escrow, por CPI ao verificador Groth16 imutável de
`risc0-solana v3.0.0` em devnet (ver o `README.md` da raiz).
