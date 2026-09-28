# Notas de zkVM

## Estado D1a.1

A raia zkVM de referência é o tag `risc0/risc0 v3.0.3`: Rust host `1.89.0`, Rust guest `1.88.0`, `rzup 0.5.1`, `cargo-risczero 3.0.3`, `risc0-zkvm 3.0.3` e `risc0-build 3.0.3`. Esses números estão registrados em arquivos do tag exato; não foram instalados nem executados localmente.

O host Rust, a toolchain guest RISC-V e a toolchain Solana/Anchor são raias distintas. Não há fonte exigindo que suas versões Rust sejam idênticas, e esta documentação não as força a coincidir.

**Ainda não existe receipt VeriCode**, para PASS ou FAIL. Também não existe build do guest, ImageID calculado ou prova local neste repositório.

## Protocolo D1a.2

O tag leve `risc0/risc0 v3.0.3` resolve diretamente para o commit
`14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`, conforme o
[ref oficial](https://api.github.com/repos/risc0/risc0/git/ref/tags/v3.0.3).

A raia zkVM futura mantém Rust host `1.89.0`, Rust guest `1.88.0`,
`rzup 0.5.1`, `cargo-risczero 3.0.3`, `risc0-zkvm 3.0.3` e
`risc0-build 3.0.3`. Os comandos de checkout, instalação e verificação estão
em [`docs/d1a2-spike-plan.md`](d1a2-spike-plan.md) e estão todos marcados
como `NÃO EXECUTADOS`.

`cargo risczero build` continua bloqueado: a fonte oficial demonstra que
Docker é necessário, mas versão, origem e modelo Engine versus Desktop/WSL
permanecem `NÃO DETERMINADOS`. Nenhum ImageID ou receipt pode ser promovido
como evidência antes desse pin e de dois builds determinísticos reproduzíveis.

## Compatibilidade dentro da raia RISC Zero

| Item | Versão/fato | Evidência no tag exato | Estado |
| --- | --- | --- | --- |
| Rust host | `1.89.0` | `risc0/risc0 v3.0.3`, [`rust-toolchain.toml`](https://github.com/risc0/risc0/blob/v3.0.3/rust-toolchain.toml) | verificado por manifest |
| Rust guest | `1.88.0` | `risc0/risc0 v3.0.3`, [`.github/workflows/main.yml`](https://github.com/risc0/risc0/blob/v3.0.3/.github/workflows/main.yml) | verificado por manifest |
| `rzup` | `0.5.1` | `risc0/risc0 v3.0.3`, [`rzup/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/rzup/Cargo.toml) | verificado por manifest |
| `cargo-risczero` | `3.0.3` | `risc0/risc0 v3.0.3`, [`risc0/cargo-risczero/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/risc0/cargo-risczero/Cargo.toml) | verificado por manifest |
| SDK | `risc0-zkvm 3.0.3`; `risc0-build 3.0.3` | manifests e lock do `counter` listados abaixo | verificado por manifest |

O exemplo `hello-world` do tag `risc0/risc0 v3.0.3` é a referência mínima para host/guest, journal e receipt. Seu [`Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/Cargo.toml) aponta para `risc0-zkvm` do mesmo workspace; [`methods/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/methods/Cargo.toml) aponta para `risc0-build`; e [`methods/guest/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/methods/guest/Cargo.toml) aponta para o mesmo `risc0-zkvm`. Assim, exemplo, crates, `cargo-risczero` e toolchain guest estão rastreados no mesmo tag upstream `v3.0.3`. Isso é compatibilidade de release por manifest/CI, não execução local. O `counter` do tag `risc0-solana v3.0.0` é a referência adicional para a fronteira Solana e conserva as limitações descritas abaixo.

## Auditoria do exemplo `counter`

No tag exato `boundless-xyz/risc0-solana v3.0.0`:

- [`zkvm/host/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/host/Cargo.toml) declara `risc0-zkvm 3.0.3` com `prove`, `anchor-client 0.31.1` e um patch Git de `curve25519-dalek` por **branch**, não por commit;
- [`zkvm/methods/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/methods/Cargo.toml) declara `risc0-build 3.0.3`;
- [`zkvm/methods/guest/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/methods/guest/Cargo.toml) declara `risc0-zkvm 3.0.3`;
- [`zkvm/Cargo.lock`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/Cargo.lock) resolve `risc0-zkvm 3.0.3`, `risc0-build 3.0.3`, `risc0-groth16 3.0.2`, `anchor-client 0.31.1`, `solana-program 2.3.0` e `solana-sdk 2.3.1`;
- [`zkvm/rust-toolchain.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/rust-toolchain.toml) usa `channel = "stable"`, sem versão exata.

Consequência: os crates RISC Zero `3.0.3` do exemplo estão demonstrados pelos manifests/lock, mas a toolchain Rust usada pelo próprio exemplo não está pinada. A CI do tag também chama uma action Rust de `risc0/risc0@main`; `main` não é evidência de uma release pinada. O próximo spike precisa registrar e fixar a toolchain efetiva antes de alegar reprodução.

## Docker e build determinístico

O README de `cargo-risczero` no tag `v3.0.3` exige Docker disponível no `PATH` para `cargo risczero build` e explica que esse build containerizado produz o ImageID determinístico.

Portanto:

- Docker não será instalado nesta correção;
- uma versão exata ainda precisa ser escolhida;
- Docker é **obrigatório antes do primeiro `cargo risczero build` e antes do gate de receipt**, não um opcional posterior;
- sem Docker e sem build determinístico, não há ImageID VeriCode aceitável para o gate.

Fonte: [`risc0/cargo-risczero/README.md` no tag `v3.0.3`](https://github.com/risc0/risc0/blob/v3.0.3/risc0/cargo-risczero/README.md).

## ImageID

- ImageID do VeriCode: não gerado.
- Gate futuro: build determinístico do guest, repetição do comando, comparação do ImageID e registro da saída real.
- O ImageID identifica o guest; não substitui `harness_hash`, `spec_hash`, `artifact_hash` ou Program ID Solana.

## Receipt PASS/FAIL

| Caso | Status | Evidência necessária |
| --- | --- | --- |
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
