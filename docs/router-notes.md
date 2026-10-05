# Notas do Router Solana

**STATUS em devnet (D4a, 2026-10-05): Router upstream implantado e imutável,
mas NÃO INICIALIZADO e inutilizável; verificador Groth16 upstream implantado,
imutável e funcional em simulação. O escrow passou a chamar o verificador
direto, sem Router. Nenhuma transação de verificação em devnet ainda.**

**D4a (2026-10-05): reconhecimento somente leitura do Router upstream em
devnet (decisão D4-1, caminho (a)) — reprovado.**
- Critério 1 ok: `6JvFfBrv…` (Router) e `THq1qFYQ…` (verificador) existem e
  são executáveis.
  - ProgramData `HbJ7h1mN…` e `ENdLkqHp…`, com upgrade authority `None` nos
    dois.
  - Implantados em 23/10/2025 pela chave `7uAQqkPQ…`, que finalizou ambos
    logo depois (`SetAuthority` sem nova autoridade). O verificador recebeu
    um `Upgrade` antes da finalização.
- Critério 2 falhou: a PDA `["router"]` (`4Sh5ofCz…`) e a entrada
  `["verifier", 73c457ba]` (`4Z7ok78x…`) não existem.
  - Só o `INITIAL_OWNER` embutido no Router upstream pode dar `initialize`.
  - `add_verifier` exige `upgrade_authority(verificador) == PDA do Router`;
    a de `THq1q…` é `None`, então esse verificador nunca poderá ser
    registrado.
- Critério 3 falhou: `simulateTransaction` de `Router.verify` → 3012
  `AccountNotInitialized` (conta `router`) para FIB, PASS, FAIL, seal
  adulterado e ImageID errado.
- Dump do verificador: 199.256 bytes, SHA-256 `34ae6e5c…`; os primeiros
  199.248 bytes (`638db2b9…`) diferem do rebuild local `dab6746d…`.
- O verificador chamado direto, em simulação: FIB, PASS e FAIL aceitos a
  99.541 CU, o mesmo valor local; seal adulterado → 6003; ImageID errado →
  6000.
- Decisão humana (D4a): caminho (b′), com CPI direta do escrow ao verificador
  imutável e sem Router. Divergência do guia §1/§8 registrada em
  `docs/decisions.md`. Relatório:
  [`docs/d4a-direct-verifier-results.md`](d4a-direct-verifier-results.md).
- Claim permitido até o D4b: "verificado por CPI ao verificador Groth16 de
  `risc0-solana v3.0.0` em `solana-program-test` local, inclusive com os
  bytes implantados em devnet". "Verificado on-chain em devnet" só com uma
  transação de liquidação em devnet e o link do Explorer.

Histórico anterior ao D4a:

**Decisão D0: pendente de spike**

**Correção D1a.1: `risc0-solana v3.0.0` permanece referência técnica; não constitui Perfil A instalável nem prova deployment.**

**D1a.2: protocolo definido; Router/CPI/deployments continuam não executados e não validados.**

**D1a.3 concluído em H1–H4: toolchains, receipt zkVM e ABI host validados;
build SBF, CPI e deployment não executados.**

**D2d (2026-10-04): receipts Groth16 reais do VeriCode (PASS e FAIL) foram
verificadas pelo programa Verifier Router em `solana-program-test` local
(Perfil A).** Adulterações de proof, selector, ImageID e journal digest
foram rejeitadas antes de cada positivo
([`docs/d2d-groth16-router-spike-results.md`](d2d-groth16-router-spike-results.md)).
Isso não é verificação on-chain em cluster: devnet, Router real e Program ID
de rede continuam `STATUS: NÃO VALIDADO`.

**D2e (2026-10-04): o `vericode_escrow` chama o Router por CPI em `release` e
`refund_on_fail`, testado em `solana-program-test` local.**
- CPI manual: discriminador `verify` `85a18d3078c65896`, 332 bytes de dados,
  contas `[router PDA, verifier entry, verificador, system program]`.
- Constantes fixadas no programa:
  - Router `6JvFfBrv…`; selector `73c457ba` (entrada `4Z7ok78x…`);
  - verificador `THq1qFYQ…`; `image_id` do Job.
