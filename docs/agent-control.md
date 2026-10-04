# Controle autônomo — D2a.1

## Objetivo atual

`D2a.1` — registrar o guia de produto e a sequência do MVP como documentos de
contexto, unificar a precedência, tabelar conflitos com o estado do
repositório e instituir o protocolo de handoff. Somente documentação; nenhum
código alterado.

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
- HEAD de baseline do D2a.1: `4d7e18f` (D2a).
- Lock raiz `191802b2…3b87`; host `f5236689…e226`; guest `1116acef…dbfa`.
- Toolchains de teste do core: homes isoladas D1a.3 `lane-a` (Rust `1.85.0`)
  e `lane-b` (Rust `1.89.0`), offline.

## Gate atual

`D2a.1` — contexto de produto e protocolo de handoff.

## Estado

`CONCLUÍDO`. Commits locais autorizados pelo humano nesta sessão; push não
autorizado.

## Decisões humanas registradas

- Guia de produto prevalece sobre o D2a quanto a `artifact_hash`
  (`docs/decisions.md`, D2a.1).
- `JobV1` rejeita `buyer == executor` (D2a, mantido).

## Ações proibidas (permanentes salvo novo objetivo)

- Rede, instalação de ferramentas, Docker sem gate explícito.
- Alterar locks, `JournalV1`, wire format ou `docs/manifest-schema.md` sem
  decisão registrada.
- Wallet, seed, keypair, chave privada, `.env` real ou Program ID.
- Solana, Anchor, validator, airdrop, transação, deploy, Router/CPI sem gate
  autorizado.
- Mock, stub ou fallback apresentado como sucesso real.
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- Projeto tecnicamente no D2 em 2026-10-04 (dia D8 do calendário); entrega
  prevista por volta de 8 out.
- Perfil A Anchor/Agave não aprovado: D1b bloqueado; sem Anchor, SPL, devnet
  ou Router.
- Código D2a ainda contém o pré-registro `Delivered` substituído.
- Mapeamento de estados do guia, release após prazo, gatilho on-chain e
  wallets devnet: decisões humanas pendentes (`docs/project-context.md`).
- ImageID `4da06f90…fb1a` não recertificado após o D2a.
- README ainda descreve status "Preflight".

## Próxima transição permitida

`D2b` — alinhar a política pura de escrow ao guia de produto, conforme
`docs/handoffs/d2a1-to-d2b.md`. Paralelamente, o humano deve decidir o
Perfil A (D1b), que é o bloqueio do caminho crítico D2–D4.
