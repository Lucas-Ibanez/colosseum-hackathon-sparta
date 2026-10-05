# Controle autônomo — R-D4a registrado (APROVADO COM RESSALVAS); próximo D4b

## Objetivo atual

D4b, conforme `docs/handoffs/r-d4a-to-d4b.md`: deploy do escrow em devnet,
smoke, finalização da upgrade authority, Jobs PASS, FAIL e timeout,
negativos no Explorer e C7, sob as condições CD1 a CD9 do R-D4a
(`docs/r-d4a-review-results.md`).

## Marcos anteriores

- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c `a10f026`/`ec980e9`, D2d `9a18f71`, D2c.1 `42b4f58`.
- R-D2 `12529b4`: **REPROVADO** para o D2e original.
- D2b.1 `0e9838e` (core), `5736565` (anchor), `75a1971` (docs).
- D2e `2f10a8f` (anchor), `c0aba7d` (docs).
- R-D2e `2d76441`: **APROVADO COM RESSALVAS** para o D4.
- Prazo, congelamento do `JournalV1` e nomes de gate: `cfdd9d7`.
- D4a: `fb4bfba` (anchor) e `24364ae` (docs).
- R-D4a: **APROVADO COM RESSALVAS** para o D4b; registrado no commit
  `docs: record R-D4a delta review`.

## Baseline (D4a)

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline = commit
  `docs: record R-D4a delta review`.
- `.so` `cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133`
  (395.064 bytes).
- Verificador: rebuild `dab6746d…` e dump de devnet `34ae6e5c…`.
- Testes: escrow 26, settlement 15, regressions 7, layout 7, fixtures 2
  (57/57), nas duas variantes do verificador; core 42/42 nas duas raias.
- IDL `e8ce2c20…`: 6 instruções, 37 erros.
- Locks inalterados: raiz `191802b2…`; host `f5236689…`; guest `1116acef…`;
  `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`.
- Fora do clone: `~/.local/share/vericode-spikes/d4/` com
  - `keys/`: deployer `617ogw9T…`, buyer `EZgGUg4J…`, executor `EdB25bVh…`
    e mint `9TE2VPFm…`;
  - `receipts-out/` (S, A, A′, B), `jobs/`, `logs/`, `out/` e `bin/`.

## Gate atual

`R-D4a` (registrado) → próximo `D4b`.

## Estado

- O Router upstream de devnet está implantado e imutável, mas não
  inicializado. O verificador upstream `THq1q…` é imutável e nunca poderá
  ser registrado nesse Router.
- Por decisão humana (caminho (b′)), o escrow chama o verificador por CPI,
  direto, sem Router. Além disso:
  - só aceita o mint Test USDC `9TE2VPFm…` (6036);
  - o `JournalV1` v1 está congelado;
  - os PoCs 1 a 7 do R-D2e estão na suíte;
  - as receipts S, A, A′ e B estão prontas.
- Nada foi escrito em devnet. O deployer tem 5 SOL (faucet web, pelo
  humano); o D4b gasta cerca de 2,03 SOL.
- R-D4a: verificador de devnet estrutural e funcionalmente equivalente ao
  rebuild; condições CD1 a CD9 para o D4b.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c, D2d, D2c.1, "Decisões humanas para o D2b.1",
  D2b.1, D2e, R-D2e, "Decisões humanas para o D4", "Prazo de 11/10,
  congelamento do JournalV1 v1 e nomes de gate", D4a e R-D4a
  (`docs/decisions.md`).

## Ações proibidas (permanentes salvo novo objetivo)

- Rede fora do autorizado por gate; instalação no perfil padrão; Docker sem
  gate explícito.
- Alterar locks existentes, `JournalV1` (v1 congelado), wire format ou o
  core sem novo `schema_version` e decisão registrada.
- Keypair fora de gate autorizado, de mainnet, dentro do clone ou com segredo
  exibido.
- Devnet, airdrop, deploy sem gate autorizado; "ZK on-chain" ou "verificado
  on-chain" sem transação de liquidação em devnet com CPI bem-sucedida e link
  do Explorer.
- Chamar a verificação de "Verifier Router" depois do D4a.
- Mock, dev mode ou receipt `Fake` como sucesso.
- Builds, testes ou provas pesadas em paralelo (7,6 GiB de RAM).
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- Sem e-stop contra bug de soundness do verificador (decisão humana D4a).
- Bytes do verificador de devnet ≠ rebuild local só por artefatos do build
  em macOS; a equivalência é estrutural e funcional (RD4A-05).
- F-04: upgrade authority do escrow a finalizar no D4b, depois do smoke.
- R-02: `fund` fora da janela, mitigado por `create_job`+`fund` atômicos.
- R-03: replay entre implantações, mitigado por `job_id` aleatório.
- RD4A-01: seed phrase impressa pela CLI se o deploy falhar sem
  `--buffer` (CD4).
- RD4A-02/03: criação do mint irreversível; endereços públicos (CD2, CD3).
- RD4A-08: vários `.so` fora do clone; implantar só `cdf6967f…` (CD1).
- RD4A-07 (a), (e), (f): comentários de teste, `.env.example` e testes
  novos ficam para o D7.
- Endereço do mint pré-financiado por terceiros pode fazer
  `create_account` falhar; o D4b deve criar o mint com
  transfer+allocate+assign ou checar antes.
- ATA precisa existir antes da liquidação.
- F-09, F-13, F-14; ImageID não recertificado; spec v1 trivial.
- Margem de RAM do prover (cerca de 160 MiB no pico).
- O WSL reinicia e limpa `/tmp`.
- Calendário: 2026-10-05; prazo de entrega 11/10.

## Próxima transição permitida

`D4b`, conforme `docs/handoffs/r-d4a-to-d4b.md`, em sessão nova (Opus 5.5,
xhigh, Plan Mode antes da primeira escrita em devnet). Finalização da
upgrade authority só depois do smoke; depois, `D7`.