- Rejeições testadas antes da CPI: artefato não entregue, selector ≠
  `73c457ba`, contas do Router falsas.
- Rejeições testadas pelo verificador: seal adulterado (6003) e seal de outro
  journal (6000).
- Rejeição testada pelo Router: entrada em e-stop (6001).
- As receipts PASS/FAIL reais liquidam; o Router e o verificador consomem
  110.701 CU.
- As contas do Router são montadas no genesis dos testes com o layout do
  fonte pinado.
- Os `.so` foram reconstruídos offline do commit pinado, idênticos ao D2d.
- Relatório: [`docs/d2e-router-settlement-results.md`](d2e-router-settlement-results.md).
- Claim permitido: "verificado por CPI ao Verifier Router em
  `solana-program-test` local".

## Objetivo

Verificar uma receipt RISC Zero Groth16 por CPI em Solana e, somente após validar a prova e os campos críticos do journal contra o Job, permitir que o programa Anchor considere o release.

## Fatos demonstrados pelo tag `v3.0.0`

- Origem: repositório oficial `risc0/risc0-solana`, redirecionado para `boundless-xyz/risc0-solana`.
- O tag é leve e resolve diretamente para o commit
  `ee415935d04a948f27a346b563391900bdad6486`, conforme o
  [ref oficial](https://api.github.com/repos/boundless-xyz/risc0-solana/git/ref/tags/v3.0.0).
- A [release `v3.0.0`](https://github.com/boundless-xyz/risc0-solana/releases/tag/v3.0.0) declara suporte ao RISC0 zkVM 3.0 e a descreve como primeira versão totalmente auditada.
- O [`Cargo.toml` do verificador Groth16](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/groth_16_verifier/Cargo.toml) declara `anchor-lang 0.31.1`, `risc0-zkvm 3.0.3` e `solana-bn254 3.0.0`.
- O [`Cargo.lock` do solana-verifier](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/Cargo.lock) resolve `anchor-lang 0.31.1`, `risc0-zkvm 3.0.3`, `risc0-groth16 3.0.2` e `solana-program 2.3.0`. As crates Solana separadas abrangem várias linhas: interfaces 1.x, crates 2.2–2.4, `solana-bn254`/`solana-define-syscall 3.0.0` e `solana-loader-v3-interface 5.0.0`.
- O [`workflow de testes`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/.github/workflows/tests.yml) instala Agave CLI `2.3.9` e Anchor CLI `0.31.1` antes de `anchor test`. Ele também usa a action Rust `risc0/risc0/.github/actions/rustup@main`, que é flutuante e não pode ser tratada como pin de release.
- O [`counter`](https://github.com/boundless-xyz/risc0-solana/tree/v3.0.0/examples/counter) é o exemplo de CPI consultado. No D1a.3, o tag foi clonado em dois checkouts Linux isolados no commit exato; os workspaces on-chain foram resolvidos, checados e testados como Rust host nas duas raias. O guest `hello-world` do RISC Zero, não o host `counter`, foi usado no gate de receipt; nenhum build SBF, CPI ou deploy ocorreu.

A versão da CLI Agave e as versões das crates Solana são dimensões diferentes. A presença de CLI `2.3.9` no workflow não transforma todas as crates em `2.3.9`, nem prova compatibilidade com a recomendação Anchor `2.1.0`.

Os SHA-256 auditados no commit exato são:
`54949aa872bd9d6886eece9c4c964a74f1b42f272abd9a8e1ab89060a4feaf6e`
para `solana-verifier/Cargo.lock`,
`49004c7c7dcedce5d1ebccd00a38356b78b5d42491d92124ba3d8b83551fb38a`
para `examples/counter/Cargo.lock` e
`ab03b523330c4bd66b8ff29eb81ef862cf6f59e56fd4b821dbd60e5648122c15`
para `examples/counter/zkvm/Cargo.lock`. Nenhum desses locks contém source
`git+`, embora o manifesto host declare um patch por branch; o protocolo deve
observar seu efeito por `cargo metadata --locked` sem atualizar os locks.

## Exemplo `counter` e fronteira de workspaces

O [`examples/counter/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/Cargo.toml) define o workspace on-chain (`programs/*` e `shared`). O [`examples/counter/zkvm/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/Cargo.toml) define outro workspace para host/methods. Isso demonstra isolamento estrutural, não compatibilidade automática.

O D1a.3 provou no exemplo, nas duas raias:

- discriminadores, instrução e ordem/flags dos account metas da CPI;
- formato exato do seal Groth16 e do journal público;
- serialização Borsh e bytes produzidos por cada workspace.

Continuam pendentes para o VeriCode:

- Program ID, cluster e programa Router realmente invocado em runtime;
- ImageID e campos do `JournalV1` ligados ao Job;
- rejeição de Job, mint e executor divergentes;
- owner/flags observados durante CPI real, não somente gerados pelos tipos.

O workspace on-chain do `counter` resolve Borsh `0.10.4`; o lock zkVM contém Borsh `0.10.4` e `1.5.7` para dependências diferentes. A compatibilidade deve ser provada por esquema e testes de bytes, não inferida do nome da crate.

### Divergência source/IDL encontrada no D1a.3

No mesmo commit `ee415935d04a948f27a346b563391900bdad6486`, os IDLs
versionados em `solana-verifier/idl/verifier_router.json` e
`solana-verifier/idl/groth_16_verifier.json` não descrevem as assinaturas do
source:

- os IDLs registram `verify(proof, image_id, journal_digest, vk)`;
- o Router source implementa `verify(seal, image_id, journal_digest)`;
- o verificador Groth16 source implementa
  `verify(proof, image_id, journal_digest)`;
- o IDL Router referencia uma seed de argumento `selector` que não aparece na
  lista de argumentos daquele IDL.

O discriminador `verify` coincide (`85 a1 8d 30 78 c6 58 96`), mas o payload
e a seed não. Portanto, o IDL versionado não pode ser promovido como contrato
de ABI. A comparação executável das instruções geradas diretamente pelas
crates passou nas raias A e B; regenerar/comparar o IDL continua pendente.

Os gates Rust passaram nas duas raias: 12 testes do verificador Groth16 e os
testes de identidade do Router/verificador inválido foram aprovados, além do
teste do counter. Esses testes não exercitam CPI, validator, contas reais ou
Program IDs e, por isso, não alteram `STATUS: NÃO VALIDADO`. Cargo também
avisou que o `[patch]` no manifesto do host zkVM é ignorado por não estar na
raiz do workspace, outra razão para não inferir compatibilidade da resolução.

A inspeção também confirmou que `examples/counter/zkvm/host/src/main.rs` cria
um `Keypair`, solicita airdrop e envia transações. Esse binário não foi
executado sem H5, e a ausência dessa execução mantém Router/CPI como
`STATUS: NÃO VALIDADO`.

## Divergência Anchor/Agave

A documentação do Anchor 0.31.x recomenda Agave `2.1.0`. O workflow do Router no tag `v3.0.0` usa `2.3.9`. Não foi encontrada fonte oficial que demonstre o `counter` com `2.1.0`, nem autorização para reclassificar `2.3.9` como versão recomendada do Anchor.

Conclusão: os gates host/ABI passaram nas duas combinações. A raia
Anchor/Agave `2.1.0` foi escolhida como candidata por seguir a recomendação
oficial; coexistência SBF/CPI permanece **a confirmar por integração**. Não
há evidência para chamá-la incompatível.

## Status por rede

| Rede | Status | Evidência encontrada |
| --- | --- | --- |
| localnet (em processo) | VERIFICADO EM `solana-program-test` (D2d, D2e) | D2d: Router `1b26b017…` e verificador `dab6746d…` compilados no Perfil A; `initialize`/`add_verifier` com dono de teste; FIB, PASS e FAIL aceitos (110.851 CU); quatro negativos por vetor rejeitados. D2e: CPI a partir do `vericode_escrow` em `release`/`refund_on_fail` com as receipts reais; negativos antes dos positivos. Sem validator nem deploy. |
| Solana devnet | Router: implantado, imutável, **não inicializado** (inutilizável). Verificador Groth16: implantado, imutável, funcional em simulação. Verificação pelo escrow: **NÃO VALIDADA** (D4b) | D4a: leitura RPC das contas e dos ProgramData, histórico de transações do loader e `simulateTransaction` (`docs/d4a-direct-verifier-results.md`). Achado por leitura on-chain, não em documentação oficial. |
| Solana mainnet-beta | NÃO VALIDADO | Nenhum Program ID/deployment Solana mainnet-beta foi comprovado nas fontes oficiais consultadas. |

O link de deployments no README oficial conduziu a uma página de contratos verificadores EVM, não a uma lista de deployments Solana. Endereço EVM não é evidência de deployment Solana.

Consequências:

- `RISC0_VERIFIER_ROUTER_PROGRAM_ID` permanece vazio: o escrow não usa
  Router desde o D4a;
- não alegar verificação ZK on-chain antes de uma transação de liquidação em
  devnet com CPI bem-sucedida e link do Explorer (D4b);
- até o D2e, a CPI ao Router só foi exercitada em `solana-program-test`
  local (R-D2e R-01). Desde o D4a, a CPI vai direto ao verificador
  `THq1q…`, testada localmente com o rebuild e com o dump de devnet;
- qualquer atestado da plataforma deve ser rotulado como fallback, separado de prova ZK.

## O que precisa ser provado

1. [x] Resolver e registrar o commit exato da tag `v3.0.0`.
2. [x] Reproduzir os gates host seguros do `counter` com Rust pinado e locks
   preservados.
3. [x] Comparar Anchor `0.31.1` + Agave `2.1.0` com Anchor `0.31.1` +
   Agave `2.3.9` no escopo H4.
4. [x] Comparar bytes, discriminadores, metas, flags, owners, seal e journal
   gerados pelas crates; [x] provar CPI runtime (D2e, somente em
   `solana-program-test` local).
5. [x] Program ID e cluster: confirmados por leitura on-chain em devnet no
   D4a. O Router está inutilizável; o verificador `THq1q…` é imutável,
   funcional em simulação e estruturalmente equivalente ao rebuild do commit
   pinado (R-D4a RD4A-05).
6. [x] Validar receipt local, ImageID e rejeição de ImageID/journal
   divergentes; [x] testar Job, mint e executor (D2b.1 e D2e, somente em
   `solana-program-test` local).
7. Comprovar devnet com uma transação de liquidação do escrow (D4b). Até lá,
   a verificação pelo escrow em devnet fica `STATUS: NÃO VALIDADO`.

O workflow oficial executa `solana-keygen new` antes de `anchor test`.
Consequentemente, `anchor test`, validator, deploy, airdrop e transações estão
fora do gate D1a.2 sem chaves e ficam `BLOQUEADOS POR AUTORIZAÇÃO`. O plano
completo, incluindo ABI, seal, journal, ImageID, account metas e testes
negativos, está em [`docs/d1a2-spike-plan.md`](d1a2-spike-plan.md).

## Fontes oficiais consultadas

- [Release `risc0-solana v3.0.0`](https://github.com/boundless-xyz/risc0-solana/releases/tag/v3.0.0)
- [Exemplo `counter` no tag](https://github.com/boundless-xyz/risc0-solana/tree/v3.0.0/examples/counter)
- [Workflow da tag](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/.github/workflows/tests.yml)
- [Manifesto do verificador Groth16](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/groth_16_verifier/Cargo.toml)
- [Lockfile do verificador](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/Cargo.lock)
- [Manifesto do programa `counter`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/programs/solana-counter/Cargo.toml)
- [Lockfile on-chain do `counter`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/Cargo.lock)
- [Lockfile zkVM do `counter`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/zkvm/Cargo.lock)
- [Release notes Anchor 0.31.0 no commit do tag `v0.31.1`](https://github.com/coral-xyz/anchor/blob/47284f8f0b9844c6b83234aa90f556bad00e12ed/docs/content/docs/updates/release-notes/0-31-0.mdx)
- [Documentação RISC Zero de contratos verificadores](https://dev.risczero.com/api/blockchain-integration/contracts/verifier)
