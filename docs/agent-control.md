# Controle autônomo — D2e concluído (local); próximo R-D2e

## Objetivo atual

Revisão adversarial separada, somente leitura, de D2b.1 + D2e antes de
qualquer gate devnet, conforme `docs/handoffs/d2e-to-r-d2e.md`.

## Marcos anteriores

- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c `a10f026`/`ec980e9`, D2d `9a18f71`, D2c.1 `42b4f58`.
- R-D2 `12529b4`: **REPROVADO** para o D2e original.
- D2b.1 `0e9838e` (core), `5736565` (anchor), `75a1971` (docs).
- D2e: `2f10a8f` (anchor) e o commit `docs: record Router settlement (D2e)`.

## Baseline

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record Router settlement (D2e)`.
- `.so` `6457aecf…ca96` (398.504 bytes). No mesmo out-dir: Router
  `1b26b017…` e verificador `dab6746d…`.
- Testes: escrow 25/25; settlement 16/16; layout 6/6; fixtures 2/2; core
  42/42 nas duas raias.
- IDL: 6 instruções, 36 erros.
- Locks:
  - raiz `191802b2…`; host `f5236689…`; guest `1116acef…`;
  - `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`.

## Gate atual

`D2e` (concluído) → próximo `R-D2e`.

## Estado

D2e concluído localmente (`docs/d2e-router-settlement-results.md`).
`release` e `refund_on_fail` partem de `Delivered`, exigem a ATA canônica e o
selector `73c457ba`, e verificam a prova por CPI ao Verifier Router em
`solana-program-test` local. R-D2: F-01, F-02, F-03, F-06, F-07, F-08, F-10,
F-11 e F-12 tratados. Router em devnet: `STATUS: NÃO VALIDADO`.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1",
  D2b.1 e D2e (`docs/decisions.md`).

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

- F-04: upgrade authorities do escrow, do Router e do verificador. Bloqueia
  o D4.
- F-05: allowlist do mint. Bloqueia o D4.
- Router em devnet não confirmado.
- Confiança residual no e-stop do dono do Router: o timeout ainda devolve.
- ATA precisa existir antes da liquidação.
- F-09, F-13, F-14, F-15: baixos ou informativos.
- Spec v1 trivial; ImageID admitido não recertificado.
- O WSL reinicia e limpa `/tmp`.
- Calendário: dia D8; prazo por volta de 8 out.

## Próxima transição permitida

`R-D2e`, conforme `docs/handoffs/d2e-to-r-d2e.md`, em sessão nova e
separada (Opus 5.5, max, somente leitura). Depois, as correções que ela
exigir ou o gate devnet (D4).
