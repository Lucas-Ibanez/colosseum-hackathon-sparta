# Controle autônomo — R-D2e registrado (APROVADO COM RESSALVAS); próximo D4

## Objetivo atual

Gate devnet (D4): escrow em Solana devnet com Test USDC, Router verificado e
links do Explorer, conforme `docs/handoffs/r-d2e-to-d4.md` e as "Decisões
humanas para o D4" (`docs/decisions.md`).

## Marcos anteriores

- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c `a10f026`/`ec980e9`, D2d `9a18f71`, D2c.1 `42b4f58`.
- R-D2 `12529b4`: **REPROVADO** para o D2e original.
- D2b.1 `0e9838e` (core), `5736565` (anchor), `75a1971` (docs).
- D2e `2f10a8f` (anchor), `c0aba7d` (docs).
- R-D2e: **APROVADO COM RESSALVAS** para o D4; registrado no commit
  `docs: record R-D2e adversarial review`.

## Baseline

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record R-D2e adversarial review`.
- `.so` `6457aecf…ca96` (398.504 bytes). No mesmo out-dir: Router
  `1b26b017…` e verificador `dab6746d…`.
- Testes: escrow 25/25; settlement 16/16; layout 6/6; fixtures 2/2; core
  42/42 nas duas raias.
- IDL `37a3028a…`: 6 instruções, 36 erros.
- Locks:
  - raiz `191802b2…`; host `f5236689…`; guest `1116acef…`;
  - `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`.

## Gate atual

`R-D2e` (registrado) → próximo `D4`.

## Estado

R-D2e (`docs/r-d2e-adversarial-review-results.md`): nenhum achado crítico ou
alto no código de D2b.1/D2e. O deploy depende das condições C1 a C7. As
decisões D4-0 a D4-6 foram delegadas pelo humano e tomadas pelo agente:
- Router upstream verificado por RPC; se não for verificável, Router próprio
  local e parada para R-D4a;
- escrow `GZqbL2Tb…` com upgrade authority finalizada depois de um smoke
  run;
- mint conferido pelo cliente (ou allowlist no programa, em (b));
- `job_id` aleatório e receipts novas;
- `create_job`+`fund` atômicos.

Errata R-04 aplicada em `router-notes.md` e `escrow-program.md`. Router em
devnet: `STATUS: NÃO VALIDADO`.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1",
  D2b.1, D2e, R-D2e e "Decisões humanas para o D4" (`docs/decisions.md`).

## Ações proibidas (permanentes salvo novo objetivo)

- Rede fora do autorizado por gate; instalação no perfil padrão; Docker sem
  gate explícito.
- Alterar locks existentes, `JournalV1`, wire format ou
  `docs/manifest-schema.md` sem decisão registrada.
- Keypair fora de gate autorizado, de mainnet, dentro do clone ou com segredo
  exibido.
- Devnet, airdrop, deploy sem gate autorizado; "ZK on-chain" em cluster sem
  transação de CPI ao Router bem-sucedida.
- Mock, dev mode ou receipt `Fake` como sucesso.
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- F-04: upgrade authorities. O D4 finaliza a do escrow; em (a), as do Router
  e do verificador upstream ficam como confiança explícita.
- F-05: mint sem allowlist no programa em (a); conferido pelo cliente.
- R-01: Router em devnet não confirmado.
- R-02: `fund` fora da janela; mitigado por `create_job`+`fund` atômicos.
- R-03: replay entre implantações; mitigado por `job_id` aleatório.
- R-05: PoCs do R-D2e ainda fora da suíte.
- R-06: e-stop irreversível do dono do Router como alavanca de liveness.
- R-07: margens de tamanho (114 bytes) e CU.
- ATA precisa existir antes da liquidação.
- F-09, F-13, F-14: baixos ou informativos.
- Spec v1 trivial; ImageID admitido não recertificado.
- O WSL reinicia e limpa `/tmp`.
- Calendário: dia D9 (2026-10-05); prazo por volta de 8 out.

## Próxima transição permitida

`D4`, conforme `docs/handoffs/r-d2e-to-d4.md`, em sessão nova (Opus 5.5,
xhigh, Plan Mode). Se o Router upstream não for verificável: Router próprio
local e parada para a revisão delta R-D4a antes de qualquer escrita em
devnet.
