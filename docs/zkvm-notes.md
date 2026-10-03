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

## Cache Cargo D1c2b.1

O gate D1c2b.1 criou a `CARGO_HOME` exclusiva
`~/.local/share/vericode-spikes/d1c2b/cargo`. Depois de completar somente o
registry público autorizado, as workspaces host/methods e guest passaram
`cargo metadata --locked --offline` e `cargo tree --locked --offline` com
Rust/Cargo `1.89.0`. Os locks locais preservam os pins diretos
`risc0-zkvm 3.0.3` e `risc0-build 3.0.3`; hashes, checksums, inventário e
comandos estão em
[`docs/d1c2b1-cache-results.md`](d1c2b1-cache-results.md).

As restrições transitivas dos crates publicados resolveram versões posteriores
às registradas nos locks oficiais do tag `v3.0.3`. Isso é uma divergência
explícita de lock, não evidência de compatibilidade. Build guest, ELF, ImageID
e receipt VeriCode continuam não executados.

## Tentativa D1c2b — cache interno do builder insuficiente

Em 2026-10-02, o build VeriCode foi iniciado com Cargo/Rust host `1.89.0`,
`RISC0_HOME` isolado, os dois locks preservados, `CARGO_NET_OFFLINE=true` e a
imagem local fixada por
`r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3`.
Docker alcançou a imagem local sem pull, mas o estágio
`cargo +risc0 fetch --locked` falhou porque o registry interno da imagem não
contém a entrada de índice de `borsh`, requerida por `vericode-core`.

A `CARGO_HOME` criada em D1c2b.1 resolve o host, mas `risc0-build 3.0.3` não
a monta nem copia automaticamente para o container. O gate parou sem
desabilitar o modo offline, sem alterar locks e sem gerar ELF, ImageID ou
receipt. O resultado completo está em
[`docs/d1c2b-guest-receipt-results.md`](d1c2b-guest-receipt-results.md).

D1c2b permanece bloqueado até uma decisão explícita sobre vendoring
temporário ou cache/imagem Docker derivada e auditável. Isso não altera o
estado de Router/CPI/devnet, que continua `STATUS: NÃO VALIDADO`.

## D1c2b.2 — vendor completo, prova no builder bloqueada

O gate D1c2b.2 criou somente em staging temporário um vendor offline das 154
crates de registry do lock guest. As 154 crates possuem
`.cargo-checksum.json`; o inventário tem 5.906 arquivos, 113.740.872 bytes e
SHA-256 de conteúdo
`6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`.
Nenhum vendor ou `.cargo/config.toml` foi adicionado ao clone.

A primeira prova com a imagem fixa, `--network none`, `--pull=never`, staging
somente-leitura e `CARGO_NET_OFFLINE=true` terminou antes do Cargo com exit
`127` e `/bin/sh: 0: Can't open cargo`. Pela regra de parada do gate, não
houve correção da invocação, repetição ou `cargo metadata` no container. O
resultado completo está em
[`docs/d1c2b2-builder-vendor-results.md`](d1c2b2-builder-vendor-results.md).

D1c2b continua bloqueado: o vendor foi demonstrado no host, mas ainda não foi
consumido com sucesso pelo builder. ELF, ImageID e receipt VeriCode continuam
inexistentes, e Router/CPI/devnet permanece `STATUS: NÃO VALIDADO`.

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

## D1c2b.2a — vendor validado dentro do builder

O entrypoint da imagem local é `Entrypoint=["/bin/sh"]`, sem `Cmd` efetivo
(`null`). Com o mesmo staging e vendor, o comando foi passado por `/bin/sh
-c`. `cargo +risc0 fetch --locked --offline` passou com exit `0`, e `cargo
+risc0 metadata --locked --offline` também passou com exit `0`.

O isolamento foi mantido por `--network none`, `CARGO_NET_OFFLINE=true`,
`--pull=never` e mount `/src:ro`. Não houve build, ELF, ImageID, receipt ou
proving. O relatório está em
[`docs/d1c2b2a-builder-entrypoint-results.md`](d1c2b2a-builder-entrypoint-results.md).

