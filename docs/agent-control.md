# Controle autônomo — D2b

## Objetivo atual

`D2b` — alinhar a política pura de escrow de `crates/vericode-core` ao guia
de produto: release ao executor em `Pass` válido até o prazo, refund ao buyer
em `Fail` válido, refund por timeout somente após `deadline_slot`, e
`artifact_hash` registrado apenas na liquidação.

## Marcos anteriores

- D1c2b `CONCLUÍDO` (`docs/d1c2b3j-final-audit.md`): receipts locais reais
  `Composite`, não Groth16.
- D2a `4d7e18f`: política pura inicial (`docs/d2a-core-escrow-policy-results.md`).
- D2a.1 `0622709`: guia de produto, sequência e protocolo de handoff.
- D2a.2 `402426f`: Perfil A delegado ao agente; keypairs efêmeros de
  devnet/localnet autorizados; README corrigido.
- Router/CPI/devnet: `STATUS: NÃO VALIDADO`.

## Baseline

- Raiz: `/home/lucas/src/vericode`; branch `main`; HEAD de baseline
  `402426f`.
- Lock raiz `191802b2…3b87`; host `f5236689…e226`; guest `1116acef…dbfa`.
- Toolchains de teste do core: homes isoladas D1a.3 `lane-a` (Rust `1.85.0`)
  e `lane-b` (Rust `1.89.0`), offline.

## Gate atual

`D2b` — política pura de escrow alinhada ao guia.

## Estado

`CONCLUÍDO`. 36/36 testes nas duas raias. Commit local autorizado pelo prompt
do gate; push não autorizado.

## Decisões humanas registradas

- Guia prevalece sobre o D2a quanto a `artifact_hash` (D2a.1).
- Estados econômicos `Created`, `Funded`, `Released`, `Refunded`; release só
  até o prazo; refund por `Fail` em qualquer slot (D2b).
- Perfil A delegado ao agente (D2a.2).
- Keypairs efêmeros só de devnet/localnet, fora do clone, `0600`, sem exibir
  segredos (`AGENTS.md` §9; D2a.2).
- `JobV1` rejeita `buyer == executor` (D2a).

## Ações proibidas (permanentes salvo novo objetivo)

- Rede, instalação de ferramentas ou Docker sem gate explícito.
- Alterar locks, `JournalV1`, wire format ou `docs/manifest-schema.md` sem
  decisão registrada.
- Keypair fora de gate autorizado, de mainnet, dentro do clone ou com segredo
  exibido; seed/chave privada impressa ou versionada; `.env` real versionado.
- Devnet, airdrop, transação, deploy, Router/CPI sem gate autorizado.
- Mock, stub ou fallback apresentado como sucesso real.
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- Projeto no D2/D3 técnico em 2026-10-04 (dia D8 do calendário); entrega por
  volta de 8 out.
- Perfil A ainda não escolhido; platform-tools SBF nunca baixadas
  (`~/.cache/solana` ausente).
- ImageID `4da06f90…fb1a` não recertificado após as mudanças no core.
- Revisão adversarial separada da política de escrow: pendente.
- Invariantes 1, 6 e 10 do guia dependem do programa Anchor.

## Próxima transição permitida

`D2c` — escolha do Perfil A pelo agente e programa Anchor local com Job e
custódia SPL, conforme `docs/handoffs/d2b-to-d2c.md`.
