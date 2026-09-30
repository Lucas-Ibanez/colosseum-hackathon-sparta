# Matriz de compatibilidade D1a.2 / execução D1a.3

Data da revisão: 2026-09-29.

Esta matriz separa o que as fontes oficiais demonstram do que a execução
isolada D1a.3 observou. Nenhum resultado abaixo equivale a execução do
VeriCode ou validação de deployment/CPI.

## Significado dos status

- **verificado por fonte:** uma documentação ou release oficial declara a relação registrada.
- **verificado por manifest:** um manifesto, lockfile ou workflow do tag exato registra a versão; isso não equivale a teste local nem a compatibilidade entre raias.
- **verificado por execução:** o comando pinado foi executado no ambiente
  isolado D1a.3, com locks preservados; o escopo exato da execução continua
  indicado na linha.
- **a confirmar por spike:** falta evidência oficial ou execução reproduzível para a combinação necessária ao VeriCode.
- **incompatível:** uma fonte demonstra incompatibilidade. Nenhuma das interseções abaixo recebeu este status: o problema atual é ausência de comprovação, não incompatibilidade demonstrada.

## Resultado

**SEM PERFIL PRONTO PARA INSTALAÇÃO.**

A D1a.2 definiu o protocolo reproduzível em
[`docs/d1a2-spike-plan.md`](d1a2-spike-plan.md), e a execução H1–H4 do D1a.3
foi concluída. Rust `1.81.0` falhou na compilação do lock do verificador;
`1.85.0` passou como fallback da raia A. As duas raias, as toolchains RISC
Zero, Docker pinado, dois builds efetivos, receipt local e vetores ABI foram
executados com locks preservados. A decisão permanece **PENDENTE** somente
porque os testes negativos de `Job/mint/executor` dependem de código VeriCode
fora do escopo e Router/CPI/deployment continuam sem H5.

Revisões auditadas: `risc0/risc0 v3.0.3` no commit
`14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`;
`boundless-xyz/risc0-solana v3.0.0` no commit
`ee415935d04a948f27a346b563391900bdad6486`; Anchor `v0.31.1` no
commit `47284f8f0b9844c6b83234aa90f556bad00e12ed`; Agave `v2.1.0`
no commit `c1080de464cfb578c301e975f498964b5d5313db`; e Agave
`v2.3.9` como tag anotado
`f20bef1cce6abb06512b76f2baddc99111980740` apontando para
`47647df756f5dd0b3c739cecaa71bcf754af6be8`.

O perfil Anchor demonstrado é Anchor/AVM/crates `0.31.1` com Agave CLI `2.1.0`. A referência `risc0-solana v3.0.0`, por outro lado, configura em seu workflow Anchor CLI `0.31.1` com Agave CLI `2.3.9`; seus lockfiles resolvem `solana-program 2.3.0` e outras crates Solana em patches distintos. Não há, nas fontes permitidas consultadas, prova de que o tag `v3.0.0` compile e execute com Agave CLI `2.1.0`, nem de que o perfil recomendado pelo Anchor possa ser trocado por `2.3.9` sem um spike.

Portanto, as raias abaixo sustentam um Perfil A candidato, mas ainda não um
conjunto aprovado para D1b:

1. **Raia Anchor:** Anchor CLI/AVM/crates `0.31.1` + Agave CLI `2.1.0`.
2. **Raia zkVM:** tag `risc0/risc0 v3.0.3`, Rust host `1.89.0`, Rust guest `1.88.0`, `rzup 0.5.1`, `cargo-risczero 3.0.3`, `risc0-zkvm 3.0.3` e `risc0-build 3.0.3`.
3. **Raia de referência Solana:** tag `boundless-xyz/risc0-solana v3.0.0`, cujo workflow usa Anchor `0.31.1` e Agave CLI `2.3.9`, e cujos manifests/locks registram as dependências detalhadas abaixo.

## Matriz revisada

