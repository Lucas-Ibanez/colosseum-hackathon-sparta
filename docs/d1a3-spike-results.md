# D1a.3 — resultados do spike de compatibilidade

Data de início: 2026-09-28. Conclusão dos gates H1–H4: 2026-09-29.

## Estado

**CONCLUÍDO NO ESCOPO H1–H4 — decisão `PENDENTE`; H5 não autorizado.**

Este documento registra somente saídas reais. Router, CPI e deployments
permanecem `STATUS: NÃO VALIDADO`.

## Baseline do repositório

- clone canônico: `$HOME/src/vericode`;
- filesystem: ext4 montado em `/`, fora de `/mnt/c`;
- branch: `main`;
- commit inicial: `082c568 docs: define D1a.2 compatibility spike protocol`;
- working tree inicial: limpo;
- commit D1a.2: já presente localmente e em `origin/main`;
- `git diff --check`: exit `0`.

## Ambiente isolado antes da instalação

- raiz: `$HOME/.local/share/vericode-spikes/d1a3`;
- capacidade disponível observada em `/home`: `953.3 GiB`;
- orçamento conservador do spike: até `40 GiB`;
- uso após os seis checkouts rasos, antes de qualquer toolchain: `407 MiB`;
- uso observado após toolchains, caches, targets, dois builds efetivos, receipt
  e checkout de repetição: `20 GiB`, dentro do orçamento de `40 GiB`;
- modelo: diretório Linux dedicado, homes separados por raia e PATH somente
  por processo/sessão;
- nenhum arquivo em `/usr/local`, `.bashrc`, `.profile`, Git global ou
  configuração Windows;
- checkouts A e B separados no mesmo commit upstream;
- caches Cargo/rustup separados para impedir que uma raia esconda downloads,
  resolução ou incompatibilidades da outra.

Variáveis reservadas por raia:

| Raia | Variáveis isoladas |
| --- | --- |
| A | `CARGO_HOME`, `RUSTUP_HOME`, `AVM_HOME` sob `homes/lane-a`; binários Agave sob `tools/lane-a` |
| B | `CARGO_HOME`, `RUSTUP_HOME`, `AVM_HOME` sob `homes/lane-b`; binários Agave sob `tools/lane-b` |
| zkVM | `CARGO_HOME`, `RUSTUP_HOME`, `RISC0_HOME` sob `homes/zkvm` |

Diretórios previstos:

- `checkouts/lane-a`, `checkouts/lane-b`, `checkouts/risc0`,
  `checkouts/anchor` e fontes Agave quando necessárias;
- `homes/lane-a`, `homes/lane-b`, `homes/zkvm`;
- `tools/lane-a`, `tools/lane-b`;
- `logs`, `artifacts` e `tmp`.

Checkouts efetivamente criados e verificados:

| Checkout | Revisão observada | Estado |
| --- | --- | --- |
| `lane-a` | `boundless-xyz/risc0-solana` `ee415935d04a948f27a346b563391900bdad6486` | detached, limpo |
| `lane-b` | `boundless-xyz/risc0-solana` `ee415935d04a948f27a346b563391900bdad6486` | detached, limpo |
| `risc0` | `risc0/risc0` `14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6` | detached, limpo |
| `risc0-rebuild` | `risc0/risc0` `14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6` | detached, limpo; diretório vazio extra apenas para invalidar o cache Docker |
| `anchor` | `coral-xyz/anchor` `47284f8f0b9844c6b83234aa90f556bad00e12ed` | detached, limpo |
| `agave-a` | `anza-xyz/agave` `c1080de464cfb578c301e975f498964b5d5313db` | detached, limpo |
| `agave-b` | `anza-xyz/agave` `47647df756f5dd0b3c739cecaa71bcf754af6be8` | detached, limpo |

Os cinco hashes de lock da seção de integridade foram recalculados nos
checkouts e coincidiram com os valores esperados. Os sete working trees
externos foram rechecados e permaneceram limpos. O diretório vazio do
`risc0-rebuild` não aparece no índice Git e não altera bytes de source ou lock.

Inventário antes/depois:

```text
df -h <raiz>
du -sh <raiz>/*
find <raiz> -type f
command -v rustc rustup cargo rzup solana anchor avm docker node npm
```

