# Decisões

Registre decisões relevantes do projeto neste formato.

## Template

- **Data:**
- **Decisão:**
- **Motivo:**
- **Evidência:**
- **Risco aberto:**

## 2026-09-28 — D0: fronteiras do MVP

- **Data:** 2026-09-28
- **Decisão:** restringir a avaliação a artefato serializado, core Rust puro e `JournalV1` mínimo; manter integração Router como não validada.
- **Motivo:** separar a regra determinística dos alvos incompatíveis do guest e Anchor e impedir alegações sem evidência.
- **Evidência:** `docs/architecture.md`, `docs/manifest-schema.md`, `docs/zkvm-notes.md` e `docs/router-notes.md`.
- **Risco aberto:** versão zkVM, serialização do journal, ImageID, Program ID/cluster do Router e CPI ainda dependem de spikes com saída real.

## 2026-09-28 — D1a: ambiente e conjunto de versões proposto (substituída por D1a.1)

- **Data:** 2026-09-28
- **Decisão:** registro histórico substituído. A proposta combinava Anchor `0.31.1` com Agave `2.3.9` e tratava a CI do Router como comprovação suficiente.
- **Motivo:** a correção D1a.1 identificou que a fonte oficial do Anchor 0.31.x recomenda Agave `2.1.0`, enquanto `2.3.9` aparece apenas no workflow do tag `risc0-solana v3.0.0`.
- **Evidência:** `docs/toolchain-matrix.md`, `docs/bootstrap-plan.md`, tag `risc0/risc0 v3.0.3`, tag `boundless-xyz/risc0-solana v3.0.0`, tag Anchor `v0.31.1`, release Agave `v2.3.9` e documentação oficial citada nesses documentos.
- **Risco aberto:** esta entrada não autoriza instalação; consultar a decisão D1a.1 abaixo.
- **Próximo gate:** substituído por D1a.1.

## 2026-09-28 — D1a.1: bloquear bootstrap até demonstrar a interseção

- **Data:** 2026-09-28
- **Decisão:** manter como referências separadas (a) Anchor/AVM/crates `0.31.1` com Agave CLI `2.1.0`, (b) RISC Zero `v3.0.3` com Rust host `1.89.0`, guest `1.88.0`, `rzup 0.5.1` e SDK/cargo-risczero `3.0.3`, e (c) `risc0-solana v3.0.0`, cujo workflow usa Anchor `0.31.1` + Agave CLI `2.3.9`. Declarar **SEM PERFIL PRONTO PARA INSTALAÇÃO** e bloquear D1b.
- **Motivo:** a recomendação oficial do Anchor diverge do workflow do Router. Os manifests/locks do Router demonstram versões de crates, mas não provam que o tag funcione com Agave CLI `2.1.0`. O exemplo `counter` usa dois workspaces, `stable` não pinado para Rust e uma action Rust `@main`; isso não comprova uma interseção reproduzível.
- **Evidência:** [`docs/toolchain-matrix.md`](toolchain-matrix.md), [`docs/bootstrap-plan.md`](bootstrap-plan.md), release notes Anchor 0.31.0 no tag `v0.31.1`, `.github/workflows/tests.yml`, manifests e lockfiles do tag `boundless-xyz/risc0-solana v3.0.0`, e manifests/CI do tag `risc0/risc0 v3.0.3`, todos ligados nesses documentos.
- **Risco aberto:** coexistência entre CLI/SBF e crates, toolchain Rust exata do `counter`, patch Git por branch, ABI/serialização da CPI, Docker exato, receipt VeriCode, Router Program ID e deployment por rede continuam não validados.
- **Próximo gate:** spike controlado do `counter` com locks preservados, Rust pinado e comparação de Anchor `0.31.1` com Agave `2.1.0` e `2.3.9`; registrar falhas sem trocar dependências silenciosamente. Somente depois definir Perfil A e reabrir D1b.

## 2026-09-28 — D1b0.1: ambiente Linux base e clone canônico

- **Data:** 2026-09-28
- **Decisão:** adotar o clone `~/src/vericode`, em filesystem Linux ext4 no Ubuntu 24.04 sob WSL2, como checkout canônico de desenvolvimento. Instalar somente `git`, `curl`, `ca-certificates`, `build-essential`, `pkg-config` e `libssl-dev` nesta etapa.
- **Motivo:** separar o ambiente Linux das limitações de paths e permissões do checkout Windows, sem antecipar toolchains bloqueadas pela D1a.1.
- **Evidência:** Ubuntu `24.04.5 LTS`, arquitetura `x86_64`, WSL versão 2, versões dos seis pacotes registradas em `docs/evidence.md`, clone na branch `main` e commit inicial `7aaf40b` com status limpo.
- **Risco aberto:** D1a.2 ainda precisa demonstrar a interseção Anchor/Agave/RISC Zero; Router/CPI e deployments continuam não validados.
- **Próximo gate:** D1a.2. Rust, RISC Zero, Solana/Agave, Anchor, Docker e Node permanecem fora do bootstrap autorizado.

## 2026-09-28 — D1a.2: protocolar o spike sem autorizar instalações

- **Data:** 2026-09-28
- **Decisão:** adotar [`docs/d1a2-spike-plan.md`](d1a2-spike-plan.md) como
  protocolo futuro das raias A (Anchor `0.31.1` + Agave `2.1.0`), B
  (Anchor `0.31.1` + Agave `2.3.9`) e zkVM (RISC Zero `v3.0.3`).
  Manter a decisão **PENDENTE**, `SEM PERFIL PRONTO PARA INSTALAÇÃO` e D1b
  bloqueado.
- **Motivo:** os tags e locks agora têm revisões/hashes exatos e os comandos,
  critérios e limites sem chave estão definidos, mas o Rust host da raia A e
  o Docker exato continuam `NÃO DETERMINADOS`. O workflow oficial usa Rust
  por action `@main` e cria keypair antes de `anchor test`; nenhuma dessas
  práticas é evidência reproduzível ou está autorizada.
- **Evidência:** `risc0/risc0 v3.0.3` em
  `14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`;
  `boundless-xyz/risc0-solana v3.0.0` em
  `ee415935d04a948f27a346b563391900bdad6486`; Anchor `v0.31.1` em
  `47284f8f0b9844c6b83234aa90f556bad00e12ed`; Agave `v2.1.0` em
  `c1080de464cfb578c301e975f498964b5d5313db`; tag anotado Agave
  `v2.3.9` `f20bef1cce6abb06512b76f2baddc99111980740` apontando para
  `47647df756f5dd0b3c739cecaa71bcf754af6be8`; manifests, workflows e
  locks exatos citados no plano.
- **Risco aberto:** Rust host A, Docker, patch Git por branch, builds
  Anchor/SBF, ABI, receipt, ImageID, Router/CPI, Program ID e deployment por
  rede continuam não executados/não validados. Testes negativos de Job, mint
  e executor dependem de código VeriCode ainda não autorizado.
- **Próximo gate:** revisão humana do protocolo e dos pins faltantes; somente
  depois autorizar um ambiente descartável e a fase sem chaves. Wallet,
  keypair, validator, deploy, airdrop e transações exigem autorização
  adicional separada.
