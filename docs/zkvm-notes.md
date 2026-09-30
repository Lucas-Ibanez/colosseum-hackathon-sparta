# Notas de zkVM

## Estado D1a.3

A raia zkVM de referência é o tag `risc0/risc0 v3.0.3`: Rust host `1.89.0`, Rust guest `1.88.0`, `rzup 0.5.1`, `cargo-risczero 3.0.3`, `risc0-zkvm 3.0.3` e `risc0-build 3.0.3`. Esses números estão registrados em arquivos do tag exato e foram usados como pins do ambiente isolado descrito abaixo.

No D1a.3, Rust host `1.89.0`, guest `1.88.0`, `rzup 0.5.1` e
`cargo-risczero 3.0.3` foram instalados e versionados somente em
`$HOME/.local/share/vericode-spikes/d1a3`. `metadata/tree --locked` do
workspace zkVM passaram sem executar o build script do `counter`. Depois de
Docker validado, o guest oficial `hello-world` foi compilado duas vezes pelo
caminho determinístico e provado/verificado localmente. Não houve instalação
Rust/RISC Zero no perfil normal do Ubuntu.

O host Rust, a toolchain guest RISC-V e a toolchain Solana/Anchor são raias distintas. Não há fonte exigindo que suas versões Rust sejam idênticas, e esta documentação não as força a coincidir.

**Ainda não existe receipt VeriCode**, para PASS ou FAIL. Existe evidência
local do `hello-world` upstream fora do repositório; ela valida a raia zkVM,
mas não substitui guest, ImageID ou `JournalV1` do produto.

## Protocolo D1a.2