Rollback: preservar o ambiente ao final para revisão. A remoção futura deve
atingir somente a raiz exata acima, depois de autorização destrutiva separada.
Nenhum rollback automático será executado.

## Escolha Rust da raia A

**Processo escolhido: testar primeiro Rust `1.81.0`; usar `1.85.0` somente
como correção oficial se `1.81.0` falhar por MSRV/compilação.**

Justificativa:

- o tag Anchor `v0.31.1` não contém `rust-toolchain` nem `rust-version`
  global e a release usa `stable`, portanto não fornece um pin reproduzível;
- o `rust-toolchain.toml` de Agave `v2.1.0` fixa `1.81.0`; esse é o menor
  candidato sustentado diretamente pelo tag e o limite inferior escolhido
  para reproduzir a raia, mas ainda não prova que Anchor/AVM/locks compilem;
- a documentação de instalação do próprio commit Anchor mostra `1.85.0` na
  saída do instalador rápido junto de Anchor `0.31.1`, e mostra `1.84.1`
  apenas como saída semelhante na instalação manual genérica; nenhum dos dois
  é declarado MSRV ou pin do tag;
- se `1.81.0` falhar por requisito de compilador, a única correção será testar
  `1.85.0`, pois é a versão demonstrada pelo instalador rápido oficial com o
  trio Anchor `0.31.1`/Agave `2.1.15`; a falha e a correção serão registradas;
- nenhuma versão intermediária será promovida por tentativa sem fonte;
- a raia B permanece em `1.89.0`, permitindo comparação explícita.

O menor compilador efetivamente compatível com os locks dessa raia foi
determinado por `cargo metadata/check/test --locked`, sem atualizar
dependências: `1.81.0` falhou e `1.85.0` passou.

Fontes oficiais:

- Anchor, commit exato, `docs/content/docs/installation.mdx`:
  <https://github.com/coral-xyz/anchor/blob/47284f8f0b9844c6b83234aa90f556bad00e12ed/docs/content/docs/installation.mdx>;
- Agave `v2.1.0`, commit exato, `rust-toolchain.toml`:
  <https://github.com/anza-xyz/agave/blob/c1080de464cfb578c301e975f498964b5d5313db/rust-toolchain.toml>;
- Agave `v2.3.9`, commit exato, `rust-toolchain.toml` (`1.86.0`):
  <https://github.com/anza-xyz/agave/blob/47647df756f5dd0b3c739cecaa71bcf754af6be8/rust-toolchain.toml>.

O instalador oficial `rustup-init 1.29.1` para
`x86_64-unknown-linux-gnu` foi baixado de `static.rust-lang.org` e conferido
contra o SHA-256 oficial
`dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71`.
Foi executado somente com `--no-modify-path --profile minimal` e homes
isolados. Nenhum perfil de shell ou PATH global foi alterado.

Instalações e verificações reais:

- raia A: Rust/Cargo `1.81.0`, seguido do fallback Rust/Cargo `1.85.0`;
  `1.85.0` tornou-se o default somente desse `RUSTUP_HOME` após a falha
  reproduzível de `1.81.0`;
- raia B: Rust/Cargo `1.89.0`;
- raia zkVM: Rust/Cargo host `1.89.0`; o checkout também requisitou o alias
  `1.89` declarado em seu `rust-toolchain.toml`, instalado no mesmo home
  isolado sem substituir o default `1.89.0`;
- AVM `0.31.1` foi compilado com `cargo install --locked --path avm` nas
  raias A e B;
- Anchor CLI `0.31.1` foi compilado por `avm install 0.31.1 --from-source`
  nas duas raias. A saída resolveu explicitamente o commit
  `47284f8f0b9844c6b83234aa90f556bad00e12ed`; o binário direto e o proxy AVM
  reportaram `anchor-cli 0.31.1`;
- os locks do Anchor continham dependências yanked (`crossbeam-channel
  0.5.13` e `spin 0.9.8`), mas foram preservados; nenhuma atualização foi
  feita.

