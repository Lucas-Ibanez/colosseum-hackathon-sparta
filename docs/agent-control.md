# Controle autônomo — R-D2 registrado; próximo D2b.1

## Objetivo atual

Registrar a revisão adversarial R-D2 (reprovação do D2e) e preparar o gate
D2b.1 com as decisões humanas já tomadas.

## Marcos anteriores

- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c `a10f026`/`ec980e9`, D2d `9a18f71`, D2c.1 `42b4f58`.
- R-D2: revisão somente leitura sobre `42b4f58`, veredito **REPROVADO** para
  o D2e; registrada no commit `docs: record R-D2 adversarial review`.

## Baseline

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline `42b4f58`.
- `.so` `d66ac76b…`; escrow 12/12; fixtures 2/2; core 36/36.
- Locks:
  - raiz `191802b2…`; host `f5236689…`; guest `1116acef…`;
  - `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`.

## Gate atual

`R-D2` (registro) → próximo `D2b.1`.

## Estado

R-D2 registrado. `D2b.1` liberado com decisões humanas fechadas (F-01 opção A,
F-02 termos admitidos, F-07 janela de prazo, F-08/F-11 incluídos).
`docs/handoffs/d2d-to-d2e.md` está marcado como **SUBSTITUÍDO**.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1 e "Decisões humanas para o D2b.1"
  (`docs/decisions.md`).

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

- F-01/F-02/F-07 (a corrigir no D2b.1).
- F-03, F-06, F-12 (condições do D2e).
- F-04 e F-05 (bloqueiam D4).
- F-09, F-13, F-14, F-15 (baixos/informativos).
- Spec v1 trivial: qualquer `(n, 2n)` passa.
- ImageID admitido depende do ELF D1c2b preservado.
- O WSL reinicia e limpa `/tmp`.
- Calendário: dia D8; prazo por volta de 8 out.

## Próxima transição permitida

`D2b.1`, conforme `docs/handoffs/r-d2-to-d2b1.md`, em sessão nova (Opus 5.5,
xhigh, Plan Mode).
