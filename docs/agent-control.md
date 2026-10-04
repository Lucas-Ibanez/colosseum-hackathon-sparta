# Controle autônomo — D2d

## Objetivo atual

`D2d` — spike do caminho forte de verificação: receipts Groth16 reais do
guest VeriCode e Verifier Router de `risc0-solana v3.0.0` em
`solana-program-test`, com testes negativos antes dos positivos. Resultado:
GO ou NO-GO para o D2e.

## Marcos anteriores

- D1c2b `CONCLUÍDO`: receipts locais `Composite`.
- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c: `a10f026` (programa) e `ec980e9` (docs; Perfil A = Anchor `0.31.1` +
  Agave `2.3.9` + Rust `1.89.0`).

## Baseline

- Raiz: `/home/lucas/src/vericode`; branch `main`; HEAD de baseline
  `ec980e9`.
- Locks do repositório inalterados:
  - raiz `191802b2…`;
  - host `f5236689…`;
  - guest `1116acef…`;
  - `anchor/` `19a1db26…`;
  - `anchor/tests-local` `be94760a…`.
- Spike fora do clone: `~/.local/share/vericode-spikes/d2d`.

## Gate atual

`D2d` — Groth16 + Router em processo.

## Estado

`CONCLUÍDO — GO`.
- Groth16 PASS/FAIL reais verificados localmente.
- Router e verificador aceitaram FIB, PASS e FAIL (110.851 CU) e rejeitaram
  proof adulterada, selector desconhecido, ImageID errado, journal digest
  errado e `add_verifier` por não-dono.
- Commit documental autorizado; push não autorizado.

## Decisões humanas registradas

- D2a.1, D2a.2, D2b e D2c (`docs/decisions.md`).
- D2d:
  - pull da imagem `risczero/risc0-groth16-prover:v2025-04-03.1` por digest
    amd64 `7f173963…`;
  - spike fora do clone;
  - orçamento de 3 h (usados cerca de 41 min).

## Ações proibidas (permanentes salvo novo objetivo)

- Rede fora do autorizado por gate; instalação no perfil padrão; Docker sem
  gate explícito.
- Alterar locks existentes, `JournalV1`, wire format ou
  `docs/manifest-schema.md` sem decisão registrada.
- Keypair fora de gate autorizado, de mainnet, dentro do clone ou com segredo
  exibido.
- Devnet, airdrop, deploy sem gate autorizado.
- Alegar "ZK on-chain" em cluster.
- Mock, dev mode ou receipt `Fake` como sucesso.
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- Prover Groth16 com margem de RAM estreita (cerca de 6,25 de 7,6 GiB);
  container sem isolamento de rede.
- Router em devnet: Program ID, dono e deployment não confirmados.
- CPI a partir do `vericode_escrow` ainda inexistente.
- Freeze authority/allowlist do mint, upgrade authority, squatting de
  `job_id` e rent (D2c).
- Vetores Groth16 só fora do clone; ImageID não recertificado.
- Revisões adversariais D2b/D2c pendentes.
- Calendário: dia D8; prazo por volta de 8 out.

## Próxima transição permitida

`D2e` — `release`/`refund_on_fail` no `vericode_escrow` com CPI ao Verifier
Router, testados em processo, conforme `docs/handoffs/d2d-to-d2e.md`.