Antes do gate H3, `command -v rustc rustup cargo rzup solana anchor avm
docker node npm` fora dos ambientes isolados não encontrou as toolchains
Linux; somente `npm` interoperável do Windows apareceu em
`/mnt/c/Program Files/nodejs/npm`. Depois da ação humana H3, apenas Docker
passou a existir nativamente no Ubuntu; Rust, RISC Zero, Agave, Anchor e Node
Linux continuam confinados aos homes/prefixos do spike ou ausentes.

## Integridade exigida

| Lock | SHA-256 esperado |
| --- | --- |
| `solana-verifier/Cargo.lock` | `54949aa872bd9d6886eece9c4c964a74f1b42f272abd9a8e1ab89060a4feaf6e` |
| `examples/counter/Cargo.lock` | `49004c7c7dcedce5d1ebccd00a38356b78b5d42491d92124ba3d8b83551fb38a` |
| `examples/counter/zkvm/Cargo.lock` | `ab03b523330c4bd66b8ff29eb81ef862cf6f59e56fd4b821dbd60e5648122c15` |
| `risc0/risc0 v3.0.3 Cargo.lock` | `15995a3395e61847c403182721c13c56f8735c8d22ed989dc350c5621d812024` |
| `examples/hello-world/methods/guest/Cargo.lock` | `b7b6d59a0c0f9418cd2ec703867aba6cca35ff25f5c556d526f2de8e2001b762` |

## Resultados por raia

### Raia A

Resultado: **Rust `1.81.0` rejeitado; fallback Rust `1.85.0` aprovado nos
gates host**, com Anchor/AVM `0.31.1` e Agave `2.1.0`.

- `cargo metadata --locked` e `cargo tree --locked --all-targets` passaram
  com `1.81.0` no `solana-verifier`, mas `cargo test --locked` falhou com
  exit `101`: `risc0-zkvm-platform 2.2.1` usa
  `#[unsafe(no_mangle)]`, ainda experimental nesse compilador;
- após instalar o único fallback previsto, `1.85.0`, `cargo check
  --workspace --locked` e `cargo test --locked` passaram em
  `solana-verifier` e `examples/counter`;
- testes: `groth_16_verifier` 12/12, `test_bad_verifier` 1/1,
  `verifier_router` 1/1 e `solana-counter` 1/1; `shared` não contém testes;
- `metadata/tree --locked` do workspace `examples/counter/zkvm` passaram sem
  executar seu `build.rs`; check/test desse workspace não foram iniciados;
- avisos de APIs depreciadas e `cfg` inesperado foram não fatais e não
  justificam atualização de dependências.

Rust `1.85.0` é um pin experimental sustentado pela execução, não uma
recomendação oficial do Anchor nem prova de CPI/Router.

### Raia B

Resultado: **aprovada nos gates host** com Rust `1.89.0`, Anchor/AVM
`0.31.1` e Agave `2.3.9`.

- `cargo metadata/tree/check/test --locked` passaram em `solana-verifier` e
  `examples/counter`;
- os mesmos 14 testes do verificador e o teste do counter passaram;
- `metadata/tree --locked` do workspace zkVM passaram; check/test não foram
  iniciados para não disparar `embed_methods()` antes de Docker;
- o source tag Agave fixa Rust `1.86.0`; o sucesso com `1.89.0` valida apenas
  a hipótese do spike e não reclassifica essa versão como pin oficial Agave.

### Raia zkVM

Checkout e locks estão verificados. Rust host `1.89.0`, guest `1.88.0`,
`rzup 0.5.1` e `cargo-risczero 3.0.3` foram instalados e verificados apenas no
home zkVM isolado. `rzup show --verbose` também reportou `r0vm 3.0.3`,
fornecido pelo pacote pai `cargo-risczero`; o guest reportou
`rustc 1.88.0-dev (de85b1d3d 2025-06-26)`. Nenhum build guest foi executado
antes de validar Docker.

O source tag confirma `rzup 0.5.1`, `cargo-risczero 3.0.3`, host Rust `1.89`
e guest Rust `1.88.0`. A instalação compilou `rzup` do checkout exato com
`--locked`, sem executar o script flutuante `https://risczero.com/install`.
O próprio `rzup` mapeou Rust guest `1.88.0`
para o release `risc0/rust r0.1.88.0` e `cargo-risczero 3.0.3` para
`risc0/risc0 v3.0.3`.

Checksums oficiais publicados para Linux x86_64:

