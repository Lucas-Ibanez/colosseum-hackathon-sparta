# Controle autônomo — D2c.1

## Objetivo atual

`D2c.1`:
1. rejeitar no `create_job` mints com freeze authority;
2. versionar no repositório os vetores Groth16 PASS/FAIL do D2d como fixtures
   em hex;
3. preparar o prompt da revisão adversarial separada do D2b/D2c/D2c.1.

## Marcos anteriores

- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c: `a10f026` (programa) e `ec980e9` (docs; Perfil A).
- D2d: `9a18f71` (GO: Groth16 + Router em `solana-program-test`).

## Baseline

- Raiz `/home/lucas/src/vericode`; branch `main`; HEAD de baseline `9a18f71`.
- Locks inalterados no gate:
  - raiz `191802b2…`; host `f5236689…`; guest `1116acef…`;
  - `anchor/` `19a1db26…`; `anchor/tests-local` `be94760a…`.
- Ambientes isolados: `vericode-spikes/d2c` (Perfil A) e `vericode-spikes/d2d`
  (spike e artefatos persistentes).

## Gate atual

`D2c.1` — mint sem freeze authority e fixtures Groth16.

## Estado

`CONCLUÍDO`.
- `.so` `d66ac76b…`.
- Escrow 12/12 e fixtures 2/2.
- IDL com 3 instruções e 25 erros.
- Core 36/36.

**Commit pendente de autorização humana.** Mensagem sugerida:
`anchor: reject freezable mints and add Groth16 fixtures (D2c.1)`.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b, D2c e D2d (`docs/decisions.md`).
- D2c.1: resolver freeze authority e fixtures antes do D2e; revisão
  adversarial em outra sessão.

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

- Revisão adversarial separada do D2b/D2c/D2c.1: pendente.
- Upgrade authority, squatting de `job_id`, rent.
- Router em devnet não confirmado; CPI do escrow inexistente (D2e).
- Margem de RAM do prover; ImageID não recertificado. As fixtures dependem do
  ELF D1c2b preservado em `vericode-spikes/d2d/artifacts`.
- O WSL reinicia e limpa `/tmp`: manter artefatos sempre em
  `~/.local/share/vericode-spikes`.
- Calendário: dia D8; prazo por volta de 8 out.

## Próxima transição permitida

1. Revisão adversarial separada, somente leitura, conforme
   `docs/handoffs/d2c1-to-review-d2b-d2c.md`.
2. Depois, `D2e` conforme `docs/handoffs/d2d-to-d2e.md`, tratando antes os
   achados bloqueantes da revisão.
