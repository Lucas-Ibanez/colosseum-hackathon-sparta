# D1c2b.3 — build determinístico do guest VeriCode

Data da execução: 2026-10-02.

## Decisão

**BLOQUEADO no build A; build B não executado.**

O staging e o vendor reproduziram integralmente a evidência de D1c2b.2a,
mas o build disparado por `risc0-build 3.0.3` não consumiu a configuração
Cargo localizada em `zkvm/methods/guest/.cargo/config.toml`. O primeiro
comando interno, ainda em modo offline, voltou a procurar `borsh` no índice
crates.io vazio do builder e terminou com exit `101`.

Pela regra de parada do gate, não houve build B, correção de configuração,
tentativa alternativa, execução do host, cálculo de ImageID ou geração de
receipt/prova.

## Preflight

- clone: `/home/lucas/src/vericode`, branch `main`;
- HEAD:
  `398e9e0f7b902120749674ed78d8ecbe6b51d91a` (`style(zkvm): remove trailing blank lines`);
- `git show --check HEAD`: exit `0`;
- alterações iniciais preservadas: somente documentação dos gates D1c2b,
  D1c2b.2 e D1c2b.2a; `git status --short -- zkvm` ficou vazio;
- `git diff --exit-code -- zkvm`: exit `0`.

Locks preservados:

| Arquivo | SHA-256 |
| --- | --- |
| `zkvm/Cargo.lock` | `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1` |
| `zkvm/methods/guest/Cargo.lock` | `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50` |

Hashes dos manifests e fontes `zkvm/` usados:

| Arquivo | SHA-256 |
| --- | --- |
| `zkvm/Cargo.toml` | `d09624eec5ecad29e2be51b4f0788f0b3bafaef46412f79023c7db6b5a716284` |
| `zkvm/host/Cargo.toml` | `08e58dcc1e6ab5c29790acbee7d874b865b2bcb8c7fd76d5b8ac2efb52b49536` |
| `zkvm/host/src/main.rs` | `33b07258a75aeb65bbc4bebcda82a219466689d3443b8ebb8a7258e53e0f4e8a` |
| `zkvm/methods/Cargo.toml` | `77a627064b517b480a27d5a0869c944f6ffe0249583c9197a574b1e1a9360b1c` |
| `zkvm/methods/build.rs` | `0edffb0f5f87b95c37b9533e956c45f1a537a763891bda167aa4297a9f4af28c` |
| `zkvm/methods/src/lib.rs` | `d6ca30ca0af1b69da188e3b2af494212f072b7bd861e46875a096a7c90c83017` |
| `zkvm/methods/guest/Cargo.toml` | `a079b4fabcf78d83f7b17b5a5caea8ab77a7e5384ce47d94c9f818665daee54e` |
| `zkvm/methods/guest/src/main.rs` | `e3775fa098ca00a6a07a9a62557b7b977d71c3bef576f4f955a13488610d3bf1` |

## Staging e vendor

O staging anterior já não existia. Foi recriado em
`/tmp/vericode-d1c2b3.BFAfl6/src` exclusivamente a partir de:

```text
git archive --format=tar \
  --output=/tmp/vericode-d1c2b3.BFAfl6/HEAD.tar HEAD
tar -xf /tmp/vericode-d1c2b3.BFAfl6/HEAD.tar \
  -C /tmp/vericode-d1c2b3.BFAfl6/src
```

SHA-256 de `HEAD.tar`:
`e6cae5c8645f5899902c182885363a988ecc3b4d7562d0ce3334faa30a8b471f`,
igual ao D1c2b.2. A lista de arquivos fora de `vendor/` e da configuração
temporária coincidiu exatamente com `git ls-tree -r --name-only HEAD`; não
havia `.git` nem arquivo de credencial no staging.

Vendor executado em
`/tmp/vericode-d1c2b3.BFAfl6/src/zkvm/methods/guest`:

```text
CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo \
RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup \
RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0 \
CARGO_NET_OFFLINE=true \
cargo +1.89.0 vendor --locked --offline \
  --manifest-path Cargo.toml vendor
```

Resultado: exit `0`. A configuração impressa pelo Cargo foi criada somente
no staging:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

Integridade reproduzida:

| Medida | Resultado |
| --- | --- |
| crates | 154 |
| `.cargo-checksum.json` | 154 |
| arquivos regulares | 5.906 |
| tamanho | 113.740.872 bytes |
| SHA-256 do inventário de paths | `b7fde08fbf2c21916868069c6045e753d26ef932b6be34f4e27979900b223bcc` |
| SHA-256 do inventário de conteúdo | `6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335` |
| SHA-256 da configuração | `77e9219c27274120197571fd165bce4121963b5ad3bc0b20b383c86ef0ce6c2b` |

## Ambiente efetivo

