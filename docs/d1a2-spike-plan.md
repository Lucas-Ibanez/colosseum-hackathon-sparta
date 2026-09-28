# D1a.2 — protocolo reproduzível do spike de compatibilidade

Data da pesquisa: 2026-09-28.

## Decisão atual

**PENDENTE — protocolo definido, instalações não autorizadas.**

Este documento transforma as incertezas da D1a.1 em um experimento futuro
reproduzível. Ele não aprova um Perfil A, não autoriza instalar toolchains e
não valida Router, CPI, localnet, devnet ou mainnet-beta.

Todos os comandos deste documento estão marcados como **NÃO EXECUTADOS**.
Nesta etapa foram executados somente comandos de inspeção do ambiente, leitura
de arquivos, consulta de refs/releases oficiais e auditoria de checkouts
temporários dos tags exatos.

## Escopo e invariantes

- O objeto sob teste é o código oficial do tag
  `boundless-xyz/risc0-solana v3.0.0`, com os lockfiles preservados.
- As raias A e B variam a CLI Agave, não reinterpretam versões de crates como
  versões de CLI.
- A raia zkVM é separada das raias Anchor/Agave.
- `main`, `latest` e `stable` não são pins aceitos como evidência reproduzível.
- Falha é resultado do spike. Não atualizar dependências, usar mock, dev mode
  ou trocar versões para produzir sucesso.
- Nenhum comando com wallet, keypair, Program ID novo, deploy, validator,
  airdrop ou transação pertence ao gate sem chaves.
- A lógica de negócio do VeriCode continuará em crate Rust pura; esta tarefa
  não cria essa crate nem qualquer scaffold.

## Revisões exatas

Os refs foram resolvidos por `git ls-remote` e confirmados pela API Git oficial
do GitHub. Nos tags leves, o SHA do ref já é o commit. No tag anotado, objeto
de tag e commit apontado são registrados separadamente.

