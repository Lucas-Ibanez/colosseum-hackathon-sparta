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

## 2026-09-30 — D1c1: implementar o contrato semântico puro de JournalV1

- **Data:** 2026-09-30
- **Decisão:** criar a workspace mínima e a crate `vericode-core` sem
  dependências externas, com `Hash32`, `JobId`, `ImageId`, `Verdict`,
  `JournalV1Commitments` e `JournalV1`. A validação compara todos os
  compromissos e retorna `PASS` ou `FAIL` como valores normais. Autorizar o
  início de D1c2 somente para guest mínimo e receipt VeriCode local em
  ambiente isolado; não liberar D1b, M1, Router/CPI ou rede.
- **Motivo:** os nove testes semânticos passaram com Rust `1.85.0` e `1.89.0`
  e `cargo tree --locked` confirmou uma árvore com apenas a crate local. A
  separação pura permite que futuros adaptadores guest e Anchor reutilizem o
  significado sem importar SDKs incompatíveis.
- **Evidência:** `cargo test --locked` com 9/9 testes em ambas as raias;
  `cargo tree --locked` contendo somente `vericode-core`; Cargo.lock gerado
  pelo Cargo; [`docs/d1c1-core-results.md`](d1c1-core-results.md).
- **Risco aberto:** serialização, layout, endianness e algoritmo de hashing
  continuam Draft v0. O D1a.3 provou uma receipt composta do `hello-world`
  com journal `391`, não uma receipt Groth16 nem um journal VeriCode. Clippy e
  rustfmt não estão instalados nas toolchains isoladas e não foram
  adicionados. M1 permanece fora de verde.
- **Próximo gate:** D1c2 deve primeiro escolher e testar explicitamente o wire
  format público, então criar o guest mínimo usando a crate pura e produzir
  receipts VeriCode reais para `PASS` e `FAIL`, sem rede ou Solana. Qualquer
  alegação Groth16 exige conversão e verificação demonstradas separadamente.

## 2026-09-30 — D1c2a: adotar wire format e hashing candidatos locais

- **Data:** 2026-09-30
- **Decisão:** adotar Borsh `0.10.4` como wire format candidato local de
  `JournalV1`, com layout fixo de 165 bytes, e SHA-256 por `sha2 0.10.9`
  para `spec_hash`, `harness_hash` e `artifact_hash`, sempre com domínios
  distintos. Implementar um único artefato de desenvolvimento de 12 bytes e
  a regra fixa `claimed_output == input * 2`. Autorizar D1c2b somente para
  guest mínimo e receipts VeriCode locais `PASS`/`FAIL`.
- **Motivo:** o tag exato `boundless-xyz/risc0-solana v3.0.0`, commit
  `ee415935d04a948f27a346b563391900bdad6486`, declara Borsh `0.10.3`
  para o código compartilhado e seus locks on-chain/zkVM resolvem
  `0.10.4`; os mesmos locks resolvem `sha2 0.10.9`. Fixar essas versões
  permite testar bytes idênticos sem inferir compatibilidade por semelhança.
- **Evidência:** `Cargo.lock`
  `191802b234a6aa0f6bb9ce58a61963c377aed435a2baa6dec9576d13d8283b87`;
  20/20 testes e `cargo tree --locked` passaram com Rust `1.85.0` e
  `1.89.0`; vetores e fontes exatas em
  [`docs/d1c2a-wire-harness-results.md`](d1c2a-wire-harness-results.md).
- **Risco aberto:** Borsh/SHA-2 ainda não foram compilados dentro de um guest
  VeriCode; não existe ImageID nem receipt VeriCode. O wire format não está
  congelado como ABI Anchor/Router, e receipt Groth16, Router/CPI, Program ID
  e rede continuam não validados. Alterar semântica do harness exige nova
  versão e novos vetores.
- **Próximo gate:** D1c2b deve criar somente o guest mínimo, usar esta mesma
  crate/lock, reproduzir os vetores no guest e gerar/verificar receipts locais
  reais de `PASS` e `FAIL`, sem dev mode, wallet, Solana ou rede.

## 2026-10-01 — D1c2b.1: fixar cache Cargo offline da raia zkVM

- **Data:** 2026-10-01
- **Decisão:** criar uma `CARGO_HOME` persistente e exclusiva em
  `~/.local/share/vericode-spikes/d1c2b/cargo`, gerar locks separados para a
  workspace host/methods e para o guest com Cargo/Rust `1.89.0`, e liberar a
  retomada restrita de D1c2b depois que `metadata` e `tree` passaram offline.
- **Motivo:** a falha original era ausência de metadata de registry, não
  incompatibilidade demonstrada. A semente pública da raia A chegou até
  `bonsai-sdk`; acesso autorizado a `index.crates.io`/`static.crates.io`
  completou a cache. Os pins diretos `risc0-zkvm` e `risc0-build` permaneceram
  `3.0.3`, sem dependência Git.
- **Evidência:** `zkvm/Cargo.lock`
  `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
  guest lock
  `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50`;
  quatro execuções `cargo metadata/tree --locked --offline` com exit `0`;
  inventário e saídas em
  [`docs/d1c2b1-cache-results.md`](d1c2b1-cache-results.md).
- **Risco aberto:** restrições transitivas caret resolveram crates RISC Zero
  posteriores às versões preservadas nos locks do tag `v3.0.3`; o lock local
  é reproduzível, mas compilação e execução continuam não demonstradas.
- **Próximo gate:** retomar D1c2b usando os locks sem regenerá-los e a cache
  offline; preservar qualquer falha real. Guest, ELF, ImageID e receipts ainda
  não existem.

## 2026-10-02 — D1c2b: bloquear no cache interno do builder Docker

- **Data:** 2026-10-02
- **Decisão:** manter D1c2b bloqueado antes do primeiro ELF e não desabilitar
  `CARGO_NET_OFFLINE`, alterar locks ou baixar dependências dentro do builder.
- **Motivo:** a imagem local fixada pelo digest `sha256:3e12f71b...d943eb3`
  foi usada sem pull, mas seu registry Cargo interno não contém `borsh`. O
  `risc0-build 3.0.3` não injeta automaticamente a `CARGO_HOME` D1c2b do host
  no container, e o `cargo +risc0 fetch --locked` falhou offline.
- **Evidência:** exit `101` no estágio Docker `[build 4/5]`, com
  `no matching package named borsh found`; locks antes/depois idênticos; busca
  temporária sem ELF ou receipt; relatório completo em
  [`docs/d1c2b-guest-receipt-results.md`](d1c2b-guest-receipt-results.md).
- **Risco aberto:** o mecanismo reproduzível para disponibilizar o registry
  fixado dentro do builder ainda não foi escolhido. Sem isso não existem dois
  builds, ImageID, journals guest nem receipts VeriCode.
- **Próximo gate:** autorizar separadamente um bootstrap offline e auditável
  do cache do builder — vendoring temporário ou imagem derivada local — sem
  mudar versões/locks; depois retomar D1c2b desde o build A.

## 2026-10-02 — D1c2b.2: manter bloqueio após falha de invocação do builder

- **Data:** 2026-10-02
- **Decisão:** aceitar o vendor temporário offline como inventário completo do
  lock guest, mas não liberar D1c2b porque o fetch dentro do builder não foi
  executado com sucesso.
- **Motivo:** `cargo vendor --locked --offline` materializou as 154 crates,
  porém o primeiro `docker run` terminou antes do Cargo com exit `127` e
  `/bin/sh: 0: Can't open cargo`. A regra do gate proibiu corrigir e repetir
  após qualquer falha.
- **Evidência:** 154 checksums presentes; inventário de conteúdo
  `6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`;
  staging somente-leitura, `--network none` e `--pull=never`; relatório em
  [`docs/d1c2b2-builder-vendor-results.md`](d1c2b2-builder-vendor-results.md).
- **Risco aberto:** a forma correta de invocar Cargo respeitando o entrypoint
  da imagem fixa ainda não foi auditada, e `fetch`/`metadata` offline no
  container continuam não comprovados.
- **Próximo gate:** inspecionar `Entrypoint`/`Cmd` da imagem por leitura e
  autorizar uma repetição única com o mesmo vendor, digest, locks e isolamento
  de rede. Não retomar build ou proving antes de ambos os comandos passarem.

## 2026-10-02 — D1c2b.2a: validar vendor no entrypoint do builder

- **Data:** 2026-10-02
- **Decisão:** liberar a retomada posterior de D1c2b somente a partir do
  build do guest; o vendor passou resolução e metadata offline dentro do
  builder.
- **Motivo:** a imagem fixa declara `Entrypoint=["/bin/sh"]` e `Cmd=null`.
  Passar o comando por `-c` corrigiu exclusivamente a invocação anterior;
  `fetch` e `metadata` passaram com exit `0`, sem rede e sem pull.
- **Evidência:** locks preservados, vendor com hash
  `6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335` e
  relatório em
  [`docs/d1c2b2a-builder-entrypoint-results.md`](d1c2b2a-builder-entrypoint-results.md).
- **Risco aberto:** compilação guest, ELF, ImageID, receipt VeriCode e proving
  continuam não demonstrados. Router/CPI/devnet permanecem não validados.
- **Próximo gate:** D1c2b, com build controlado do guest; não executar proving
  ou rede blockchain como parte deste resultado.

## 2026-10-02 — D1c2b.3: bloquear no build A por configuração vendor não descoberta

- **Data:** 2026-10-02
- **Decisão:** interromper D1c2b.3 após a falha do build A; não executar build
  B, não mover a configuração Cargo e não alterar produto ou locks.
- **Motivo:** o vendor reproduziu integralmente a evidência de D1c2b.2a, mas
  o Dockerfile gerado por `risc0-build 3.0.3` executa a partir de `/src`. O
  `cargo +risc0 fetch --locked` interno não consumiu a configuração em
  `zkvm/methods/guest/.cargo/config.toml` e falhou offline procurando `borsh`
  no índice crates.io interno.
- **Evidência:** build A exit `101`; 154 crates/checksums e inventário
  `6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`;
  locks preservados; `methods.rs` residual vazio; relatório em
  [`docs/d1c2b3-guest-build-results.md`](d1c2b3-guest-build-results.md).
- **Risco aberto:** ainda não há mecanismo auditado que torne a substituição
  vendorizada visível à invocação real do build upstream. ELF, ImageID,
  comparação A/B, receipts e proving VeriCode continuam inexistentes.
- **Próximo gate:** autorizar separadamente a correção temporária do ponto de
  descoberta da configuração Cargo no contexto `/src`, provar o fetch
  offline pela mesma invocação do Dockerfile e só então repetir D1c2b.3.

## 2026-10-02 — D1c2b.3a: validar vendor na raiz do contexto Docker

- **Data:** 2026-10-02
- **Decisão:** aceitar o vendor temporário em `/src/vendor`, acompanhado de
  `/src/.cargo/config.toml`, como mecanismo auditado para a futura repetição
  do build determinístico; liberar a retomada de D1c2b.3 sem executar o build
  neste gate.
- **Motivo:** o `risc0-build 3.0.3` gera `WORKDIR /src` e `COPY . .`. Com a
  configuração na raiz do mesmo contexto, uma única execução offline do
  Dockerfile de diagnóstico encontrou o vendor e concluiu `fetch` e
  `metadata` com exit `0`.
- **Evidência:** 154 crates/checksums, inventário
  `6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`;
  `docker build --pull=false --network=none` sem tag; metadata de 652.545
  bytes com SHA-256
  `7274178febd20364c33b76d89dad4b315bbc144b3534dd23f66a6242e1c95f46`;
  relatório em
  [`docs/d1c2b3a-root-vendor-results.md`](d1c2b3a-root-vendor-results.md).