| Item | Valor efetivo |
| --- | --- |
| Cargo host | `cargo 1.89.0 (c24e10642 2025-06-23)` |
| Rust host | `rustc 1.89.0 (29483883e 2025-08-04)` |
| Rust guest instalado | `rustc 1.88.0-dev (de85b1d3d 2025-06-26)` |
| `cargo-risczero` | `3.0.3` |
| `rzup` | `0.5.1` |
| Docker client/daemon | `29.8.1` / `29.8.1` |
| Cargo | `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin/cargo` |
| rustup | `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin/rustup` |
| cargo-risczero | `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin/cargo-risczero` |
| rzup | `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin/rzup` |
| Docker | `/usr/bin/docker` |

Imagem local usada, sem comando de pull:

```text
risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3
```

`docker image inspect` confirmou o mesmo digest e
`Entrypoint=["/bin/sh"]`; não há `Cmd`. `RISC0_DEV_MODE`,
`BONSAI_API_KEY`, `BONSAI_API_URL`, `RISC0_PROVER`, `RISC0_EXECUTOR` e
`RISC0_SERVER_PATH` estavam ausentes e foram explicitamente removidas do
ambiente do build.

## Build A e saída real

Target exclusivo: `/tmp/vericode-d1c2b3-build-a.No4B0N`.

Comando:

```text
env -u RISC0_DEV_MODE -u BONSAI_API_KEY -u BONSAI_API_URL \
  -u RISC0_PROVER -u RISC0_EXECUTOR -u RISC0_SERVER_PATH \
  CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo \
  RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup \
  RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0 \
  CARGO_NET_OFFLINE=true \
  CARGO_TARGET_DIR=/tmp/vericode-d1c2b3-build-a.No4B0N \
  RISC0_DOCKER_CONTAINER_TAG=r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3 \
  PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin \
  cargo +1.89.0 build --locked --offline \
    --manifest-path /tmp/vericode-d1c2b3.BFAfl6/src/zkvm/Cargo.toml \
    -p vericode-host
```

Resultado: exit `101`. Trecho literal determinante:

```text
#5 [build 2/5] WORKDIR /src
#6 [build 3/5] COPY . .
#7 [build 4/5] RUN cargo +risc0 fetch --locked --target riscv32im-risc0-zkvm-elf --manifest-path zkvm/methods/guest/Cargo.toml
#7 1.540 error: no matching package named `borsh` found
#7 1.540 location searched: crates.io index
#7 1.540 required by package `vericode-core v0.1.0 (/src/crates/vericode-core)`
#7 1.540     ... which satisfies path dependency `vericode-core` (locked to 0.1.0) of package `vericode-guest v0.1.0 (/src/zkvm/methods/guest)`
#7 1.540 As a reminder, you're using offline mode (--offline) which can sometimes cause surprising resolution failures
#7 ERROR: process "/bin/sh -c cargo +risc0 fetch --locked --target riscv32im-risc0-zkvm-elf --manifest-path $CARGO_MANIFEST_PATH" did not complete successfully: exit code: 101
```

O `risc0-build 3.0.3` local confirma em `src/docker.rs` que o Dockerfile usa
`WORKDIR /src`, copia o contexto e executa `fetch`/`build` com
`--manifest-path`; sua chamada `docker build` não contém `--pull`. O log
resolveu a imagem local fixada em `0.1s` e não mostrou download. O Cargo
interno permaneceu offline. O gate não afirma isolamento de namespace de rede
do `docker build`, pois essa opção não é exposta pela invocação upstream.

O fato observável é que, nessa invocação a partir de `/src`, a configuração
aninhada ao guest não substituiu crates.io pelo vendor. Isso difere do
D1c2b.2a, no qual o container foi iniciado com o diretório de trabalho
`/src/zkvm/methods/guest` e `fetch`/`metadata` passaram.

## Build B, ELF e ImageID

- build B: **não executado**, conforme regra de parada após falha do build A;
- ELF VeriCode: **não gerado**;
- SHA-256/tamanho do ELF: **não determinados**;
- ImageID emitido/calculado: **não determinado**;
- comparação byte a byte A/B: **não executável**.

Não foi criado o diretório esperado `riscv-guest`. O único `methods.rs`
residual no target A tinha tamanho `0` byte; ele não contém ELF nem ImageID e
permanece somente em `/tmp`.

## Limites preservados

- nenhum arquivo em `zkvm/`, core, schema, manifest ou lock foi alterado;
- nenhum lock foi regenerado ou relaxado;
- nenhuma rede de registry Cargo foi usada: host e builder estavam offline;
- não houve comando de pull, imagem derivada, instalação ou atualização;
- o comando de build do host avançou apenas até a falha do build script; o
  host não foi concluído nem executado;
- não houve `cargo test`, `r0vm`, receipt, proving, seal ou Groth16;
- nenhum ELF, método gerado ou target foi versionado;
- nenhuma operação Solana/Anchor/Router/CPI, wallet, keypair, `.env`,
  validator, transação, deploy, commit ou push;
- Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.

## Risco e próximo gate

D1c2b permanece bloqueado antes do primeiro ELF VeriCode. Um gate separado
deve definir e auditar como tornar a substituição vendorizada visível à
invocação real do `risc0-build` cujo diretório de trabalho é `/src`, sem
alterar locks, versões ou produto. Só depois dessa prova deve D1c2b.3 repetir
os builds A/B. Execução local, receipts e proving continuam fora deste gate.
