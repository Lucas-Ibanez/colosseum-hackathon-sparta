# Controle autônomo D2a

## Objetivo atual

Concluir exclusivamente o marco D2a do VeriCode: implementar em
`crates/vericode-core` a política pura de escrow — identidades de 32 bytes,
valor validado, `JobV1` imutável, estados e erros explícitos, elegibilidade
de release, validação de `Verdict::Pass`, comparação de todos os
compromissos do journal, executor e mint fixos no Job, rejeição de release
duplicado e ausência de admin bypass — com testes determinísticos e evidência
real. Parar antes de Solana, Anchor, wallet, validator, Router/CPI, rede,
Docker, deploy, front-end, commit ou push.

## Marco anterior

D1c2b `CONCLUÍDO` em 2026-10-03 (`docs/d1c2b3j-final-audit.md`), reverificado
nos arquivos atuais no preflight D2a. Receipts locais são `Composite`, não
Groth16. Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Baseline

- Raiz: `/home/lucas/src/vericode`.
- Branch: `main`.
- HEAD de baseline: `a3eb9e067c6810a4c72fff250047007a5f672acb`.
- Árvore inicial: limpa; `git diff --check` com exit `0`.
- `/home/lucas/.rustup`: ausente.
- Lock raiz (core): SHA-256
  `191802b234a6aa0f6bb9ce58a61963c377aed435a2baa6dec9576d13d8283b87`.
- Toolchains de teste: homes isoladas D1a.3 `lane-a` (Rust `1.85.0`) e
  `lane-b` (Rust `1.89.0`), offline.

## Gate atual

`D2a` — política pura de escrow no core.

## Estado

`CONCLUÍDO` no escopo autorizado; subescopo refund/prazo/timeout em
`AGUARDANDO_AUTORIZAÇÃO`. Alterações sem commit, aguardando revisão humana.

## Decisões humanas deste marco

- `artifact_hash` esperado é registrado uma única vez, somente pelo executor
  do Job, no estado `Funded`.
- `JobV1` rejeita `buyer == executor`.

## Ações autorizadas

- Criar `crates/vericode-core/src/escrow.rs` e declarar o módulo em `lib.rs`.
- Executar `cargo test`/`cargo tree` `--locked --offline` com homes isoladas e
  target fora do clone.
- Criar `docs/escrow-state-machine.md` e
  `docs/d2a-core-escrow-policy-results.md`; atualizar, quando necessário,
  `docs/architecture.md`, `docs/decisions.md`, `docs/evidence.md` e este
  arquivo.

## Ações proibidas

- Acesso à rede; instalar, atualizar ou remover ferramentas; Docker.
- Alterar locks, `Cargo.toml`, `zkvm/`, `JournalV1`, wire format ou
  `docs/manifest-schema.md`.
- Criar `Anchor.toml`, programas, IDL, wallet, seed, keypair, chave privada,
  `.env` ou Program ID.
- Solana, Anchor, validator, airdrop, transação, deploy, Router/CPI, front-end.
- Implementar ou alegar verificação de receipt, seal, Groth16, Router ou CPI.
- Inventar regra de refund, prazo ou timeout.
- Mock, stub ou fallback apresentado como sucesso real.
- Commit, push, operação destrutiva, stash, rebase, amend ou reescrita de
  histórico sem autorização explícita.

## Evidências exigidas

- Preflight Git e verificação do D1c2b nos arquivos atuais.
- Testes determinísticos: release elegível, `Verdict::Fail`, cada compromisso
  divergente, executor divergente, mint divergente, Job não funded, release
  duplicado, estado terminal inválido e ausência de bypass administrativo.
- `cargo test` e `cargo tree` reais nas duas toolchains isoladas.
- Locks inalterados, `git diff --check`, diff integral revisado e busca de
  segredos.

## Riscos abertos

- Política de refund, prazo/timeout, efeito econômico de `Verdict::Fail` e
  quem aciona release on-chain: `AGUARDANDO_AUTORIZAÇÃO`.
- Um Job `Delivered` com artefato FAIL não tem saída até a decisão de refund.
- ImageID `4da06f90…fb1a` não recertificado após a adição do módulo escrow.
- Errata de transcrição no relatório D1c2b.3i registrada em `decisions.md`.
- Rustfmt/Clippy ausentes nas toolchains isoladas; não executados.
- Router/CPI/devnet: `STATUS: NÃO VALIDADO`.

## Última verificação

D2a em 2026-10-04: 37/37 testes, builds e `cargo tree` idênticos com Rust
`1.85.0` e `1.89.0`, locked/offline; locks inalterados; `git diff --check`
exit `0`; nenhuma ocorrência de segredo. Relatório em
`docs/d2a-core-escrow-policy-results.md`.

## Próxima transição permitida

Nenhuma dentro de D2a. Commit local exige autorização explícita. Decisão
humana sobre refund/prazo/`FAIL` antes de qualquer gate Anchor; Router e
devnet exigem objetivo separado.
