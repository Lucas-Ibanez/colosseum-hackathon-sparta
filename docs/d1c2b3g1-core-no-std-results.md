# D1c2b.3g.1 — compatibilidade `no_std` do core

Data: 2026-10-02

## Decisão

**GO** para produzir os dois builds independentes do guest a partir de um
commit auditado.

O primeiro build efetivo revelou que `vericode-core` ativava `std` pelas
features padrão de `borsh` e `sha2`. Isso introduzia um segundo
`panic_impl` ao ligar o binário `riscv32im-risc0-zkvm-elf`, cujo runtime é
`no_std`. A correção mínima torna o core `no_std`, usa `alloc::vec::Vec` e
desativa somente as features padrão dessas duas dependências. Não houve
alteração de schema, serialização, hashing, regra de verdict, lock ou código
em `zkvm/`.

Um probe guest real com a toolchain RISC Zero e as flags oficiais do builder
compilou o core corrigido e gerou um ELF RISC-V. Esse probe valida a correção,
mas não conta como um dos dois builds finais: ele foi executado em staging de
diagnóstico anterior ao commit auditado e ainda não produziu o binário
combinado/ImageID do método.

## Baseline e diff

- clone: `/home/lucas/src/vericode`, branch `main`;
- HEAD inicial do gate:
  `15253772ab0c74fb5a2d8413f400e6ac9b5c11ed`;
- arquivos alterados: `crates/vericode-core/Cargo.toml` e
  `crates/vericode-core/src/lib.rs`;
- `borsh = 0.10.4` e `sha2 = 0.10.9` permanecem fixados nas mesmas versões;
- locks preservados:
  - host/methods: `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
  - guest: `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`.

Alteração funcional de plataforma:

```text
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
borsh = { version = "=0.10.4", default-features = false }
sha2 = { version = "=0.10.9", default-features = false }
```

Os SHA-256 das duas fontes no clone e no staging de diagnóstico coincidiram:

| Arquivo | SHA-256 |
| --- | --- |
| `crates/vericode-core/Cargo.toml` | `2d7845c7a5c0681c49fc9b02a47d1b5dca408119aede97f8719b837b2df42b3e` |
| `crates/vericode-core/src/lib.rs` | `f3c74c2c537c0da8eabd5e288a9879e27aa393dac1890d7049a5908d14af52b1` |

Uma auditoria separada de `cargo tree --locked --offline -e features` no
workspace guest mostrou `borsh` somente com `derive`/`borsh-derive` e `sha2`
somente com `compress` no caminho relevante; nenhuma feature `std` dessas
duas dependências foi ativada pelo core.

## Diagnósticos preservados

### Vendor guest insuficiente para o build script host

A primeira tentativa usou apenas o vendor de 154 crates do lock guest. Ela
falhou antes do Docker porque o build script host necessita também de
dependências exclusivas do lock host, começando por `bincode 1.3.3`. A
interseção dos locks tem 151 pacotes registry; a união tem 461. O diagnóstico
levou à geração offline, com `cargo vendor --sync`, de um vendor temporário
da união exata dos dois locks.

O vendor de união tinha 461 crates/checksums, 23.021 arquivos e 451.175.743
bytes. Todos os 22.560 arquivos declarados nos checksums foram verificados.
Seus hashes canônicos foram:

- paths: `6c473290fb268a36fca5b5a049dbbd8162a01cfaa00f9e21984b6e812909249c`;
- conteúdo: `f3e0096fe561936f40a05e0b9905aa9f61d64e882f66bf8a9acca620f78c2328`.

### Build amplo revelou dois problemas distintos

Um `cargo build` amplo do host, com vendor de união, alcançou o Docker e
confirmou que fetch e compilação guest usavam o lock reconciliado. Ele falhou
por duas causas independentes:

1. o lock host contém uma combinação incompatível de APIs entre
   `risc0-zkvm 3.0.3` e transitivas `risc0-circuit-* 4.0.5`;
2. o guest falhou com `duplicate lang item panic_impl`, pois o core ativava
   `std`.

O primeiro problema não é necessário para compilar isoladamente o pacote
`vericode-methods`, mas permanece risco para o gate posterior de receipts. O
segundo foi corrigido neste gate.

### Testes host não promovidos a sucesso

`cargo +1.85.0 test` não iniciou porque essa toolchain não existe na
`RUSTUP_HOME` prescrita. Não houve sync ou instalação. A tentativa com
`cargo +1.89.0 test --locked --offline` também não compilou: o lock raiz
exige `cfg-if 1.0.3`, cujo archive não existe na cache D1c2b. Nenhuma fonte
adicional foi usada e nenhum lock foi alterado. Os 20 testes D1c2a
permanecem evidência histórica; não são reapresentados como execução deste
gate.

### Probe guest e flags oficiais

Um primeiro probe direto sem as flags do builder falhou corretamente no
backend de `getrandom`. A repetição, em target novo, usou as flags extraídas
do `risc0-build 3.0.3`:

```text
-C passes=lower-atomic
-C link-arg=-Ttext=0x00200800
-C link-arg=--fatal-warnings
-C panic=abort
--cfg getrandom_backend="custom"
```

O comando efetivo declarou todas as homes obrigatórias no processo externo,
fixou a imagem local por digest, proibiu pull e desabilitou a rede do
container:

```text
env \
  CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo \
  RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup \
  RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0 \
  PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin \
  RUSTUP_AUTO_UPDATE=0 CARGO_NET_OFFLINE=true \
  docker run --rm --pull=never --network none \
  -e CARGO_NET_OFFLINE=true -e RUSTUP_AUTO_UPDATE=0 \
  -e 'RUSTFLAGS=<flags oficiais acima>' \
  -v <staging>:/src:ro -v <target>:/target -w /src \
  risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3 \
  -c 'cargo +risc0 build --release --locked --offline \
      --target riscv32im-risc0-zkvm-elf \
      --manifest-path /src/zkvm/methods/guest/Cargo.toml \
      --target-dir /target'
```

Resultado real: exit `0`, `Finished release profile [optimized]` em 24,94 s.

| Propriedade do probe | Resultado |
| --- | --- |
| Formato | ELF32 little-endian, executável RISC-V, estático |
| Tamanho | 147.880 bytes |
| SHA-256 | `3fc668dfd97db52343df267c2148aa49181c813a426a9549782743e09fc6c42d` |
| Entry point | `0x2058e8` |

## Limites e próxima transição

- O probe não substitui os dois builds finais e não fornece ImageID.
- O vendor, staging e target são temporários; nada deles foi copiado ao clone.
- O lock host incompatível continua aberto para o gate de receipts, sem
  autorização implícita para o alterar.
- Não houve rede, pull, instalação, atualização, dev mode, proving, Solana,
  Anchor, wallet, validator, Router/CPI, deploy, front-end ou push.
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

Após auditoria e commit local deste gate, a próxima transição permitida é
exportar duas vezes o novo commit, reproduzir/auditar o vendor de união em
cada contexto, compilar somente `vericode-methods` com targets separados e
comparar os binários combinados e ImageIDs antes de qualquer receipt.
