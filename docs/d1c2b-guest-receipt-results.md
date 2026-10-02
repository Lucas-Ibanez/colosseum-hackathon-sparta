# D1c2b — guest VeriCode e receipts locais reais

Data da execução: 2026-10-02.

## Decisão

**BLOQUEADO antes da geração do primeiro ELF.**

O build host chegou ao builder Docker oficial fixado por digest, mas o
`cargo +risc0 fetch --locked` executado dentro da imagem falhou em modo
offline porque o registry interno da imagem não contém a entrada de índice de
`borsh`, dependência de `vericode-core`. Pelas restrições deste gate, a rede
de registry não foi habilitada e nenhuma dependência, lock ou versão foi
alterada.

Consequentemente, o segundo build, a execução do host, os receipts `PASS` e
`FAIL` e os testes negativos não foram iniciados. Não existe evidência de ELF,
ImageID ou receipt VeriCode neste gate.

## Preflight e baseline

- clone: `/home/lucas/src/vericode`, filesystem `ext4`, branch `main`;
- HEAD:
  `398e9e0f7b902120749674ed78d8ecbe6b51d91a style(zkvm): remove trailing blank lines`;
- `git status --short`: vazio;
- `git show --check --format=short HEAD`: exit `0`;
- `git status -sb`: `## main...origin/main`;
- `git rev-list --left-right --count origin/main...HEAD`: `0 0`;
- diretório temporário: `/tmp/vericode-d1c2b.ItvJy9`;
- nenhuma variável entre `RISC0_DEV_MODE`, `RISC0_PROVER`,
  `RISC0_EXECUTOR`, `RISC0_SERVER_PATH`, `BONSAI_API_KEY`,
  `BONSAI_API_URL` e `DOCKER_HOST` estava definida no ambiente inicial.

Hashes aferidos antes do build e novamente após a falha:

| Arquivo | SHA-256 |
| --- | --- |
| `zkvm/Cargo.lock` | `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1` |
| `zkvm/methods/guest/Cargo.lock` | `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50` |
| `zkvm/Cargo.toml` | `d09624eec5ecad29e2be51b4f0788f0b3bafaef46412f79023c7db6b5a716284` |
| `zkvm/host/Cargo.toml` | `08e58dcc1e6ab5c29790acbee7d874b865b2bcb8c7fd76d5b8ac2efb52b49536` |
| `zkvm/host/src/main.rs` | `33b07258a75aeb65bbc4bebcda82a219466689d3443b8ebb8a7258e53e0f4e8a` |
| `zkvm/methods/Cargo.toml` | `77a627064b517b480a27d5a0869c944f6ffe0249583c9197a574b1e1a9360b1c` |
| `zkvm/methods/build.rs` | `0edffb0f5f87b95c37b9533e956c45f1a537a763891bda167aa4297a9f4af28c` |
| `zkvm/methods/guest/Cargo.toml` | `a079b4fabcf78d83f7b17b5a5caea8ab77a7e5384ce47d94c9f818665daee54e` |
| `zkvm/methods/guest/src/main.rs` | `e3775fa098ca00a6a07a9a62557b7b977d71c3bef576f4f955a13488610d3bf1` |
| `zkvm/methods/src/lib.rs` | `d6ca30ca0af1b69da188e3b2af494212f072b7bd861e46875a096a7c90c83017` |

## Ferramentas e fontes exatas

O checkout oficial `risc0/risc0 v3.0.3` resolveu para
`14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`. O `--help` real de
`cargo-risczero 3.0.3` confirmou o comando
`cargo risczero build --manifest-path <PATH>`. O código versionado em
`zkvm/methods/build.rs` usa `risc0-build 3.0.3` e
`embed_methods_with_options`, portanto o build da workspace host dispara o
mesmo builder Docker e gera as constantes ELF/ImageID consumidas pelo host.

Versões e caminhos efetivamente usados:

| Item | Versão/caminho |
| --- | --- |
| Cargo host | `cargo 1.89.0 (c24e10642 2025-06-23)` via `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin/cargo` |
| rustup isolado | `rustup 1.29.1 (d95a37b6a 2026-08-13)` via `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin/rustup` |
| Rust guest | `rustc 1.88.0-dev (de85b1d3d 2025-06-26)` |
| cargo-risczero | `3.0.3`, extensão em `homes/zkvm/risc0/extensions/v3.0.3-cargo-risczero-x86_64-unknown-linux-gnu/` |
| rzup | `0.5.1` via `homes/zkvm/cargo/bin/rzup` |
| Cargo registry | `/home/lucas/.local/share/vericode-spikes/d1c2b/cargo` |
| RISC0 home | `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0` |
| Docker | `/usr/bin/docker`, cliente/daemon `29.8.1` |

O fonte de `risc0-zkvm 3.0.3` confirma que
`RISC0_PROVER=local` seleciona `LocalProver` quando a feature `prove` está
ativa. O manifesto host também ativa `disable-dev-mode`. Nenhum proving foi
executado porque o build não chegou a produzir ELF.