- `rust-toolchain-x86_64-unknown-linux-gnu.tar.gz` do release
  `r0.1.88.0`:
  `222651797ba58f0bafd959191a48d23b35b3266d9e082fc49fb4d84733064fce`;
- `cargo-risczero-x86_64-unknown-linux-gnu.tgz` do release `v3.0.3`:
  `71a0adaa64235be13e9b8471bea5c4ded64bf97c6bd1eb9c2285b966c0b62fee`.

Fontes oficiais:

- <https://github.com/risc0/risc0/blob/14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6/rust-toolchain.toml>;
- <https://github.com/risc0/risc0/blob/14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6/rzup/Cargo.toml>;
- <https://github.com/risc0/risc0/blob/14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6/rzup/src/components.rs>;
- <https://github.com/risc0/risc0/releases/tag/v3.0.3>;
- <https://github.com/risc0/rust/releases/tag/r0.1.88.0>.

## Docker

**Modelo escolhido: Docker Engine dentro do WSL**, instalado pelo repositório
APT oficial e fixado por versões completas. Motivos: Ubuntu 24.04 é suportado
oficialmente; o WSL desta máquina usa `systemd`; não exige alteração no
Windows nem licença do Docker Desktop; e reproduz diretamente o daemon
esperado por `cargo risczero build`.

Versões observadas no índice oficial `noble/stable/binary-amd64` em
2026-09-28:

| Pacote | Versão exata selecionada |
| --- | --- |
| `docker-ce` | `5:29.8.1-1~ubuntu.24.04~noble` |
| `docker-ce-cli` | `5:29.8.1-1~ubuntu.24.04~noble` |
| `containerd.io` | `2.3.6-1~ubuntu.24.04~noble` |
| `docker-buildx-plugin` | `0.37.1-1~ubuntu.24.04~noble` |
| `docker-compose-plugin` | `5.5.1-1~ubuntu.24.04~noble` |

O SHA-256 observado da chave oficial Docker é
`1500c1f56fa9e26b9b8f42452a553675796ade0807cdce11975eb98170b3a570`.
O procedimento pinado abaixo foi **EXECUTADO PELO USUÁRIO**, pois exigia
`sudo`; não foi executado pelo agente:

```bash
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg \
  -o /etc/apt/keyrings/docker.asc
echo '1500c1f56fa9e26b9b8f42452a553675796ade0807cdce11975eb98170b3a570  /etc/apt/keyrings/docker.asc' \
  | sha256sum -c -
sudo chmod a+r /etc/apt/keyrings/docker.asc
echo 'deb [arch=amd64 signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu noble stable' \
  | sudo tee /etc/apt/sources.list.d/docker.list >/dev/null
sudo apt-get update
sudo apt-get install \
  docker-ce='5:29.8.1-1~ubuntu.24.04~noble' \
  docker-ce-cli='5:29.8.1-1~ubuntu.24.04~noble' \
  containerd.io='2.3.6-1~ubuntu.24.04~noble' \
  docker-buildx-plugin='0.37.1-1~ubuntu.24.04~noble' \
  docker-compose-plugin='5.5.1-1~ubuntu.24.04~noble'
sudo systemctl enable --now docker
sudo usermod -aG docker '<usuario-linux>'
```

`docker-ce-rootless-extras 5:29.8.1-1~ubuntu.24.04~noble` também foi instalado
automaticamente como dependência. `dpkg-query`, depois da instalação,
confirmou as seis versões. O serviço ficou `active` e `enabled`; cliente e
servidor reportaram Docker Engine `29.8.1`, Buildx `0.37.1` e Compose `5.5.1`.
O grupo `docker` contém o usuário Linux esperado e o socket é
`root:docker`, modo `0660`. Como o processo Codex já existia antes da mudança
de grupo, os comandos Docker foram executados com `sg docker`, sem `sudo` e
sem mudança de shell persistente.

A primeira tentativa humana de inclusão no grupo atingiu `root`, não o
usuário da sessão. A correção com o nome Linux explícito foi necessária e
confirmada por `getent group docker`. Isso torna o placeholder explícito no
procedimento parte da evidência, não detalhe cosmético. Pertencer ao grupo
`docker` concede privilégios equivalentes a root e permanece risco aberto.