| Componente | Tag/release | Tipo | Objeto de tag/ref | Commit apontado | Fonte oficial |
| --- | --- | --- | --- | --- | --- |
| RISC Zero | `risc0/risc0 v3.0.3` | leve | `14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6` | `14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6` | [ref](https://api.github.com/repos/risc0/risc0/git/ref/tags/v3.0.3), [release](https://github.com/risc0/risc0/releases/tag/v3.0.3) |
| RISC Zero Solana | `boundless-xyz/risc0-solana v3.0.0` | leve | `ee415935d04a948f27a346b563391900bdad6486` | `ee415935d04a948f27a346b563391900bdad6486` | [ref](https://api.github.com/repos/boundless-xyz/risc0-solana/git/ref/tags/v3.0.0), [release](https://github.com/boundless-xyz/risc0-solana/releases/tag/v3.0.0) |
| Anchor | `otter-sec/anchor v0.31.1` | leve | `47284f8f0b9844c6b83234aa90f556bad00e12ed` | `47284f8f0b9844c6b83234aa90f556bad00e12ed` | [ref](https://api.github.com/repos/otter-sec/anchor/git/ref/tags/v0.31.1), [release](https://github.com/otter-sec/anchor/releases/tag/v0.31.1) |
| Agave | `anza-xyz/agave v2.1.0` | leve | `c1080de464cfb578c301e975f498964b5d5313db` | `c1080de464cfb578c301e975f498964b5d5313db` | [ref](https://api.github.com/repos/anza-xyz/agave/git/ref/tags/v2.1.0), [release](https://github.com/anza-xyz/agave/releases/tag/v2.1.0) |
| Agave | `anza-xyz/agave v2.3.9` | anotado | `f20bef1cce6abb06512b76f2baddc99111980740` | `47647df756f5dd0b3c739cecaa71bcf754af6be8` | [ref](https://api.github.com/repos/anza-xyz/agave/git/ref/tags/v2.3.9), [objeto](https://api.github.com/repos/anza-xyz/agave/git/tags/f20bef1cce6abb06512b76f2baddc99111980740), [release](https://github.com/anza-xyz/agave/releases/tag/v2.3.9) |

## Auditoria das fontes exatas

### Anchor `v0.31.1`

- As [release notes 0.31.0 no tag
  `v0.31.1`](https://github.com/otter-sec/anchor/blob/v0.31.1/docs/content/docs/updates/release-notes/0-31-0.mdx)
  recomendam Solana/Agave `2.1.0` e mostram o instalador
  `https://release.anza.xyz/v2.1.0/install`.
- [`avm/Cargo.toml`](https://github.com/otter-sec/anchor/blob/v0.31.1/avm/Cargo.toml),
  [`lang/Cargo.toml`](https://github.com/otter-sec/anchor/blob/v0.31.1/lang/Cargo.toml)
  e [`spl/Cargo.toml`](https://github.com/otter-sec/anchor/blob/v0.31.1/spl/Cargo.toml)
  declaram `0.31.1`.
- `anchor-lang` declara `solana-program = "2"` e Borsh `0.10.3`; isso não
  fixa a CLI Agave nem garante os patches resolvidos por outro lockfile.
- O tag não contém `rust-toolchain` na raiz. A página de instalação contém um
  exemplo de ambiente com Rust `1.85.0`, mas não o declara como pin ou requisito
  de compatibilidade para Anchor `0.31.1`. Portanto o Rust host da raia A é
  **NÃO DETERMINADO**.
- O [`Cargo.lock`](https://github.com/otter-sec/anchor/blob/v0.31.1/Cargo.lock)
  auditado tem SHA-256
  `eea40a7afa9640b643b7ce5f52167b0b1f518dfb8f1d0748879a86f101908bcf`.

### RISC Zero `v3.0.3`

- [`rust-toolchain.toml`](https://github.com/risc0/risc0/blob/v3.0.3/rust-toolchain.toml)
  fixa o canal host `1.89`. O pin `1.89.0` deste protocolo é a
  concretização experimental registrada pela D1a.1, não uma alegação de que
  o arquivo upstream contém o patch completo.
- [`.github/workflows/main.yml`](https://github.com/risc0/risc0/blob/v3.0.3/.github/workflows/main.yml)
  fixa `RISC0_RUST_TOOLCHAIN_VERSION=1.88.0` e instala esse componente pelo
  `rzup` do próprio checkout.
- [`rzup/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/rzup/Cargo.toml)
  declara `rzup 0.5.1`. O
  [README do rzup](https://github.com/risc0/risc0/blob/v3.0.3/rzup/README.md)
  aceita `rzup install <NAME> <VERSION>`; omitir versão escolheria `latest` e
  não é permitido neste protocolo.
- [`risc0/cargo-risczero/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/risc0/cargo-risczero/Cargo.toml)
  declara `cargo-risczero 3.0.3`.
- Os manifests
  [host](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/Cargo.toml),
  [methods](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/methods/Cargo.toml)
  e [guest](https://github.com/risc0/risc0/blob/v3.0.3/examples/hello-world/methods/guest/Cargo.toml)
  do `hello-world` apontam para `risc0-zkvm` e `risc0-build` do mesmo
  checkout.
- O [README de
  `cargo-risczero`](https://github.com/risc0/risc0/blob/v3.0.3/risc0/cargo-risczero/README.md)
  exige Docker no `PATH` para `cargo risczero build` e associa o build
  containerizado ao ImageID determinístico. A versão exata de Docker é
  **NÃO DETERMINADA**.

### `risc0-solana v3.0.0`

- [`.github/workflows/tests.yml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/.github/workflows/tests.yml)
  instala Agave `2.3.9`, Anchor CLI `0.31.1` e Node major `22`, mas usa a
  action Rust flutuante `risc0/risc0/.github/actions/rustup@main`. O mesmo
  workflow executa `solana-keygen new` antes de `anchor test`; essa sequência
  não pertence ao gate sem chaves.
- O [manifesto do verificador
  Groth16](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/groth_16_verifier/Cargo.toml)
  declara `anchor-lang 0.31.1`, `risc0-zkvm 3.0.3` e
  `solana-bn254 3.0.0`.
- O [lock do
  verificador](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/Cargo.lock)
  resolve `anchor-lang 0.31.1`, `risc0-zkvm 3.0.3`,
  `risc0-groth16 3.0.2` e `solana-program 2.3.0`, além de crates Solana
  em linhas 1.x, 2.2–2.4, 3.0 e 5.0. Isso não equivale à CLI `2.3.9`.
- [`examples/counter/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/Cargo.toml)
  e [`examples/counter/zkvm/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/Cargo.toml)
  são workspaces separados; essa separação não prova ABI ou CPI.
- O [programa
  counter](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/programs/solana-counter/Cargo.toml)
  usa Anchor `0.31.1` e o Router por CPI. A crate
  [`shared`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/shared/Cargo.toml)
  declara Borsh `0.10.3`, resolvido como `0.10.4` nos locks.
- [`examples/counter/zkvm/rust-toolchain.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/rust-toolchain.toml)
  usa `stable`, que não é um pin reproduzível.
- O [manifesto do
  host](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/host/Cargo.toml)
  declara um patch de `curve25519-dalek` por branch
  `rustls-dep-hell`. Nenhum dos três locks auditados contém source `git+`;
  o efeito real desse patch deve ser observado por `cargo metadata --locked`,
  não inferido.
- Os manifests
  [methods](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/methods/Cargo.toml)
  e [guest](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/methods/guest/Cargo.toml)
  declaram `risc0-build 3.0.3` e `risc0-zkvm 3.0.3`.
- O [lock
  zkVM](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/Cargo.lock)
  resolve `anchor-client 0.31.1`, `solana-program 2.3.0`,
  `solana-sdk 2.3.1`, `risc0-build 3.0.3`,
  `risc0-zkvm 3.0.3`, `risc0-groth16 3.0.2` e Borsh
  `0.10.4`/`1.5.7`.

## Baseline imutável dos locks

| Workspace | Lock | SHA-256 auditado |
| --- | --- | --- |
| Verificador | `solana-verifier/Cargo.lock` | `54949aa872bd9d6886eece9c4c964a74f1b42f272abd9a8e1ab89060a4feaf6e` |
| `counter` on-chain | `examples/counter/Cargo.lock` | `49004c7c7dcedce5d1ebccd00a38356b78b5d42491d92124ba3d8b83551fb38a` |
| `counter` zkVM | `examples/counter/zkvm/Cargo.lock` | `ab03b523330c4bd66b8ff29eb81ef862cf6f59e56fd4b821dbd60e5648122c15` |
| RISC Zero root | `risc0/risc0 v3.0.3 Cargo.lock` | `15995a3395e61847c403182721c13c56f8735c8d22ed989dc350c5621d812024` |
| `hello-world` guest | `examples/hello-world/methods/guest/Cargo.lock` | `b7b6d59a0c0f9418cd2ec703867aba6cca35ff25f5c556d526f2de8e2001b762` |

Os três primeiros hashes pertencem ao commit `risc0-solana`
`ee415935d04a948f27a346b563391900bdad6486`; os dois últimos ao commit
`risc0` `14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`. Qualquer mudança
nesses bytes invalida a rodada. `cargo update`, geração de lock novo ou
aceitação automática de atualização são proibidos.

## Matriz das três raias

| Raia | Pins do experimento | O que pode concluir | Estado |
| --- | --- | --- | --- |
| A — recomendação Anchor | Anchor CLI/AVM/crates `0.31.1`; Agave CLI `2.1.0`; Rust host **NÃO DETERMINADO**; locks acima | Se o tag compila/testa nos gates sem chave com a versão oficialmente recomendada pelo Anchor | Bloqueada até pin e autorização do Rust host |
| B — referência `risc0-solana` | Anchor CLI `0.31.1`; Agave CLI `2.3.9`; Rust host candidato `1.89.0` explicitamente pinado para o experimento; locks acima | Se a combinação do workflow pode ser reproduzida sem `@main` nos gates sem chave | Candidata a spike, não perfil aprovado |
| zkVM compartilhada | `risc0/risc0 v3.0.3`; Rust host `1.89.0`; guest `1.88.0`; `rzup 0.5.1`; `cargo-risczero`, `risc0-zkvm` e `risc0-build 3.0.3`; Docker **NÃO DETERMINADO** | Compatibilidade interna host/guest e build/receipt quando Docker for pinado | Bloqueada antes de `cargo risczero build` |

O Rust `1.89.0` da raia B é hipótese controlada para substituir a action
`@main` e alinhar a parte zkVM; não é evidência de compatibilidade com Anchor.
Uma decisão humana deve confirmar esse pin e escolher o Rust da raia A.

Para A e B, os três locks de `risc0-solana` são imutáveis; as crates Anchor e
Solana são resolvidas por esses locks, não “instaladas” globalmente. Para a
raia zkVM, os dois locks de `risc0 v3.0.3` e o lock
`examples/counter/zkvm/Cargo.lock` são imutáveis. Todas as raias usam o
rollback, a evidência obrigatória e os gates humanos definidos ao final deste
documento.

## Protocolo futuro — todos os comandos NÃO EXECUTADOS

### 1. Ambiente descartável e captura inicial

Executar somente após confirmação humana H1, preferencialmente em uma
distribuição WSL descartável ou outro ambiente Linux isolado. O clone canônico
do VeriCode não é diretório de execução do spike.

```sh
# NÃO EXECUTADO
pwd
findmnt -T . -n -o FSTYPE,TARGET
uname -a
git --version
command -v rustc rustup cargo rzup solana anchor avm docker node npm
```

Criar o checkout temporário e verificar o commit, sem branch de trabalho:

```sh
# NÃO EXECUTADO
git clone --depth 1 --branch v3.0.0 \
  https://github.com/boundless-xyz/risc0-solana.git risc0-solana-v3.0.0
git -C risc0-solana-v3.0.0 rev-parse HEAD
git -C risc0-solana-v3.0.0 status --short
```

Resultado obrigatório: HEAD
`ee415935d04a948f27a346b563391900bdad6486` e status limpo.

Capturar os locks antes de qualquer Cargo:

```sh
# NÃO EXECUTADO
sha256sum \
  risc0-solana-v3.0.0/solana-verifier/Cargo.lock \
  risc0-solana-v3.0.0/examples/counter/Cargo.lock \
  risc0-solana-v3.0.0/examples/counter/zkvm/Cargo.lock
```

### 2. Instalação futura da raia A

Pré-requisitos: Rust host A escolhido por evidência e aprovado; ambiente
descartável aprovado; rede e alterações de PATH aprovadas.

```sh
# NÃO EXECUTADO — RUST_HOST_A permanece NÃO DETERMINADO
rustup toolchain install <RUST_HOST_A_EXATO> --profile minimal
sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.0/install)"
cargo +<RUST_HOST_A_EXATO> install --git \
  https://github.com/otter-sec/anchor --tag v0.31.1 --locked avm
avm install 0.31.1
avm use 0.31.1
```

O bootstrap inicial do próprio `rustup` não está determinado pelas fontes
permitidas desta tarefa e exige decisão separada. O placeholder acima não pode
ser executado.

Verificação futura:

```sh
# NÃO EXECUTADO
rustc +<RUST_HOST_A_EXATO> --version --verbose
cargo +<RUST_HOST_A_EXATO> --version --verbose
solana --version
cargo build-sbf --version
avm --version
avm list
anchor --version
```

Sucesso da raia A: versões exatas, metadata/árvores/testes sem chave aprovados,
locks e fontes intactos e nenhum keypair/Program ID novo. Falha: qualquer
desvio de versão, atualização de lock, source flutuante efetiva, geração de
chave, build/teste com erro ou saída não reproduzível.

### 3. Instalação futura da raia B

Pré-requisitos: aprovação explícita do Rust host experimental `1.89.0` e das
mesmas mutações de ambiente da raia A.

Node `22` e Yarn aparecem no workflow somente na etapa de `anchor test`.
Eles não serão instalados para o gate Rust sem chaves; sua necessidade fica
adiada ao gate H5.

```sh
# NÃO EXECUTADO
rustup toolchain install 1.89.0 --profile minimal
sh -c "$(curl -sSfL https://release.anza.xyz/v2.3.9/install)"
cargo +1.89.0 install --git \
  https://github.com/otter-sec/anchor --tag v0.31.1 --locked avm
avm install 0.31.1
avm use 0.31.1
```

Verificação futura:

```sh
# NÃO EXECUTADO
rustc +1.89.0 --version --verbose
cargo +1.89.0 --version --verbose
solana --version
cargo build-sbf --version
avm --version
avm list
anchor --version
```

Sucesso e falha usam os mesmos critérios da raia A. Passar na raia B não
transforma Agave `2.3.9` em recomendação oficial do Anchor.

### 4. Instalação futura da raia zkVM

O checkout oficial deve estar no commit
`14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`.

```sh
# NÃO EXECUTADO
git clone --depth 1 --branch v3.0.3 \
  https://github.com/risc0/risc0.git risc0-v3.0.3
git -C risc0-v3.0.3 rev-parse HEAD
sha256sum risc0-v3.0.3/Cargo.lock \
  risc0-v3.0.3/examples/hello-world/methods/guest/Cargo.lock
rustup toolchain install 1.89.0 --profile minimal \
  --component rust-src,rustfmt
cargo +1.89.0 install --locked --path risc0-v3.0.3/rzup
rzup install rust 1.88.0
rzup install cargo-risczero 3.0.3
```

Verificação futura:

```sh
# NÃO EXECUTADO
rustc +1.89.0 --version --verbose
cargo +1.89.0 --version --verbose
rzup --version
rzup show
cargo risczero --version
docker version
```

Não há comando de instalação Docker neste protocolo: versão, origem, modelo
Docker Engine versus Desktop/WSL e rollback são **NÃO DETERMINADOS**. Até essa
decisão, `cargo risczero build`, ImageID e receipt ficam bloqueados.

Depois de H3, os comandos determinísticos candidatos são:

```sh
# NÃO EXECUTADO — bloqueado até Docker exato ser aprovado
cargo +1.89.0 metadata --locked --format-version 1 \
  --manifest-path risc0-v3.0.3/examples/hello-world/Cargo.toml
cargo risczero build \
  --manifest-path risc0-v3.0.3/examples/hello-world/methods/guest/Cargo.toml
```

O build deverá ser repetido em diretórios de saída limpos e os dois ImageIDs
devem ser idênticos. Sucesso da raia zkVM dentro do limite D1a.2 exige versões
exatas, locks inalterados, dois builds determinísticos e uma receipt oficial
local verificada com o ImageID esperado. Os casos VeriCode PASS/FAIL dependem
de código de produto ainda não autorizado e permanecem bloqueados. Falha
inclui ImageID divergente, dev mode, alteração de lock, versão não pinada ou
receipt oficial não verificável; no gate VeriCode futuro, panic usado como
FAIL também será falha.

### 5. Resolução sem chaves

Cada comando roda primeiro na raia B e depois na A, em cópias limpas separadas.
As saídas completas, exit code, versões e duração devem ser armazenados fora
dos checkouts e identificados por raia.

```sh
# NÃO EXECUTADO — repetir em cada workspace aplicável
cargo metadata --locked --format-version 1
cargo tree --locked --all-targets
cargo test --locked
git diff --exit-code
git status --short
sha256sum Cargo.lock
```

Escopo:

- `solana-verifier`: `cargo metadata`, `cargo tree` e
  `cargo test --locked`; estes testes Rust não devem iniciar validator.
- `examples/counter` e `examples/counter/zkvm`: metadata/tree primeiro.
- Compilação SBF ou guest só entra após uma revisão específica comprovar que o
  comando escolhido não cria keypair/Program ID. `anchor build` não é
  presumido seguro.
- `anchor test` permanece bloqueado porque o workflow oficial prepara keypair
  e o comando pode iniciar validator/deploy.

O source do patch por branch deve aparecer na saída de metadata/tree se for
efetivo. Se `--locked` pedir alteração do lock, a rodada falha; não remover
`--locked`.

### 6. ABI e fronteira entre workspaces

O gate de ABI usa vetores de bytes guardados como evidência do spike, fora do
produto. Deve comparar, byte a byte:

1. discriminadores Anchor das instruções `increment_nonce` e `verify`;
2. instruction data completa, incluindo `Seal`, ImageID e digest do journal;
3. ordem dos account metas, `is_signer`, `is_writable`, executabilidade,
   owner esperado e PDAs;
4. `Seal`: selector de 4 bytes e prova Groth16 de 256 bytes, incluindo a
   negação de `pi_a` feita pelo cliente;
5. journal Borsh produzido pelo guest e consumido on-chain, sob Borsh
   `0.10.4` efetivamente resolvido;
6. digest do journal calculado pelo host e pelo programa;
7. ImageID esperado e claim digest;
8. rejeição de seal, selector, ImageID, journal, account meta, flag e owner
   divergentes.

Fontes exatas do vetor de referência:

- [tipo Borsh compartilhado](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/shared/src/lib.rs);
- [guest](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/methods/guest/src/main.rs);
- [CPI do counter](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/programs/solana-counter/src/lib.rs);
- [codificação do seal](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/verifier_router/src/client.rs);
- [contas e roteamento](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/verifier_router/src/router/mod.rs);
- [verificação Groth16](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/groth_16_verifier/src/lib.rs).

Job, mint e executor são campos do futuro VeriCode, não do `counter`. Seus
testes negativos devem ser especificados agora, mas permanecem
**BLOQUEADOS/NÃO EXECUTADOS** até existir implementação autorizada. Nonce e
account do exemplo não podem ser apresentados como substitutos.

Critério de sucesso de ABI: todos os produtores/consumidores geram bytes
idênticos e todas as mutações negativas são rejeitadas pelo componente que
detém a responsabilidade. Divergência, ambiguidade ou teste impossível é
falha/`NÃO DETERMINADO`, nunca compatibilidade.

## Limite sem wallet/keypair

### Permitido após autorização do spike sem chaves

- resolver refs, commits, manifests e locks;
- `cargo metadata --locked` e `cargo tree --locked`;
- testes Rust que comprovadamente não iniciem validator;
- compilação previamente revisada que comprovadamente não gere keypair;
- serialização, hashing e comparação de bytes em harness temporário;
- verificação local de receipt somente após os pins zkVM/Docker.

### Exige autorização adicional separada

- `solana-keygen`;
- `anchor test` quando criar payer/keypair, iniciar validator ou fazer deploy;
- `anchor deploy` e qualquer deploy implícito;
- validator, airdrop, transação ou RPC de cluster;
- criação de Program ID;
- qualquer teste dependente de wallet.

Se um gate oficial depender de chave, seu estado será
`BLOQUEADO POR AUTORIZAÇÃO`. Não se altera o exemplo para burlar o gate.

## Confirmações humanas

| Gate | Confirmação necessária |
| --- | --- |
| H1 | Autorizar ambiente descartável, downloads e instalação isolada de toolchains. |
| H2 | Escolher e justificar Rust host da raia A; confirmar `1.89.0` como pin experimental da B. |
| H3 | Escolher versão/origem/modelo exatos de Docker antes de qualquer `cargo risczero build`. |
| H4 | Autorizar individualmente comandos de compilação já auditados como sem chave. |
| H5 | Autorizar separadamente qualquer keypair, validator, `anchor test`, deploy, airdrop ou transação; não faz parte do gate atual. |

## Evidência obrigatória por rodada

- ambiente, distribuição, arquitetura e mount;
- URL, tag, objeto de tag quando aplicável e commit;
- `git status --short` inicial/final;
- versões completas de todas as ferramentas;
- hashes dos cinco locks aplicáveis antes/depois;
- comandos, stdout/stderr, exit codes e timestamps;
- `cargo metadata` e `cargo tree` arquivados;
- inventário de arquivos antes/depois para provar ausência de keypair;
- hashes dos vetores ABI, seal, journal, ImageID e artefatos permitidos;
- falhas completas, sem edição ou substituição;
- comparação A versus B com somente a variável declarada alterada.

## Rollback futuro

- Preferir descartar integralmente o ambiente WSL isolado após arquivar a
  evidência; remover uma distribuição WSL é destrutivo e exige autorização
  própria.
- Se forem usados homes isolados, registrar previamente cada diretório de
  Rust/Cargo/RISC Zero/Agave/AVM e remover somente esses diretórios após
  confirmação humana.
- Não executar remoção ampla, não alterar o clone canônico e não remover
  ferramentas compartilhadas com outros projetos.
- Para Docker, rollback permanece **NÃO DETERMINADO** até a escolha entre
  Engine e Desktop/integração WSL.
- Após qualquer rollback, repetir inventário de PATH, processos, mounts e
  arquivos e preservar o log.

## Regras de decisão após o spike

- **GO para propor Perfil A:** uma raia passa todos os gates autorizados com
  pins exatos, locks intactos, bytes ABI idênticos e evidência reproduzível.
  Router/CPI/deployment continuam separados até seus próprios gates.
- **NO-GO:** lock alterado, versão não pinada, geração de chave não autorizada,
  divergência ABI, build/teste incompatível ou resultado irreproduzível.
- **PENDENTE:** falta pin, Docker, autorização, código VeriCode necessário para
  testes negativos ou evidência oficial de deployment.

Decisão D1a.2 atual: **PENDENTE**. Não iniciar D1b.
