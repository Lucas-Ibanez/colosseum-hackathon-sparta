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

## 2026-09-28/29 — D1a.3: concluir H1–H4 e manter decisão pendente

- **Data:** 2026-09-29
- **Decisão:** manter `PENDENTE` e D1b bloqueado, mas registrar como Perfil A
  candidato o lado Anchor/Solana com Rust `1.85.0`, Anchor/AVM/crates
  `0.31.1` e Agave `2.1.0`, separado da raia zkVM com Rust host `1.89.0`,
  guest `1.88.0`, `rzup 0.5.1` e componentes RISC Zero `3.0.3`. Preservar
  Agave `2.3.9`/Rust `1.89.0` como raia B de referência, não como recomendação
  Anchor.
- **Motivo:** `1.81.0`, pin do source Agave `v2.1.0`, falhou no lock real;
  `1.85.0` foi o único fallback oficial previsto e passou. As duas raias
  passaram os gates host/ABI, mas a A é a alternativa conservadora que
  preserva a recomendação Anchor. O critério D1a.2 ainda exige testes
  negativos de `Job/mint/executor`, impossíveis sem o futuro código VeriCode,
  além de CPI runtime sob autorização posterior.
- **Evidência:** sete checkouts limpos; cinco hashes de lock preservados;
  Docker Engine `29.8.1` e builder por digest; dois builds efetivos com ELF
  SHA-256 `383b3e63ffd0387ad20c0dee3fa78cc9da25ee164773e59814f02312b5d3bb3f`
  e ImageID
  `ab2f61e0cc5244ee8ee0712a697251be92a2c6539faddf300ab56414dc69149d`
  idênticos; receipt local válido; rejeições de ImageID/journal divergentes;
  vetores ABI idênticos em A/B. Resultados completos em
  [`docs/d1a3-spike-results.md`](d1a3-spike-results.md).
- **Risco aberto:** os IDLs Router/Groth16 divergem do source; o patch Git do
  host zkVM é ignorado; não houve build SBF, conta Solana real, CPI, Program ID
  por rede ou deployment. O grupo `docker` é privilegiado e o tarball Agave
  `v2.1.0` não possui digest oficial publicado.
- **Próximo gate:** autorizar separadamente o contrato/harness puro Rust do
  `JournalV1` para testar Job, mint e executor e regenerar/comparar IDLs. H5
  continua proibido até decisão específica sobre keypair, validator, CPI e
  rede.