Rollback futuro, destrutivo e **NÃO EXECUTADO**: parar/desabilitar o serviço,
remover o usuário do grupo, purgar somente os seis pacotes Docker registrados,
remover somente o source/keyring acima e decidir separadamente se
`/var/lib/docker` deve ser preservado ou removido. O ambiente do spike e os
caches foram mantidos para revisão.

Fontes oficiais:

- instalação Ubuntu: <https://docs.docker.com/engine/install/ubuntu/>;
- modo rootless e pré-requisitos: <https://docs.docker.com/engine/security/rootless/>;
- índice de pacotes Noble:
  <https://download.docker.com/linux/ubuntu/dists/noble/stable/binary-amd64/Packages>.

## Build determinístico e receipt local

O código pinado de `risc0-build 3.0.3` seleciona por padrão a imagem
`risczero/risc0-guest-builder:r0.1.88.0`. O pull oficial resolveu para:

```text
risczero/risc0-guest-builder@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3
OS/arquitetura: linux/amd64
```

Os builds usaram o digest explícito, não somente a tag, por meio de
`RISC0_DOCKER_CONTAINER_TAG`. Comando-base executado, com homes e target
absolutos sob a raiz descartável omitidos aqui por portabilidade:

```text
CARGO_TARGET_DIR=<target-da-rodada> \
RISC0_DOCKER_CONTAINER_TAG='r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3' \
cargo risczero build \
  --manifest-path <risc0-v3.0.3>/examples/hello-world/methods/guest/Cargo.toml
```

Resultados:

| Rodada | Cache de compilação | Exit | ImageID | SHA-256/bytes do ELF |
| --- | --- | ---: | --- | --- |
| 1 | `COPY`, `fetch` e `build` executados | `0` | `ab2f61e0cc5244ee8ee0712a697251be92a2c6539faddf300ab56414dc69149d` | `383b3e63ffd0387ad20c0dee3fa78cc9da25ee164773e59814f02312b5d3bb3f`, 152412 bytes |
| 2 | todas as camadas reutilizadas; controle auxiliar, não contado como rebuild independente | `0` | igual | igual |
| 3 | segundo checkout no mesmo commit; diretório vazio no contexto invalidou `COPY` e ambos os `RUN`, que foram reexecutados | `0` | igual | igual; `cmp` confirmou identidade byte a byte |

O segundo checkout continuou com Git limpo, no commit
`14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`, e o lock guest manteve
`b7b6d59a0c0f9418cd2ec703867aba6cca35ff25f5c556d526f2de8e2001b762`.
O `Dockerfile` gerado pelo próprio tag executou `cargo +risc0 fetch --locked`
e `cargo +risc0 build --release --locked`. O aviso de fallback para cálculo
lento do ImageID ocorreu porque `r0vm` não estava no `PATH` interno da
invocação; não foi fatal e o `r0vm 3.0.3 --id` confirmou o mesmo ImageID.

O host oficial `hello-world`, com `RISC0_PROVER=ipc`, `r0vm 3.0.3` e
`RISC0_DEV_MODE=0`, também gerou e verificou um receipt real em memória, com
journal `391` e exit `0`. Esse fluxo usa `embed_methods()` nativo: seu ELF
teve SHA-256
`26cce9de2cb64920c62d229f8a1ad3fd9b99b3279a89581ba4c4b293b92aa11f`
e ImageID
`e73560e9b9f216f50c8e0e21d409c858f5c675874d2c8a88ae9d8d1926604758`,
diferentes do build Docker. Portanto ele foi mantido como controle e não foi
apresentado como receipt do ELF determinístico.

Para ligar o receipt ao artefato Docker, `r0vm 3.0.3` recebeu diretamente o
ELF da rodada 1 e as quatro palavras little-endian equivalentes a
`ExecutorEnv::write(&17u64).write(&23u64)`. Resultado:

- receipt composite local: 209578 bytes, SHA-256
  `eb9c3300d0216e6e9ecfce34aac61343f3dfc14e458d106c1149213490487994`;
- `cargo risczero verify --path <receipt> <ImageID correto>`: exit `0`,
  `Receipt is valid`;
- a mesma verificação com o primeiro byte do ImageID alterado: exit `1`,
  rejeição por claim digest divergente;
