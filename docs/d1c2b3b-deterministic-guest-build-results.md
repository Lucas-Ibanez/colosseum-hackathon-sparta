# D1c2b.3b-retry — build guest com Rust isolado

## Decisão

**BLOQUEADO.** O build A alcançou a compilação do guest no builder oficial,
mas terminou com exit `101`: o lock guest fixa `enum-ordinalize 4.4.2` e
`enum-ordinalize-derive 4.4.2`, cujos manifests declaram
`rust-version = "1.89"`, enquanto o builder fornece `rustc 1.88.0-dev`.
Conforme a regra de parada, o build B não foi iniciado. Nenhum ELF ou ImageID
VeriCode foi produzido.

Este resultado não autoriza mudar lock, versão transitiva, toolchain ou imagem.
A reconciliação exige um gate humano separado.

## Origem e integridade

- clone: `/home/lucas/src/vericode`, branch `main`;
- commit exportado: `398e9e0f7b902120749674ed78d8ecbe6b51d91a`;
- `git show --check HEAD`: exit `0`;
- alterações iniciais preservadas: somente documentação D1c2b já em curso;
- diff inicial em `zkvm/`: zero;
- lock da workspace: `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
- lock guest: `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50`.

Staging A, criado exclusivamente por `git archive HEAD`:

- fonte: `/tmp/vericode-d1c2b3b-retry-a.4IMR3I/src`;
- SHA-256 do archive:
  `e6cae5c8645f5899902c182885363a988ecc3b4d7562d0ce3334faa30a8b471f`;
- `.git`: ausente.

## Ambiente isolado

Todo comando Rust/RISC Zero usou explicitamente:

```text
env \
  CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo \
  RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup \
  RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0 \
  PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin \
  RUSTUP_AUTO_UPDATE=0 CARGO_NET_OFFLINE=true <COMANDO>
```

| Ferramenta | Versão observada |
| --- | --- |
| Cargo host | `1.89.0 (c24e10642 2025-06-23)` |
| Rust host | `1.89.0 (29483883e 2025-08-04)` |
| Cargo guest | `1.88.0-dev (873a06493 2025-05-10)` |
| Rust guest | `1.88.0-dev (de85b1d3d 2025-06-26)` |
| rzup | `0.5.1` |
| cargo-risczero | `3.0.3` |

+Cargo, Rustup, cargo-risczero e rzup vieram de
`/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin`;
Docker veio de `/usr/bin/docker`. O `r0vm` oficial existe em
`RISC0_HOME/extensions/v3.0.3-cargo-risczero-x86_64-unknown-linux-gnu/r0vm`,
mas não foi executado porque não houve ELF.

`/home/lucas/.rustup` estava e permaneceu ausente. `RISC0_DEV_MODE`,
variáveis Bonsai e de prover remoto estavam ausentes. Não houve instalação ou
atualização.

## Builder e vendor A

Imagem local fixada:

```text
risczero/risc0-guest-builder:r0.1.88.0
sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3
```

O build usou a tag `r0.1.88.0` fixada pelo digest acima. A imagem foi
resolvida localmente pelo Docker interno de `risc0-build`; não foi observado
pull. O build real não expõe `--network none`, portanto não se alega
isolamento de rede no nível Docker. Cargo permaneceu offline e usou o vendor
raiz.

Com o wrapper, foi executado na raiz do staging:

```text
cargo +1.89.0 vendor --locked --offline \
  --manifest-path zkvm/methods/guest/Cargo.toml vendor
```

Configuração emitida pelo Cargo e gravada somente no staging:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

| Medida | Resultado |
| --- | ---: |
| crates | 154 |
| checksums | 154 |
| arquivos | 5.906 |
| bytes | 113.740.872 |
| hash de conteúdo | `6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335` |
| hash de caminhos | `b7fde08fbf2c21916868069c6045e753d26ef932b6be34f4e27979900b223bcc` |

O hash de conteúdo coincide com D1c2b.3a.

## Build A

Target: `/tmp/vericode-d1c2b3b-retry-target-a.4o8FCy`.

Com o wrapper, target e digest acima, e variáveis de dev mode/Bonsai/prover
explicitamente removidas:

```text
cargo +1.89.0 build --locked --offline \
  --manifest-path /tmp/vericode-d1c2b3b-retry-a.4IMR3I/src/zkvm/Cargo.toml \
  -p vericode-host
```

Resultado: exit `101`. O fetch interno passou; a falha ocorreu em:

```text
cargo +risc0 build --release --locked \
  --target riscv32im-risc0-zkvm-elf \
  --manifest-path zkvm/methods/guest/Cargo.toml
```

+Saída determinante:

```text
error: rustc 1.88.0-dev is not supported by the following packages:
  enum-ordinalize@4.4.2 requires rustc 1.89
  enum-ordinalize-derive@4.4.2 requires rustc 1.89
Either upgrade rustc or select compatible dependency versions with
cargo update <name>@<current-ver> --precise <compatible-ver>
```

O estágio Docker e o processo externo terminaram com exit `101`.
`risc0-build 3.0.3` propagou a falha a partir de `src/lib.rs:784:84`.

Por leitura, o lock fixa:

- `enum-ordinalize 4.4.2`, checksum
  `89dd01549b09589510cf0647475075d12071456586d70f5c75c98ae2a5537677`;
- `enum-ordinalize-derive 4.4.2`, checksum
  `a65863d15a4ce2888bd2f0f543cc963d3879c3a022c8ee43f6141d479a3ac815`.

Ambos os manifests vendorizados declaram Rust mínimo `1.89`.
`educe 0.6.0` introduz `enum-ordinalize`; no lock, `ark-ec 0.5.0`,
`ark-ff 0.5.0` e `ark-poly 0.5.0` dependem de `educe`.

## Build B, ELF e ImageID

O build B não foi iniciado; não foram criados staging ou target B. No target A
restou apenas
`debug/build/vericode-methods-52b8e0047f057c74/out/methods.rs`, vazio.
Não foi encontrado `*.elf` nem binário `vericode-guest`.

| Evidência | Build A | Build B |
| --- | --- | --- |
| build | falhou, exit `101` | não executado |
| ELF e tamanho | não produzidos | não produzidos |
| SHA-256 do ELF | indisponível | indisponível |
| ImageID emitido/calculado | indisponível | indisponível |
| comparação byte a byte | não aplicável | não aplicável |

`r0vm --help` e cálculo de ImageID não foram executados: não havia ELF e o
gate exigia parada imediata.

## Limites e próximo gate

- zero alteração em `zkvm/`, core, schema, manifests e locks;
- nenhum vendor, target, staging ou ELF foi adicionado ao clone;
- nenhum build B, host, teste, proving, receipt, seal ou Groth16;
- nenhum dev mode, Bonsai, credencial ou serviço remoto;
- nenhuma operação Solana/Anchor/Router, wallet, keypair, `.env`, validator,
  transação ou deploy;
- nenhuma instalação, commit ou push;
- Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.

O lock guest atual é incompatível com o Rust guest fixado. D1c2b não pode
avançar para execução local do host ou receipts. O próximo gate deve auditar a
cadeia transitiva e autorizar explicitamente uma reconciliação de lock/MSRV
baseada em fontes oficiais; não se deve repetir o build antes disso.
