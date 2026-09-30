# D1c1 — núcleo puro e contrato semântico inicial

Data: 2026-09-30.

## Decisão

**GO para iniciar D1c2 em escopo local, mínimo e isolado.**

Esse `GO` vale somente para definir explicitamente o wire format, criar o
guest mínimo e produzir receipts VeriCode locais de `PASS` e `FAIL`. Não
desbloqueia D1b, não autoriza Solana/Anchor/rede e não promove M1 para verde.

## Baseline e reconciliação D1a.3

- clone: `/home/lucas/src/vericode`, ext4, fora de `/mnt/c`;
- commit inicial: `6dd9d9e2d9c8d3d0390b06a7c1ae4af89bbeab5e`;
- working tree inicial: limpo;
- `git show --check HEAD`: exit `0`;
- toolchains Rust usadas nesta tarefa: somente os homes isolados do D1a.3;
- nenhuma instalação definitiva de Rust/Anchor/Agave/RISC Zero foi feita por
  D1b; o Perfil A continua candidato.

Evidências que não devem ser confundidas:

| Item | Estado real |
| --- | --- |
| Toolchains do spike | Instaladas somente em `$HOME/.local/share/vericode-spikes/d1a3/homes/*`; não são bootstrap D1b. |
| Perfil A | Candidato: Rust `1.85.0`, Anchor/AVM/crates `0.31.1`, Agave `2.1.0`; não promovido. |
| Receipt local D1a.3 | Receipt composta do `hello-world` upstream, produzida a partir do ELF Docker e verificada localmente com journal inteiro `391`. |
| Receipt VeriCode | Não existe antes ou durante D1c1. |
| Receipt Groth16 | Não demonstrada pelo relatório D1a.3; não houve conversão/prova Groth16 registrada para a receipt local. |

Consequência: a receipt D1a.3 valida parte da raia zkVM, mas não valida
`JournalV1`, `PASS`, `FAIL`, Router/CPI ou prova ZK on-chain do VeriCode. M1
permanece fora de verde.

## Escopo implementado

A workspace Cargo mínima contém uma única crate:

- `crates/vericode-core`, edição Rust 2021, sem dependências;
- `#![forbid(unsafe_code)]`;
- tipos `Hash32`, `JobId`, `ImageId`, `Verdict`,
  `JournalV1Commitments`, `JournalV1` e `JournalValidationError`;
- `JOURNAL_V1_SCHEMA_VERSION = 1`;
- construtores e getters explícitos;
- `JournalV1::validate_against`, que compara versão, Job, especificação,
  harness, artefato e ImageID antes de retornar o verdict.

`Hash32` contém exatamente 32 bytes. `JobId` e `ImageId` são newtypes
distintos sobre esse valor. Isso define a representação semântica da API Rust
atual, não um wire format público.

## Invariantes cobertos

1. A construção com as mesmas entradas produz o mesmo valor semântico.
2. `PASS` é um resultado normal após validação dos compromissos.
3. `FAIL` é igualmente um resultado normal e verificável; a biblioteca não
   usa `panic`, `assert`, `unwrap` ou `expect` nesse caminho.
4. Divergências de versão, `job_id`, `spec_hash`, `harness_hash`,
   `artifact_hash` e `image_id` produzem erros distintos.
5. Validar compromissos não confere autoridade de release. Em particular,
   `Ok(Verdict::Fail)` não autoriza pagamento.
6. Executor e mint não pertencem ao journal; continuam no futuro estado do
   Job e no contrato Anchor.
7. Nenhuma serialização, layout, endianness, algoritmo de hash ou receipt foi
   definido pela crate.

## Fixture de desenvolvimento restrita

Os testes usam somente o registro fixo:

```text
vericode:restricted-artifact:v1:input=7;expected=14
```

Ele possui 51 bytes e SHA-256 pré-calculado
`f62bbea1308c1c0d38baa8cc6263016376745af45e931aab86dd1c35a784fb26`.
É fixture de desenvolvimento para um valor restrito e versionado, não código,
repositório, patch ou suporte a artefatos arbitrários. A crate recebe o
compromisso já calculado; hashing do artefato está fora de D1c1.

## Comandos e resultados reais

O `Cargo.lock` foi criado por Cargo `1.85.0`:

```text
cargo +1.85.0 generate-lockfile
```

Ele contém somente `vericode-core 0.1.0`; SHA-256 observado:
`1c62e4534f511d900e151f5a8365f0397b6a362fd4f3545d7ab7278c72121657`.

### Rust 1.85.0 — raia A isolada

```text
cargo +1.85.0 test --locked
cargo +1.85.0 tree --locked
```

Resultado: compilação concluída; 9 testes unitários passaram, 0 falharam; 0
doctests; árvore efetiva contendo somente
`vericode-core v0.1.0 (/home/lucas/src/vericode/crates/vericode-core)`.

### Rust 1.89.0 — raia B isolada

```text
cargo +1.89.0 test --locked
cargo +1.89.0 tree --locked
```

Resultado: compilação concluída; 9 testes unitários passaram, 0 falharam; 0
doctests; a mesma árvore contendo somente a crate local.

Os targets foram direcionados para `/tmp/vericode-d1c1/lane-a` e
`/tmp/vericode-d1c1/lane-b`; nenhum artefato de build foi criado no clone.

### Formatação e lint

`rustup component list --installed` reportou somente `cargo`, `rust-std` e
`rustc` nas toolchains `1.85.0` e `1.89.0`. `rustfmt` e Clippy não estão
instalados nesses homes isolados; `cargo fmt --check` e `cargo clippy` não
foram executados, e nenhum componente foi instalado para substituí-los.

## Dependências efetivas

`crates/vericode-core/Cargo.toml` possui `[dependencies]` vazio. O lock e o
resultado de `cargo tree --locked` confirmam ausência de dependências diretas
ou transitivas, inclusive Solana, Agave, Anchor, RISC Zero, Docker, Node e
SDKs de blockchain. A biblioteca usa somente a biblioteca padrão.

## Limites preservados

- nenhum guest, host zkVM ou receipt VeriCode;
- nenhuma serialização final de `JournalV1`;
- nenhuma receipt Groth16;
- nenhum programa Anchor, escrow, mint ou vínculo on-chain de executor;
- nenhum Router/CPI, Program ID, validator, localnet ou devnet;
- nenhuma wallet, keypair, seed, `.env`, airdrop, transação ou deploy;
- nenhuma instalação ou atualização de ferramenta/sistema;
- nenhum push.

Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Gate D1c2

D1c2 pode começar porque o contrato semântico puro passou nas duas versões
Rust exigidas e não possui acoplamento externo. O próximo gate deve:

1. decidir e testar o wire format canônico antes de publicar bytes;
2. manter a regra de negócio na crate pura;
3. criar um guest mínimo para o mesmo fixture restrito;
4. produzir receipts VeriCode reais de `PASS` e `FAIL`, sem dev mode;
5. comparar journal decodificado com todos os compromissos esperados;
6. classificar explicitamente o tipo de receipt. Uma receipt composta não
   pode ser apresentada como Groth16 sem conversão e verificação reais.

D1c2 não autoriza wallet, Solana, Anchor, Router, CPI ou rede.