- harness temporário compilado contra os `.rlib` do lock oficial: journal
  `u64 = 391`, bytes `8701000000000000`; após alterar um byte público do
  journal, `Receipt::verify` rejeitou por claim digest divergente.

Nenhum desses comandos abriu rede Solana, criou chave, iniciou validator ou
realizou deploy/transação. O receipt e os ELFs permanecem apenas na raiz
descartável, fora do repositório.

Artefatos Agave instalados somente nos prefixos isolados:

- `v2.1.0`: o manifesto oficial resolve o commit
  `c1080de464cfb578c301e975f498964b5d5313db`, mas a API oficial do release
  retorna `digest: null` para o tarball Linux; checksum oficial publicado:
  `NÃO DETERMINADO`. O SHA-256 observado no arquivo baixado foi
  `7fe66a4fde1c0a905b022c4db3f68c399bcf6294abc9d81bdaf61930b3af90e1`;
  `solana --version` reportou `2.1.0`/`c1080de4` e `cargo-build-sbf`
  reportou `2.1.0`, platform-tools `1.43`;
- `v2.3.9`: o tarball
  `solana-release-x86_64-unknown-linux-gnu.tar.bz2` possui digest oficial
  `sha256:0f66dfbdbf766ba02527ff4351c7ace43d97d23b8e6bf9710476f47efbfe97b9`
  e manifesto resolvendo
  `47647df756f5dd0b3c739cecaa71bcf754af6be8`; o arquivo conferiu com esse
  digest, `solana --version` reportou `2.3.9`/`47647df7` e
  `cargo-build-sbf` reportou `2.3.9`, platform-tools `1.48`.

Fontes: releases oficiais
<https://github.com/anza-xyz/agave/releases/tag/v2.1.0> e
<https://github.com/anza-xyz/agave/releases/tag/v2.3.9>, artefatos
`solana-release-x86_64-unknown-linux-gnu.{tar.bz2,yml}`. A ausência de digest
publicado em `v2.1.0` será preservada como risco, não substituída por inferência.

## ABI e serialização

Auditoria estática e comparação executável concluídas no commit
`risc0-solana` `ee415935d04a948f27a346b563391900bdad6486`. Um harness temporário
fora dos checkouts foi compilado diretamente contra os `.rlib` produzidos
pelos locks das raias A e B. Ele não criou crate, lock ou código de produto no
VeriCode e produziu os mesmos bytes nas duas raias.

Constatações sustentadas pelo source tag:

- Anchor `v0.31.1` calcula o discriminador padrão com os primeiros 8 bytes de
  SHA-256 de `global:<nome>`
  (`lang/syn/src/codegen/program/common.rs`); os cálculos independentes deram
  `increment_nonce = 54 95 d1 e9 e4 42 c3 ed` e
  `verify = 85 a1 8d 30 78 c6 58 96`;
- `IncrementNonceArguments` é Borsh `0.10.3`, na ordem
  `account: [u8; 32]` e `nonce: u32`; host, guest e programa importam o mesmo
  crate `shared`, e o guest incrementa o nonce antes de serializar o journal;
- `Seal` serializa `selector: [u8; 4]` seguido de `Proof`; `Proof` contém
  `pi_a[64]`, `pi_b[128]`, `pi_c[64]`, totalizando 260 bytes. O encoder divide
  o seal Groth16 cru 0..64/64..192/192..256 e nega `pi_a`;
- o programa Counter desserializa todo o journal, rejeita nonce diferente de
  `estado + 1`, rejeita account diferente do signer e calcula
  `journal_digest = SHA-256(journal_outputs)` antes da CPI;
- ordem de contas de `IncrementNonce` no source: `program_data` mutável,
  `router`, `router_account`, `verifier_entry` PDA, `prover` signer,
  `verifier_program` unchecked e `system_program`;
- a CPI Router recebe, nessa ordem lógica, `Seal`, `image_id` e
  `journal_digest`; valida selector/PDA, programa executável igual ao
  `VerifierEntry` e estado não interrompido antes de chamar o verificador
  Groth16;
- o verificador incorpora `image_id` e `journal_digest` ao claim e rejeita
  prova/public inputs inválidos.