- **Risco aberto:** o build A/B, ELF, ImageID, execução host e receipts
  VeriCode ainda não foram produzidos. O mecanismo continua temporário e não
  foi adicionado ao clone. Router/CPI/devnet permanecem não validados.
- **Próximo gate:** repetir D1c2b.3 com staging raiz vendorizado, dois targets
  independentes e comparação byte a byte; não antecipar host, receipt ou
  proving.

## 2026-10-02 — D1c2b.3b.1: remediar Rustup padrão criado por erro

- **Data:** 2026-10-02
- **Decisão:** aprovar a remediação e manter D1c2b.3b aguardando reinício
  controlado.
- **Motivo:** uma consulta sem `RUSTUP_HOME` criou a única toolchain padrão em
  `/home/lucas/.rustup`. O Rustup isolado desinstalou exatamente
  `1.89.0-x86_64-unknown-linux-gnu`; o root acidental foi removido após a
  confirmação de que seus resíduos eram apenas diretórios vazios.
- **Evidência:** [`docs/d1c2b3b-rustup-remediation.md`](d1c2b3b-rustup-remediation.md);
  root padrão ausente depois, toolchain D1a.3 preservada, `.cargo` não tocado,
  `zkvm/` e locks inalterados.
- **Risco aberto:** D1c2b.3b ainda não gerou ELF, ImageID ou receipt VeriCode.
  Todo comando futuro deve declarar `CARGO_HOME`, `RUSTUP_HOME`, `RISC0_HOME`
  e `PATH` isolados.

## 2026-10-02 — D1c2b.3b-retry: bloquear por MSRV do lock guest

- **Data:** 2026-10-02
- **Decisão:** interromper após a falha do build A; não iniciar build B, não
  alterar locks/versões/toolchains e não avançar para host ou receipts.
- **Motivo:** o builder usa `rustc 1.88.0-dev`, mas o lock guest fixa
  `enum-ordinalize 4.4.2` e `enum-ordinalize-derive 4.4.2`; ambos declaram
  `rust-version = "1.89"`. O compilador recusou a combinação com exit `101`.
- **Evidência:** ambiente isolado; `/home/lucas/.rustup` permaneceu ausente;
  vendor A com 154 crates/checksums e inventário `6d447859...cdf335`;
  fetch passou, compilação guest falhou, `methods.rs` ficou vazio e nenhum
  ELF/ImageID foi produzido. Relatório em
  [`docs/d1c2b3b-deterministic-guest-build-results.md`](d1c2b3b-deterministic-guest-build-results.md).
- **Risco aberto:** ainda não há dois builds, comparação byte a byte, ImageID,
  execução host ou receipt VeriCode.
- **Próximo gate:** auditar a cadeia transitiva e autorizar uma reconciliação
  de lock/MSRV com fontes oficiais. Não repetir o build por inferência.

## 2026-10-02 — D1c2b.3c: reconhecer candidato oficial de reconciliação MSRV

- **Data:** 2026-10-02
- **Decisão:** classificar o resultado como **CANDIDATO DE RECONCILIAÇÃO
  COMPROVADO**, sem alterar o lock neste gate e sem liberar ainda a execução
  do guest.
- **Motivo:** `educe 0.6.0` aceita `enum-ordinalize ^4.2`; o lock gerado com
  Rust host 1.89 escolheu `enum-ordinalize 4.4.2` e derive `4.4.2`, ambos com
  MSRV 1.89, acima do builder guest 1.88. Os locks dos tags exatos
  `risc0/risc0 v3.0.3` e `risc0-solana v3.0.0` preservam versões
  `4.3.0`/`4.3.1`, ambas com MSRV 1.60 e checksums confirmados.
- **Evidência:** árvore Cargo locked/offline; manifests e archives dos caches
  isolados; locks oficiais nos commits `14b5d588…` e `ee415935…`; build D1a.3
  do hello-world com builder 1.88; relatório em
  [`docs/d1c2b3c-guest-msrv-lock-audit.md`](d1c2b3c-guest-msrv-lock-audit.md).
- **Risco aberto:** o lock atual permanece incompatível; a evidência upstream
  não substitui a futura alteração mecânica, novo vendor e dois builds do
  guest VeriCode. Não existem ELF, ImageID ou receipts VeriCode.
- **Próximo gate:** autorizar uma reconciliação exclusivamente de lock contra
  o conjunto transitivo comprovado pelos tags, validar metadata/tree offline
  e só então repetir os builds determinísticos. Não trocar builder, versões
  diretas ou wire format.

## 2026-10-02 — D1c2b.3d: manter bloqueio por cache offline incompleta

- **Data:** 2026-10-02
- **Decisão:** aceitar e manter a reconciliação mecânica do lock guest, mas
  classificar o gate como **BLOQUEADO** até que a mesma `CARGO_HOME` consiga
  concluir `metadata/tree --locked --offline`.
- **Motivo:** o diff contém somente `risc0-groth16 3.0.5 -> 3.0.2`,
  `enum-ordinalize 4.4.2 -> 4.3.0` e derive `4.4.2 -> 4.3.1`, com checksums
  oficiais e `syn ^2` comprovado. Porém, a cache D1c2b possui apenas as
  entradas de índice dessas versões, não seus archives; todos os comandos de
  validação falharam offline no primeiro archive ausente.
