# D1c2b.3a — vendor detectável no Dockerfile real

Data da execução: 2026-10-02

## Decisão

**GO** para retomar o build determinístico do guest em um gate posterior.

O vendor offline foi colocado somente em um staging temporário como
`/src/vendor`, com a configuração emitida pelo Cargo em
`/src/.cargo/config.toml`. Uma única execução de
`docker build --pull=false --network=none`, sem tag, reproduziu o
`WORKDIR /src`, o contexto e o comando de fetch usados pelo Dockerfile gerado
por `risc0-build 3.0.3`. Tanto `cargo +risc0 fetch --locked --offline` quanto
`cargo +risc0 metadata --locked --offline` terminaram com exit `0`.

Este resultado comprova somente descoberta e consumo do vendor no contexto
real do Dockerfile. Não houve compilação do guest, ELF, ImageID, receipt,
proving ou execução do host.

## Estado inicial preservado

- Clone canônico: `/home/lucas/src/vericode`.
- Branch: `main`.
- Commit exportado: `398e9e0f7b902120749674ed78d8ecbe6b51d91a`
  (`style(zkvm): remove trailing blank lines`).
- `git show --check HEAD`: exit `0`.
- Alterações preexistentes: somente documentos dos gates D1c2b, D1c2b.2,
  D1c2b.2a e D1c2b.3; foram preservadas.
- `git diff --name-only -- zkvm`: vazio.
- Nenhum `vendor/` ou `.cargo/config.toml` compartilhado existia no clone.

Locks antes da execução:

| Arquivo | SHA-256 |
| --- | --- |
| `zkvm/Cargo.lock` | `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1` |
| `zkvm/methods/guest/Cargo.lock` | `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50` |

## Fonte exata do comportamento do builder

Foi auditado o arquivo local de registry correspondente a
`risc0-build 3.0.3`:

`/home/lucas/.local/share/vericode-spikes/d1c2b/cargo/registry/src/`\
`index.crates.io-1949cf8c6b5b557f/risc0-build-3.0.3/src/docker.rs`

SHA-256:
`ab72d824357c0b51238945725ce8b3c7a366d6f589b32eca6313680b47f84595`.

O fonte constrói o estágio com:

```text
.from_alias("build", &docker_tag)
.workdir("/src")
.copy(".", ".")
```

e monta o fetch a partir de `cargo +risc0 fetch`, seguido por `--locked`,
`--target riscv32im-risc0-zkvm-elf` e `--manifest-path
$CARGO_MANIFEST_PATH`. O `build.rs` do VeriCode passa a raiz do repositório
como contexto; seu SHA-256 foi
`0edffb0f5f87b95c37b9533e956c45f1a537a763891bda167aa4297a9f4af28c`.

Conclusão causal: a configuração anterior em
`zkvm/methods/guest/.cargo/config.toml` era detectável quando o processo
iniciava nesse diretório, mas não pelo processo iniciado em `/src`. A
configuração temporária precisava estar em `/src/.cargo/config.toml`, ao lado
do `/src/vendor` referenciado por ela.

## Imagem e ferramentas efetivamente usadas

- Imagem local:
  `risczero/risc0-guest-builder:r0.1.88.0`.
- Digest e ID local:
  `sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3`.
- `Entrypoint=["/bin/sh"]`; a configuração inspecionada não declara `Cmd`.
- Cargo host: `cargo 1.89.0 (c24e10642 2025-06-23)`.
- Rust host: `rustc 1.89.0 (29483883e 2025-08-04)`.
- Cargo guest: `cargo 1.88.0-dev (873a06493 2025-05-10)`.
- Docker: `29.8.1`, build `4a63305`.
- `CARGO_HOME` usado para o vendor:
  `/home/lucas/.local/share/vericode-spikes/d1c2b/cargo`.

A imagem foi resolvida localmente em `0.0s`; não houve `docker pull`.

## Staging e vendor raiz

O staging foi criado em `/tmp/vericode-d1c2b3a.7TFWFb` e populado somente
por:

```text
git archive --format=tar \
  --output=/tmp/vericode-d1c2b3a.7TFWFb/HEAD.tar HEAD
tar -xf /tmp/vericode-d1c2b3a.7TFWFb/HEAD.tar \
  -C /tmp/vericode-d1c2b3a.7TFWFb/src
```

SHA-256 do arquivo exportado:
`e6cae5c8645f5899902c182885363a988ecc3b4d7562d0ce3334faa30a8b471f`.
O staging não contém `.git`, credenciais ou dados pessoais. Depois da
exportação, somente `vendor/`, `.cargo/config.toml` e o Dockerfile de
diagnóstico foram acrescentados.

O vendor foi gerado a partir da raiz do staging:

```text
env \
  CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo \
  RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup \
  RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0 \
  CARGO_NET_OFFLINE=true \
  /home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin/cargo \
  +1.89.0 vendor --locked --offline \
  --manifest-path zkvm/methods/guest/Cargo.toml vendor
```