Vetores inicialmente calculados de forma independente e depois confirmados
pelos tipos Rust/Anchor compilados nas duas raias:

| Vetor | Definição | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| journal | account `00..1f` + nonce `0x01020304` em little-endian | 36 | `2da62609505271023b2bf0cb7232d6698081c33053ba7eded17f94d9f81b559b` |
| `increment_nonce` | discriminador + seal zero + comprimento Borsh + journal | 308 | `a5dfb290eaf95af427d76b26300de833cf930e6e9b0827b1b6b73fb7f9f9e2bb` |
| Router `verify` | discriminador + seal zero + ImageID zero + digest zero | 332 | `7317f2f468481680232d63a18897ddd0675055d9aff76af5e30f38b9daf9972c` |
| Groth16 `verify` | discriminador + proof zero + ImageID zero + digest zero | 328 | `d4799e74c1bf6819ea0b8d2139a9802853f0eab5361e40b931d5752d2e0aecb2` |

O journal completo em hexadecimal é
`000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f04030201`.
O harness confirmou ainda:

- layout do seal `selector[4] + pi_a[64] + pi_b[128] + pi_c[64]` e negação
  efetiva de `pi_a`;
- ordem dos sete account metas do `increment_nonce`;
- flags `(signer, writable)`:
  `[(false,true), (false,false), (false,false), (false,false),
  (true,false), (false,false), (false,false)]`;
- owner de `ProgramData` igual ao ID do `solana-counter`, owners de
  `VerifierRouter`/`VerifierEntry` e ID do tipo Program iguais ao ID declarado
  do Router.

Isso comprova os bytes e metadados gerados pelos crates do exemplo em A e B;
não cria nem valida a ABI futura do VeriCode.

Fontes exatas no tag `v3.0.0`:

- `examples/counter/shared/src/lib.rs`;
- `examples/counter/zkvm/methods/guest/src/main.rs`;
- `examples/counter/zkvm/host/src/main.rs`;
- `examples/counter/programs/solana-counter/src/lib.rs`;
- `solana-verifier/programs/verifier_router/src/{lib.rs,client.rs,router/mod.rs}`;
- `solana-verifier/programs/groth_16_verifier/src/lib.rs`;
- URL-base:
  <https://github.com/boundless-xyz/risc0-solana/tree/v3.0.0>.

**Divergência real:** os IDLs versionados
`solana-verifier/idl/{verifier_router,groth_16_verifier}.json` não correspondem
ao source do mesmo tag. Eles descrevem `verify(proof, image_id,
journal_digest, vk)`, enquanto o source implementa Router
`verify(seal, image_id, journal_digest)` e Groth16
`verify(proof, image_id, journal_digest)`. O IDL Router também referencia uma
seed de argumento `selector` que não aparece na lista de argumentos daquele
IDL. Os discriminadores coincidem, mas o payload não. Até regeneração e
comparação controladas, esses IDLs não podem ser usados como contrato de ABI.

Lacunas preservadas:

- o exemplo oficial valida `account + nonce`, não possui `Job`, mint ou o
  conjunto de campos VeriCode; portanto não demonstra rejeição de
  `Job/mint/executor` divergentes;
- os testes upstream rejeitaram prova e public inputs Groth16 corrompidos; o
  receipt local rejeitou ImageID e journal divergentes, mas não houve conta
  Solana real para exercitar owner/flags durante CPI;
- Router/CPI real, Program IDs por rede e qualquer deployment permanecem
  `STATUS: NÃO VALIDADO`.

## Allowlist H4 sem chaves

A inspeção estática dos manifests, build scripts e testes encontrou:

- `examples/counter/zkvm/host/src/main.rs` cria `Keypair::new()`, solicita
  airdrop e envia transações; o binário host não foi executado;
- o workspace `examples/counter/Cargo.toml` contém apenas `programs/*` e
  `shared`; o workspace zkVM é separado;
- os build scripts de `groth_16_verifier` e `verifier_router` apenas escrevem
  arquivos em `OUT_DIR`/propagam `INITIAL_OWNER`; não criam keypair nem fazem
  RPC;
- `examples/counter/zkvm/methods/build.rs` e
  `examples/hello-world/methods/build.rs` chamam `embed_methods()`; o primeiro
  permaneceu não executado porque seu host cria keypair/RPC, enquanto o
  segundo foi auditado e executado localmente após Docker;
