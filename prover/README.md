# `vericode-prover`

Prover local do VeriCode (gate D7, endurecido no D10a). Prova o guest
admitido para um Job, com o prover local do RISC Zero `3.0.3`, e comprime a
receipt para Groth16, no formato que `vericode job settle` (`cli/`) submete
ao escrow em devnet.

- O guest **não é recompilado**: o binário versionado em
  [`artifacts/vericode-guest.bin`](artifacts/README.md) é embutido no
  executável. Antes de cada operação, o prover confere o tamanho (180.300 B), o
  SHA-256 (`e09ba8cf…`) e o ImageID (`4da06f90…fb1a`, igual a
  `ADMITTED_IMAGE_ID_V1`). Se algum divergir, aborta.
- **Só prova local (D10a, RD7-03).** O prover instancia o `LocalProver` do
  RISC Zero diretamente, nunca `default_prover()`. `check`, `prove` e
  `compress` recusam, antes de qualquer trabalho:
  - `RISC0_PROVER` diferente de `local` (vazio conta como ausente);
  - qualquer variável `BONSAI_*`, cujo valor nunca é impresso;
  - qualquer `RISC0_DEV_MODE`.

  Assim, nenhuma variável de ambiente desvia a prova para o Bonsai, para um
  `r0vm` externo ou para o dev mode.
- Sem dev mode: além dessa recusa, a feature `disable-dev-mode` faz o
  `risc0-zkvm` rejeitar `RISC0_DEV_MODE`. O prover exige `Composite` e depois
  `Groth16`, e nunca aceita `Fake`.
- O frame de entrada é o de `zkvm/host`: `job_id ‖ artefato (12 B) ‖
  image_id`. O journal da receipt é comparado, byte a byte, com
  `evaluate_restricted_artifact` do `vericode-core`.

## Requisitos

| Item | Valor |
| --- | --- |
| Rust | `1.89.0` (`cargo +1.89.0 …`), com compilador C/C++ para os crates `risc0-circuit-*-sys` |
| Lock | `prover/Cargo.lock` (workspace próprio; semeado do lock `ec0dd8d6…` do harness do D4a; nenhuma fonte git). Os locks de `zkvm/` não são usados nem alterados |
| Docker (só `compress`) | x86_64, com a imagem `risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331` (5,21 GB) já presente localmente. O prover **nunca faz pull**: baixe a imagem antes, uma vez, por esse digest |
| Rede no primeiro build | rustup (toolchain `1.89.0`), crates.io e `recursion_zkr.zip` (linha abaixo). Com esses três já presentes, `--offline` funciona |
| Memória | a compressão Groth16 chegou a cerca de 6,3–7,6 GiB (com swap) num WSL de 7,6 GiB. Rode **uma prova por vez** e sem builds em paralelo; exit 137 = falta de memória |
| `recursion_zkr.zip` | o build script de `risc0-circuit-recursion 4.0.2` baixa esse arquivo do S3 da RISC Zero e confere o SHA-256 `744b999f0a35b3c86753311c7efb2a0054be21727095cf105af6ee7d3f4d8849`. Para compilar offline, aponte `RECURSION_SRC_PATH` para uma cópia local com esse hash |

## Comandos (a partir da raiz de um clone)

Desde o D10a, o prover só prova localmente e recusa um ambiente que diga o
contrário (ver acima). `check` imprime `prover=LocalProver env=ok`. Os
binários do D9 (seção "Gravação" do roteiro) são anteriores a essa recusa:
com eles, continue fixando o ambiente abaixo.

```bash
export RISC0_PROVER=local                         # ou deixe sem definir
unset RISC0_DEV_MODE BONSAI_API_KEY BONSAI_API_URL
# opcional, para compilar offline: export RECURSION_SRC_PATH=/caminho/recursion_zkr.zip
# opcional, para não usar /tmp: export TMPDIR=/caminho/tmp

cargo +1.89.0 build --locked --release --manifest-path prover/Cargo.toml
cargo +1.89.0 test  --locked --release --manifest-path prover/Cargo.toml

P=prover/target/release/vericode-prover
RUN=~/vericode-run; mkdir -p $RUN          # receipts fora do clone
$P check                                   # guest admitido (SHA-256, ImageID, selector) e ambiente local
$P prove <job_id_hex> 21 42 $RUN/P     # Composite; journal == core
$P compress $RUN/P                     # Groth16 via Docker local
$P verify $RUN/P                       # reconfere a receipt e os vetores
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
| `docker-shim.log` | a linha exata do `docker run` executado e cada chamada recusada (`REFUSED`) |
| `groth16-work/` | arquivos de trabalho do prover Groth16 (`RISC0_WORK_DIR`); com Docker rootful, `proof.json` pertence a `root` |

## Docker

O `risc0-groth16 3.0.2` chama `docker run --rm -v <work>:/mnt
risczero/risc0-groth16-prover:v2025-04-03.1`, por tag e com rede. Antes da
compressão, o prover põe [`docker-shim/`](docker-shim/docker) na frente do
`PATH` do próprio processo e confere, pelo shim, que a imagem existe
localmente pelo digest (`docker image inspect`).

Desde o D10a (achado RD7-02 do R-D7), o shim é uma **allowlist exata de
argv**. Os chamadores de `docker` nesse processo foram levantados no fonte
pinado:
- `risc0-groth16 3.0.2`: `--version` e `run --rm -v <work>:/mnt <tag>`;
- o próprio prover: `image inspect --format {{.Id}} <digest>`.

O shim aceita só esses três argv e remonta o comando do zero:

| argv recebido | comando executado |
| --- | --- |
| `--version` | `docker --version` |
| `image inspect --format {{.Id}} <digest>` | `docker --context default image inspect --format {{.Id}} <digest>` |
| `run --rm -v <work>:/mnt <tag>` | `docker --context default run --pull=never --network=none --rm -v <work>:/mnt <digest>` |

- `<work>` tem de ser um caminho absoluto, canônico, de um diretório
  existente, diferente de `/` e sem `:`, `,` ou quebra de linha. O prover
  passa `<dir>/groth16-work`, ou `RISC0_WORK_DIR`, se definido.
- **Qualquer outro argv é recusado** (exit 2 e linha `REFUSED` no log). Isso
  inclui outro subcomando, `container run`, opções globais como
  `--context`, flags a mais, outra imagem e a tag fora da posição.
- **Daemon local.** `--context default` faz o Docker ignorar `DOCKER_CONTEXT`
  e o contexto atual da configuração. O shim recusa um `DOCKER_HOST` que não
  seja um socket `unix://` local.

O container Groth16 roda, portanto, só a imagem local por digest, sem pull e
sem rede, no daemon local. O teste `tests/docker_shim.rs` cobre os casos S1 a
S7 do R-D7 e os casos novos, com um `docker` falso que só registra argv.
`VERICODE_REAL_DOCKER` muda o executável real (padrão `/usr/bin/docker`).

## O que a receipt prova e o que não prova

A receipt atesta que o guest admitido executou o harness fixo sobre o artefato
`(input, claimed_output)` vinculado ao `job_id` e publicou o journal com o
veredito. Ela não prova que um código está correto: a spec v1 é trivial
(`saída = entrada × 2`), e o executor escolhe a entrada. A verificação
on-chain acontece no escrow, por CPI ao verificador Groth16 imutável de
`risc0-solana v3.0.0` em devnet (ver o `README.md` da raiz).