Configuração emitida pelo próprio Cargo e salva apenas no staging como
`/src/.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

SHA-256 da configuração:
`77e9219c27274120197571fd165cbe4121963b5ad3bc0b20b383c86ef0ce6c2b`.

Inventário reproduzido:

| Medida | Resultado |
| --- | ---: |
| Diretórios de crates | 154 |
| `.cargo-checksum.json` | 154 |
| Arquivos | 5.906 |
| Tamanho (`du -sb`) | 113.740.872 bytes |
| SHA-256 dos caminhos ordenados | `b7fde08fbf2c21916868069c6045e753d26ef932b6be34f4e27979900b223bcc` |
| SHA-256 do inventário de conteúdo | `6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335` |

O hash de conteúdo coincide exatamente com o valor exigido pelo gate.

## Dockerfile temporário de diagnóstico

O único Dockerfile acrescentado ao staging foi:

```dockerfile
FROM risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3 AS build

WORKDIR /src
COPY . .
ENV CARGO_NET_OFFLINE=true

RUN cargo +risc0 fetch --locked --offline --target riscv32im-risc0-zkvm-elf --manifest-path zkvm/methods/guest/Cargo.toml
RUN cargo +risc0 metadata --locked --offline --format-version 1 --manifest-path zkvm/methods/guest/Cargo.toml > /tmp/vericode-metadata.json \
    && test -s /tmp/vericode-metadata.json \
    && wc -c /tmp/vericode-metadata.json \
    && sha256sum /tmp/vericode-metadata.json
```

SHA-256 do Dockerfile:
`ef3d93198f9241456333072f3bc2ec3e72b07c12d10eb96894ec3e7a38d2a607`.

Ele reproduz o `FROM`, `WORKDIR`, contexto copiado e fetch relevantes do
Dockerfile upstream, acrescentando somente `--offline` e o metadata de
leitura autorizado. Não contém `cargo build`, `cargo check`, testes ou
comando de proving.

## Execução única no Docker

Comando real:

```text
docker build --pull=false --network=none --progress=plain \
  --file Dockerfile.d1c2b3a .
```

Saída relevante, sem omitir falha (não houve):

```text
#2 [internal] load metadata for docker.io/risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3
#2 DONE 0.0s
#8 [4/5] RUN cargo +risc0 fetch --locked --offline --target riscv32im-risc0-zkvm-elf --manifest-path zkvm/methods/guest/Cargo.toml
#8 DONE 1.0s
#9 [5/5] RUN cargo +risc0 metadata --locked --offline --format-version 1 --manifest-path zkvm/methods/guest/Cargo.toml ...
#9 0.443 652545 /tmp/vericode-metadata.json
#9 0.450 7274178febd20364c33b76d89dad4b315bbc144b3534dd23f66a6242e1c95f46  /tmp/vericode-metadata.json
#9 DONE 0.5s
#10 naming to moby-dangling@sha256:413ea0353676c4bdf235d3882c83e3c39b5cd65b0514a09da0d9174e3c9091bd 0.0s done
#10 DONE 4.5s
```

Resultado do comando: exit `0`. A saída final é deliberadamente sem tag;
nenhuma imagem foi nomeada, salva ou enviada. O contexto transferido tinha
114,26 MB. `fetch` não apareceu como `CACHED`, portanto a validação foi
executada neste build.

## Interpretação e limites

O sucesso corrige exclusivamente a posição da configuração e do vendor no
staging. Ele demonstra que o Cargo iniciado em `/src` encontra
`/src/.cargo/config.toml`, substitui crates.io por `/src/vendor` e resolve o
lock guest sem rede.

Não foram executados:

- `cargo build`, `cargo risczero build`, `cargo check` ou testes;
- compilação guest ou host;
- geração de `target/`, ELF, ImageID, receipt, seal ou prova;
- RISC0 dev mode, Bonsai ou proving remoto;
- registry, Docker pull, tag, save ou push;
- instalação ou atualização de ferramenta;
- Solana, Anchor, Router, CPI, wallet, keypair, `.env`, validator, transação
  ou deploy;
- commit ou push Git.

Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`. Receipt VeriCode e
Groth16 continuam não comprovadas.

## Riscos e próximo gate

O vendor raiz ainda é um mecanismo temporário de staging; ele não faz parte
do produto nem está versionado. O build determinístico A/B, ELF e ImageID
ainda não foram executados. A dependência transitiva já registrada nos locks
locais permanece inalterada.

D1c2b pode retomar pelo gate de build determinístico do guest, recriando o
mesmo staging raiz vendorizado e preservando os dois locks. Execução do host,
receipts `PASS`/`FAIL` e proving continuam gates posteriores e não foram
autorizados nesta execução.
