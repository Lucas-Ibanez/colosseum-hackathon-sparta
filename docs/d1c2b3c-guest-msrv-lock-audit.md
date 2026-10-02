# D1c2b.3c — auditoria MSRV do lock guest

Data da auditoria: 2026-10-02.

## Decisão

**CANDIDATO DE RECONCILIAÇÃO COMPROVADO.**

O lock guest atual não é compatível com o compilador do builder fixado. Ele
resolve `enum-ordinalize 4.4.2` e `enum-ordinalize-derive 4.4.2`; os dois
manifests declaram `rust-version = "1.89"`, enquanto a imagem
`risczero/risc0-guest-builder:r0.1.88.0` fornece `rustc 1.88.0-dev`.

Há, porém, uma combinação transitiva com proveniência suficiente para um gate
futuro de reconciliação de lock: tanto o tag exato `risc0/risc0 v3.0.3` quanto
o tag exato `boundless-xyz/risc0-solana v3.0.0` preservam
`enum-ordinalize 4.3.0` e `enum-ordinalize-derive 4.3.1`. Os archives
correspondentes declaram MSRV `1.60`, seus SHA-256 coincidem com os checksums
dos locks oficiais, e o hello-world do primeiro tag já foi construído em
D1a.3 com o mesmo builder guest 1.88 fixado por digest.

Esta decisão não altera o lock e não libera diretamente um novo build. Ela
autoriza apenas propor um gate mecânico e limitado de reconciliação. Esse gate
deverá preservar os pins diretos `risc0-zkvm = 3.0.3` e
`risc0-build = 3.0.3`, o builder fixado, as fontes do guest e o wire format
VeriCode, e deverá repetir toda a validação offline antes de compilar.

## Preflight e integridade preservada

- clone: `/home/lucas/src/vericode`, filesystem Linux `ext4`;
- branch observada: `main`;
- HEAD observado: `398e9e0f7b902120749674ed78d8ecbe6b51d91a`;
- `git diff --check`: exit `0` antes da documentação deste gate;
- `/home/lucas/.rustup`: ausente;
- `zkvm/Cargo.lock`:
  `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
- `zkvm/methods/guest/Cargo.lock`:
  `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50`.

O working tree já continha exclusivamente documentos dos gates D1c2b
anteriores, modificados ou não rastreados. Eles foram preservados; não houve
stash, reset, checkout, descarte ou sobrescrita. Nenhum arquivo em `zkvm/`
foi modificado.

Saída inicial de `git status --short`:

```text
 M docs/decisions.md
 M docs/evidence.md
 M docs/zkvm-notes.md
?? docs/d1c2b-guest-receipt-results.md
?? docs/d1c2b2-builder-vendor-results.md
?? docs/d1c2b2a-builder-entrypoint-results.md
?? docs/d1c2b3-guest-build-results.md
?? docs/d1c2b3a-root-vendor-results.md
?? docs/d1c2b3b-deterministic-guest-build-results.md
?? docs/d1c2b3b-rustup-remediation.md
```

## Causa reproduzida

O manifesto direto do projeto fixa:

```text
vericode-guest 0.1.0
└── risc0-zkvm =3.0.3
```

O manifesto publicado de `risc0-zkvm 3.0.3` não fixa todos os seus
transitivos exatamente: declara `risc0-groth16 = "3.0.2"`, isto é,
`^3.0.2`. O lock local, criado com Cargo/Rust host 1.89 e o resolver em modo
fallback, selecionou `risc0-groth16 3.0.5`. A cadeia observada por
`cargo tree --locked --offline --invert` e confirmada pelos manifests é:

```text
vericode-guest 0.1.0
└── risc0-zkvm 3.0.3                   (requisito direto =3.0.3)
    └── risc0-groth16 3.0.5            (requisito ^3.0.2)
        ├── ark-bn254 0.5.0
        │   ├── ark-ec 0.5.0
        │   │   └── educe 0.6.0
        │   └── ark-ff 0.5.0
        │       └── educe 0.6.0
        ├── ark-ec 0.5.0
        │   └── educe 0.6.0
        ├── ark-ff 0.5.0
        │   └── educe 0.6.0
        └── ark-groth16 0.5.0
            ├── ark-ec 0.5.0 ──> educe 0.6.0
            ├── ark-ff 0.5.0 ──> educe 0.6.0
            └── ark-poly 0.5.0 ──> educe 0.6.0

