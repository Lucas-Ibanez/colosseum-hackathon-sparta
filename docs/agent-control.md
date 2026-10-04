# Controle autônomo — D2a.2

## Objetivo atual

`D2a.2` — registrar as autorizações humanas de 2026-10-04 (Perfil A
delegado, keypairs efêmeros de devnet/localnet, README) nos documentos de
regra e contexto. Somente documentação; nenhum código, lock, ferramenta ou
keypair criado.

## Marcos anteriores

- D1c2b `CONCLUÍDO` em 2026-10-03 (`docs/d1c2b3j-final-audit.md`): receipts
  locais reais `Composite`, não Groth16.
- D2a `CONCLUÍDO` em 2026-10-04, commit `4d7e18f`
  (`docs/d2a-core-escrow-policy-results.md`): política pura de escrow, 37/37
  testes em Rust `1.85.0` e `1.89.0`. O item de pré-registro do
  `artifact_hash` foi substituído pelo D2a.1 e será corrigido no D2b.
- Router/CPI/devnet: `STATUS: NÃO VALIDADO`.

## Baseline

- Raiz: `/home/lucas/src/vericode`; branch `main`.
- HEAD de baseline do D2a.2: `0622709` (D2a.1).
- Lock raiz `191802b2…3b87`; host `f5236689…e226`; guest `1116acef…dbfa`.
- Toolchains de teste do core: homes isoladas D1a.3 `lane-a` (Rust `1.85.0`)
  e `lane-b` (Rust `1.89.0`), offline.

## Gate atual

`D2a.2` — registro das autorizações humanas: Perfil A delegado ao agente,
keypairs efêmeros de devnet/localnet autorizados e README corrigido. O D2a.1
(contexto de produto e protocolo de handoff) foi concluído no commit
`0622709`.

## Estado

`CONCLUÍDO`, alterações ainda sem commit: o commit
`docs: record delegated toolchain and devnet key authority` aguarda
autorização explícita. Push não autorizado.

## Decisões humanas registradas

- Guia de produto prevalece sobre o D2a quanto a `artifact_hash`
  (`docs/decisions.md`, D2a.1).
- Escolha do Perfil A Anchor/Agave delegada ao agente, a ser feita no D2c com
  evidência (`docs/decisions.md`, D2a.2).
- Agente pode criar keypairs efêmeros somente de devnet/localnet, fora do
  clone, `0600`, sem exibir segredos (`AGENTS.md` §9; D2a.2).
- `JobV1` rejeita `buyer == executor` (D2a, mantido).

## Ações proibidas (permanentes salvo novo objetivo)

- Rede, instalação de ferramentas, Docker sem gate explícito.
- Alterar locks, `JournalV1`, wire format ou `docs/manifest-schema.md` sem
  decisão registrada.
- Keypair fora de gate autorizado, de mainnet, dentro do clone ou com segredo
  exibido; seed/chave privada impressa ou versionada; `.env` real versionado.
- Solana, Anchor, validator, airdrop, transação, deploy, Router/CPI sem gate
  autorizado.
- Mock, stub ou fallback apresentado como sucesso real.
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- Projeto tecnicamente no D2 em 2026-10-04 (dia D8 do calendário); entrega
  prevista por volta de 8 out.
- Perfil A Anchor/Agave ainda não escolhido (delegado ao agente, gate D2c);
  até lá, sem Anchor, SPL, devnet ou Router.
- Código D2a ainda contém o pré-registro `Delivered` substituído.
- Mapeamento de estados do guia, release após prazo e gatilho on-chain:
  decisões humanas pendentes, a confirmar no Plan Mode do D2b.
- ImageID `4da06f90…fb1a` não recertificado após o D2a.

## Próxima transição permitida

`D2b` — alinhar a política pura de escrow ao guia de produto, conforme
`docs/handoffs/d2a1-to-d2b.md`. Em seguida, o D2c: escolha do Perfil A pelo
agente e skeleton Anchor local com custódia SPL.