- **Evidência:** lock guest
  `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
  lock host preservado `c55eecfa…`; `metadata`, `tree` e três árvores inversas
  com exit `101`; relatório em
  [`docs/d1c2b3d-guest-lock-reconciliation-results.md`](d1c2b3d-guest-lock-reconciliation-results.md).
- **Risco aberto:** o lock final ainda não foi materializado/validado pela
  home prescrita; não existem build guest, ELF, ImageID ou receipts VeriCode.
- **Próximo gate:** autorizar a cópia offline apenas dos três archives
  públicos exatos dos caches D1a.3 para a cache D1c2b, conferindo checksums,
  e repetir metadata/tree antes de qualquer vendor ou build.

## 2026-10-02 — D1c2b.3e: aguardar fonte para o archive restante

- **Data:** 2026-10-02
- **Decisão:** classificar o gate como **AGUARDANDO_AUTORIZAÇÃO** depois de
  copiar e validar somente dois dos três archives ausentes; não usar outra
  cache nem rede por inferência.
- **Motivo:** o inventário completo encontrou 154 pacotes registry: 151 já
  válidos no destino, dois archives válidos na única fonte permitida e
  `risc0-groth16 3.0.2` ausente em toda essa raiz. Após as duas cópias, Cargo
  avançou offline até falhar exclusivamente nesse archive restante.
- **Evidência:** cache destino em 153/154; checksums dos dois archives
  copiados iguais ao lock; `metadata`, `tree` e três inversas com exit `101`;
  relatório em
  [`docs/d1c2b3e-offline-lock-closure-results.md`](d1c2b3e-offline-lock-closure-results.md).
- **Risco aberto:** o lock guest final ainda não fecha offline; vendor,
  builds, ELF, ImageID e receipts VeriCode permanecem não executados.
- **Próximo gate:** exigir autorização humana para uma fonte local adicional
  exata do único archive restante, validando SHA-256 antes de qualquer cópia.

## 2026-10-02 — D1c2b.3e-retry: fechar o lock guest offline

- **Data:** 2026-10-02
- **Decisão:** classificar o gate como **GO** para produzir o vendor
  temporário auditável do lock reconciliado.
- **Motivo:** após autorização humana explícita, três cópias locais do
  archive restante foram encontradas em `lane-a`/`lane-b`, todas idênticas e
  com o checksum exato. Uma única cópia levou a cache destino a 154/154
  archives válidos; metadata, árvore completa e quatro inversas passaram
  locked/offline.
- **Evidência:** `risc0-groth16-3.0.2.crate` com 40.149 bytes e SHA-256
  `724285dc…fae9`; zero archive ausente ou divergente; seis comandos Cargo
  com exit `0`; relatório em
  [`docs/d1c2b3e-offline-lock-closure-results.md`](d1c2b3e-offline-lock-closure-results.md).
- **Risco aberto:** o vendor do lock final, os dois builds, ELF, ImageID e
  receipts VeriCode ainda não existem. Router/CPI/devnet permanecem
  `STATUS: NÃO VALIDADO`.
- **Próximo gate:** criar e auditar vendor novo em staging temporário, sem
  reutilizar hashes do lock anterior e sem iniciar build antes da auditoria.

## 2026-10-02 — D1c2b.3f: aprovar vendor do lock guest final

- **Data:** 2026-10-02
- **Decisão:** classificar o gate como **GO** para dois builds independentes
  do guest, ainda sem autorizar receipts antes da comparação dos ELF/ImageID.
- **Motivo:** staging novo do commit `3c505a8` produziu, locked/offline, um
  vendor que corresponde exatamente aos 154 pacotes registry do lock final.
  Checksums de pacote e 5.750 hashes de arquivo passaram; metadata resolveu
  154/154 pacotes por paths no vendor e tree passou.
- **Evidência:** 154 crates, 154 `.cargo-checksum.json`, 5.904 arquivos,
  113.733.052 bytes; hashes de paths `dc242b5e…` e conteúdo `3217344b…`;
  relatório em
  [`docs/d1c2b3f-final-vendor-results.md`](d1c2b3f-final-vendor-results.md).
- **Risco aberto:** ainda não existe ELF ou ImageID VeriCode, e o vendor em
  `/tmp` é efêmero. Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.
- **Próximo gate:** dois contextos/targets separados, imagem local por digest,
  Cargo offline e comparação byte a byte, tamanho, SHA-256 e ImageID.

## 2026-10-02 — D1c2b.3g.1: tornar o core compatível com `no_std`

- **Data:** 2026-10-02
- **Decisão:** aceitar a alteração mínima `no_std + alloc` no core e
  desativar as features padrão de `borsh`/`sha2`, com **GO** restrito para os
  dois builds independentes após commit auditado.
- **Motivo:** o builder guest real rejeitou o core anterior por `duplicate
  lang item panic_impl`; o runtime RISC Zero é `no_std`, enquanto as features
  padrão das dependências ativavam `std`. A correção preserva versões,
  locks, wire format, hashes e regra de verdict.
- **Evidência:** staging com fontes byte a byte iguais ao diff; Docker local
  por digest, sem pull e sem rede; guest locked/offline compilado com flags
  oficiais; ELF32 RISC-V de 147.880 bytes e SHA-256 `3fc668df…c42d`;
  relatório em
  [`docs/d1c2b3g1-core-no-std-results.md`](d1c2b3g1-core-no-std-results.md).
- **Risco aberto:** o probe não é um dos dois builds finais e não tem
  ImageID. O lock host possui incompatibilidade transitiva para receipts, e
  os testes host deste gate não iniciaram porque falta o archive offline
  `cfg-if 1.0.3`. Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.
- **Próximo gate:** commit local limitado, dois exports independentes desse
  commit, vendors da união auditados, targets separados e compilação isolada
  de `vericode-methods`, seguida de comparação integral antes de receipts.

## 2026-10-02 — D1c2b.3g: aceitar os dois builds determinísticos

- **Data:** 2026-10-02
- **Decisão:** classificar o gate como **GO** para receipts locais reais,
  somente após auditoria e commit da evidência de build.
- **Motivo:** A e B partiram de exports independentes do commit `be23e01`,
  vendors próprios da união dos locks, targets separados e nonces distintos.
  Ambos os builds oficiais de `vericode-methods` passaram; ELF e método
  combinado são idênticos byte a byte.
- **Evidência:** ELF 147.880 bytes/SHA-256 `3fc668df…c42d`; método combinado
  180.304 bytes/SHA-256 `5c3c82c4…88bc`; array de ImageID igual em ambos os
  `methods.rs`; dois `r0vm --id` iguais a `35b05ee4…bf41`; relatório em
  [`docs/d1c2b3g-deterministic-build-results.md`](d1c2b3g-deterministic-build-results.md).
- **Risco aberto:** os artefatos são temporários e não há receipt VeriCode.
  O lock host pode impedir proving por incompatibilidade transitiva já
  observada; nenhum lock está autorizado a mudar por inferência.
- **Próximo gate:** usar o método validado no host real locked/offline para
  PASS, FAIL e negativos; manter `Verdict::Fail` como saída normal.

## 2026-10-03 — D1c2b.3h: reconciliar o lock host e refazer os builds finais

- **Data:** 2026-10-03
- **Decisão:** aceitar o conjunto host compatível comprovado pelo lock oficial
  de `risc0-zkvm 3.0.3` e classificar o gate como **GO** para receipts locais
  reais após commit auditado.
- **Motivo:** o lock anterior resolvia circuitos/zkp novos e falhava com sete
  erros de API. A reconciliação estrita de onze versões fechou offline,
  compilou o host e passou 2/2 testes. Como os paths do vendor combinado
  mudaram, dois builds independentes do guest final foram refeitos em vez de
  reutilizar o artefato anterior.
- **Evidência:** lock host `f5236689…e226`; metadata/tree e onze inversas
  passaram; vendors A/B 467/467 iguais; ELF final 147.876 bytes e SHA-256
  `63fac491…5408`; método 180.300 bytes e SHA-256 `e09ba8cf…78f5`; dois
  `r0vm` deram ImageID `4da06f90…fb1a`; relatório em
  [`docs/d1c2b3h-host-lock-and-final-build-results.md`](d1c2b3h-host-lock-and-final-build-results.md).
- **Risco aberto:** artefatos são temporários; ainda não existe receipt
  VeriCode PASS/FAIL. A limitação de isolamento de rede do builder upstream
  permanece explícita. Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.
- **Próximo gate:** executar o host real sem dev mode, produzir receipts PASS
  e FAIL, verificar ImageID/journals e executar todos os negativos exigidos.

## 2026-10-03 — D1c2b.3i: aceitar receipts locais reais

- **Data:** 2026-10-03
- **Decisão:** classificar o gate como **GO** para a auditoria final do marco
  D1c2b.
- **Motivo:** o prover local real, com dev mode desabilitado por feature e
  removido do ambiente, produziu receipts PASS, FAIL e wrong-image do tipo
  `Composite`. O host e um auditor temporário independente verificaram os
  arquivos, journals e negativos.
- **Evidência:** três receipts de 221.540 bytes com hashes distintos; PASS e
  FAIL verificam contra ImageID `4da06f90…fb1a`; `Verdict::Fail` é saída
  normal; wrong verify ImageID, journal ImageID divergente e Job ID divergente
  foram rejeitados; relatório em
  [`docs/d1c2b3i-local-receipts-results.md`](d1c2b3i-local-receipts-results.md).
- **Risco aberto:** receipts são efêmeros e `Composite`, não Groth16. Nenhuma
  alegação on-chain foi feita; Router/CPI/devnet continuam
  `STATUS: NÃO VALIDADO`.
- **Próximo gate:** auditoria final somente leitura do histórico, árvore,
  locks, relatórios, hashes e fronteiras; depois marcar o controle como
  `CONCLUÍDO` e completar o Goal somente se nada obrigatório restar.

## 2026-10-03 — D1c2b.3j: concluir exclusivamente o marco D1c2b

- **Data:** 2026-10-03
- **Decisão:** marcar D1c2b como **CONCLUÍDO** e parar.
- **Motivo:** a auditoria final separada recertificou Git limpo, locks/grafos
  offline, vendors A/B, dois builds determinísticos, ImageID, testes host,
  receipts PASS/FAIL reais e todos os negativos. Nenhuma tarefa obrigatória
  permanece.
- **Evidência:** relatório final em
  [`docs/d1c2b3j-final-audit.md`](d1c2b3j-final-audit.md); matriz integral de
  conclusão; HEAD auditado `d866e73`; ImageID `4da06f90…fb1a`; receipts
  persistidos reverificados e não Fake.
- **Limites:** receipts locais `Composite`, artefatos efêmeros e nenhuma
  alegação ZK on-chain. Router/CPI/devnet continuam
  `STATUS: NÃO VALIDADO`.
- **Próxima transição:** nenhuma dentro de D1c2b. Qualquer escopo posterior
  exige nova autoridade e objetivo separado.

## 2026-10-04 — D2a: política pura de escrow no core (item 1 substituído por D2a.1)

- **Data:** 2026-10-04
- **Decisão:** implementar em `crates/vericode-core/src/escrow.rs` somente a
  política pura de escrow, sem Solana, Anchor ou RISC Zero. Decisões humanas
  explícitas desta sessão: (1) o `artifact_hash` esperado é registrado uma
  única vez, somente pelo executor do Job, no estado `Funded`, levando a
  `Delivered { artifact_hash }`; o release compara o journal contra esse
  valor; (2) `JobV1` rejeita `buyer == executor`. Regras derivadas dos
  documentos existentes: funding somente pelo buyer, com mint e amount exatos
  do Job; identidades toda-zero e amount zero rejeitados; release somente com
  todos os compromissos, `Verdict::Pass`, executor e mint do Job; `Payout`
  copiado do Job; `Released` terminal; nenhum parâmetro administrativo.
- **Fronteira:** o guia operacional e a arquitetura dizem que o core não toma
  "decisão de pagamento"/"autoridade de release". Esta decisão delimita a
  exceção: o core passa a conter o predicado puro de elegibilidade, reutilizável
  pelo futuro programa; ele não custodia, não transfere, não verifica prova e
  não detém autoridade. O guia operacional não foi editado neste gate.
- **Motivo:** o passo 4 da sequência de construção exige especificar a
  política e a máquina de estados antes do Anchor; a crate pura permite testar
  a política sem SDKs incompatíveis.
- **Evidência:** 37/37 testes (20 anteriores + 17 de escrow) e `cargo tree`
  idêntico com Rust `1.85.0` e `1.89.0`, locked/offline; locks inalterados;
  [`docs/d2a-core-escrow-policy-results.md`](d2a-core-escrow-policy-results.md)
  e [`docs/escrow-state-machine.md`](escrow-state-machine.md).
- **Pendente — `AGUARDANDO_AUTORIZAÇÃO`:** refund e seu estado terminal;
  prazo/timeout e quem os aciona; efeito econômico de `Verdict::Fail`; quem
  aciona `release` on-chain; layout/serialização do Job. Nenhuma dessas regras
  foi inventada.
- **Errata D1c2b.3i:** `docs/d1c2b3i-local-receipts-results.md` (linhas 49 e
  55) transcreve o harness hash com 68 hex e o artifact hash PASS com 60 hex.
  Os valores canônicos, testados em `crates/vericode-core/src/lib.rs`, são
  harness `01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50`
  e artifact PASS
  `d5aa9223d6d2a1ba23bd73ca325b411c75027a739c285a95ff63b963442a224c`. O
  relatório histórico foi preservado; a errata não altera as receipts, que
  foram verificadas por bytes exatos.
- **Risco aberto:** ImageID `4da06f90…fb1a` não foi recertificado após a
  adição do módulo; um Job `Delivered` com artefato FAIL não tem saída até a
  decisão de refund; o executor pode registrar qualquer artefato que satisfaça
  a regra trivial de desenvolvimento. Router/CPI/devnet continuam
  `STATUS: NÃO VALIDADO`.
- **Próximo gate:** decisão humana sobre refund/prazo/`FAIL`; somente depois
  especificar o gate Anchor local, sem Router inicialmente.

## 2026-10-04 — D2a.1: adotar o guia de produto e a sequência como contexto

- **Data:** 2026-10-04
- **Decisão:** registrar `docs/context/guia-mvp-agentes-de-codigo.md` como
  norma de produto e `docs/context/sequencia-mvp.md` como cronograma, ambos
  sem alteração de conteúdo, e unificar a ordem de precedência em
  `docs/project-context.md`. Decisão humana explícita desta sessão: no
  conflito sobre `artifact_hash`, **o guia prevalece** — o comprador não fixa
  o hash, o executor não o pré-registra e o programa registra o hash do
  journal apenas na liquidação, comparando `job_id`, `spec_hash`,
  `harness_hash` e ImageID. Isso substitui o item 1 da decisão D2a. Adotados
  do guia para o D2b: `Fail` válido devolve somente ao buyer; refund por
  timeout somente após `deadline_slot`, com o slot informado como entrada
  (o core não lê relógio). Instituído o protocolo
  `docs/handoff-protocol.md` em `AGENTS.md` e `CLAUDE.md`.
- **Motivo:** os documentos fornecidos pelo humano devem nortear a construção;
  sem precedência explícita, guia de produto, guia operacional e decisões
  anteriores divergiam em `artifact_hash`, refund, prazo e wallets.
- **Evidência:** documentos registrados e conferidos por `git diff --check`;
  conflitos tabelados em `docs/project-context.md`.
- **Ainda pendente:** mapeamento dos estados do guia (`Draft`, `Proving`,
  `Submitted`, `Failed`) para estados on-chain × worker/UI; se release é
  permitido após o prazo; quem aciona refund/release on-chain; Perfil A
  Anchor/Agave (D1b); criação de wallets devnet frente ao princípio 9 de
  `AGENTS.md`.
- **Risco aberto:** o projeto está tecnicamente no D2 em 2026-10-04 (dia D8
  do calendário). O código D2a ainda implementa o pré-registro substituído
  até o D2b.
- **Próximo gate:** D2b — alinhar a política pura de escrow ao guia, conforme
  `docs/handoffs/d2a1-to-d2b.md`.

## 2026-10-04 — D2a.2: delegar Perfil A, autorizar keypairs devnet e corrigir README

- **Data:** 2026-10-04
- **Decisão humana explícita:**
  1. o agente tem livre escolha da combinação Anchor/Agave (Perfil A),
     considerando o histórico do projeto, os objetivos do MVP e a
     documentação oficial;
  2. o agente tem livre poder para criar keypairs;
  3. o agente pode corrigir o README quando necessário e validado.
- **Aplicação registrada:**
  - Perfil A: a escolha será feita no gate D2c, por critério verificável —
    combinação que compila SBF o programa VeriCode e o exemplo `counter` de
    `risc0-solana v3.0.0` com locks preservados, preferindo a fidelidade ao
    Router (cuja CI usa Agave `2.3.9`) se ambas passarem, e registrando o
    motivo. Observação prévia, não decisão: o lock do Router resolve
    `solana-program 2.3.0`, e o Agave `2.1.0` não publica digest oficial do
    tarball. Instalações seguem homes isolados e pins exatos, sem alterar o
    perfil Ubuntu padrão.
  - Keypairs: o princípio 9 de `AGENTS.md` passa a permitir que o agente crie
    keypairs efêmeros **somente de devnet/localnet**, em gate autorizado, fora
    do clone, com permissão `0600`, sem exibir seed phrase ou chave privada e
    publicando apenas chaves públicas. Continuam proibidos: versionar,
    imprimir ou solicitar segredos; mainnet; dinheiro real.
  - README: atualizado somente com capacidades comprovadas por relatório.
- **Motivo:** remover os dois bloqueios humanos do caminho crítico D2–D4
  dentro do prazo, sem afrouxar evidência, isolamento ou claims.
- **Risco aberto:** a interpretação de "livre poder" foi limitada a
  devnet/localnet e a gates autorizados; ampliar isso exige nova decisão.
  Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.
- **Próximo gate:** D2b (política de escrow alinhada ao guia), seguido do D2c
  (Perfil A e skeleton Anchor local com custódia SPL).

## 2026-10-04 — D2b: alinhar a política pura de escrow ao guia de produto

- **Data:** 2026-10-04
- **Decisões humanas confirmadas no Plan Mode:**
  1. estados econômicos `Created` (≙ `Draft`), `Funded`,
     `Released { artifact_hash }` e
     `Refunded { Fail { artifact_hash } | Timeout }`; `Proving`, `Submitted` e
     `Failed` são estados de worker/UI, sem transição econômica;
  2. release por `Pass` somente com `current_slot <= deadline_slot`;
  3. refund por `Fail` válido permitido antes ou depois do prazo.
- **Leitura registrada do guia:** "refund por timeout só ocorre após
  `deadline_slot`" = `current_slot > deadline_slot`; no próprio slot do prazo
  o timeout falha e o release ainda é aceito. O slot é parâmetro de entrada.
- **Consequência:** o prazo particiona o tempo — até ele, executor (`Pass`) ou
  buyer (`Fail`); depois, somente buyer (`Fail` ou `Timeout`). Nunca há dois
  destinos competindo no mesmo slot. O vínculo do journal compara schema,
  Job, spec, harness e image por `JournalV1::validate_against`; o
  `artifact_hash` é registrado no estado terminal.
- **Evidência:** 36/36 testes com Rust `1.85.0` e `1.89.0`, locked/offline;
  matriz de 810 chamadas com exatamente 8 aceitas; árvores de dependência
  idênticas; locks, `lib.rs` e `JournalV1` inalterados;
  [`docs/d2b-escrow-guide-alignment-results.md`](d2b-escrow-guide-alignment-results.md).
- **Risco aberto:** ImageID não recertificado após mudanças no core; executor
  que prova após o prazo perde o release; revisão adversarial separada
  pendente; invariantes 1, 6 e 10 dependem do programa Anchor.
- **Próximo gate:** D2c — escolha do Perfil A pelo agente (D2a.2) e programa
  Anchor local com Job e custódia SPL, conforme
  `docs/handoffs/d2b-to-d2c.md`.

## 2026-10-04 — D2c: escolher o Perfil A e criar o programa Anchor local

- **Data:** 2026-10-04
- **Decisão (agente, sob D2a.2):** Perfil A = Anchor `0.31.1` + Agave
  `2.3.9` (platform-tools `v1.48`) + Rust host `1.89.0`, em homes isoladas
  `~/.local/share/vericode-spikes/d2c`.
- **Decisões humanas no Plan Mode:** workspace separado `anchor/`; Job PDA
  `["job", job_id]` (job_id globalmente único) e vault PDA `["vault", job]`;
  `refund_on_timeout` permissionless com destino fixo no buyer;
  `solana-program-test =2.3.9`. Durante a execução: prosseguir apesar do erro
  do prompt sobre `~/.cargo` e aceitar o Criterion v2.3.3 baixado
  automaticamente pelo SDK Agave.
- **Motivo:** a raia B compila SBF o `counter` oficial (dependência do Router)
  com locks preservados e o programa VeriCode; a raia A falha no `counter`
  porque o `rustc 1.79.0-dev` das platform-tools `v1.43` não aceita
  `unsafe(no_mangle)` em `risc0-zkvm-platform 2.2.1`. Como o Router é o
  caminho forte, a fidelidade à CI do `risc0-solana v3.0.0` prevalece sobre a
  recomendação genérica do Anchor.
- **Decisões técnicas registradas:**
  - lock do programa semeado com o lock oficial do `counter`, porque a
    resolução livre trouxe `anchor-* 0.31.2` e crates edition 2024 que o
    Cargo 1.84 das platform-tools não lê;
  - feature `token_2022` do `anchor-spl` exigida pelo código gerado de `init`
    no Anchor 0.31.1; o programa aceita somente o SPL Token clássico;
  - testes em workspace separado, com lock próprio.
- **Evidência:** `.so` B `04cc2a84…`; 10/10 testes em processo; IDL com
  exatamente 3 instruções; core 36/36 nas duas raias; locks antigos
  inalterados;
  [`docs/d2c-anchor-local-escrow-results.md`](d2c-anchor-local-escrow-results.md)
  e [`docs/escrow-program.md`](escrow-program.md).
- **Errata do handoff D2b→D2c:** o prompt afirmava `~/.cargo` ausente; ele
  existe desde 2026-09-29 (cache preexistente) e permaneceu idêntico ao
  snapshot. `~/.avm` também é preexistente (2026-09-29).
- **Pendente:**
  - freeze authority/allowlist do mint;
  - upgrade authority no deploy;
  - squatting de `job_id`; rent;
  - caminho de verificação (Router/CPI ou fallback atestado) para `release` e
    `refund_on_fail`.
- **Risco aberto:** platform-tools e Criterion sem digest oficial publicado;
  revisões adversariais D2b/D2c pendentes; Router/CPI/devnet
  `STATUS: NÃO VALIDADO`.
- **Próximo gate:** D2d, conforme `docs/handoffs/d2c-to-d2d.md`.

## 2026-10-04 — D2d: GO do caminho forte (Groth16 + Router em processo)

- **Data:** 2026-10-04
- **Decisões humanas no Plan Mode:**
  - autorizar a consulta e o pull da imagem
    `risczero/risc0-groth16-prover:v2025-04-03.1` (tag fixa em
    `risc0-groth16 3.0.2`) por digest amd64;
  - código do spike fora do clone;
  - orçamento de 3 h (usados cerca de 41 min).
- **Decisão:** **GO** para integrar `release`/`refund_on_fail` com CPI ao
  Verifier Router no D2e.
- **Motivo:**
  - receipts Groth16 reais de PASS e FAIL foram verificadas localmente;
  - o Router e o verificador de `risc0-solana v3.0.0`, compilados no Perfil
    A, rejeitaram proof adulterada, selector desconhecido, ImageID errado e
    journal digest errado, e aceitaram FIB, PASS e FAIL em
    `solana-program-test`.
- **Evidência:**
  - imagem `sha256:7f173963…e331`;
  - seals PASS `43735170…` e FAIL `93a12d12…`;
  - `.so` Router `1b26b017…` e verificador `dab6746d…`;
  - 110.851 CU por `verify`;
  - [`docs/d2d-groth16-router-spike-results.md`](d2d-groth16-router-spike-results.md).
- **Claim permitido:** "verificada pelo programa Verifier Router em
  `solana-program-test` local". Proibido: "ZK on-chain" em cluster.
  Devnet/Router real/CPI do `vericode_escrow`: `STATUS: NÃO VALIDADO`.
- **Registro de fronteira:** `~/.docker/buildx/current` (62 bytes, sem
  segredo) foi criado no perfil padrão pela consulta `buildx imagetools`; o
  container do prover roda sem isolamento de rede (código upstream).
- **Risco aberto:**
  - margem de RAM do prover (cerca de 6,25 de 7,6 GiB);
  - Program ID e dono do Router em devnet não confirmados;
  - vetores Groth16 só fora do clone;
  - revisões adversariais D2b/D2c pendentes.
- **Próximo gate:** D2e, conforme `docs/handoffs/d2d-to-d2e.md`.

## 2026-10-04 — D2c.1: rejeitar mints com freeze authority e versionar fixtures Groth16

- **Data:** 2026-10-04
- **Decisão humana:** resolver antes do D2e os riscos "freeze authority do
  mint" e "vetores Groth16 só fora do clone", conforme as recomendações do
  agente.
- **Decisões aplicadas:**
  - `create_job` rejeita mints com freeze authority, por constraint Anchor; o
    erro `MintHasFreezeAuthority` (6024) foi acrescentado ao fim do enum. A
    regra é uma pré-condição de custódia no programa, não regra econômica no
    core.
  - Os vetores públicos PASS/FAIL do D2d foram versionados em hex (não
    binário, por causa de `* text=auto eol=lf`) em
    `anchor/tests-local/fixtures/groth16/`, com teste de consistência.
- **Motivo:** sem a regra, o buyer poderia congelar o vault e impedir o
  pagamento ao executor. O SPL Token `7.0.0` impede adicionar freeze
  authority depois (`MintCannotFreeze`), então a checagem na criação basta.
  *Errata D2b.1 (R-D2 F-10):* o `7.0.0` é o crate cliente; o programa
  executado nos testes é o `spl_token-3.5.0.so` do `solana-program-test
  2.3.9`, no qual o comportamento foi confirmado. Repetir em devnet.
  As fixtures tornam o D2e reproduzível sem Docker nem prova.
- **Evidência:** `.so` `d66ac76b…`; escrow 12/12; fixtures 2/2; IDL com 3
  instruções e 25 erros; core 36/36; locks inalterados;
  [`docs/d2c1-mint-freeze-and-fixtures-results.md`](d2c1-mint-freeze-and-fixtures-results.md).
- **Registro:** o WSL foi reiniciado às 19:01. O `/tmp` foi limpo (helper e
  artefatos D1c2b originais); as cópias persistentes em
  `vericode-spikes/d2d/artifacts` conferem.
- **Risco aberto:** revisão adversarial separada pendente
  (`docs/handoffs/d2c1-to-review-d2b-d2c.md`); demais riscos do D2c/D2d
  inalterados.

## 2026-10-04 — R-D2: revisão adversarial do D2b/D2c/D2c.1 (REPROVADO para o D2e)

- **Data:** 2026-10-04
- **Decisão:** a revisão adversarial separada (Opus 5.5, max, somente leitura)
  reprova o início do D2e como especificado em `docs/handoffs/d2d-to-d2e.md`.
  O código on-chain de `create_job`/`fund`/`refund_on_timeout` não tem achado
  alto explorável; o bloqueio vem da política que o D2e levaria on-chain.
- **Motivo:**
  - F-01 (crítico, latente): qualquer pessoa gera um journal FAIL vinculado a
    qualquer Job avaliando um artefato arbitrário (o guest recebe `job_id`,
    artefato e `image_id` do provador); `refund_on_fail` aceita esse journal
    em qualquer slot, inclusive enquanto o release do PASS é possível.
  - F-02 (alto, latente): o buyer escolhe `image_id`/`spec_hash`/`harness_hash`
    em `create_job` e, portanto, o guest que decide o veredito.
  - F-03 (alto, latente): o plano do D2e não fixa o selector do Router; o dono
    do Router pode registrar um verificador arbitrário.
- **Evidência:** HEAD `42b4f58`; core 36/36 nas duas raias; `.so`
  `d66ac76b…` reproduzido; escrow 12/12, fixtures 2/2; 10 PoCs fora do clone,
  10/10; [`docs/r-d2-adversarial-review-results.md`](r-d2-adversarial-review-results.md).
- **Risco aberto:** F-04 upgrade authority (alto, bloqueia D4); F-05 mint sem
  allowlist (médio, bloqueia D4); F-06 a F-11 baixos; Router em devnet
  `NÃO VALIDADO`.
- **Registro:** feito por sessão com permissão de escrita a partir da resposta
  da revisão (que não podia editar arquivos).

## 2026-10-04 — Decisões humanas para o D2b.1 (correção do R-D2)

- **Data:** 2026-10-04
- **Decisão humana:** adotar as recomendações do agente para F-01, F-02, F-07,
  F-08 e F-11 no gate D2b.1.
  1. **F-01 — opção (A), compromisso de entrega assinado pelo executor.**
     - `deliver(artifact_hash)` só a partir de `Funded`, só pelo executor
       (signer), uma única vez, até o prazo (inclusive).
     - Só o hash fica on-chain, com a mesma semântica de
       `hash_restricted_artifact` do journal.
     - `release` e `refund_on_fail` exigem `Delivered { h }` e journal com
       `artifact_hash == h`.
     - `refund_on_timeout` aceita `Funded` e `Delivered` após o prazo.
     - **Divergência explícita** do guia §5 ("o programa registra o
       `artifact_hash` apenas na liquidação") e da decisão D2a.1 ("o executor
       não pré-registra"). Motivo: R-D2 F-01. Com o guest atual, o registro
       só na liquidação permite que qualquer pessoa force um refund com um
       FAIL de artefato arbitrário. A precedência (`docs/project-context.md`)
       admite decisão humana posterior que registre a divergência e o motivo.
  2. **F-02 — termos admitidos da v1.** `create_job` mantém os parâmetros
     (IDL estável), mas rejeita:
     - `spec_hash` diferente de `hash_restricted_spec(&RESTRICTED_SPEC_V1)`;
     - `harness_hash` diferente de
       `hash_harness_version(DETERMINISTIC_HARNESS_VERSION)`, ambos calculados
       pelo core como fonte única;
     - `image_id` diferente da constante `ADMITTED_IMAGE_ID_V1 =
       4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a`
       (ELF D1c2b preservado).
  3. **F-07 — janela de prazo.**
     `Clock.slot + MIN_DEADLINE_WINDOW_SLOTS <= deadline_slot <= Clock.slot +
     MAX_DEADLINE_WINDOW_SLOTS`, com `MIN = 1_500` (≈ 10 min a 400 ms/slot,
     cobrindo a prova Groth16 de cerca de 90 s com margem) e
     `MAX = 1_512_000` (≈ 7 dias). O executor deve ser diferente das PDAs do
     Job e do vault.
  4. **F-08 e F-11 no próprio D2b.1:** `address = job.mint` nas contas de mint
     e as lacunas de teste do R-D2.
- **Mantido para o D2e:** F-03 (selector `73c457ba` e PDA do verifier entry
  fixados), F-06 (ATA canônica), F-12 (digest sobre os 165 bytes e
  `job.image_id`). Antes do D4: F-04 e F-05.
- **Limitação declarada:** a spec v1 de desenvolvimento aceita qualquer par
  `(n, 2n)` escolhido pelo executor. A decisão (A) impede terceiros de trocar
  o artefato, mas não torna a tarefa não trivial. Os claims devem dizer isso.
- **Risco aberto:** o ImageID admitido depende do ELF D1c2b preservado; um
  rebuild do guest com o core atual exige recertificação e atualização da
  constante.
- **Próximo gate:** D2b.1, conforme `docs/handoffs/r-d2-to-d2b1.md`.

## 2026-10-04 — D2b.1: vincular a liquidação à entrega do executor e aos termos admitidos da v1

- **Data:** 2026-10-04
- **Decisões aplicadas:** as "Decisões humanas para o D2b.1" acima (F-01
  opção A, F-02, F-07, F-08, F-11), sem decisão de produto nova.
- **Desenho técnico aprovado no Plan Mode:**

  | Item | Onde | Conteúdo |
  | --- | --- | --- |
  | `JobV1::deliver(state, deliverer, artifact_hash, current_slot)` | core | só de `Funded`, só o executor, até o prazo, uma vez → `Delivered { artifact_hash }` |
  | `release` / `refund_on_fail` | core | só de `Delivered { h }`, com `validate_against` sobre `h` (`ArtifactHashMismatch`); de `Funded` → `NotDelivered` |
  | `refund_on_timeout` | core | aceita `Funded` e `Delivered` |
  | `JobV1::admit(admitted_image_id, current_slot)` | core | spec e harness da v1 calculados pelo core; ImageID igual ao parâmetro; janela `MIN = 1_500`, `MAX = 1_512_000` por `checked_sub`, sem overflow; hash incalculável não admite nada |
  | `ADMITTED_IMAGE_ID_V1`, leitura do `Clock` | programa | o core é compilado no guest, então a constante do ImageID no core seria circular |
  | executor ≠ PDA do Job e do vault | programa | pré-condição sobre endereços Solana, como a regra de freeze |
  | `address = job.mint @ MintMismatch` | programa | contas mint de `fund` e `refund_on_timeout` (F-08) |

  - Erros novos no fim: core `NotDelivered`, `AlreadyDelivered`,
    `DelivererMismatch`, `SpecNotAdmitted`, `HarnessNotAdmitted`,
    `ImageIdNotAdmitted`, `DeadlineOutOfWindow`; programa 6025–6032
    (6032 `ExecutorIsProgramAccount`).
  - `EscrowStatus::Delivered` com tag 5; 6000–6024 e tags 0–4 inalterados;
    `INIT_SPACE` 276.
- **Divergência explícita do guia §5 e do D2a.1:** o `artifact_hash` deixa de
  ser registrado "apenas na liquidação" e passa a ser compromisso de entrega
  do executor. Motivo: R-D2 F-01, registrado na decisão humana acima.
- **Errata F-10:**
  - `MintCannotFreeze` atribuído ao programa executado `spl_token-3.5.0.so`;
  - "constraints validam somente estrutura" corrigido;
  - unicidade do `job_id` qualificada "por implantação" (F-15);
  - "artefato vinculado ao Job" redefinido como o artefato do `deliver`;
  - a frase "em nenhum slot dois destinos competem" foi substituída pela
    propriedade testada `at_most_one_destination_per_state_and_slot`, com
    premissa explícita;
  - o relatório `docs/d2c1-mint-freeze-and-fixtures-results.md` ficou fora do
    escopo do gate; a errata está aqui e no relatório D2b.1.
- **Evidência:**
  - core 42/42 nas duas raias, sem warnings;
  - 5 mutações mortas, incluindo a política F-01;
  - `.so` `ea0dd92c…1a37`;
  - escrow 24/24, layout 4/4, fixtures 2/2;
  - contra o `.so` do D2c.1, 11 dos 24 testes falham onde a correção atua
    (F-08: `3` do SPL em vez de 6011);
  - `create_job` 22.083 → 36.267 CU; `deliver` 4.949 CU;
  - IDL com 4 instruções e 33 erros;
  - locks inalterados;
  - [`docs/d2b1-delivery-binding-results.md`](d2b1-delivery-binding-results.md).
- **Risco aberto:**
  - ImageID admitido não recertificado (guest não reconstruído; `escrow.rs`
    está na crate do guest);
  - spec v1 trivial;
  - F-03, F-06 e F-12 para o D2e; F-04 e F-05 antes do D4;
  - revisão adversarial de D2b.1 + D2e pendente.
- **Próximo gate:** D2e, conforme `docs/handoffs/d2b1-to-d2e.md`.

## 2026-10-04 — D2e: liquidação por veredito com CPI ao Verifier Router (local)

- **Data:** 2026-10-04
- **Decisões confirmadas no Plan Mode** (as "a confirmar" de
  `docs/handoffs/d2b1-to-d2e.md`):
  1. CPI manual ao Router (discriminador `verify` `85a18d3078c65896` +
     `RouterSeal` com o Borsh do `Seal` + `image_id` + digest), sem
     dependência nova;
  2. Router ID fixo `6JvFfBrv…` (upstream), sem features de cluster;
  3. `release`/`refund_on_fail` permissionless;
  4. contas do Router montadas no genesis dos testes, com o layout do fonte
     pinado; `.so` reconstruídos offline do commit pinado;
  5. Program ID do verificador também fixado (`THq1qFYQ…`).
- **Aplicado:**
  - ordem decode → core → ATA canônica → selector → SHA-256 dos mesmos 165
    bytes → CPI com `job.image_id` → transferência → estado;
  - F-03: selector `73c457ba` e entrada `["verifier", 73c457ba]` fixos;
  - F-06: ATA canônica nas três liquidações, incluindo `refund_on_timeout`;
  - F-08: `address = job.mint` nas contas novas;
  - F-12: digest e `image_id`, como na ordem acima.
  - Erros 6033 `UnexpectedSelector`, 6034 `JournalMalformed`, 6035
    `DestinationNotCanonical`; 6000–6032 e as tags 0–5 inalterados.
- **Correção de premissa:** `release`/`refund_on_fail` consomem 135–142 k CU
  e cabem no limite padrão de 200 k. O compute budget é margem, não
  requisito.
- **Evidência:**
  - Router `1b26b017…` e verificador `dab6746d…` reproduzidos offline;
  - `.so` `6457aecf…` reproduzido em target limpo;
  - escrow 25/25, settlement 16/16, layout 6/6, fixtures 2/2;
  - contra o `.so` do D2b.1: escrow 24/25 (só o teste F-06 falha) e
    settlement 0/16;
  - IDL com 6 instruções e 36 erros;
  - locks e perfil padrão inalterados;
  - [`docs/d2e-router-settlement-results.md`](d2e-router-settlement-results.md).
- **Claim permitido:** "verificado por CPI ao Verifier Router em
  `solana-program-test` local". Proibido: "ZK on-chain" em cluster.
- **Risco aberto:**
  - Router em devnet `NÃO VALIDADO`;
  - confiança residual no e-stop do dono do Router e nas upgrade authorities
    (F-04);
  - F-05 (allowlist do mint);
  - ATA precisa existir antes da liquidação;
  - ImageID não recertificado;
  - spec v1 trivial;
  - revisão adversarial de D2b.1 + D2e pendente.
- **Próximo gate:** R-D2e, conforme `docs/handoffs/d2e-to-r-d2e.md`.

## 2026-10-05 — R-D2e: revisão adversarial de D2b.1 + D2e (APROVADO COM RESSALVAS para o D4)

- **Data:** 2026-10-05
- **Decisão:** a revisão adversarial separada (Opus 5.5, max, somente leitura)
  aprova com ressalvas o início do gate devnet (D4). Não há achado crítico ou
  alto no código de D2b.1/D2e. F-01, F-02, F-03, F-06, F-08, F-10, F-11 e
  F-12 estão resolvidos; F-07 está parcial (R-02).
- **Motivo:**
  - R-01 (médio): as constantes do Router são os endereços upstream; os `.so`
    testados só executam neles (`DeclaredProgramIdMismatch`), e o Router
    testado embute o `INITIAL_OWNER` de teste. O Router em devnet exige
    decisão antes do deploy.
  - R-02 (baixo): `fund` não respeita a janela do prazo (Job `Funded` depois
    do prazo; PoC-4).
  - R-03 (baixo, reclassifica F-15): a mesma prova liquida o Job `0x11` numa
    segunda implantação (PoC-8).
  - R-04 a R-07: claims, lacunas de teste, liveness do e-stop e margens.
- **Condições do D4:**
  - C1: Router (upstream verificado, Router próprio com revisão delta ou
    nenhum);
  - C2: F-04;
  - C3: F-05;
  - C4: `job_id` aleatório e receipts novas;
  - C5: `create_job`+`fund` atômicos ou checagem da janela;
  - C6: claim on-chain só com transação devnet de CPI;
  - C7: `MintCannotFreeze`, tamanhos e CU repetidos em devnet.
- **Evidência:**
  - HEAD `c0aba7d`; core 42/42 A/B;
  - `.so` `6457aecf…` reproduzido; Router `1b26b017…` e verificador
    `dab6746d…` reconstruídos offline;
  - escrow 25, settlement 16, layout 6, fixtures 2; IDL `37a3028a…`;
  - 8 PoCs fora do clone, 8/8 (`rd2e_poc.rs` `a8aced16…`);
  - [`docs/r-d2e-adversarial-review-results.md`](r-d2e-adversarial-review-results.md).
- **Aplicado no registro:** errata R-04 (a)–(d) em `docs/router-notes.md` e
  `docs/escrow-program.md`. Os relatórios D2e e R-D2e ficam como registro
  histórico; a errata deles está no relatório do R-D2e.
- **Risco aberto:** F-04 e F-05 (bloqueiam D4); R-01, R-02, R-03, R-05,
  R-06 e R-07; F-09, F-13, F-14; ImageID não recertificado; spec v1 trivial.
- **Registro:** feito por sessão com permissão de escrita a partir da resposta
  da revisão, que não podia editar arquivos.
- **Próximo gate:** D4, conforme `docs/handoffs/r-d2e-to-d4.md`.

## 2026-10-05 — Decisões humanas para o D4 (escolhas delegadas ao agente)

- **Data:** 2026-10-05
- **Decisão humana:** "Eu autorizo que esta sessão registre o R-D2e agora" e
  "Considerando o nosso projeto, seus objetivos e tudo o que já foi feito até
  aqui, tome as melhores escolhas para o nosso MVP." O humano delegou ao
  agente as decisões D4-0 a D4-6 propostas pelo R-D2e, incluindo as
  irreversíveis: finalizar a upgrade authority do escrow e reutilizar o
  program ID `GZqbL2Tb…`.
- **Escolhas do agente:**
  1. **D4-0 Rede.**
     - Somente `https://api.devnet.solana.com` (JSON-RPC e
       `requestAirdrop`), começando por uma fase somente leitura.
     - crates.io só para o cliente devnet fora do clone, com lock semeado de
       `anchor/tests-local/Cargo.lock`.
     - Links do Explorer são gerados, não acessados. Nenhum outro destino;
       nenhum pull Docker.
  2. **D4-1 Router (R-01/C1): (a) upstream, se verificável.**
     - Critérios obrigatórios:
       1. `6JvFfBrv…` e `THq1qFYQ…` executáveis, com ProgramData e upgrade
          authorities lidos;
       2. PDA `["router"]` (`4Sh5ofCz…`) com o dono registrado; entrada
          `["verifier", 73c457ba]` (`4Z7ok78x…`) com `verifier = THq1q…` e
          `estopped = false`;
       3. `simulateTransaction` de `Router.verify`: FIB oficial e fixtures
          PASS/FAIL aceitos; seal adulterado e ImageID errado rejeitados.
     - O dump do verificador comparado a `dab6746d…` é registrado, mas uma
       divergência de bytes não reprova sozinha (toolchain).
     - **Se (a) falhar, segue automaticamente para (b), só local:**
       - fork do commit `ee415935` fora do clone, com novos `declare_id` do
         Router e do verificador e `INITIAL_OWNER` = pubkey do deployer;
       - verificador com upgrade authority = PDA do Router; Router com
         upgrade authority final;
       - troca das quatro constantes do escrow e de `tests/layout.rs`;
       - teste de ciclo completo em `solana-program-test`;
       - depois, **PARAR** para a revisão delta separada R-D4a antes de
         qualquer escrita em devnet.
     - (c), sem Router, não é escolhida: tira do MVP a verificação da prova
       pelo contrato. Só entra por decisão humana nova, se (b) também
       bloquear.
  3. **D4-2 Upgrade authority (F-04/C2).**
     - O escrow é implantado com o keypair existente
       `~/.local/share/vericode-spikes/d2c/keys/vericode_escrow-keypair.json`
       (`GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`), sem mudar código.
     - Ordem:
       1. deploy e `solana program show`;
       2. um smoke run real (Job S: create+fund, deliver+release com receipt
          própria), rotulado "smoke pré-finalização, não evidência";
       3. `solana program set-upgrade-authority --final` e `program show` de
          novo;
       4. só então as transações de evidência.
     - Se o smoke falhar, não finalizar: diagnosticar. Mudança de código exige
       R-D4a.
     - Um bug depois da finalização exige novo program ID, mudança de código e
       R-D4a.
     - Em (a), as autoridades do Router e do verificador upstream são
       registradas como confiança explícita, não controlada pelo projeto
       (inclui o e-stop, R-06).
     - Em (b), o deployer, como dono do Router, mantém e-stop e
       `add_verifier`, documentados como confiança explícita.
  4. **D4-3 Mint (F-05/C3).**
     - Em (a): opção (ii). O cliente confere e exibe, antes de cada operação,
       mint, `decimals = 6`, `freeze_authority = None` e os termos do Job.
       A limitação fica declarada; o programa não muda.
     - Em (b): opção (i). Constante do mint admitido = pubkey do Test USDC
       (keypair em `d4/keys`), erro novo no fim (6036), coberta pela R-D4a.
  5. **D4-4 Jobs e receipts (C4/R-03).**
     - `job_id` aleatório de 32 bytes por Job (CSPRNG do SO), nunca `0x11`.
     - Receipts novas por Job, provadas a partir do ELF D1c2b preservado
       (`d2d/artifacts/vericode-guest.bin`, ImageID `4da06f90…fb1a`), sem
       reconstruir o guest.
     - Harness `d4/receipts` (cópia de `d2d/receipts`) com o subcomando
       `prove`, que reproduz o frame do host (`job_id ‖ artefato 12 B ‖
       image_id`), mais `compress`.
     - Prover local sem dev mode; Groth16 pela imagem Docker local
       `sha256:7f173963…`, com `--pull=never` e um cenário por vez.
     - As fixtures `0x11` são só fallback rotulado.
  6. **D4-5 Janela (C5/R-02).**
     - `create_job` e `fund` na mesma transação; a prova é gerada antes.
     - `deliver` e `release` na mesma transação, quando couberem em 1.232
       bytes.
     - O executor confere os slots restantes antes de entregar.
     - Mudar `fund` no core fica para gate próprio, depois do MVP.
  7. **D4-6 Chaves e SOL.**
     - Keypairs efêmeros de devnet novos em `~/.local/share/vericode-spikes/d4/keys`
       (`0600`), só com pubkeys publicadas, dentro dos papéis do princípio 9
       (`AGENTS.md`):
       - deployer/payer, buyer, executor e mint do Test USDC;
       - em (b), também os keypairs de programa do Router e do verificador,
         como parte do papel de deployer (precedente: D2c e D2d).
     - Nenhum keypair "terceiro": os negativos são assinados pelo buyer ou
       pelo deployer, e as liquidações são permissionless.
     - SOL por `solana airdrop`, dentro dos limites. Se for insuficiente,
       **BLOQUEADO**: a sessão informa ao humano a pubkey do deployer e o
       valor, para obter SOL pelo faucet web. Nunca pede chave.
  8. **Orçamento e sequência.**
     - D4: 1 dia, com timestamps por fase.
     - Depois do D4, D5: CLI reproduzível no repositório, README com
       versões, hashes, links e limitações (M6/M7) e roteiro da demo.
     - Worker e UI só se sobrar tempo (guia §3).
     - R-05 (PoCs na suíte) entra no próximo gate que alterar
       `anchor/tests-local`; em (b), já no D4.