educe 0.6.0
└── enum-ordinalize ^4.2, feature `derive`
    └── enum-ordinalize-derive ^4.4.1
```

O requisito `^4.2` de `educe` admite versões `4.x` posteriores. Como o
manifesto `vericode-guest` não declara `rust-version`, a geração do lock com
o host 1.89 registrou `latest Rust 1.89.0 compatible versions` e escolheu
`enum-ordinalize 4.4.2`. Essa versão, por sua vez, admite e resolveu
`enum-ordinalize-derive 4.4.2`. O Cargo que gerou o lock não estava
resolvendo para o compilador guest 1.88 do builder; a incompatibilidade só
apareceu quando esse compilador consumiu o lock.

As duas crates problemáticas aparecem em ambos os locks VeriCode. É o
`zkvm/methods/guest/Cargo.lock`, porém, que o builder usa na compilação guest
e que causou a falha observada.

## MSRV, origem e checksums

Todos os pacotes desta tabela vieram de
`registry+https://github.com/rust-lang/crates.io-index`. Os manifests e
archives foram lidos dos caches isolados já existentes; não houve acesso à
rede.

| Pacote | Versão no lock guest | `rust-version` | Checksum/SHA-256 | Dependente imediato relevante |
| --- | ---: | ---: | --- | --- |
| `risc0-zkvm` | `3.0.3` | não declarado | `3fcce11648a9ff60b8e7af2f0ce7fbf8d25275ab6d414cc91b9da69ee75bc978` | `vericode-guest` |
| `risc0-groth16` | `3.0.5` | não declarado | `b0ca702ea7d0162766defe7ed6a79bda4a747ad9e2684000a6edd14df0a6d1f3` | `risc0-zkvm` |
| `ark-bn254` | `0.5.0` | não declarado | `d69eab57e8d2663efa5c63135b2af4f396d66424f88954c21104125ab6b3e6bc` | `risc0-groth16` |
| `ark-ec` | `0.5.0` | `1.75` | `43d68f2d516162846c1238e755a7c4d131b892b70cc70c471a8e3ca3ed818fce` | `risc0-groth16`, `ark-bn254`, `ark-groth16` |
| `ark-ff` | `0.5.0` | `1.75` | `a177aba0ed1e0fbb62aa9f6d0502e9b46dad8c2eab04c14258a1212d2557ea70` | `risc0-groth16` e crates arkworks |
| `ark-groth16` | `0.5.0` | não declarado | `88f1d0f3a534bb54188b8dcc104307db6c56cdae574ddc3212aec0625740fc7e` | `risc0-groth16` |
| `ark-poly` | `0.5.0` | `1.75` | `579305839da207f02b89cd1679e50e67b4331e2f9294a57693e5051b7703fe27` | `ark-ec`, `ark-groth16` |
| `educe` | `0.6.0` | `1.60` | `1d7bc049e1bd8cdeb31b68bbd586a9464ecf9f3944af3958a7a9d0f8b9799417` | `ark-ec`, `ark-ff`, `ark-poly` |
| `enum-ordinalize` | `4.4.2` | **`1.89`** | `89dd01549b09589510cf0647475075d12071456586d70f5c75c98ae2a5537677` | `educe`, com feature `derive` |
| `enum-ordinalize-derive` | `4.4.2` | **`1.89`** | `a65863d15a4ce2888bd2f0f543cc963d3879c3a022c8ee43f6141d479a3ac815` | `enum-ordinalize` |

Os SHA-256 calculados dos archives de `risc0-zkvm`, `risc0-groth16`,
`educe` e das duas crates `enum-ordinalize` coincidiram com os checksums do
lock atual. Não se inferiu MSRV para os manifests que não o declaram; nesses
casos a tabela registra explicitamente “não declarado”.