| Componente | Versão exata proposta | Status | Motivo da escolha | Dependências de compatibilidade | Evidência exata | Comando de verificação após eventual instalação | Risco residual |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Windows host | Windows `10.0.26200`, x64, já observado | verificado por fonte | É o host real registrado no D0; não é a plataforma canônica das toolchains Linux. | WSL2 e virtualização | Estado local D0; [Microsoft WSL](https://learn.microsoft.com/windows/wsl/install) | `Get-ComputerInfo \| Select-Object WindowsProductName,WindowsVersion,OsBuildNumber,OsArchitecture` | Política/preview do Windows pode afetar WSL. |
| WSL | WSL2 no clone canônico | verificado por execução | O kernel `microsoft-standard-WSL2` foi observado no D1b0.1. | Windows compatível | Saída D1b0.1; `docs/evidence.md` | `uname -r`; no host: `wsl --status` e `wsl --list --verbose` | É pré-requisito de ambiente, não evidência de compatibilidade das toolchains. |
| Linux no WSL | Ubuntu `24.04.5 LTS`, x86_64, ext4 | verificado por execução | É o ambiente canônico observado em `~/src/vericode`, fora de `/mnt/c`. | WSL2 | Saída D1b0.1; `docs/evidence.md` | `cat /etc/os-release`; `uname -m`; `findmnt -T . -n -o FSTYPE,TARGET` | Não demonstra compatibilidade das toolchains de produto. |
| Git | Ubuntu package `1:2.43.0-1ubuntu7.3` | verificado por execução | O clone canônico e o Git Linux foram validados no D1b0.1. | Ubuntu 24.04.5 | Saída D1b0.1; `docs/evidence.md` | `git --version`; `git status --short` | O aviso de permissão do exclude global permanece não bloqueante; não alterar configuração global. |
| Rust host da raia zkVM | `1.89.0` | verificado por execução | É o canal do `rust-toolchain.toml` do tag RISC Zero escolhido e foi observado no home isolado. | `rustup`; Linux x86_64 | `risc0/risc0`, tag `v3.0.3`, [`rust-toolchain.toml`](https://github.com/risc0/risc0/blob/v3.0.3/rust-toolchain.toml); [`docs/d1a3-spike-results.md`](d1a3-spike-results.md) | `rustc +1.89.0 --version`; `rustup show` | O checkout também instalou o alias `1.89`; o default isolado permaneceu `1.89.0`. |
| Cargo da raia zkVM | `1.89.0`, fornecido pela toolchain Rust `1.89.0` | verificado por execução | Mantém Cargo associado à release Rust, sem pin independente inventado. | Rust `1.89.0` | [release Rust 1.89.0](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/); execução D1a.3 | `cargo +1.89.0 --version --verbose` | Build guest e receipt passaram; o `counter` Solana continua fora desse gate. |
| Rust guest RISC-V | `1.88.0` | verificado por execução | A CI do tag fixa `RISC0_RUST_TOOLCHAIN_VERSION=1.88.0`; `rzup show`, o binário guest e os builds Docker confirmaram a instalação/linha escolhida. | `rzup` e SDK RISC Zero do mesmo tag | `risc0/risc0`, tag `v3.0.3`, [`.github/workflows/main.yml`](https://github.com/risc0/risc0/blob/v3.0.3/.github/workflows/main.yml); execução D1a.3 | `rzup show --verbose`; guest `rustc --version` | O exemplo `counter` upstream continua declarando `stable`; o spike substituiu isso por pins somente na sessão. |
| `rzup` | `0.5.1` | verificado por execução | Compilado do checkout exato `v3.0.3` com `--locked`, sem script `latest`. | Rust host | `risc0/risc0`, tag `v3.0.3`, [`rzup/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/rzup/Cargo.toml); execução D1a.3 | `rzup --version`; `rzup show` | O lock preservado inclui crates yanked; não foram atualizadas. |
| `cargo-risczero` | `3.0.3` | verificado por execução | Instalado por `rzup 0.5.1` com versão explícita; dois builds efetivos usaram o digest Docker pinado. | `rzup`; Rust guest `1.88.0`; Docker Engine e builder exatos | `risc0/risc0`, tag `v3.0.3`, [`risc0/cargo-risczero/Cargo.toml`](https://github.com/risc0/risc0/blob/v3.0.3/risc0/cargo-risczero/Cargo.toml), [`README.md`](https://github.com/risc0/risc0/blob/v3.0.3/risc0/cargo-risczero/README.md); execução D1a.3 | `cargo risczero --version`; dois `cargo risczero build` | O ImageID/ELF repetiu; isso não valida guest/JournalV1 do VeriCode. |
| crates RISC Zero | `risc0-zkvm =3.0.3`; `risc0-build =3.0.3` | verificado por manifest e execução | Os manifests/locks usam esse conjunto; o `hello-world` foi compilado, provado e verificado localmente. | Guest/tooling da mesma linha `3.0.3` | fontes do tag e [`docs/d1a3-spike-results.md`](d1a3-spike-results.md) | `cargo tree --locked`; build e receipt registrados | O lock também resolve `risc0-groth16 3.0.2`; números de pacote não devem ser forçados a coincidir. |
| Exemplo RISC Zero da release | `risc0/risc0 v3.0.3`, `examples/hello-world` | verificado por execução | Dois builds Docker efetivos produziram ELF/ImageID idênticos; `r0vm` gerou receipt do ELF Docker e a verificação local passou. | Toolchains host/guest do tag; builder `sha256:3e12f71b…943eb3` | fontes do tag e [`docs/d1a3-spike-results.md`](d1a3-spike-results.md) | `r0vm --id`; `cargo risczero verify` | Não é receipt VeriCode nem teste de Solana; o build nativo gerou ELF/ImageID diferentes e foi tratado apenas como controle. |
| `risc0-solana` | tag `v3.0.0` | verificado por fonte e execução | A release declara suporte ao zkVM 3.0 e primeira versão totalmente auditada; dois checkouts resolveram o commit exato e permaneceram limpos. | Conjuntos exatos dos manifests/locks do tag | [`release v3.0.0`](https://github.com/boundless-xyz/risc0-solana/releases/tag/v3.0.0); [`tree v3.0.0`](https://github.com/boundless-xyz/risc0-solana/tree/v3.0.0); execução D1a.3 | `git rev-parse HEAD`; `git status --short` | Release e checkout não provam deployment, CPI VeriCode ou compatibilidade SBF com Agave CLI 2.1.0. |
| Exemplo Solana de referência | `risc0-solana v3.0.0`, `examples/counter` | verificado por execução parcial | `metadata/tree/check/test --locked` passaram nos workspaces sem build guest, em A e B. | Anchor, Router, Borsh e RISC Zero | fontes do tag e execução D1a.3 | ver resultados em [`docs/d1a3-spike-results.md`](d1a3-spike-results.md) | O `stable` foi substituído por pin via `+toolchain`; Cargo confirmou que o patch Git no manifesto `host` é ignorado por não estar na raiz do workspace. |
| Rust host da raia A | `1.85.0` | verificado por execução | `1.81.0`, pin do Agave `v2.1.0`, falhou ao compilar `risc0-zkvm-platform 2.2.1`; o fallback oficial previsto `1.85.0` passou. | Anchor/AVM `0.31.1`; locks do tag | Agave `v2.1.0` [`rust-toolchain.toml`](https://github.com/anza-xyz/agave/blob/v2.1.0/rust-toolchain.toml); instalação Anchor no commit exato; execução D1a.3 | `rustc +1.85.0 --version`; `cargo +1.85.0 test --locked` | Pin experimental do spike, não recomendação/MSRV oficial do Anchor. |
| Anchor CLI e AVM | `0.31.1` | verificado por execução | AVM e CLI compilados do commit exato em A e B, com locks preservados. | Agave recomendado `2.1.0` | `coral-xyz/anchor`, tag `v0.31.1`: [`avm/Cargo.toml`](https://github.com/coral-xyz/anchor/blob/47284f8f0b9844c6b83234aa90f556bad00e12ed/avm/Cargo.toml), [`lang/Cargo.toml`](https://github.com/coral-xyz/anchor/blob/47284f8f0b9844c6b83234aa90f556bad00e12ed/lang/Cargo.toml), [`spl/Cargo.toml`](https://github.com/coral-xyz/anchor/blob/47284f8f0b9844c6b83234aa90f556bad00e12ed/spl/Cargo.toml); execução D1a.3 | `avm --version`; `anchor --version` | Instalação passou; isso não valida `anchor test`, CPI ou deploy. |
| Agave CLI da raia Anchor | `2.1.0` | verificado por execução da versão e gates Rust | É a versão oficialmente recomendada para Anchor 0.31.x e foi extraída do release oficial no prefixo A. | Anchor CLI/crates `0.31.1` | Anchor `v0.31.1`; [Agave `v2.1.0`](https://github.com/anza-xyz/agave/releases/tag/v2.1.0); execução D1a.3 | `solana --version`; `cargo-build-sbf --version` | O tarball não tem digest oficial publicado; nenhum build SBF/Anchor test foi executado. |
| Agave CLI no workflow `risc0-solana` | `2.3.9` | verificado por execução da versão e gates Rust | O workflow instala essa versão; o tarball conferiu com o digest oficial e foi extraído no prefixo B. | Anchor CLI `0.31.1`; Rust experimental `1.89.0` | `risc0-solana v3.0.0` workflow; [Agave `v2.3.9`](https://github.com/anza-xyz/agave/releases/tag/v2.3.9); execução D1a.3 | `solana --version`; `cargo-build-sbf --version` | Não transforma 2.3.9 em recomendação do Anchor; `anchor test` continua proibido por H5. |
| Crates Anchor/Solana do verificador | `anchor-lang 0.31.1`; `risc0-zkvm 3.0.3`; `solana-bn254 3.0.0`; `solana-program 2.3.0` resolvido | verificado por manifest e execução | São as declarações e resoluções reais do verificador do tag; `metadata/tree/check/test --locked` passaram em A e B. | Lockfile do mesmo diretório | `risc0-solana`, tag `v3.0.0`: [`solana-verifier/programs/groth_16_verifier/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/programs/groth_16_verifier/Cargo.toml), [`solana-verifier/Cargo.lock`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/Cargo.lock); execução D1a.3 | `cargo tree --locked --all-targets`; `cargo test --locked` | O lock contém crates Solana separadas em múltiplas linhas, inclusive interfaces 1.x, crates 2.2–2.4, `solana-bn254`/`solana-define-syscall 3.0.0` e `solana-loader-v3-interface 5.0.0`; isso não equivale à versão da CLI. |
| Crates do `counter` on-chain | `anchor-lang 0.31.1`; `solana-program 2.3.0` resolvido; Borsh `0.10.4` resolvido | verificado por manifest e execução | Registra a resolução efetiva do workspace on-chain; `metadata/tree/check/test --locked` e o harness de bytes passaram em A e B. | `Cargo.toml` e lockfile do mesmo tag | `risc0-solana`, tag `v3.0.0`: [`programs/solana-counter/Cargo.toml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/programs/solana-counter/Cargo.toml), [`examples/counter/Cargo.lock`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/examples/counter/Cargo.lock); execução D1a.3 | `cargo tree --locked --all-targets`; `cargo test --locked`; harness ABI registrado | O manifesto shared declara `borsh = "0.10.3"`, intervalo que o lock resolve em `0.10.4`; o ABI do VeriCode ainda não existe. |
| Coexistência Anchor + `risc0-solana` | Perfil A candidato: Anchor `0.31.1` + Agave `2.1.0` + Rust A `1.85.0`, separado da raia zkVM pinada | a confirmar por código/integração | As duas raias passaram os gates host e produziram bytes ABI idênticos; a raia A é preferida por seguir a recomendação Anchor. | CPI runtime, `JournalV1`, Job/mint/executor, Program ID e cluster ainda faltam | locks preservados e [`docs/d1a3-spike-results.md`](d1a3-spike-results.md) | futuro harness VeriCode e, sob H5, CPI real | IDLs versionados divergem do source; patch host é ignorado; nenhuma integração VeriCode foi executada. |
| Docker | Engine no WSL; `docker-ce 5:29.8.1-1~ubuntu.24.04~noble`; Engine `29.8.1` | verificado por execução | Instalação humana pelo repositório oficial Noble; daemon ativo/habilitado e usado nos builds. | `containerd.io 2.3.6-1~ubuntu.24.04~noble`; Buildx `0.37.1`; Compose `5.5.1`; builder RISC Zero por digest | README do tag; [Docker Ubuntu](https://docs.docker.com/engine/install/ubuntu/); [`docs/d1a3-spike-results.md`](d1a3-spike-results.md) | `docker version`; dois builds efetivos com ImageID/ELF idênticos | Grupo `docker` equivale a privilégio root; rollback não executado; imagem/cache ocupam espaço fora do repositório. |
| Node | Host `24.15.0`; fora do perfil crítico | a confirmar por spike | Não há front-end nem cliente TS no MVP atual. A CI do tag usa apenas major 22, sem demonstrar Node 24. | Somente futuro front-end/cliente TS | `risc0-solana`, tag `v3.0.0`, [`.github/workflows/tests.yml`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/.github/workflows/tests.yml) | `node --version` quando houver gate Node | Node 24 não é declarado compatível nem incompatível; não fazer downgrade/upgrade agora. |
| Gerenciador Node | Nenhum selecionado para o VeriCode | a confirmar por spike | O produto não tem pacote Node; Yarn `1.22.22` existe apenas como referência do verificador oficial. | Futuro pacote Node | `risc0-solana`, tag `v3.0.0`, [`solana-verifier/package.json`](https://github.com/boundless-xyz/risc0-solana/blob/v3.0.0/solana-verifier/package.json) | futuro: comando do gerenciador escolhido | Não criar `package.json` ou lockfile nesta fase. |
| Navegador e wallet | Necessidade futura; sem produto/versão | a confirmar por spike | Só entram com cliente/front-end. Nenhuma carteira deve ser criada no bootstrap. | UX, cluster e assinatura ainda não definidos | [Solana clients](https://solana.com/docs/clients) | futuro, sem expor segredo | Seleção e suporte a devnet permanecem fora do gate. |

## Interseção entre as raias

O `counter` demonstra uma estrutura com dois workspaces Cargo, mas isso é apenas isolamento de compilação:

- o workspace Anchor contém o programa e a crate `shared`;
- o workspace zkVM contém host e methods/guest;
- a CPI cruza a fronteira por instruções, account metas, discriminadores, Program IDs e bytes serializados, não por “compatibilidade automática” entre workspaces;
- os dois lados precisam concordar sobre seal, journal, ImageID e Borsh; o exemplo resolve Borsh `0.10.4` para o código compartilhado, enquanto o lock zkVM também contém Borsh `1.5.7` para outras dependências;
- o D1a.3 provou os bytes upstream da interface nas duas raias com locks
  preservados; isso ainda não prova a CPI runtime nem a interface futura do
  VeriCode. Separar diretórios não resolve divergência de CLI/SBF nem valida
  deployment.

Não há fonte que demonstre incompatibilidade definitiva entre Agave `2.1.0` e
os crates resolvidos pelo Router. Os gates host do spike passaram, mas build
SBF/CPI não foi autorizado; o status correto da integração continua **a
confirmar por código/integração**.

## Gates antes de D1b

1. [x] Reproduzir resolução/check/test sem chave do `counter` do tag
   `v3.0.0` em ambiente isolado, com commits, locks e versões registrados.
2. [x] Testar os gates Rust da raia Anchor `0.31.1` + Agave `2.1.0` sem
   alterar lockfiles; Rust `1.81.0` falhou e `1.85.0` passou.
3. [x] Comparar com a raia do workflow `0.31.1` + Agave `2.3.9`, sem
   promovê-la a recomendação oficial do Anchor.
4. [x] Fixar por linha de comando as toolchains Rust da reprodução, sem criar
   arquivo de produto e sem usar action `@main`.
5. [x] Provar discriminadores, payloads, seal, journal, ordem/flags/owners dos
   account metas do exemplo nas duas raias; permanecem pendentes o
   `JournalV1`/testes negativos VeriCode e a CPI runtime.
6. [x] Selecionar, instalar e validar Docker exato; repetir ELF/ImageID e
   verificar receipt local do artefato determinístico.

Até esses gates fecharem, não criar `rust-toolchain.toml`, `.anchorversion`, `Anchor.toml`, `Cargo.toml`, lockfiles ou scaffold do VeriCode.

## Protocolo D1a.2

As raias do spike são:

1. raia A: Anchor/AVM/crates `0.31.1` + Agave CLI `2.1.0`; Rust `1.81.0`
   rejeitado e fallback Rust `1.85.0` aprovado nos gates host;
2. raia B: Anchor CLI `0.31.1` + Agave CLI `2.3.9`, com Rust host
   `1.89.0` aprovado nos gates host como pin experimental do spike;
3. raia zkVM: RISC Zero `v3.0.3`, Rust host `1.89.0`, guest
   `1.88.0`, `rzup 0.5.1` e componentes `3.0.3`; Docker Engine/Noble e
   imagem builder exatos, com builds e receipt verificados.

Comandos, hashes dos locks, limites sem chave, critérios de sucesso/falha,
rollback e confirmações humanas estão em
[`docs/d1a2-spike-plan.md`](d1a2-spike-plan.md). O que foi efetivamente
executado, incluindo falhas, está separado em
[`docs/d1a3-spike-results.md`](d1a3-spike-results.md); comandos ainda
pendentes permanecem explicitamente `NÃO EXECUTADOS`.
