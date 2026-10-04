# Controle autônomo — D2b.1 concluído; próximo D2e

## Objetivo atual

Implementar o D2e: `release`/`refund_on_fail` a partir de `Delivered`, com
CPI ao Verifier Router e selector fixado, conforme
`docs/handoffs/d2b1-to-d2e.md`.

## Marcos anteriores

- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c `a10f026`/`ec980e9`, D2d `9a18f71`, D2c.1 `42b4f58`.
- R-D2 `12529b4`: revisão somente leitura sobre `42b4f58`, veredito
  **REPROVADO** para o D2e.
- D2b.1: `0e9838e` (core), `5736565` (anchor) e o commit
  `docs: record D2b.1 delivery binding`.

## Baseline

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record D2b.1 delivery binding`.
- `.so` `ea0dd92c…1a37` (367.288 bytes); escrow 24/24; layout 4/4;
  fixtures 2/2; core 42/42 nas duas raias.
- IDL: 4 instruções, 33 erros.
- Locks:
  - raiz `191802b2…`; host `f5236689…`; guest `1116acef…`;
  - `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`.

## Gate atual

`D2b.1` (concluído) → próximo `D2e`.

## Estado

D2b.1 concluído (`docs/d2b1-delivery-binding-results.md`):
- F-01: compromisso de entrega `deliver` e liquidação por veredito só a
  partir de `Delivered { h }`;
- F-02: termos da v1 admitidos em `create_job`;
- F-07: janela de prazo e executor ≠ PDAs;
- F-08: mint amarrado a `job.mint`;
- F-10: claims corrigidos;
- F-11: lacunas de teste fechadas.

`docs/handoffs/d2d-to-d2e.md` continua **SUBSTITUÍDO**; o D2e vigente está
em `docs/handoffs/d2b1-to-d2e.md`.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1" e
  D2b.1 (`docs/decisions.md`).

## Ações proibidas (permanentes salvo novo objetivo)

- Rede fora do autorizado por gate; instalação no perfil padrão; Docker sem
  gate explícito.
- Alterar locks existentes, `JournalV1`, wire format ou
  `docs/manifest-schema.md` sem decisão registrada.
- Keypair fora de gate autorizado, de mainnet, dentro do clone ou com segredo
  exibido.
- Devnet, airdrop, deploy sem gate autorizado; "ZK on-chain" em cluster.
- Mock, dev mode ou receipt `Fake` como sucesso.
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- F-03, F-06, F-12: condições do D2e.
- F-04 e F-05: bloqueiam o D4.
- F-09, F-13, F-14, F-15: baixos ou informativos.
- Spec v1 trivial: o executor escolhe a entrada, e qualquer `(n, 2n)` passa.
- ImageID admitido depende do ELF D1c2b preservado. Não foi recertificado
  depois das mudanças no core, que é compilado pelo guest.
- Revisão adversarial de D2b.1 + D2e pendente.
- O WSL reinicia e limpa `/tmp`.
- Calendário: dia D8; prazo por volta de 8 out.

## Próxima transição permitida

`D2e`, conforme `docs/handoffs/d2b1-to-d2e.md`, em sessão nova (Opus 5.5,
xhigh, Plan Mode).