O tag leve `risc0/risc0 v3.0.3` resolve diretamente para o commit
`14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`, conforme o
[ref oficial](https://api.github.com/repos/risc0/risc0/git/ref/tags/v3.0.3).

A raia zkVM executada mantém Rust host `1.89.0`, Rust guest `1.88.0`,
`rzup 0.5.1`, `cargo-risczero 3.0.3`, `risc0-zkvm 3.0.3` e
`risc0-build 3.0.3`. O protocolo original está em
[`docs/d1a2-spike-plan.md`](d1a2-spike-plan.md); resultados executados e
comandos ainda pendentes estão separados em
[`docs/d1a3-spike-results.md`](d1a3-spike-results.md).

O D1a.3 instalou, por ação humana, Docker Engine rootful dentro do WSL pelo
repositório APT oficial Ubuntu/Noble, com
`docker-ce 5:29.8.1-1~ubuntu.24.04~noble` e pacotes auxiliares exatos. O
daemon `29.8.1` foi validado e a imagem builder `r0.1.88.0` foi fixada pelo
digest `sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3`.
Dois builds efetivos reproduziram os mesmos bytes e ImageID; detalhes estão em
[`docs/d1a3-spike-results.md`](d1a3-spike-results.md).

## Compatibilidade dentro da raia RISC Zero

| Item | Versão/fato | Evidência no tag exato | Estado |
| --- | --- | --- | --- |
| Rust host | `1.89.0` | `risc0/risc0 v3.0.3`, [`rust-toolchain.toml`](https://github.com/risc0/risc0/blob/v3.0.3/rust-toolchain.toml) | manifest + execução isolada |
| Rust guest | `1.88.0-dev (de85b1d3d 2025-06-26)` | `risc0/risc0 v3.0.3`, [`.github/workflows/main.yml`](https://github.com/risc0/risc0/blob/v3.0.3/.github/workflows/main.yml) | manifest + execução isolada |
| `rzup` | `0.5.1` | `risc0/risc0 v3.0.3`, [`rzup/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/rzup/Cargo.toml) | compilado do checkout exato com `--locked` |
| `cargo-risczero` | `3.0.3` | `risc0/risc0 v3.0.3`, [`risc0/cargo-risczero/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/risc0/cargo-risczero/Cargo.toml) | instalado e usado em dois builds efetivos |
| SDK | `risc0-zkvm 3.0.3`; `risc0-build 3.0.3` | manifests, locks, `cargo tree --locked`, build e receipt do `hello-world` | resolução e execução local verificadas |

O exemplo `hello-world` do tag `risc0/risc0 v3.0.3` é a referência mínima para host/guest, journal e receipt. Seu [`Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/Cargo.toml) aponta para `risc0-zkvm` do mesmo workspace; [`methods/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/methods/Cargo.toml) aponta para `risc0-build`; e [`methods/guest/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/methods/guest/Cargo.toml) aponta para o mesmo `risc0-zkvm`. Assim, exemplo, crates, `cargo-risczero` e toolchain guest estão rastreados no mesmo tag upstream `v3.0.3`. O exemplo foi construído e provado localmente após Docker, sem dev mode. O `counter` do tag `risc0-solana v3.0.0` é a referência adicional para a fronteira Solana e conserva as limitações descritas abaixo.

## Auditoria do exemplo `counter`

No tag exato `boundless-xyz/risc0-solana v3.0.0`:

- [`zkvm/host/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/host/Cargo.toml) declara `risc0-zkvm 3.0.3` com `prove`, `anchor-client 0.31.1` e um patch Git de `curve25519-dalek` por **branch**, não por commit;
- [`zkvm/methods/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/methods/Cargo.toml) declara `risc0-build 3.0.3`;
- [`zkvm/methods/guest/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/methods/guest/Cargo.toml) declara `risc0-zkvm 3.0.3`;
- [`zkvm/Cargo.lock`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/Cargo.lock) resolve `risc0-zkvm 3.0.3`, `risc0-build 3.0.3`, `risc0-groth16 3.0.2`, `anchor-client 0.31.1`, `solana-program 2.3.0` e `solana-sdk 2.3.1`;
- [`zkvm/rust-toolchain.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/rust-toolchain.toml) usa `channel = "stable"`, sem versão exata.

Consequência: os crates RISC Zero `3.0.3` do exemplo estão demonstrados pelos
manifests/lock, mas a toolchain Rust usada pelo próprio exemplo não está
pinada. A reprodução D1a.3 substituiu `stable` por `+1.85.0`/`+1.89.0` na
linha de comando, sem alterar manifests. Cargo também confirmou que o
`[patch]` no manifesto `host` é ignorado por não estar na raiz do workspace;
portanto o patch Git por branch não foi efetivo na resolução observada. A
action `risc0/risc0@main` continua inadequada como evidência pinada.

## Docker e build determinístico

O README de `cargo-risczero` no tag `v3.0.3` exige Docker disponível no `PATH` para `cargo risczero build` e explica que esse build containerizado produz o ImageID determinístico.

Portanto:

- Docker Engine `29.8.1` foi instalado e validado antes do primeiro build;
- a imagem builder foi fixada por digest, não por tag flutuante;
- dois builds efetivos, em targets/checkouts separados, produziram ELF
  byte a byte idêntico e ImageID igual;
- Docker continua **obrigatório antes de qualquer futuro build determinístico
  do guest VeriCode**, não é opcional posterior.

Fonte: [`risc0/cargo-risczero/README.md` no tag `v3.0.3`](https://github.com/risc0/risc0/blob/v3.0.3/risc0/cargo-risczero/README.md).

## ImageID

- ImageID do `hello-world` determinístico:
  `ab2f61e0cc5244ee8ee0712a697251be92a2c6539faddf300ab56414dc69149d`.
- SHA-256 do ELF repetido:
  `383b3e63ffd0387ad20c0dee3fa78cc9da25ee164773e59814f02312b5d3bb3f`.
- ImageID do VeriCode: não gerado; o valor acima é somente evidência da
  toolchain e não pode ser reutilizado pelo produto.
- O ImageID identifica o guest; não substitui `harness_hash`, `spec_hash`, `artifact_hash` ou Program ID Solana.

O build nativo disparado por `embed_methods()` produziu outro ELF/ImageID;
por isso seu receipt em memória foi tratado apenas como controle. O receipt
promovido como evidência do spike foi gerado diretamente a partir do ELF
Docker e verificado com `RISC0_DEV_MODE=0`.

## Receipt PASS/FAIL

| Caso | Status | Evidência necessária |
| --- | --- | --- |
| `hello-world` upstream | VERIFICADO LOCALMENTE | Receipt real do ELF Docker verificado contra o ImageID; journal `391`; ImageID e journal adulterados rejeitados. |
| `PASS` | NÃO EXECUTADO | Receipt real verificada localmente com journal `JournalV1` e ImageID esperado. |
| `FAIL` | NÃO EXECUTADO | Receipt real verificada localmente contendo `Verdict::Fail`, sem panic/assert. |

Dev mode não satisfaz esses gates. Falha de execução não pode ser apresentada como receipt `FAIL`.

## Fontes oficiais consultadas

- [zkVM Quick Start](https://dev.risczero.com/api/zkvm/quickstart)
- [Building zkVM Hello World](https://dev.risczero.com/api/zkvm/tutorials/hello-world)
- [Receipts 101](https://dev.risczero.com/api/zkvm/receipts)
- [Terminologia: Image ID, Journal e Receipt](https://dev.risczero.com/terminology)
- [`risc0/risc0 v3.0.3`](https://github.com/risc0/risc0/tree/v3.0.3)
- [`examples/hello-world` em `v3.0.3`](https://github.com/risc0/risc0/tree/v3.0.3/examples/hello-world)
- [`risc0-solana v3.0.0`, exemplo `counter`](https://github.com/boundless-xyz/risc0-solana/tree/v3.0.0/examples/counter)