## Comparação com os tags exatos

### `risc0/risc0 v3.0.3`

- commit verificado:
  `14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`;
- checkout detached e limpo;
- lock raiz:
  `15995a3395e61847c403182721c13c56f8735c8d22ed989dc350c5621d812024`;
- lock `examples/hello-world/methods/guest/Cargo.lock`:
  `b7b6d59a0c0f9418cd2ec703867aba6cca35ff25f5c556d526f2de8e2001b762`;
- `risc0-zkvm 3.0.3` e `risc0-groth16 3.0.2`;
- `educe 0.6.0`;
- `enum-ordinalize 4.3.0`, checksum
  `fea0dcfa4e54eeb516fe454635a95753ddd39acda650ce703031c6973e315dd5`;
- `enum-ordinalize-derive 4.3.1`, checksum
  `0d28318a75d4aead5c4db25382e8ef717932d0346600cacae6357eb5941bc5ff`.

Os manifests cacheados dessas duas versões candidatas declaram edição 2021 e
`rust-version = "1.60"`. Os SHA-256 dos archives cacheados coincidiram com
os checksums acima. Em D1a.3, esse lock hello-world foi consumido pelo builder
`r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3`:
os builds efetivos 1 e 3 terminaram com exit `0` e produziram ELFs idênticos.
Isso comprova, para o exemplo oficial, que o conjunto lockado é aceito pelo
compilador guest 1.88.

Fontes oficiais exatas:

- [`risc0/zkvm/Cargo.toml`](https://github.com/risc0/risc0/blob/14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6/risc0/zkvm/Cargo.toml);
- [`Cargo.lock` raiz](https://github.com/risc0/risc0/blob/14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6/Cargo.lock);
- [lock guest do hello-world](https://github.com/risc0/risc0/blob/14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6/examples/hello-world/methods/guest/Cargo.lock).

### `boundless-xyz/risc0-solana v3.0.0`

- commit verificado:
  `ee415935d04a948f27a346b563391900bdad6486`;
- checkout detached e limpo;
- lock `examples/counter/zkvm/Cargo.lock`:
  `ab03b523330c4bd66b8ff29eb81ef862cf6f59e56fd4b821dbd60e5648122c15`;
- lock `examples/counter/zkvm/methods/guest/Cargo.lock`:
  `d2af16712e91ef84c4b6af1bcb00b6707103c403a2595a13ca8c57b58a877595`;
- o manifesto guest declara `risc0-zkvm = "3.0.3"`;
- o lock guest fixa o mesmo conjunto relevante do tag RISC Zero:
  `risc0-zkvm 3.0.3`, `risc0-groth16 3.0.2`, `educe 0.6.0`,
  `enum-ordinalize 4.3.0` e derive `4.3.1`;
- o archive publicado `risc0-groth16 3.0.2` tem SHA-256
  `724285dc79604abfb2d40feaefe3e335420a6b293511661f77d6af62f1f5fae9`,
  igual ao checksum do lock desse tag.

Fontes oficiais exatas:

- [manifesto guest do counter](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/zkvm/methods/guest/Cargo.toml);
- [lock guest do counter](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/zkvm/methods/guest/Cargo.lock);
- [lock do workspace zkVM](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/zkvm/Cargo.lock).

### Diferença relevante

| Pacote | Lock guest VeriCode atual | Locks oficiais exatos |
| --- | ---: | ---: |
| `risc0-zkvm` | `3.0.3` | `3.0.3` |
| `risc0-groth16` | `3.0.5` | `3.0.2` |
| `ark-*` relevante | `0.5.0` | `0.5.0` |
| `educe` | `0.6.0` | `0.6.0` |
| `enum-ordinalize` | `4.4.2` / MSRV `1.89` | `4.3.0` / MSRV `1.60` |
| `enum-ordinalize-derive` | `4.4.2` / MSRV `1.89` | `4.3.1` / MSRV `1.60` |

Não se exige igualdade byte a byte entre locks de workspaces diferentes. A
comparação demonstra uma versão transitiva publicada e identificada por
checksum, usada pelos dois tags exatos com `risc0-zkvm 3.0.3`, e não apenas
uma semelhança numérica.

## Limite da reconciliação candidata

Um gate futuro poderá propor uma alteração exclusivamente mecânica do lock
guest para reconciliar o conjunto transitivo com o preservado pelos tags
oficiais. Ele deverá tratar o conjunto acoplado, e não baixar somente uma
crate por tentativa:

- `risc0-groth16 3.0.5 -> 3.0.2`;
- `enum-ordinalize 4.4.2 -> 4.3.0`;
- `enum-ordinalize-derive 4.4.2 -> 4.3.1`;
- demais transitivos RISC Zero acoplados, somente quando exigidos pelo lock
  oficial e comprovados antes da alteração.

Essa proposta preserva os manifests e fontes VeriCode, inclusive o pin direto
`risc0-zkvm = 3.0.3`, `vericode-core`, Borsh `0.10.4`, SHA-2 `0.10.9` e os
165 bytes candidatos de `JournalV1`. Ainda assim, a compatibilidade integral
do guest VeriCode só poderá ser afirmada depois de uma futura resolução
offline autorizada, novo vendor, `cargo tree/metadata --locked --offline` e
dois builds efetivos. Nenhum desses passos foi executado neste gate.

## Comandos de inspeção e resultados

Todos os comandos Cargo usaram explicitamente:

```text
CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo
RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup
RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0
PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin
RUSTUP_AUTO_UPDATE=0
CARGO_NET_OFFLINE=true
```

Comandos e saídas relevantes:

```text
cargo +1.89.0 tree --locked --offline \
  --manifest-path zkvm/methods/guest/Cargo.toml \
  --invert enum-ordinalize@4.4.2
exit 0; cadeia risc0-zkvm -> risc0-groth16 -> arkworks -> educe confirmada

cargo +1.89.0 tree --locked --offline \
  --manifest-path zkvm/methods/guest/Cargo.toml \
  --invert enum-ordinalize-derive@4.4.2
exit 0; derive introduzido por enum-ordinalize, ativado por educe

cargo +1.89.0 metadata --locked --offline --format-version 1 \
  --manifest-path zkvm/methods/guest/Cargo.toml
exit 0; requisitos SemVer, MSRV e origens acima confirmados
```

Uma primeira tentativa de filtrar a saída de metadata com `jq` terminou com
`jq: command not found`; nenhuma instalação foi feita. A mesma metadata foi
repetida offline e filtrada em memória por `python3`, com exit `0`.

As demais inspeções foram `git show` nos dois commits exatos, `awk`/`rg`/`sed`
nos manifests e locks locais e `sha256sum` nos locks e archives. Não houve
resolução nova, download, build, teste, Docker ou escrita em cache.

## Riscos abertos

- o lock VeriCode continua incompatível com o builder guest 1.88 até um gate
  futuro efetivamente alterá-lo e validá-lo;
- ainda não há ELF, ImageID, dois builds determinísticos, execução host ou
  receipt VeriCode PASS/FAIL;
- a evidência upstream comprova a combinação candidata e seu MSRV, mas não
  substitui a compilação do guest VeriCode após a reconciliação;
- Groth16, Router, CPI e devnet permanecem `STATUS: NÃO VALIDADO`.

## Validação final

- `git diff --check`: exit `0`;
- branch/HEAD: `main` em
  `398e9e0f7b902120749674ed78d8ecbe6b51d91a`;
- os dois SHA-256 de lock permaneceram iguais aos valores do preflight;
- `/home/lucas/.rustup`: ausente (`test ! -e` com exit `0`);
- `git diff --name-only -- zkvm crates Cargo.toml Cargo.lock
  docs/architecture.md docs/manifest-schema.md`: saída vazia;
- busca por `vendor/` e `.cargo/` sob `zkvm/`: saída vazia;
- o status final acrescenta somente este relatório ao conjunto documental
  preexistente.

Não houve instalação, rede, Docker, compilação, teste, alteração em `zkvm/`,
lockfile ou vendor, commit ou push neste gate.