D1c2b pode retomar pelo build do guest. Router/CPI/devnet continuam
`STATUS: NÃO VALIDADO`.

## D1c2b.3 — build A bloqueado pela descoberta da configuração Cargo

O staging foi recriado de `git archive HEAD`; o vendor repetiu 154 crates,
154 checksums e o SHA-256 de conteúdo
`6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`.
Mesmo assim, o build A disparado por `risc0-build 3.0.3` falhou no primeiro
`cargo +risc0 fetch --locked` interno com `no matching package named borsh
found`.

A diferença observável em relação ao D1c2b.2a é o diretório de trabalho: o
Dockerfile gerado usa `WORKDIR /src`, enquanto a configuração vendorizada
permanece em `zkvm/methods/guest/.cargo/config.toml`. Nessa invocação real, a
substituição por `vendor/` não foi consumida. O build B não foi iniciado e não
existe ELF, SHA-256 de ELF ou ImageID VeriCode.

Detalhes e saída real:
[`docs/d1c2b3-guest-build-results.md`](d1c2b3-guest-build-results.md).
D1c2b volta a ficar bloqueado antes da compilação guest. Router/CPI/devnet
continuam `STATUS: NÃO VALIDADO`.

## D1c2b.3a — vendor raiz detectado pelo Dockerfile real

Um novo staging criado por `git archive HEAD` colocou o vendor somente na
raiz (`/src/vendor`) e a configuração emitida pelo Cargo em
`/src/.cargo/config.toml`. O inventário reproduziu 154 crates, 154 checksums,
5.906 arquivos e SHA-256 de conteúdo
`6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`.

Uma única execução de `docker build --pull=false --network=none`, sem tag,
reproduziu `WORKDIR /src` e `COPY . .` do Dockerfile gerado por
`risc0-build 3.0.3`. O `cargo +risc0 fetch --locked --offline` passou com
exit `0`; o `cargo +risc0 metadata --locked --offline` também passou e gerou
652.545 bytes, SHA-256
`7274178febd20364c33b76d89dad4b315bbc144b3534dd23f66a6242e1c95f46`.

Isso valida somente a descoberta do vendor no contexto real do builder. Não
houve compilação, ELF, ImageID, receipt ou proving. O relatório completo está
em
[`docs/d1c2b3a-root-vendor-results.md`](d1c2b3a-root-vendor-results.md).
D1c2b pode retomar pelo build determinístico A/B em gate posterior;
Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.

## D1c2b.3c — causa MSRV e candidato oficial de reconciliação

A auditoria locked/offline reconstruiu o caminho do lock guest:
`risc0-zkvm 3.0.3 -> risc0-groth16 3.0.5 -> arkworks 0.5.0 -> educe
0.6.0 -> enum-ordinalize 4.4.2 -> enum-ordinalize-derive 4.4.2`.
`educe` aceita `enum-ordinalize ^4.2`; como o lock foi criado com Rust host
1.89 e o manifesto guest não declara `rust-version`, o resolver escolheu as
duas versões `4.4.2`, ambas com MSRV `1.89`. O builder guest 1.88 não pode
compilá-las.

Os locks dos tags exatos `risc0/risc0 v3.0.3` e
`boundless-xyz/risc0-solana v3.0.0` preservam
`risc0-groth16 3.0.2`, `enum-ordinalize 4.3.0` e derive `4.3.1`. Os dois
archives `enum` declaram MSRV `1.60` e seus SHA-256 coincidem com os checksums
dos locks. O hello-world do tag RISC Zero já foi construído em D1a.3 com o
mesmo builder guest 1.88. Por isso, a decisão é
**CANDIDATO DE RECONCILIAÇÃO COMPROVADO**, não compatibilidade já aplicada.

