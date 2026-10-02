# D1c2b.1 — bootstrap controlado do cache Cargo zkVM

Data da execução: 2026-10-01.

## Decisão

**GO restrito para retomar D1c2b.** O bloqueio de resolução foi removido: os
locks separados da workspace host/methods e da workspace guest foram gerados
com Rust/Cargo `1.89.0`, todas as crates públicas fixadas foram materializadas
em uma nova `CARGO_HOME`, e `cargo metadata --locked --offline` e
`cargo tree --locked --offline` passaram nas duas workspaces.

Esse GO cobre somente a disponibilidade reproduzível das dependências. Nenhum
guest foi compilado e nenhuma compatibilidade de execução, ELF, ImageID,
receipt ou prova foi demonstrada. Há deriva transitiva em relação aos locks do
tag upstream; os locks deste gate passam a ser a baseline local que deverá ser
preservada no próximo gate.

## Pré-condições e integridade

- clone: `/home/lucas/src/vericode`, filesystem `ext4`, branch `main`;
- HEAD inicial: `a241cdc7eddea4ed7fe608ac84ec72ff83c20f4c` (`feat(core): add deterministic harness and journal wire candidate`);
- estado inicial: somente `?? zkvm/`, preservado como trabalho interrompido do D1c2b;
- nenhum arquivo já existente sob `zkvm/` foi editado;
- foram criados apenas `zkvm/Cargo.lock` e
  `zkvm/methods/guest/Cargo.lock` dentro dessa árvore;
- os oito manifests/fontes preexistentes foram aferidos antes e depois da
  resolução; seus hashes permaneceram iguais.

Versões diretas auditadas nos manifests locais:

| Manifest | Dependência direta |
| --- | --- |
| `zkvm/host/Cargo.toml` | `bincode = =1.3.3`; `risc0-zkvm = =3.0.3` com `disable-dev-mode` e `prove` |
| `zkvm/methods/Cargo.toml` | `risc0-build = =3.0.3` |
| `zkvm/methods/guest/Cargo.toml` | `risc0-zkvm = =3.0.3`, sem features default |
| `crates/vericode-core/Cargo.toml` | `borsh = =0.10.4`; `sha2 = =0.10.9` |

Os demais vínculos são paths locais. Não há dependência Git, branch, `main` ou
versão alternativa nos manifests ou nos locks gerados.

## Diagnóstico do bloqueio

A `CARGO_HOME` zkVM original não continha a entrada sparse necessária para
`risc0-zkvm 3.0.3`; por isso a tentativa D1c2b anterior falhou antes da
compilação com `no matching package named risc0-zkvm found`.

A raia A continha cópias públicas de registry para `risc0-zkvm 3.0.3` e
`risc0-build 3.0.3`, incluindo index, archive e fonte. Os SHA-256 observados
dos archives, iguais aos checksums do registry, foram:

- `risc0-zkvm-3.0.3.crate`:
  `3fcce11648a9ff60b8e7af2f0ce7fbf8d25275ab6d414cc91b9da69ee75bc978`;
- `risc0-build-3.0.3.crate`:
  `1bbb512d728e011d03ce0958ca7954624ee13a215bcafd859623b3c63b2a3f60`.

Foi semeado na nova home somente o subconjunto público de `registry/index`
necessário para as dependências diretas/imediatas auditadas, mais o
`config.json` do mesmo índice. Nenhum archive ou source foi copiado na
semente inicial. Essa primeira resolução offline avançou até a dependência
host condicional `bonsai-sdk`, ausente no índice da raia A, e falhou
exatamente com:

```text
error: no matching package named `bonsai-sdk` found
location searched: crates.io index
required by package `risc0-zkvm v3.0.3`
```

Isso isolou a causa: faltava metadata pública do registry; não faltava nem foi
alterada uma toolchain.

## CARGO_HOME isolada e rede

A home persistente deste gate é:

```text
/home/lucas/.local/share/vericode-spikes/d1c2b/cargo
```

A execução usou a toolchain zkVM D1a.3 somente pelos proxies/binários já
instalados, com `RUSTUP_HOME` da raia zkVM e `CARGO_HOME` nova. A home Cargo da
raia A nunca foi usada para executar Cargo; serviu apenas como fonte de leitura
e da semente pública de index.

A autorização de rede foi usada uma vez para completar o índice sparse e uma
vez para buscar os archives fixados:

- `index.crates.io`: completar metadata pública, incluindo `bonsai-sdk`;
- `static.crates.io`: baixar os arquivos `.crate` definidos pelos locks.

O `config.json` do índice declara `https://static.crates.io/crates` como
endpoint de download e `https://crates.io` como API. Nenhum uso da API foi
necessário pelos comandos executados. Não houve acesso Git nem a outro
registry.

O inventário final da nova home contém 422 arquivos de index, 458 archives e
458 diretórios de source, totalizando `596M`. Não há `.git`, `bin/`,
`config.toml`, credenciais ou tokens. Fora de `registry/`, Cargo criou apenas
seus arquivos internos `.package-cache` (vazio), `.package-cache-mutate`
(vazio) e `.global-cache` (126976 bytes); nenhum deles foi copiado da raia A.

## Resolução e locks

Rust/Cargo efetivos:

```text
cargo 1.89.0 (c24e10642 2025-06-23)
rustc 1.89.0 (29483883e 2025-08-04)
host: x86_64-unknown-linux-gnu
```

A primeira resolução online escolheu `ruint 1.20.1`, que declara Rust 1.90,
portanto esse lock intermediário não foi mantido. A resolução final foi
refeita com `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback`; Cargo
registrou `latest Rust 1.89.0 compatible versions` e escolheu `ruint 1.17.2`.
Nenhuma versão direta foi relaxada.