- os testes de `groth_16_verifier` não contêm `Keypair`, RPC,
  `ProgramTest`, validator, airdrop ou transação.

Comandos realmente executados, sempre com homes/targets isolados e locks
preservados:

```text
cargo metadata --locked --format-version 1
cargo tree --locked --all-targets
cargo check --workspace --locked
cargo test --locked
cargo risczero build --manifest-path <hello-world-guest>/Cargo.toml
r0vm --elf <ELF-determinístico> --receipt <receipt>
cargo risczero verify --path <receipt> <ImageID>
```

`metadata/tree` foram executados nos três workspaces de cada raia.
`check/test` foram executados somente em `solana-verifier` e
`examples/counter`. Nenhum `cargo test`, `cargo check` ou binário host do
`counter` foi executado no workspace zkVM. O guest `hello-world` foi
compilado duas vezes de forma efetiva pelo caminho Docker e seu ELF foi
provado/verificado localmente.

**Divergência de resolução observada em ambas as raias:** Cargo avisou que o
`[patch]` declarado em `examples/counter/zkvm/host/Cargo.toml` é ignorado por
estar em pacote não raiz; o workspace efetivo é
`examples/counter/zkvm/Cargo.toml`. Logo, o patch Git por branch não pode ser
tratado como proteção ou correção efetiva nessa execução.

Após todos esses comandos, os cinco SHA-256 dos lockfiles exigidos continuaram
idênticos aos valores iniciais e os sete checkouts externos permaneceram
limpos. O lock adicional do workspace oficial `risc0/examples` também
permaneceu limpo no Git.

## Limites preservados

- nenhum `solana-keygen`, `solana address`, `anchor init`, `anchor test`,
  `anchor deploy`, validator, airdrop ou transação;
- nenhuma wallet, keypair, seed phrase ou `.env` criada/lida; nenhum Program
  ID novo gerado, configurado ou usado em rede;
- nenhum código ou scaffold VeriCode;
- nenhum schema/journal alterado;
- nenhum push.

Uma busca nominal no clone canônico não encontrou `.env`, arquivo com
`keypair` no nome nem conteúdo sob `target/deploy`; nenhum arquivo desse tipo
foi aberto.

## Decisão atual

**PENDENTE**, conforme a regra estrita do D1a.2. H1–H4 foram concluídos: ambas
as raias Rust passaram com locks intactos; Docker foi instalado e validado;
dois builds efetivos produziram ELF/ImageID idênticos; o receipt do ELF Docker
foi produzido, verificado e rejeitou ImageID/journal alterados; os vetores ABI
do exemplo coincidiram entre A e B.

### Perfil A candidato, ainda não promovido

- lado Anchor/Solana: Rust host `1.85.0`, Anchor CLI/AVM/crates `0.31.1` e
  Agave CLI `2.1.0`;
- lado zkVM separado: Rust host `1.89.0`, guest `1.88.0`, `rzup 0.5.1`,
  `cargo-risczero 3.0.3`, `risc0-zkvm/risc0-build 3.0.3` e imagem builder
  pelo digest registrado acima;
- infraestrutura: Docker Engine `29.8.1` no Ubuntu/WSL, locks upstream
  preservados e nenhuma versão compartilhada artificialmente entre os dois
  workspaces.

A raia A é preferida à B porque preserva a recomendação oficial Anchor/Agave
e também passou os gates permitidos. A raia B continua como reprodução da CI
upstream, não como recomendação Anchor. Rust `1.85.0` permanece pin
experimental demonstrado por execução, não MSRV oficial.

O perfil não é promovido a `GO` porque o critério do D1a.2 também exige os
testes negativos de `Job`, mint e executor. Esses campos pertencem ao futuro
`JournalV1` do VeriCode e não existem no exemplo; criá-los seria código de
produto fora do escopo autorizado. Além disso, a divergência IDL/source exige
regeneração controlada, e CPI/owner em runtime, Program ID/cluster e deployment
dependem de H5 ou de autorização posterior. Router permanece
`STATUS: NÃO VALIDADO`; D1b continua bloqueado até decisão humana sobre o
próximo gate de código/ABI.
