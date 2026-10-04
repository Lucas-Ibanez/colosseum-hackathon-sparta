# Controle autônomo — D2c

## Objetivo atual

`D2c`:
1. escolher por evidência o Perfil A Anchor/Agave/Rust;
2. criar o programa Anchor local `vericode_escrow` (`create_job`, `fund`,
   `refund_on_timeout`), com vault PDA de SPL Token e regra econômica no
   `vericode-core`, testado em processo.

## Marcos anteriores

- D1c2b `CONCLUÍDO`: receipts locais reais `Composite`, não Groth16.
- D2a `4d7e18f`, D2a.1 `0622709`, D2a.2 `402426f`, D2b `58838ae`.
- D2c, código: `a10f026`.
- Router/CPI/devnet: `STATUS: NÃO VALIDADO`.

## Baseline

- Raiz: `/home/lucas/src/vericode`; branch `main`; HEAD de baseline
  `58838ae`.
- Locks inalterados:
  - raiz `191802b2…3b87`;
  - host `f5236689…e226`;
  - guest `1116acef…dbfa`.
- Locks novos:
  - `anchor/Cargo.lock` `19a1db26…6765`;
  - `anchor/tests-local/Cargo.lock` `be94760a…a377`.
- Perfil A (toolchains isoladas): Anchor `0.31.1` + Agave `2.3.9`
  (platform-tools `v1.48`) + Rust host `1.89.0`, em
  `~/.local/share/vericode-spikes/d2c/homes/lane-b` e `tools/lane-b`.
- Core: homes D1a.3 `lane-a` (`1.85.0`) e `lane-b` (`1.89.0`).
- Program ID de localnet: `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`;
  keypair em `d2c/keys`, `0600`.

## Gate atual

`D2c` — Perfil A e programa Anchor local com custódia SPL.

## Estado

`CONCLUÍDO`.
- Perfil A = raia B.
- `.so` `04cc2a84…`.
- 10/10 testes em processo.
- IDL com 3 instruções.
- Core 36/36 nas duas raias.

Commits locais autorizados pelo prompt do gate; push não autorizado.

## Decisões humanas registradas

- D2a.1, D2a.2 e D2b (ver `docs/decisions.md`).
- D2c:
  - workspace `anchor/`;
  - seeds `["job", job_id]`/`["vault", job]`;
  - `refund_on_timeout` permissionless;
  - `solana-program-test =2.3.9`;
  - prosseguir apesar da errata de `~/.cargo`;
  - aceitar o Criterion v2.3.3 baixado pelo SDK Agave.

## Ações proibidas (permanentes salvo novo objetivo)

- Rede fora do autorizado por gate; instalação no perfil padrão; Docker sem
  gate explícito.
- Alterar locks existentes, `JournalV1`, wire format ou
  `docs/manifest-schema.md` sem decisão registrada.
- Keypair fora de gate autorizado, de mainnet, dentro do clone ou com segredo
  exibido.
- Devnet, airdrop, deploy, Router/CPI sem gate autorizado.
- Mock, stub ou fallback apresentado como sucesso real.
- Push, reescrita de histórico, stash, reset ou operação destrutiva.

## Riscos abertos

- Ainda não existem `release`, `refund_on_fail` e verificação de prova; a
  receipt Groth16 não foi produzida.
- Freeze authority/allowlist do mint; upgrade authority no deploy; squatting
  de `job_id`; rent não recuperado.
- Platform-tools e Criterion sem digest oficial publicado.
- Artefatos D1c2b (ELF, receipts, host) apenas em `/tmp`, efêmeros.
- ImageID `4da06f90…fb1a` não recertificado após mudanças no core.
- Revisões adversariais separadas D2b/D2c: pendentes.
- Calendário: dia D8 do cronograma; projeto entre D2 e D3 técnicos.

## Próxima transição permitida

`D2d` — spike do caminho forte de verificação (receipt Groth16 local e
Verifier Router em processo, com teste negativo primeiro), conforme
`docs/handoffs/d2c-to-d2d.md`.
