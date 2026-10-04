# D2c.1 — mint sem freeze authority e fixtures Groth16 versionadas

Data: 2026-10-04

## Resultado

**CONCLUÍDO.** Dois riscos abertos foram tratados antes do D2e.

1. **Freeze authority do mint.**
   - `create_job` passou a rejeitar mints com freeze authority (erro 6024,
     `MintHasFreezeAuthority`).
   - Sem isso, um buyer poderia depositar e depois congelar o vault,
     impedindo o pagamento ao executor mesmo com `Pass` válido.
   - O SPL Token `7.0.0` não permite adicionar freeze authority a um mint
     criado sem ela (`processor.rs`: `MintCannotFreeze`), então a checagem na
     criação basta.
2. **Fixtures Groth16.**
   - Os vetores públicos PASS/FAIL do D2d (selector, ImageID, journal, digest
     e seal) foram versionados em hex em `anchor/tests-local/fixtures/groth16/`.
   - Um teste confere hashes e vínculo com o Job pelo core.
   - O D2e passa a ser reproduzível a partir do repositório, sem Docker nem
     prova.

Nenhuma mudança em `crates/vericode-core`, `JournalV1`, `zkvm/` ou locks. Não
houve rede, Docker, devnet, deploy nem push.

## Preflight

- HEAD `9a18f71`; árvore limpa; `git diff --check` exit `0`.
- Hashes de fontes, locks, harness do spike e vetores D2d (`sha256sum -c`)
  iguais aos registrados.
- Plan Mode aprovado.

**Incidente:** o WSL foi reiniciado às 19:01 durante o gate.
- O primeiro build falhou com exit `127`, porque o helper de ambiente estava
  no scratchpad em `/tmp`, que foi limpo.
- Os artefatos originais do D1c2b em `/tmp` também sumiram. As cópias
  persistentes em `vericode-spikes/d2d/artifacts` conferem por `sha256sum -c`
  (6/6), e os vetores D2d conferem (17/17).
- O helper foi recriado com o mesmo conteúdo registrado no D2c e o build foi
  repetido.

## Diff

| Arquivo | Alteração |
| --- | --- |
| `anchor/programs/vericode-escrow/src/lib.rs` | constraint `mint.freeze_authority.is_none() @ MintHasFreezeAuthority` em `CreateJob`; erro 6024 acrescentado ao fim do enum (códigos 6000–6023 inalterados) |
| `anchor/tests-local/tests/escrow.rs` | helper `create_mint_with_freeze`; testes `create_job_rejects_a_mint_with_freeze_authority` e `freeze_authority_cannot_be_added_to_the_job_mint` |
| `anchor/tests-local/tests/groth16_fixtures.rs` | novo: 2 testes de consistência das fixtures |
| `anchor/tests-local/fixtures/groth16/{pass,fail}.txt`, `README.md` | novos: vetores em hex e proveniência |
| `docs/escrow-program.md`, `decisions.md`, `evidence.md`, `agent-control.md`, `project-context.md`, `handoffs/d2d-to-d2e.md` | registro |
| `docs/handoffs/d2c1-to-review-d2b-d2c.md` | prompt da revisão adversarial separada |

A checagem de freeze authority fica no programa, não no core. É uma
pré-condição de custódia sobre a conta SPL, e o core não conhece propriedades
do mint.

## Comandos e saídas reais

Perfil A, ambiente isolado `d2c` lane-b (`env -i`, homes isoladas, target
fora do clone).

```text
cargo-build-sbf --manifest-path programs/vericode-escrow/Cargo.toml --sbf-out-dir d2c/out/d2c1 -- --locked
→ exit 0, 1m30s; 13 warnings, todos da expansão de macros do Anchor 0.31.1 (como no D2c)
→ vericode_escrow.so 298.224 bytes, SHA-256 d66ac76bc8ef66488facc6473b7679eac2fa5e605fba409a6cd61acce20baaae

SBF_OUT_DIR=d2c/out/d2c1 cargo +1.89.0 test --locked   (anchor/tests-local)
→ exit 0
  tests/escrow.rs: 12 passed; 0 failed
    (inclui create_job_rejects_a_mint_with_freeze_authority e
     freeze_authority_cannot_be_added_to_the_job_mint)
  tests/groth16_fixtures.rs: 2 passed; 0 failed

anchor-0.31.1 idl build -p vericode_escrow
→ exit 0; instruções [create_job, fund, refund_on_timeout];
  25 erros (6024 MintHasFreezeAuthority);
  SHA-256 ec12eea6c485174fca456fb128db67271446ef161343c0dd8bfc1b5fbcbc51fa (não versionada)

core: cargo +1.85.0 / +1.89.0 test --locked --offline → 36 passed nas duas raias
```

Sem warning no código de teste.

## Testes novos

| Teste | Verifica |
| --- | --- |
| `create_job_rejects_a_mint_with_freeze_authority` | mint com `freeze_authority = Some` → erro 6024; Job e vault não criados |
| `freeze_authority_cannot_be_added_to_the_job_mint` | `SetAuthority(FreezeAccount)` no mint aceito → `TokenError::MintCannotFreeze` (premissa que torna suficiente checar só na criação) |
| `pass_fixture_is_bound_to_the_vericode_job` | ver abaixo |
| `fail_fixture_is_bound_to_the_vericode_job` | ver abaixo |

Os testes de fixture conferem, para PASS e FAIL:
- selector `73c457ba` e seal de 256 bytes;
- SHA-256 de seal e journal iguais ao D2d;
- `journal_digest == SHA-256(journal)`;
- ImageID `4da06f90…`;
- `JournalV1` decodificado e vinculado ao Job `[0x11; 32]`, com spec e
  harness canônicos e artefato `(7,14)`/`(7,15)`; veredito `Pass`/`Fail`.

## Fixtures

| Arquivo | SHA-256 |
| --- | --- |
| `fixtures/groth16/pass.txt` | `b3b7977dae666c8a268d406aee38a6bdb251f6327687fdbff92adacab28d22df` |
| `fixtures/groth16/fail.txt` | `0f6bf3cbcef06ca177144fdc6f1000817c67804ecfca2ef2a88eb26aa4688722` |

Os arquivos contêm só dados públicos. O seal é o valor cru, com `pi_a` não
negado. Eles não são verificação on-chain por si.

## Locks e fronteiras

- Inalterados:
  - `anchor/Cargo.lock` `19a1db26…6765`;
  - `anchor/tests-local/Cargo.lock` `be94760a…a377`;
  - raiz `191802b2…`; host `f5236689…`; guest `1116acef…`.
- Listagem de `~/.cargo` com o mesmo SHA-256 `d9e12578…`; `~/.avm`
  inalterado.
- `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes.
- Nenhuma escrita nova em `~/.docker`.
- `git diff --check` exit `0`; nenhum segredo, keypair ou `.env` no diff.

## Riscos abertos

- Os demais riscos do D2c/D2d permanecem: upgrade authority, squatting de
  `job_id`, rent, Router em devnet, margem de RAM do prover, ImageID não
  recertificado.
- As fixtures dependem do ELF D1c2b preservado em
  `vericode-spikes/d2d/artifacts`. Um rebuild do guest com o core atual pode
  gerar outro ImageID.
- Revisão adversarial separada do D2b/D2c/D2c.1: **pendente**, com prompt em
  `docs/handoffs/d2c1-to-review-d2b-d2c.md`.