O lock VeriCode não foi alterado. Um gate posterior deve reconciliar o
conjunto transitivo com os locks oficiais, preservar os pins diretos e repetir
resolução offline, vendor e builds A/B. Até lá não há ELF, ImageID ou receipt
VeriCode; Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`. Detalhes em
[`docs/d1c2b3c-guest-msrv-lock-audit.md`](d1c2b3c-guest-msrv-lock-audit.md).

## D1c2b.3b.1 — remediação do Rustup padrão

Durante o preflight de D1c2b.3b, uma consulta a `cargo +1.89.0` sem
`RUSTUP_HOME` criou acidentalmente a toolchain padrão em
`/home/lucas/.rustup/toolchains/1.89.0-x86_64-unknown-linux-gnu`.
O root continha somente essa toolchain, diretórios vazios de downloads/tmp e
seu `update-hashes`. A toolchain foi desinstalada com o Rustup isolado e
`RUSTUP_AUTO_UPDATE=0`; os diretórios vazios foram removidos com alvos exatos.

Após a remediação, `/home/lucas/.rustup` está ausente, a toolchain D1a.3
continua em `~/.local/share/vericode-spikes/d1a3/homes/zkvm`, e
`/home/lucas/.cargo` não foi tocado. A retomada de D1c2b.3b exige declarar
explicitamente `CARGO_HOME`, `RUSTUP_HOME`, `RISC0_HOME` e `PATH` em todo
comando Cargo/Rust. O build determinístico, ELF, ImageID e receipt VeriCode
continuam não executados.

## D1c2b.3b-retry — build guest bloqueado por MSRV transitivo

A repetição controlada declarou `CARGO_HOME`, `RUSTUP_HOME`, `RISC0_HOME`,
`PATH`, `RUSTUP_AUTO_UPDATE=0` e `CARGO_NET_OFFLINE=true` em todos os
comandos Rust/RISC Zero. `/home/lucas/.rustup` permaneceu ausente. O staging
A e o vendor raiz reproduziram 154 crates e o SHA-256 esperado
`6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`.

O fetch interno passou, mas o build guest falhou com exit `101`: o lock fixa
`enum-ordinalize 4.4.2` e `enum-ordinalize-derive 4.4.2`, cujos manifests
declaram Rust mínimo `1.89`; o builder usa `rustc 1.88.0-dev`. O build B
não foi iniciado. Restou apenas `methods.rs` vazio; não há ELF nem ImageID.

O relatório está em
[`docs/d1c2b3b-deterministic-guest-build-results.md`](d1c2b3b-deterministic-guest-build-results.md).
D1c2b fica bloqueado até um gate explícito de reconciliação lock/MSRV.
Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.

## D1c2b.3d — lock reconciliado, validação offline bloqueada pela cache

O lock guest foi alterado somente no conjunto comprovado pelos tags exatos:
`risc0-groth16 3.0.5 -> 3.0.2`, `enum-ordinalize 4.4.2 -> 4.3.0` e
`enum-ordinalize-derive 4.4.2 -> 4.3.1`. Os checksums coincidem com os locks
oficiais; não houve package adicional ou removido. A referência interna do
derive mudou de `syn 3.0.6` para o `syn 2.0.119` já presente, conforme o
requisito `syn ^2` da versão `4.3.1`.

O novo SHA-256 do lock guest é
`1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
o lock host/methods permaneceu
`c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`.
Os pins diretos `risc0-zkvm = 3.0.3` e `risc0-build = 3.0.3` não mudaram.

A decisão do gate é **BLOQUEADO**: `metadata`, `tree` e as árvores inversas
locked/offline terminaram com exit `101`, pois a `CARGO_HOME` D1c2b contém o
índice, mas não o archive `enum-ordinalize 4.3.0` nem os outros dois archives
reconciliados. Nenhum artefato foi copiado de outra home sem autorização. O
relatório está em
[`docs/d1c2b3d-guest-lock-reconciliation-results.md`](d1c2b3d-guest-lock-reconciliation-results.md).
Não há ELF, ImageID ou receipt VeriCode; Router/CPI/devnet continuam
`STATUS: NÃO VALIDADO`.

