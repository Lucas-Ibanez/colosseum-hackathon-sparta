# D1c2a — wire format candidato, hashing e harness puro

Data: 2026-09-30.

## Decisão

**GO para iniciar D1c2b em escopo local, mínimo e sem rede.**

O `GO` autoriza somente criar um guest RISC Zero mínimo que reutilize
`vericode-core`, reproduzir os vetores deste documento e produzir/verificar
receipts VeriCode locais reais de `PASS` e `FAIL`. Não autoriza Anchor,
Solana, Router/CPI, Groth16, wallet, validator, airdrop, transação, deploy ou
front-end. M1 permanece fora de verde.

## Baseline

- clone: `/home/lucas/src/vericode`, ext4, fora de `/mnt/c`;
- commit inicial:
  `aa236e6f0ba3112867a4a925ea7bd063d448b7f1 feat(core): define JournalV1 semantic contract`;
- working tree inicial: limpo;
- `git show --check HEAD`: exit `0`;
- toolchains: somente os homes isolados do D1a.3;
- nenhuma instalação, atualização ou operação de rede Solana nesta tarefa.

## Escolha do formato e fontes

O wire format candidato local usa **Borsh `0.10.4`**. A escolha não veio de
`latest` nem de `main`:

- o tag leve `boundless-xyz/risc0-solana v3.0.0`, commit
  `ee415935d04a948f27a346b563391900bdad6486`, declara
  `borsh = "0.10.3"` em
  [`examples/counter/shared/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/shared/Cargo.toml);
- o [lock on-chain do mesmo commit](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/Cargo.lock)
  resolve `borsh 0.10.4`, checksum
  `115e54d64eb62cdebad391c19efc9dce4981c690c85a33a12199d99bb9546fee`;
- o [manifesto do guest `counter`](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/zkvm/methods/guest/Cargo.toml)
  declara a mesma linha `0.10.3`, e o
  [lock zkVM](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/zkvm/Cargo.lock)
  também resolve `0.10.4` com o mesmo checksum;
- a [especificação oficial Borsh](https://borsh.io/) define inteiros
  little-endian, structs na ordem dos campos e enums por ordinal `u8`;
  a API usada é a documentação imutável de
  [`borsh 0.10.4`](https://docs.rs/borsh/0.10.4/borsh/).

O hashing usa **SHA-256 por `sha2 0.10.9`**. Os dois locks acima resolvem
essa versão com checksum
`a7507d819769d01a365ab707794a4084392c824f54a7a6a7862f8c3d0892b283`.
A implementação oficial está no tag exato
[`RustCrypto/hashes sha2-v0.10.9`, `sha2/Cargo.toml`](https://github.com/RustCrypto/hashes/blob/sha2-v0.10.9/sha2/Cargo.toml)
e sua API SHA-256 em
[`docs.rs/sha2/0.10.9`](https://docs.rs/sha2/0.10.9/sha2/).

O manifesto VeriCode fixa `borsh = "=0.10.4"` e
`sha2 = "=0.10.9"`. O lock local foi gerado offline por Cargo `1.85.0`;
ele lista 30 pacotes, incluindo a crate local, e tem SHA-256:

```text
191802b234a6aa0f6bb9ce58a61963c377aed435a2baa6dec9576d13d8283b87
```

Comando real:

```text
env CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/cargo \
    RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/rustup \
    PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/cargo/bin:/usr/bin:/bin \
    CARGO_NET_OFFLINE=true \
    cargo +1.85.0 generate-lockfile
```

Saída resumida: `Locking 29 packages`, `Adding borsh v0.10.4`; nenhum
download foi solicitado.

A presença das mesmas versões no lock guest upstream sustenta
compatibilidade prevista, não prova que o futuro guest VeriCode já compila.
Essa prova pertence a D1c2b.

## Artefato, especificação e harness

O único artefato aceito é `RestrictedArtifactV1`, um registro de
desenvolvimento com três `u32`:

1. `schema_version = 1`;
2. `input`;
3. `claimed_output`.

Seu wire candidato Borsh possui exatamente 12 bytes. Ele não aceita código,
arquivos, repositórios, patches, rede, relógio, RPC ou I/O externo.

A especificação fixa é:

```text
schema_version = 1
multiplier     = 2
max_input     = 1_000_000
```

O harness de versão `1`:

- exige bytes canônicos e versão suportada;
- rejeita `input > 1_000_000` como erro explícito;
- calcula `input.checked_mul(2)` e preserva overflow como erro;
- retorna `Verdict::Pass` quando a saída alegada coincide;
- retorna `Verdict::Fail` normalmente quando a alegação é diferente;
- calcula os três compromissos e constrói `JournalV1`;
- não executa I/O e não confere autoridade de release.

## Hashing canônico

Cada compromisso é SHA-256 de um domínio ASCII terminado em NUL seguido do
payload Borsh:

| Compromisso | Domínio | Payload |
| --- | --- | --- |
| `spec_hash` | `vericode:spec:v1\0` | `RestrictedSpecV1` |
| `harness_hash` | `vericode:harness:v1\0` | versão `u32` do harness |
| `artifact_hash` | `vericode:artifact:v1\0` | `RestrictedArtifactV1` |

Vetores calculados independentemente com
`printf %s <preimagem-hex> | xxd -r -p | sha256sum` antes de serem
comparados pelo teste:

| Valor | Payload Borsh hexadecimal | SHA-256 |
| --- | --- | --- |
| spec fixa | `010000000200000040420f00` | `af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778` |
| harness v1 | `01000000` | `01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50` |
| artefato PASS, `7 -> 14` | `01000000070000000e000000` | `d5aa9223d6d2a1ba23bd73ca325b411c75027a739c285a95ff63b963442a224c` |
| artefato FAIL, `7 -> 15` | `01000000070000000f000000` | `343ad7781fe806e047dd7f965e2a41713ce0f668f4d88ebf351d21ef88ee55c6` |
| spec alterada, multiplicador 3 | `010000000300000040420f00` | `3ae3ca9d296cdb907b5318fe2c4df9e8893be08bd21fdb5c3db6d04c1f50f3d6` |
| harness v2 | `02000000` | `890480c6ec39ff8f3704114e30b317a20980fab27a4069d8178a713b26f7791e` |

## Layout e vetores de JournalV1

O candidato local tem 165 bytes, sem prefixo de tamanho:

| Offset | Bytes | Campo |
| ---: | ---: | --- |
| 0 | 4 | `schema_version: u32 LE` |
| 4 | 32 | `job_id` |
| 36 | 32 | `spec_hash` |
| 68 | 32 | `harness_hash` |
| 100 | 32 | `artifact_hash` |
| 132 | 32 | `image_id` |
| 164 | 1 | `verdict`: `PASS=00`, `FAIL=01` |

Os vetores usam `job_id = 11 × 32` e `image_id = 55 × 32`. O segundo é
somente placeholder de desenvolvimento; D1c2a não gerou ImageID.

PASS:

```text
010000001111111111111111111111111111111111111111111111111111111111111111af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb77801124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50d5aa9223d6d2a1ba23bd73ca325b411c75027a739c285a95ff63b963442a224c555555555555555555555555555555555555555555555555555555555555555500
```

FAIL:

```text
010000001111111111111111111111111111111111111111111111111111111111111111af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb77801124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50343ad7781fe806e047dd7f965e2a41713ce0f668f4d88ebf351d21ef88ee55c6555555555555555555555555555555555555555555555555555555555555555501
```

O decoder exige o tamanho exato, versão 1 e ordinal conhecido. A futura ABI
Anchor/Router continua separada e não validada.

## Testes de conformidade

Os 20 testes cobrem:

- construção determinística;
- `PASS` e `FAIL` normais;
- entrada fora do limite como erro;
- hashes literais de spec, harness e artefato;
- alteração de um byte do artefato, da spec e da versão do harness;
- encode/decode sem perda;
- bytes literais dos journals PASS e FAIL gerados pelo harness real;
- rejeição de truncamento, tag desconhecida e schema incompatível;
- rejeição de compromisso divergente após decode;
- divergências semânticas de Job, spec, harness e ImageID.

### Rust 1.85.0 — raia A isolada

```text
env CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/cargo \
    RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/rustup \
    PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/cargo/bin:/usr/bin:/bin \
    CARGO_TARGET_DIR=/tmp/vericode-d1c2a/lane-a \
    CARGO_NET_OFFLINE=true \
    cargo +1.85.0 test --locked
```

Resultado: 20 testes unitários passaram, 0 falharam; 0 doctests.
`cargo +1.85.0 tree --locked` passou e mostrou somente
`vericode-core`, Borsh/SHA-2 e suas dependências Rust transitivas.

### Rust 1.89.0 — raia B isolada

```text
env CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-b/cargo \
    RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-b/rustup \
    PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-b/cargo/bin:/usr/bin:/bin \
    CARGO_TARGET_DIR=/tmp/vericode-d1c2a/lane-b \
    CARGO_NET_OFFLINE=true \
    cargo +1.89.0 test --locked
```

Resultado: 20 testes unitários passaram, 0 falharam; 0 doctests.
`cargo +1.89.0 tree --locked` produziu a mesma árvore.

`cargo tree --locked` não contém Solana, Agave, Anchor, RISC Zero, Docker,
Node ou SDK de blockchain. Os targets ficaram fora do clone.

### Formatação e lint

`rustup component list --installed` confirmou apenas `cargo`, `rust-std`
e `rustc` em ambas as toolchains. Rustfmt e Clippy continuam ausentes;
`cargo fmt --check` e `cargo clippy` não foram executados, e nenhum
componente foi instalado.

## Limites preservados

- nenhum `zkvm/`, guest, host RISC Zero, receipt, prova ou ImageID;
- nenhuma alegação de receipt Groth16;
- nenhum `programs/`, `Anchor.toml`, ABI on-chain ou Router/CPI;
- nenhum Job on-chain, escrow, mint ou vínculo de executor;
- nenhuma wallet, keypair, seed, `.env`, Program ID ou artefato de deploy;
- nenhum validator, airdrop, transação, deploy, rede Solana ou push;
- nenhuma instalação ou alteração global.

Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Riscos para D1c2b e gates posteriores

1. O guest VeriCode ainda precisa provar que compila a mesma crate e produz
   exatamente estes bytes; a presença das versões no lock upstream é somente
   evidência de compatibilidade prevista.
2. O ImageID real só existirá após build determinístico do guest; o
   placeholder dos vetores não pode ser reutilizado.
3. Ainda não existem receipts VeriCode `PASS`/`FAIL`. A receipt composta
   do `hello-world` D1a.3 não satisfaz esse gate e não foi provada Groth16.
4. O formato permanece candidato até teste host/guest e futura revisão da
   fronteira on-chain. Não é ABI Anchor nem Router.
5. Alterar regra, domínio, ordem, tipo, versão ou dependência exige bump
   explícito, novos vetores e decisão documental; mudança silenciosa é falha.

## Gate D1c2b

D1c2b pode começar porque hashing, wire candidato e harness passaram nas duas
raias host, offline e com lock preservado. O próximo gate deve manter
`vericode-core` como única regra, construir um guest mínimo pelo caminho
determinístico já auditado, comparar os bytes do journal e produzir/verificar
receipts locais reais de `PASS` e `FAIL` sem dev mode.
