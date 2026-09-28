# Notas do Router Solana

**STATUS: NÃO VALIDADO**

**Decisão D0: pendente de spike**

**Correção D1a.1: `risc0-solana v3.0.0` permanece referência técnica; não constitui Perfil A instalável nem prova deployment.**

## Objetivo

Verificar uma receipt RISC Zero Groth16 por CPI em Solana e, somente após validar a prova e os campos críticos do journal contra o Job, permitir que o programa Anchor considere o release.

## Fatos demonstrados pelo tag `v3.0.0`

- Origem: repositório oficial `risc0/risc0-solana`, redirecionado para `boundless-xyz/risc0-solana`.
- A [release `v3.0.0`](https://github.com/boundless-xyz/risc0-solana/releases/tag/v3.0.0) declara suporte ao RISC0 zkVM 3.0 e a descreve como primeira versão totalmente auditada.
- O [`Cargo.toml` do verificador Groth16](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/groth_16_verifier/Cargo.toml) declara `anchor-lang 0.31.1`, `risc0-zkvm 3.0.3` e `solana-bn254 3.0.0`.
- O [`Cargo.lock` do solana-verifier](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/Cargo.lock) resolve `anchor-lang 0.31.1`, `risc0-zkvm 3.0.3`, `risc0-groth16 3.0.2` e `solana-program 2.3.0`. As crates Solana separadas abrangem várias linhas: interfaces 1.x, crates 2.2–2.4, `solana-bn254`/`solana-define-syscall 3.0.0` e `solana-loader-v3-interface 5.0.0`.
- O [`workflow de testes`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/.github/workflows/tests.yml) instala Agave CLI `2.3.9` e Anchor CLI `0.31.1` antes de `anchor test`. Ele também usa a action Rust `risc0/risc0/.github/actions/rustup@main`, que é flutuante e não pode ser tratada como pin de release.
- O [`counter`](https://github.com/boundless-xyz/risc0-solana/tree/v3.0.0/examples/counter) é o exemplo de CPI consultado. Seus manifests e lockfiles foram lidos no tag exato; ele não foi clonado, compilado, executado ou implantado pelo VeriCode.

A versão da CLI Agave e as versões das crates Solana são dimensões diferentes. A presença de CLI `2.3.9` no workflow não transforma todas as crates em `2.3.9`, nem prova compatibilidade com a recomendação Anchor `2.1.0`.

## Exemplo `counter` e fronteira de workspaces

O [`examples/counter/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/Cargo.toml) define o workspace on-chain (`programs/*` e `shared`). O [`examples/counter/zkvm/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/Cargo.toml) define outro workspace para host/methods. Isso demonstra isolamento estrutural, não compatibilidade automática.

Para o VeriCode, um spike ainda precisa provar que ambos os lados concordam sobre:

- Program ID e programa Router realmente invocado;
- discriminadores, instrução e ordem/flags dos account metas da CPI;
- formato exato do seal Groth16 e do journal público;
- ImageID e campos do `JournalV1` ligados ao Job;
- serialização Borsh e bytes produzidos por cada workspace.

O workspace on-chain do `counter` resolve Borsh `0.10.4`; o lock zkVM contém Borsh `0.10.4` e `1.5.7` para dependências diferentes. A compatibilidade deve ser provada por esquema e testes de bytes, não inferida do nome da crate.

## Divergência Anchor/Agave

A documentação do Anchor 0.31.x recomenda Agave `2.1.0`. O workflow do Router no tag `v3.0.0` usa `2.3.9`. Não foi encontrada fonte oficial que demonstre o `counter` com `2.1.0`, nem autorização para reclassificar `2.3.9` como versão recomendada do Anchor.

Conclusão: coexistência **a confirmar por spike**. Não há prova suficiente para chamá-la compatível e também não há evidência para chamá-la incompatível.

## Status por rede

| Rede | Status | Evidência encontrada |
| --- | --- | --- |
| localnet | REFERÊNCIA OFICIAL, NÃO EXECUTADA | O tag contém `examples/counter` e `solana-verifier`; nenhum comando foi executado pelo VeriCode. |
| Solana devnet | NÃO VALIDADO | Nenhum Program ID/deployment Solana devnet foi comprovado nas fontes oficiais consultadas. |
| Solana mainnet-beta | NÃO VALIDADO | Nenhum Program ID/deployment Solana mainnet-beta foi comprovado nas fontes oficiais consultadas. |

O link de deployments no README oficial conduziu a uma página de contratos verificadores EVM, não a uma lista de deployments Solana. Endereço EVM não é evidência de deployment Solana.

Consequências:

- `RISC0_VERIFIER_ROUTER_PROGRAM_ID` permanece vazio;
- não alegar verificação ZK on-chain;
- não implementar CPI antes de confirmar Program ID, cluster, interface/IDL, perfil de versões e transação reproduzível;
- qualquer atestado da plataforma deve ser rotulado como fallback, separado de prova ZK.

## O que precisa ser provado

1. Resolver e registrar o commit exato da tag `v3.0.0` no checkout do spike.
2. Reproduzir o `counter` com toolchain Rust pinada e locks preservados.
3. Comparar Anchor `0.31.1` + Agave `2.1.0` com a combinação do workflow, Anchor `0.31.1` + Agave `2.3.9`.
4. Provar CPI e compatibilidade de bytes entre os workspaces; workspaces separados, sozinhos, não resolvem isso.
5. Confirmar em fonte oficial o Program ID e o cluster antes de preencher qualquer variável.
6. Validar receipt Groth16, ImageID e digest do journal; testar rejeição para Job, mint, executor, ImageID e journal divergentes.
7. Comprovar devnet separadamente, se houver deployment oficial. Até lá, manter `STATUS: NÃO VALIDADO`.

## Fontes oficiais consultadas

- [Release `risc0-solana v3.0.0`](https://github.com/boundless-xyz/risc0-solana/releases/tag/v3.0.0)
- [Exemplo `counter` no tag](https://github.com/boundless-xyz/risc0-solana/tree/v3.0.0/examples/counter)
- [Workflow da tag](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/.github/workflows/tests.yml)
- [Manifesto do verificador Groth16](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/groth_16_verifier/Cargo.toml)
- [Lockfile do verificador](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/Cargo.lock)
- [Manifesto do programa `counter`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/programs/solana-counter/Cargo.toml)
- [Lockfile on-chain do `counter`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/Cargo.lock)
- [Lockfile zkVM do `counter`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/Cargo.lock)
- [Release notes Anchor 0.31.0 no tag `v0.31.1`](https://github.com/otter-sec/anchor/blob/v0.31.1/docs/content/docs/updates/release-notes/0-31-0.mdx)
- [Documentação RISC Zero de contratos verificadores](https://dev.risczero.com/api/blockchain-integration/contracts/verifier)