- **Motivo:**
  - (a) primeiro: sem código novo nem revisão delta, é o caminho mais curto
    para o claim C6.
  - (b) automático em vez de (c): preserva a verificação da prova pelo
    contrato, que é o núcleo do MVP, e a R-D4a mantém o gate de segurança.
  - Finalizar a autoridade: um escrow upgradeable é um bypass administrativo
    (princípio 7). O smoke antes da finalização reduz o risco de um programa
    inutilizável sem volta.
  - Mint (ii) em (a): evita mudança de código e revisão. Mint (i) em (b): a
    mudança e a revisão já acontecem.
  - `job_id` aleatório e receipts novas fecham o replay de R-03 nas
    transações exibidas.
  - Atomicidade de `create_job`+`fund`: mitiga R-02 sem mexer no core.
- **Risco aberto:**
  - Em (a): confiança nas autoridades upstream do Router e do verificador.
  - Finalização irreversível do escrow.
  - Mint não restrito no programa em (a).
  - Disponibilidade do faucet.
  - ImageID não recertificado; spec v1 trivial.
- **Próximo gate:** D4, conforme `docs/handoffs/r-d2e-to-d4.md`.

## 2026-10-05 — Prazo de 11/10, congelamento do JournalV1 v1 e nomes de gate

- **Data:** 2026-10-05
- **Decisões humanas:**
  1. **Prazo de entrega: 11/10**, não por volta de 8/10. Nas palavras do
     humano: "Não se apegue muito às datas, o tempo não é até dia 8, mas sim
     11". As datas de `docs/context/sequencia-mvp.md` passam a ser
     indicativas, e vale a ordem dos gates. Substitui o limite de 1 dia do
     D4 (item 8 de "Decisões humanas para o D4").
  2. **`JournalV1` congelado como está hoje**: "eu aprovo congelar o
     JournalV1 como está hoje (165 bytes)".
     - Ficam congelados o layout, os offsets e o hashing atuais de
       `docs/manifest-schema.md` e do core, sem mudar nenhum byte.
     - Isso cumpre o D4 do plano ("congela o formato do journal… especificação
       assinada pelo time").
     - A atualização de `docs/manifest-schema.md` (de "candidato" para "v1
       congelado") fica para o D4, em Plan Mode (`CLAUDE.md`).
     - Uma mudança futura exige novo `schema_version`, novo guest e ImageID e
       nova admissão no programa; como o escrow será finalizado, também um
       novo program ID.
  3. **Ajustes de documentação autorizados** (proposta do agente aceita):
     - correção do prazo nos documentos vivos;
     - tabela "gate → dia do plano" em `docs/project-context.md`;
     - prompt do D4 ajustado.
- **Escolhas do agente dentro dessa autorização:**
  - **Nomes de gate.** A partir do D4, o ID do gate é o dia do plano cujo
    conteúdo ele entrega. O gate seguinte ao D4 é o `D7` (CLI de ponta a
    ponta, com README e demo do D9), e não "D5" como dizia o item 8 de
    "Decisões humanas para o D4". O handoff do D4 passa a ser
    `docs/handoffs/d4-to-d7.md`.
  - **R-05 no D4.** Os PoCs 1 a 7 do R-D2e entram em `anchor/tests-local`
    como testes de regressão nos dois caminhos.
    - Só testes: o programa, os `Cargo.toml` e os locks não mudam. No
      caminho (a), o `.so` continua `6457aecf…`, então não há R-D4a.
    - O PoC-8 depende de uma variante com outro program ID e fica registrado
      apenas no R-D2e.
- **Motivo:**
  - Com três dias a mais, o D4 pode cumprir o que o plano pede para o dia
    (journal congelado) e fechar R-05 sem pressa.
  - Os nomes D2a…D2e esconderam que D3, D5 e D6 já estavam feitos.
- **Próximo gate:** D4, conforme `docs/handoffs/r-d2e-to-d4.md` (ajustado).

## 2026-10-05 — D4a: Router upstream reprovado em devnet; CPI direta ao verificador Groth16 imutável e mint admitido

- **Data:** 2026-10-05
- **Resultado do D4-1, caminho (a) — reprovado** (reconhecimento somente
  leitura, `docs/d4a-direct-verifier-results.md`):
  1. Critério 1 ok: `6JvFfBrv…` e `THq1qFYQ…` executáveis; ProgramData com
     upgrade authority `None` nos dois, finalizados em 23/10/2025 pela chave
     upstream `7uAQqk…`.
  2. Critério 2 falhou: a PDA `["router"]` e a entrada `73c457ba` não
     existem. Só o `INITIAL_OWNER` upstream pode inicializar o Router, e
     `add_verifier` exige o verificador sob a PDA do Router; com upgrade
     authority `None`, `THq1q…` nunca poderá ser registrado.
  3. Critério 3 falhou: `Router.verify` → 3012 `AccountNotInitialized` nos
     5 casos simulados.
  - O verificador `THq1q…` chamado direto, em simulação: FIB, PASS e FAIL
    aceitos a 99.541 CU; seal adulterado → 6003; ImageID errado → 6000. Os
    bytes implantados (`34ae6e5c…`) diferem do rebuild local `dab6746d…`.
- **Decisão humana (AskUserQuestion nesta sessão), entre (b) Router próprio,
  já decidido, e (b′) verificador direto: (b′).** O escrow chama por CPI o
  verificador Groth16 imutável `THq1q…`, sem Router. Também foi decidido
  gerar já as receipts novas de S, A, A′ e B.
- **Divergência explícita do guia** §1 (item 5: "verificação on-chain pelo
  Verifier Router") e §8 ("chama a verificação compatível pelo Router"), e
  da decisão D4-1 (caminho (b) automático).
  - Motivo:
    - o Router upstream de devnet é permanentemente inutilizável com esse
      verificador;
    - um Router próprio traria um dono com e-stop e `add_verifier`
      (confiança explícita) e cerca de 6,4 SOL de rent, o que provavelmente
      exigiria o faucet web humano;
    - o verificador chamado é o mesmo programa e os mesmos argumentos que o
      Router repassaria; com selector e verificador já fixados no escrow, o
      Router só acrescentava o e-stop;
    - o verificador é imutável e de terceiros, então ninguém, nem o projeto,
      pode trocá-lo ou pausá-lo.
  - Custo aceito: não há e-stop contra um bug de soundness do verificador
    (aviso do fonte upstream). Com o escrow finalizado, a correção exigiria
    um novo program ID.
  - Claim: "verificado por CPI ao verificador Groth16 de `risc0-solana
    v3.0.0`"; nunca "Verifier Router". "Verificado on-chain em devnet" só
    depois de uma transação de liquidação em devnet (D4b).
- **Escolhas do agente dentro da decisão:**
  - **Mint admitido (D4-3, opção (i)).** Motivo: o mesmo do caminho (b): a
    mudança de código e a revisão já acontecem.
    - Constante `ADMITTED_MINT = 9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F`
      (keypair em `d4/keys`, fora do clone).
    - Erro 6036 `MintNotAdmitted` no fim do enum, checado depois da regra de
      freeze (6024).
  - **Selector `73c457ba` (6033) mantido** antes da CPI, como vínculo
    explícito aos parâmetros do verificador.
  - **Renomeações:** `RouterSeal` → `Groth16Seal` (mesmo layout Borsh) e
    `ROUTER_VERIFY_DISCRIMINATOR` → `VERIFY_DISCRIMINATOR` (mesmos bytes).
  - **Contas:** `SettleWithProof` passa a ter 7 contas (sem `router_program`,
    `router` e `verifier_entry`); a CPI tem 328 bytes de dados e a única
    conta é o system program.
  - **`JournalV1` v1 congelado aplicado** em `docs/manifest-schema.md`
    (decisão humana "Prazo de 11/10…"), sem mudança de código.
  - **R-05:** os PoCs 1 a 7 do R-D2e entram como `tests/regressions.rs`.
- **Evidência:**
  - F1: `d4/logs/f1-recon.log` (contas, ProgramData, histórico do loader,
    simulações); dump `34ae6e5c…`;
  - `.so` D4a 395.064 bytes `cdf6967f…`; IDL `e8ce2c20…` (6 instruções, 37
    erros);
  - suíte 57/57 com o verificador `dab6746d…` e 57/57 com os bytes de
    devnet: escrow 26, settlement 15, regressions 7, layout 7, fixtures 2;
  - contra o `.so` do D2e: 25/26, 1/7 e 0/15 onde a mudança atua;
  - `release` 121.885 CU e `refund_on_fail` 123.214–126.214 CU;
  - receipts S, A, A′ e B (`job_id` aleatório) verificadas localmente e
    aceitas pelo verificador de devnet em simulação; o journal A com o seal
    de S é rejeitado (6000);
  - baseline do R-D2e reproduzido; locks e perfil padrão inalterados;
  - [`docs/d4a-direct-verifier-results.md`](d4a-direct-verifier-results.md).
- **Risco aberto:**
  - sem e-stop;
  - bytes do verificador de devnet ≠ rebuild local (equivalência funcional
    testada);
  - upgrade authority do escrow ainda não finalizada (D4b);
  - R-02 (fund fora da janela);
  - ImageID não recertificado; spec v1 trivial;
  - disponibilidade do faucet.
- **Próximo gate:** R-D4a (revisão delta somente leitura), conforme
  `docs/handoffs/d4a-to-r-d4a.md`; depois, D4b (devnet).

## 2026-10-05 — R-D4a: revisão delta do D4a (APROVADO COM RESSALVAS para o D4b)

- **Data:** 2026-10-05
- **Revisor:** Claude Code (Opus 5.5, esforço max), sessão separada e somente
  leitura; HEAD revisado `24364ae` sobre `fb4bfba`.
- **Resultado:** **APROVADO COM RESSALVAS** para o D4b; nenhum achado crítico
  ou alto.
  - RD4A-01 (médio, operação): a CLI `solana` 2.3.9 imprime a seed phrase do
    buffer efêmero se o deploy falhar no meio; usar `--buffer`.
  - RD4A-02 (baixo): a criação do mint admitido é irreversível; o programa não
    confere decimals; Token-2022 no endereço → 3007 permanente.
  - RD4A-03 (baixo): program ID, mint e `job_id`s S/A/B são públicos;
    pré-financiamento ou squatting pode bloquear o D4b (hoje: nenhum).
  - RD4A-04 (info): o selector `73c457ba` é checagem de formato, não vínculo
    criptográfico (mutante M2).
  - RD4A-05 (info): o verificador de devnet é estruturalmente equivalente ao
    rebuild (VK, control root, identity control ID, tags); as diferenças vêm
    do build em macOS.
  - RD4A-06 (info): sem e-stop; o impacto se limita à decisão entre as duas
    partes fixas do Job.
  - RD4A-07 (baixo): erratas e lacunas menores de docs e testes.
  - RD4A-08 (baixo): implantar e finalizar só o `.so` `cdf6967f…`, conferido
    antes e pelo dump.
- **Condições para o D4b:** CD1 a CD9 (`docs/r-d4a-review-results.md`, seção 5).
- **Evidência:** checagem do D4a reproduzida sem divergência; 10 PoCs (8 nas
  duas variantes do verificador, 2 contra mutante); análise de bytes;
  [`docs/r-d4a-review-results.md`](r-d4a-review-results.md).
- **Errata aplicada no registro (CD8, parte de docs):**
  - RD4A-04: a entrada "D4a…" acima chama o selector de "vínculo explícito
    aos parâmetros do verificador"; o correto é "checagem de formato do
    seal". Corrigido em `docs/escrow-program.md`.
  - RD4A-05: "equivalência funcional" passa a "equivalência estrutural e
    funcional" em `docs/escrow-program.md`, `docs/router-notes.md` e
    `docs/agent-control.md`. O relatório D4a fica como registro histórico.
  - RD4A-07 (b), (c) e (d): fixtures README sem o Router como verificador
    atual; custo medido do deploy ≈2,03 SOL (não 2,75); deployer com 5 SOL.
  - Ficam para o D7: RD4A-07 (a) comentário em `settlement.rs`, (b)
    comentário em `groth16_fixtures.rs`, (e) `.env.example` e (f) testes
    novos.
- **Escolhas do agente para o D4b** (autorização humana do registro, com
  "autorizo"):
  - `amount` de cada Job = 1 Test USDC (1.000.000 unidades);
  - estoque cunhado de uma vez para o buyer = 1.000.000 Test USDC (D4b e
    D7);
  - prazos: Job S ≈ slot + 3.000; Jobs A e B ≈ slot + 9.000 (cerca de 1 h,
    para os negativos); Job C = slot + 1.500 (mínimo da janela);
  - SOL ao buyer ≈ 0,05 e, se necessário, ≈ 0,01 ao executor;
  - saída bruta da CLI nunca no terminal, só em log `0600`, filtrada;
  - commits locais do D4b autorizados ao enviar o prompt; push proibido.
- **Risco aberto:** F-04 até a finalização; R-02; R-03/F-09 com `job_id`s
  públicos; F-13; F-14; ImageID não recertificado; spec v1 trivial; sem e-stop.
- **Registro:** feito por sessão com permissão de escrita a partir da resposta
  da revisão, que não podia editar arquivos.
- **Próximo gate:** D4b, conforme `docs/handoffs/r-d4a-to-d4b.md`.

## 2026-10-05 — D4b: escrow implantado e finalizado em devnet; liquidações PASS, FAIL e timeout

- **Data:** 2026-10-05
- **Autorização:** o prompt do D4b (`docs/handoffs/r-d4a-to-d4b.md`), enviado
  pelo humano, autoriza as escritas em devnet e os commits locais. O plano com
  os comandos de E1 a E8, o hash do `.so`, o `--buffer` e a ordem CD5 foi
  aprovado pelo humano em Plan Mode antes da primeira escrita.
- **Resultado (evidência em devnet):**
  - escrow `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH` implantado com o
    `.so` do D4a; dump de 395.064 bytes = `cdf6967f…`;
  - upgrade authority **`none`**, finalizada depois do smoke (tx
    `4AsofYxr…`, slot 507.798.793);
  - mint Test USDC `9TE2VPFm…` criado: Tokenkeg, 6 decimais, sem freeze;
    1.000.000 Test USDC cunhados para o buyer.
  - **Jobs:**
    - S (smoke, não evidência) e A: `Released`;
    - B: `RefundedOnFail`;
    - C: `RefundedOnTimeout`.
  - Os negativos aterrissaram com o código esperado e estado igual: 6017,
    6014, 6000 e 6003 (no verificador), 6033, 6019, 6014 e 6021.
  - C7: `MintCannotFreeze` (`Custom(16)`) no Tokenkeg de devnet.
  - Relatório: [`docs/d4b-devnet-results.md`](d4b-devnet-results.md).
- **Claim liberado (CD7):** "verificada em devnet por CPI ao verificador
  Groth16 imutável de risc0-solana v3.0.0", sempre com os links das
  liquidações A e B. Nunca "Verifier Router".
- **Escolhas do agente dentro do plano aprovado:**
  - **Pagadores:**
    - buyer: `create+fund` e os refunds;
    - executor: `deliver` e `release`;
    - deployer: os negativos e o C7, como terceiro, porque a liquidação é
      permissionless.
  - **Prazo do Job C:** slot + 1.560, isto é, o mínimo de 1.500 com 60 slots
    de margem para a aterrissagem.
  - **Job C** com `job_id` aleatório novo (`96598a41…`).
  - **Deploy com `--use-rpc`**, para manter o tráfego só em
    `api.devnet.solana.com`, sem o cliente TPU.
  - **Cliente devnet** fora do clone:
    - transações montadas e assinadas offline pelos builders de
      `anchor/tests-local/tests/common/mod.rs` (lock `be94760a…`, sem
      crates.io);
    - RPC por Python stdlib.
- **Desvio registrado:**
  - A tentativa 1 da CLI falhou com "Data writes to account failed: Max
    retries exceeded", com só 32 escritas aterrissadas; o RPC público limita
    o envio em rajada.
  - O buffer `3CVLy…` foi completado com 361 escritas `Write` do loader v3,
    cadenciadas a cerca de 3 tx/s e assinadas pelo deployer (authority do
    buffer), com bytes só do `.so` conferido.
  - A tentativa 2 foi o comando exato do plano: a CLI comparou o buffer
    chunk a chunk e implantou.
  - O dump confirmou os bytes antes da finalização (CD1).
- **Motivo:** repetir a CLI exigiria cerca de 10 tentativas sob limite de
  taxa. A retomada cadenciada não muda o que é implantado: CLI e dump
  conferem cada byte.
- **Risco aberto:**
  - sem e-stop (escrow e verificador imutáveis);
  - rent preso (F-13);
  - mint authority = deployer;
  - `job_id`s S/A/B/C consumidos;
  - RPC público com limite de taxa, que exige cadência na CLI do D7;
  - R-02 e R-03;
  - ImageID não recertificado; spec v1 trivial;
  - RD4A-07 (a), (e) e (f) para o D7.
- **Próximo gate:** D7, conforme `docs/handoffs/d4b-to-d7.md`.

## 2026-10-05 — D7: CLI e prover no repositório; fluxo de ponta a ponta em devnet

- **Data:** 2026-10-05
- **Decisões humanas** (recomendadas pelo agente e ratificadas ao enviar o
  prompt do D7, versão revisada de `docs/handoffs/d4b-to-d7.md`):
  1. **CLI em Rust em `cli/`**, com workspace e lock próprios. Monta as
     instruções a partir das contas e dos discriminadores da IDL do D4a
     (`e8ce2c20…`) e usa o `vericode-core` para hashes e journal. Nenhum lock
     existente muda.
  2. **Dependências:** lock novo semeado de `anchor/tests-local/Cargo.lock` e
     de `d4/receipts/Cargo.lock`, compilado `--offline`. Um único fetch do
     crates.io ficou autorizado se faltasse um crate.
  3. **Prover em `prover/`**, com workspace e lock próprios (semente
     `ec0dd8d6…`), fora de `zkvm/`:
     - binário do guest versionado em `prover/artifacts/vericode-guest.bin`,
       com proveniência, e `*.bin binary` em `.gitattributes`;
     - SHA-256 e ImageID conferidos antes de provar;
     - Groth16 pelo Docker local por digest, com `--pull=never
       --network=none`.
  4. **Escritas em devnet em lista fechada:**
     - SOL do deployer: 0,1 ao buyer e 0,02 ao executor;
     - Job P (PASS);
     - Job T (timeout, com 6021 antes do prazo);
     - negativos: journal de outro Job no P (6014) e a invariante 9 (6007 em
       A, 6008 em B).
- **Escolhas do agente, aprovadas no plano (Plan Mode):**
  - Ordem T antes de P, para a espera do prazo de T correr junto com a prova
    de P.
  - O negativo 6014 tem a forma da liquidação positiva, `deliver`+`release`
    numa transação, com o journal do Job A. Num P em `Funded`, um `release`
    sozinho daria 6025.
  - Artefato do P: `(21, 42)`. Prazos: T = slot + 1.560 (janela mínima + 60
    de margem); P = slot + 9.000.
  - CLI:
    - binário `vericode`, sem clap;
    - RPC por `reqwest` (blocking, rustls com webpki-roots), cadenciado e
      com backoff;
    - recusa clusters cujo genesis não seja o de devnet;
    - keypairs só por caminho, recusados dentro de work tree Git ou com
      permissão para grupo ou outros;
    - `--expect-error` para negativos, com simulação, `skipPreflight` e
      contas iguais antes e depois;
    - `check` (só leitura, com hash do ProgramData) como extra;
    - `--tamper-seal` para a demo, testado só offline.
  - Prover:
    - guest embutido com `include_bytes!`;
    - shim Docker versionado, que o prover põe no `PATH` do próprio processo;
    - `RISC0_WORK_DIR` padrão em `<dir>/groth16-work`;
    - subcomandos `check` e `verify` como extras.
  - `anchor/tests-local`:
    - fixtures `d4b/` com as receipts de devnet;
    - `tests/d4b_receipts.rs` (replay do D4b e invariante 9);
    - variantes do endereço do mint em `escrow.rs`;
    - verificador ausente em `settlement.rs`.
- **Resultado (evidência):**
  - nenhum crate novo: o lock da CLI e o do prover só renomeiam o pacote
    raiz das sementes;
  - prover 4/4; CLI 14/14, com as 6 instruções byte a byte iguais aos
    builders da suíte;
  - suíte 61/61 com o verificador do rebuild e com os bytes de devnet;
  - core 42/42 A/B;
  - devnet:
    - P `Released`, com o verificador invocado em
      [`4oWhwZfU…`](https://explorer.solana.com/tx/4oWhwZfUzZhVhrTydrhdBx1TJxWVH2mdWdKwiUhtHtMnhmeJg3MprEshvp592hYMgsaiwwhyvsDHek1egS9zKM1L?cluster=devnet);
    - T `RefundedOnTimeout`
      ([`3fiNWgTW…`](https://explorer.solana.com/tx/3fiNWgTWdscCB8kRTE4gxacQgNXCZ36NHNhazzgUtRBiF7ub7Zs3NDBBZ7qwEdXGEy2HGUsZyyZX9sv7gxFsVZ7?cluster=devnet));
    - 6021, 6014, 6007 e 6008 aterrissados com estado igual;
  - RD4A-07 (a), (e) e (f) fechados;
  - relatório: [`docs/d7-cli-results.md`](d7-cli-results.md).
- **Claim:** inalterado (CD7). "Verificada em devnet por CPI ao verificador
  Groth16 imutável de risc0-solana v3.0.0", com links. Agora também com a
  liquidação feita pela CLI do repositório.
- **Incidente registrado:** duas execuções da suíte foram interrompidas por
  pressão de memória do WSL. A sessão do editor caiu e levou o build (exit
  137). A correção foi rodar destacado (`setsid nohup`), com
  `CARGO_BUILD_JOBS=2` e `nice`. O código não falhou.
- **Risco aberto:**
  - sem e-stop;
  - rent preso (F-13);
  - mint authority = deployer: reproduzir as escritas exige receber Test
    USDC do projeto;
  - `job_id` público;
  - limite de taxa do RPC;
  - memória do WSL;
  - ImageID não recertificado; spec v1 trivial;
  - errata pendente em `docs/manifest-schema.md` ("a implantar em devnet"),
    arquivo que exige Plan Mode.
- **Próximo gate:** R-D7, revisão adversarial final do MVP, conforme
  `docs/handoffs/d7-to-r-d7.md`.

## 2026-10-05 — R-D7: revisão adversarial final do MVP (APROVADO COM RESSALVAS para o D9 e o D10–D12)

- **Data:** 2026-10-05
- **Revisor:** Claude Code (Opus 5.5, esforço max), sessão separada e somente
  leitura; HEAD revisado `c550a98` (sobre `0ec421b`, `deececa`, `a7c6e8a`).
- **Resultado:** **APROVADO COM RESSALVAS**; nenhum achado crítico ou alto.
  - RD7-01 (baixo): `--expect-error escrow:N` aceita falha do verificador com o
    mesmo código (6000–6003); só o rótulo erra, nada move.
  - RD7-02 (baixo): o shim do Docker não é allowlist de argv; o único chamador
    (`risc0-groth16 3.0.2`) tem argv fixo; o README promete demais.
  - RD7-03 (baixo): o prover usa `default_prover()`; `RISC0_PROVER`/`BONSAI_*`
    mudam o caminho de prova (solidez intacta).
  - RD7-04 (baixo): `--tamper-seal` e o casamento do `--expect-error` sem teste
    em `cli/`; "testado offline" sem respaldo.
  - RD7-05 (baixo): lacunas de reprodutibilidade no README e no roteiro, e
    erratas (`manifest-schema.md:70`, `escrow-program.md:137`).
  - RD7-06 a RD7-10 (info): hardlink e modo do `--log`; leituras sem
    `minContextSlot`; negativo que vira liquidação (`UNEXPECTED`); mensagens
    cosméticas; `--rpc-url`/proxy e `check` sem hash do verificador.
- **RD4A:** 01, 04, 05, 07 e 08 fechados; 02 e 03 mitigados; 06 aceito.
- **Condições:** CR1 a CR8 (`docs/r-d7-review-results.md`, seção 5).
- **Evidência:**
  - checagem do D7 reproduzida sem divergência a partir de um clone
    (`ad14a995…`), `--locked --offline`: core 42/42 ×2, CLI 14/14, suíte 61/61
    ×2, prover 4/4 + `check`/`verify`/`prove` novo;
  - 14 transações de devnet conferidas;
  - PoCs com RPC falso local, chaves, shim e prover;
  - varredura de segredos no histórico;
  - [`docs/r-d7-review-results.md`](r-d7-review-results.md).
- **Risco aberto:** sem e-stop; rent preso; mint authority = deployer; RPC
  público; memória do WSL; ImageID não recertificado; spec v1 trivial;
  RD7-01 a 10.
- **Registro:** feito por sessão com permissão de escrita a partir da resposta
  da revisão, que não podia editar arquivos.
- **Próximo gate:** D9, conforme `docs/handoffs/r-d7-to-d9.md`.

## 2026-10-05 — Decisões humanas para o D9

- **Data:** 2026-10-05
- **Origem:** as decisões pendentes do R-D7 foram propostas pelo agente da
  sessão de registro. O humano autorizou o registro ("autorizo") e as
  ratifica ao enviar o prompt `docs/handoffs/r-d7-to-d9.md`.
- **Decisões:**
  1. **Lista fechada de escritas em devnet (CR5)**, nesta ordem:
     - W1: Job T′, `job create --deadline-offset 1560`;
     - W2: logo depois, `job refund-timeout --expect-error escrow:6021` (≥ 60
       slots antes do prazo; CR2);
     - W3: Job P′, `job create --deadline-offset 9000`;
     - prova `(21,42)` de P′ (local, não é escrita);
     - W4: negativo 6014 em P′ (`settle --deliver --expect-error
       escrow:6014`) com a receipt do Job P do D7 (`d7/receipts/P`),
       declarada como tal (CR6);
     - W5: `settle --deliver --tamper-seal --expect-error verifier:6003` em
       P′;
     - W6: `settle --deliver` positivo de P′ → `Released`, com o verificador
       invocado;
     - W7: dupla liquidação em P′ (`--expect-error escrow:6007`);
     - W8: `job refund-timeout --wait` em T′ → `RefundedOnTimeout`.

     Nunca `escrow:6000` a `escrow:6003` (RD7-01). Sem airdrop nem
     transferência de SOL.
  2. **Ambiente limpo, opção (a) da CR4:** clone novo do HEAD, cópias novas
     das homes isoladas (offline) e targets novos, `--locked --offline`;
     nunca o perfil padrão. O relatório declara que "limpo" significa isso,
     não uma máquina nova.
  3. **Sem mudança de código no D9.** RD7-01, 02, 03, 04 e 07 vão para o
     D10a, com testes e revisão delta curta, antes do worker e das telas
     (CR8).
  4. **O vídeo de reserva é gravado pelo humano** depois do D9, seguindo a
     seção "Gravação" de `docs/demo-script.md`. O agente prepara os comandos
     e as falas, mas não grava.
- **Sequência depois do D9:** D10a (endurecimento da CLI e do prover) →
  revisão delta curta → D10 (worker) → D11–D12 (telas finas) → revisão curta
  da interface → vídeo definitivo e submissão até 11/10.
- **Motivo:**
  - um vídeo de reserva gravado antes de qualquer mudança de código garante
    uma entrega mesmo que as fases seguintes falhem;
  - os achados RD7-01/03/07 afetam exatamente o que o worker e as telas vão
    usar, por isso são corrigidos antes delas.