| Lock local | Saída do Cargo | Pacotes no arquivo | SHA-256 |
| --- | --- | ---: | --- |
| `zkvm/Cargo.lock` | `Locking 459 packages to latest Rust 1.89.0 compatible versions` | 461 | `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1` |
| `zkvm/methods/guest/Cargo.lock` | `Locking 155 packages to latest Rust 1.89.0 compatible versions` | 156 | `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50` |

Pins diretos e checksums no lock local:

| Pacote | Versão | Checksum |
| --- | --- | --- |
| `risc0-zkvm` | `3.0.3` | `3fcce11648a9ff60b8e7af2f0ce7fbf8d25275ab6d414cc91b9da69ee75bc978` |
| `risc0-build` | `3.0.3` | `1bbb512d728e011d03ce0958ca7954624ee13a215bcafd859623b3c63b2a3f60` |
| `bincode` | `1.3.3` | `b1f45e9417d87227c7a56d22e471c6206462cba514c7590c09aff4cf6d1ddcad` |
| `borsh` (VeriCode) | `0.10.4` | `115e54d64eb62cdebad391c19efc9dce4981c690c85a33a12199d99bb9546fee` |
| `sha2` (VeriCode) | `0.10.9` | `a7507d819769d01a365ab707794a4084392c824f54a7a6a7862f8c3d0892b283` |

## Comparação com o tag oficial RISC Zero

O checkout oficial auditado está em
`risc0/risc0 v3.0.3`, commit
`14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`. Os refs leves locais
`v3.0.2` e `v3.0.3` apontam para esse mesmo commit; a referência deste gate é
explicitamente `v3.0.3`. Os locks oficiais comparados foram o lock raiz e
`examples/hello-world/methods/guest/Cargo.lock` desse checkout, com SHA-256
`15995a3395e61847c403182721c13c56f8735c8d22ed989dc350c5621d812024`
e `b7b6d59a0c0f9418cd2ec703867aba6cca35ff25f5c556d526f2de8e2001b762`,
respectivamente.

Não se exige igualdade byte a byte entre workspaces distintos. Os pins
diretos `risc0-zkvm 3.0.3` e `risc0-build 3.0.3` coincidem. As restrições
caret dos crates publicados, porém, resolveram versões transitivas posteriores
às preservadas pelo lock do tag:

| Pacote | Lock local | Lock oficial do tag |
| --- | --- | --- |
| `risc0-binfmt` | `3.0.5` | `3.0.2` |
| `risc0-core` | `3.0.2` | `3.0.0` |
| `risc0-groth16` | `3.0.5` | `3.0.2` |
| `risc0-zkp` | `3.0.5` | `3.0.2` |
| `risc0-zkvm-platform` | `2.2.3` | `2.2.0` |
| `risc0-zkos-v1compat` | `2.2.3` | `2.2.0` |
| `rzup` | `0.5.2` | `0.5.1` |
| `ruint` | `1.17.2` | `1.15.0` |
| Borsh usado pelo SDK | `1.8.1` | `1.5.7` no lock raiz; `1.5.1` no guest hello-world |
| `sha2` | `0.10.9` | `0.10.8` |

Essa deriva não muda os pins diretos nem impede a reprodução offline do lock
local, mas será risco de compilação/execução no D1c2b. Não houve tentativa de
forçar versões transitivas por inferência.

## Comandos executados e resultados

Todos os comandos Cargo abaixo usaram a nova `CARGO_HOME`, o
`RUSTUP_HOME` zkVM D1a.3 e `cargo +1.89.0`.

```text
cargo generate-lockfile --manifest-path zkvm/Cargo.toml
  offline inicial: FAIL, `bonsai-sdk` ausente
  com índice autorizado: 459 pacotes; lock intermediário rejeitado por ruint/Rust 1.90
  offline + resolver fallback: PASS, 459 pacotes compatíveis com Rust 1.89.0

cargo generate-lockfile --manifest-path zkvm/methods/guest/Cargo.toml
  offline + resolver fallback: PASS, 155 pacotes compatíveis com Rust 1.89.0

cargo fetch --locked --manifest-path zkvm/Cargo.toml
  PASS; downloads públicos via static.crates.io
cargo fetch --locked --offline --manifest-path zkvm/methods/guest/Cargo.toml
  PASS; nenhuma rede necessária

cargo metadata --locked --offline --manifest-path zkvm/Cargo.toml
cargo tree --locked --offline --manifest-path zkvm/Cargo.toml
cargo metadata --locked --offline --manifest-path zkvm/methods/guest/Cargo.toml
cargo tree --locked --offline --manifest-path zkvm/methods/guest/Cargo.toml
  quatro comandos: PASS, exit 0
```

Não foram executados `cargo build`, `cargo risczero build`, `cargo test`,
Docker ou scripts de guest.

## Limites e riscos residuais

- nenhuma toolchain ou componente foi instalado, atualizado ou removido;
- nenhum ELF, ImageID, receipt, seal ou prova foi gerado;
- nenhuma wallet, keypair, seed phrase, `.env`, Program ID, validator,
  airdrop, transação, CPI, deploy ou rede Solana foi usada;
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`;
- a compilação futura deve usar exatamente os dois locks e esta cache offline;
- a deriva transitiva acima precisa permanecer visível no relatório D1c2b;
- compatibilidade do guest Rust `1.88.0`, builder Docker, build determinístico,
  ImageID e receipts VeriCode continuam inteiramente não testadas neste gate.

D1c2b pode retomar pelo primeiro build bloqueado por este gate, sem regenerar
locks e sem rede de registry. Qualquer falha de compilação deve ser preservada;
este resultado não autoriza trocar versões.