## D1c2b.3e — cache guest permanece com um archive ausente

O inventário integral do lock guest encontrou 154 archives de registry. A
cache D1c2b possuía 151 válidos. A única fonte autorizada continha e validou
`enum-ordinalize 4.3.0` e `enum-ordinalize-derive 4.3.1`, que foram copiados
individualmente; ela não contém `risc0-groth16 3.0.2` em nenhum caminho.

A cache destino agora contém 153/154 archives exigidos. Com escrita local na
cache e rede desabilitada, `metadata`, `tree` e as três árvores inversas
falharam com exit `101` exclusivamente porque o archive restante exigiria
download. Nenhuma fonte alternativa foi usada. O resultado está em
[`docs/d1c2b3e-offline-lock-closure-results.md`](d1c2b3e-offline-lock-closure-results.md).

D1c2b está `AGUARDANDO_AUTORIZAÇÃO` para uma fonte local adicional e exata do
único archive, sempre condicionada ao checksum do lock. Não há vendor final,
ELF, ImageID ou receipt VeriCode; Router/CPI/devnet continuam
`STATUS: NÃO VALIDADO`.

## D1c2b.3e-retry — lock guest fechado offline

Com autorização humana explícita, a busca foi ampliada somente para as caches
locais D1a.3 `lane-a/cargo` e `lane-b/cargo`. Três cópias de
`risc0-groth16-3.0.2.crate` foram encontradas; todas tinham 40.149 bytes,
eram idênticas por `cmp` e tinham o SHA-256 do lock
`724285dc79604abfb2d40feaefe3e335420a6b293511661f77d6af62f1f5fae9`.
Uma única cópia de `lane-a` foi feita para a cache D1c2b.

O inventário integral final passou com 154/154 archives válidos, zero
ausentes e zero divergências. `cargo metadata --locked --offline`, `cargo
tree --locked --offline` e as inversas de `risc0-groth16 3.0.2`,
`enum-ordinalize 4.3.0`, derive `4.3.1` e `syn 2.0.119` passaram com exit `0`
no ambiente isolado. Nenhuma rede foi usada, e locks/`zkvm/` permaneceram
inalterados.

O gate está **GO** somente para recriar e auditar o vendor do lock final. Não
há ainda dois builds, ELF, ImageID ou receipt VeriCode. Router/CPI/devnet
continuam `STATUS: NÃO VALIDADO`.

## D1c2b.3f — vendor auditável do lock final

Um staging novo do commit `3c505a8` produziu o vendor raiz locked/offline do
lock guest reconciliado. Ele contém 154 crates, 154 arquivos
`.cargo-checksum.json`, 5.904 arquivos regulares e 113.733.052 bytes. O
inventário de paths tem SHA-256 `dc242b5e…35d7a`; o inventário de conteúdo,
`3217344b…da05`.

Um parser independente associou exatamente os 154 checksums do lock aos
manifests correspondentes. Os 5.750 arquivos declarados pelas crates foram
relidos: zero ausente, divergente ou extra não listado. Três configurações
Cargo internas e checksummed foram inspecionadas e contêm somente flags/alias
de documentação, sem fonte, registry, rede, token ou credencial.

Metadata confirmou 156 pacotes totais e todos os 154 pacotes registry sob os
paths do vendor; tree também passou locked/offline. O relatório completo está
em [`docs/d1c2b3f-final-vendor-results.md`](d1c2b3f-final-vendor-results.md).

O gate está **GO** para dois builds independentes com contextos e targets
separados. Ainda não há ELF, ImageID ou receipt VeriCode. Router/CPI/devnet
continuam `STATUS: NÃO VALIDADO`.

## D1c2b.3g.1 — core compatível com o runtime guest `no_std`

O primeiro build efetivo confirmou que o vendor apenas guest era insuficiente
para o build script host e que o vendor correto precisa representar a união
dos locks. Essa união foi produzida offline com 461 pacotes registry e todos
os checksums de pacote/arquivo passaram.