## Imagem Docker local

A inspeção ocorreu sem `sudo` e sem pull:

```text
risczero/risc0-guest-builder:r0.1.88.0
id=sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3
size=6765197936
repo_digest=risczero/risc0-guest-builder@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3
```

O build fixou
`RISC0_DOCKER_CONTAINER_TAG=r0.1.88.0@sha256:3e12f71b...d943eb3`.
O log mostrou resolução da imagem em `0.0s`; não houve comando
`docker pull`. O `risc0-build 3.0.3` chama `docker build` sem `--pull`.

## Comandos e falhas reais

O comando-base preservou os locks, o modo offline, o target temporário e a
ausência de dev mode/Bonsai:

```text
env -u RISC0_DEV_MODE -u BONSAI_API_KEY -u BONSAI_API_URL \
    -u RISC0_PROVER -u RISC0_EXECUTOR \
    CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo \
    RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup \
    RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0 \
    PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin \
    CARGO_NET_OFFLINE=true \
    RISC0_DOCKER_CONTAINER_TAG=r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3 \
    CARGO_TARGET_DIR=/tmp/vericode-d1c2b.ItvJy9/target-a \
    cargo +1.89.0 build --locked --offline \
      --manifest-path zkvm/Cargo.toml -p vericode-host
```

### Tentativa de ativação inicial

A primeira invocação omitiu `RISC0_HOME` e parou antes do Docker com exit
`101`:

```text
Risc Zero Rust toolchain not found. Try running `rzup install rust`
```

O fonte exato de `risc0-build 3.0.3` mostrou que `rzup::Rzup::new()` consulta
`RISC0_HOME`; `docs/d1a3-spike-results.md` registra essa variável como parte
da ativação isolada. A repetição adicionou somente esse caminho já instalado.
Nenhuma instalação ou mudança de fonte ocorreu.

### Build A com ambiente completo

A repetição alcançou o builder local, mas o estágio de fetch retornou exit
`101`:

```text
[build 4/5] RUN cargo +risc0 fetch --locked \
  --target riscv32im-risc0-zkvm-elf \
  --manifest-path zkvm/methods/guest/Cargo.toml

error: no matching package named `borsh` found
location searched: crates.io index
required by package `vericode-core v0.1.0 (/src/crates/vericode-core)`
As a reminder, you're using offline mode (--offline)
```

O Dockerfile gerado recebeu `CARGO_NET_OFFLINE=true`, como exigido. A cache
Cargo D1c2b do host contém o registry completo, mas `risc0-build 3.0.3` não a
monta nem a copia para o container; `DockerOptions` oferece apenas raiz de
contexto, variáveis e referência da imagem. A imagem local, por sua vez, não
contém a metadata necessária de `borsh` em seu registry interno.

Desabilitar o modo offline, baixar dentro do container, alterar o lock ou
trocar versões violaria este gate. Criar imagem derivada, vendor ou mecanismo
de montagem também seria uma nova decisão de bootstrap, não uma correção
silenciosa autorizada aqui.

## Artefatos e testes não alcançados

- build A: falhou antes do ELF;
- build B: não iniciado;
- comparação byte a byte: não executada;
- ELF SHA-256/tamanho: não existem;
- ImageID gerado/calculado: não existe;
- journals `PASS`/`FAIL`: não executados;
- receipts locais: não gerados;
- tipo/tamanho/hash de receipt: não existem;
- negativos de ImageID e JobId: não executados;
- proving local: não iniciado.

A busca em `/tmp/vericode-d1c2b.ItvJy9` não encontrou `*.receipt`, ELF nem
binário `vericode-guest`. Existe somente um `methods.rs` intermediário vazio
de `0` bytes no target host parcial; nada sob `/tmp` será versionado.

## Limites preservados

- nenhum arquivo sob `zkvm/` foi editado;
- os dois `Cargo.lock` mantiveram seus hashes;
- nenhuma rede Cargo, Docker pull, instalação ou atualização ocorreu;
- `RISC0_DEV_MODE` permaneceu ausente;
- nenhum Bonsai, token, credencial ou serviço remoto foi usado;
- nenhuma operação Solana, wallet, keypair, `.env`, Program ID, validator,
  airdrop, transação, CPI, deploy ou push ocorreu;
- não há claim de Groth16, Router/CPI ou verificação on-chain.

Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Próximo gate recomendado

Antes de retomar D1c2b, é necessário um gate estreito e explicitamente
autorizado para tornar o registry já fixado disponível **dentro** do builder
Docker sem rede: por exemplo, vendoring temporário auditável ou uma imagem
derivada local com conteúdo público da `CARGO_HOME` D1c2b, sempre preservando
os locks e registrando digest e inventário. Só depois esse mecanismo deve ser
validado com `cargo +risc0 fetch --locked` offline e D1c2b pode recomeçar pelos
dois builds independentes.