Com o vendor de união, o Docker alcançou a ligação do guest e revelou
`duplicate lang item panic_impl`: `vericode-core` ativava `std` pelas
features padrão de `borsh` e `sha2`, enquanto o runtime guest RISC Zero é
`no_std`. O core passou a declarar `#![no_std]`, usar `alloc::vec::Vec` e
desativar somente essas features padrão. Versões, locks, wire format,
hashing e regra de verdict não mudaram.

Um probe em target novo, com a imagem local fixada por digest,
`--pull=never`, `--network none`, Cargo locked/offline e as flags oficiais do
builder, terminou com exit `0`. O ELF32 RISC-V tem 147.880 bytes e SHA-256
`3fc668dfd97db52343df267c2148aa49181c813a426a9549782743e09fc6c42d`.
Ele prova a compatibilidade do ajuste, mas não substitui os dois builds
finais nem fornece ImageID.

O build amplo também revelou incompatibilidade no lock host entre
`risc0-zkvm 3.0.3` e transitivas `risc0-circuit-* 4.0.5`; isso permanece
risco explícito para receipts. O relatório está em
[`docs/d1c2b3g1-core-no-std-results.md`](d1c2b3g1-core-no-std-results.md).
Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.

## D1c2b.3g — builds A/B determinísticos

Dois exports do commit `be23e01` receberam vendors independentes da união
exata dos locks (461 crates, 22.560 arquivos de conteúdo verificados em cada
um), configuração raiz idêntica, nonces diferentes e targets separados.
Metadata host e guest resolveu todos os pacotes registry dentro de cada
vendor.

Os dois builds isolados de `vericode-methods` terminaram com exit `0` usando
a imagem guest local fixada pelo digest `3e12f71…3eb3`, Cargo offline e sem
pull. O ELF guest A/B tem 147.880 bytes e SHA-256 `3fc668df…c42d`; o método
combinado A/B tem 180.304 bytes e SHA-256 `5c3c82c4…88bc`. Ambos passaram
`cmp` byte a byte.

O array de ImageID emitido nos dois `methods.rs` é idêntico. Chamadas
separadas de `r0vm --id` produziram
`35b05ee4e627a02f5f7f24b350f99afa4c75cdd8ab556d0e8be0380b5a6fbf41`
para A e B. O relatório completo está em
[`docs/d1c2b3g-deterministic-build-results.md`](d1c2b3g-deterministic-build-results.md).

O gate está **GO** para receipts VeriCode locais reais após commit/auditoria.
Ainda não há receipt; Router/CPI/devnet continuam
`STATUS: NÃO VALIDADO`.

## D1c2b.3h — lock host compatível e artefato guest final

O lock host anterior misturava `risc0-zkvm 3.0.3` com transitivas mais novas
e falhava por incompatibilidade de API. Onze versões foram reconciliadas
offline contra o lock oficial cacheado da mesma crate; `risc0-zkvm 3.0.3` e
`risc0-zkvm-platform 2.2.3` foram preservados. O novo lock tem SHA-256
`f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226`.

Metadata, árvore completa e onze árvores inversas passaram locked/offline. O
host compilou depois de uma adaptação tipada de `Digest` sem efeito no wire
format, e seus 2/2 testes passaram.

A nova união dos locks tem 467 crates. Dois vendors independentes foram
comparados integralmente e dois builds em targets separados produziram ELF
de 147.876 bytes/SHA-256 `63fac491…5408` e método combinado de 180.300
bytes/SHA-256 `e09ba8cf…78f5`, ambos idênticos por `cmp`. Arrays gerados e
dois `r0vm` coincidiram no ImageID
`4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a`.

O relatório completo está em
[`docs/d1c2b3h-host-lock-and-final-build-results.md`](d1c2b3h-host-lock-and-final-build-results.md).
O gate está **GO** para receipts locais reais. Router/CPI/devnet continuam
`STATUS: NÃO VALIDADO`.
