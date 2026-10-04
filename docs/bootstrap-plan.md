# Plano de bootstrap controlado D1a.2 + D1b0.1

O D1b0.1 concluiu somente o ambiente WSL, os pacotes-base autorizados e o
clone canônico. As toolchains Rust/Anchor/Agave/RISC Zero executadas no D1a.3
permanecem em uma raiz de spike isolada e não constituem o bootstrap D1b.
Docker Engine foi posteriormente instalado de forma nativa no Ubuntu por ação
humana H3 e está registrado separadamente abaixo.

## Estado do gate

**D1b BLOQUEADO — SEM PERFIL PRONTO PARA INSTALAÇÃO.**

> Atualização 2026-10-04 (decisão D2a.2): a escolha do Perfil A foi delegada
> ao agente, que deve fundamentá-la em evidência executada e documentação
> oficial pinada no gate D2c. O bloqueio continua até essa escolha ser
> registrada em `docs/decisions.md`.

A documentação oficial do Anchor 0.31.x recomenda Agave `2.1.0`, enquanto o
workflow do tag `risc0-solana v3.0.0` instala Agave CLI `2.3.9` com Anchor CLI
`0.31.1`. O D1a.3 demonstrou que os gates host sem chave e os bytes ABI do
exemplo passam nas duas raias, e escolheu a raia `2.1.0` como Perfil A
candidato. Isso ainda não prova build SBF/CPI nem os testes negativos do
`JournalV1` VeriCode.

Não executar as antigas etapas D1b de Rust, RISC Zero, Agave, Anchor ou Node
enquanto este bloqueio estiver aberto. Docker já foi instalado exclusivamente
para H3/H4 e não autoriza as demais etapas.

O gate D1b0.1 não altera esse bloqueio: ele prepara apenas o sistema Linux e o clone.

A D1a.2 também não altera o bloqueio: ela definiu em
[`docs/d1a2-spike-plan.md`](d1a2-spike-plan.md) um protocolo auditável para
as raias Anchor/Agave e zkVM, então ainda sem executar instalações.

A D1a.3 concluiu H1–H4 em ambiente externo isolado. Sete checkouts ficaram
limpos e os locks foram preservados; Rust `1.81.0` falhou e o fallback
`1.85.0` passou na raia A; a raia B passou com `1.89.0`; Docker Engine/Noble
foi instalado e validado; dois builds efetivos repetiram ELF/ImageID; receipt
e vetores ABI passaram, incluindo rejeição de ImageID/journal divergentes.
Build SBF/CPI e testes negativos de `Job/mint/executor` não foram executados.
Portanto D1b segue bloqueado e o conjunto escolhido é candidato, não Perfil A
promovido.

## Pré-requisito separado: WSL2 + Ubuntu — concluído

Ambiente confirmado em 2026-09-28:

- distribuição `Ubuntu-24.04` em WSL versão 2;
- Ubuntu `24.04.5 LTS` (Noble), arquitetura `x86_64`;
- home Linux inicializada;
- clone canônico `~/src/vericode` em filesystem ext4, fora de `/mnt/c`;
- remote e branch iguais ao repositório de origem: `origin`, branch `main`, commit inicial `7aaf40b`;
- status do clone limpo antes das alterações documentais.

Pacotes-base efetivamente validados:

| Pacote | Versão instalada |
| --- | --- |
| `git` | `1:2.43.0-1ubuntu7.3` |
| `curl` | `8.5.0-2ubuntu10.15` |
| `ca-certificates` | `20260601~24.04.1` |
| `build-essential` | `12.10ubuntu1` |
| `pkg-config` | `1.8.1-2build1` |
| `libssl-dev` | `3.0.13-0ubuntu3.15` |

Foi executado `apt-get update`, seguido da instalação explícita somente dos seis pacotes acima com `--no-install-recommends`. O APT atualizou dependências necessárias, incluindo `libc` e bibliotecas do `curl`; não foi executado `apt upgrade`.

O aviso `unable to access .../.config/git/ignore: Permission denied` permanece não bloqueante. Não alterar configuração global do Git; o `.gitignore` do repositório é a proteção relevante.

## Spike obrigatório antes do bootstrap — progresso D1a.3

O spike usa ambiente isolado, preserva os lockfiles do tag e não cria código
de produto. Estado das etapas:

1. [x] Resolver e registrar o commit da tag
   `boundless-xyz/risc0-solana v3.0.0`.
2. [x] Inventariar e preservar os três lockfiles da referência e os dois
   locks RISC Zero auditados.
3. [x] Reproduzir os gates Cargo sem chave da raia do workflow: Anchor
   `0.31.1` + Agave `2.3.9`, Rust pinado `1.89.0`.
4. [x] Repetir os gates Cargo sem chave da raia recomendada pelo Anchor:
   Anchor `0.31.1` + Agave `2.1.0`; Rust `1.81.0` falhou e `1.85.0` passou.
5. [x] Comparar builds guest, ImageID, receipt, account metas e bytes de
   serialização permitidos por H4; build SBF/CPI e campos VeriCode permanecem
   gates separados.
6. [x] Registrar falhas como falhas, sem alterar código, manifests ou locks.

O exemplo oficial `counter` cria keypair, solicita airdrop e envia transações.
Esses passos não foram autorizados nem executados. O `hello-world` RISC Zero,
auditado como independente de Solana, foi usado para o build/receipt local.

O protocolo D1a.2 separa o gate sem chaves do gate com wallet/deploy:
`cargo metadata --locked`, `cargo tree --locked`, testes Rust sem validator e
comparações de bytes podem ser autorizados primeiro. `solana-keygen`,
`anchor test` com payer/validator/deploy, `anchor deploy`, airdrop e
transações exigem autorização adicional e não podem ser usados para completar
silenciosamente o primeiro gate.

## Ordem de instalação somente após desbloqueio

Quando o spike demonstrar um único Perfil A e a decisão for atualizada, a ordem proposta será:

1. WSL2 e Ubuntu 24.04 LTS, como pré-requisito separado — **concluído no D1b0.1**.
2. Dependências nativas mínimas do Ubuntu, com versões efetivas registradas — **concluído no D1b0.1**.
3. `rustup` e Rust host exato exigido pelo perfil.
4. `rzup`, `cargo-risczero` e toolchain guest exatos do mesmo conjunto RISC Zero.
5. Docker em versão exata — **concluído em H3/H4**, antes do primeiro
   `cargo risczero build` e do gate de receipt.
6. Agave CLI exato demonstrado pelo spike.
7. AVM/Anchor CLI exatos e crates correspondentes.
8. Node e gerenciador somente quando existir front-end, cliente TS ou teste oficial que realmente os exija.

Cada etapa exigirá confirmação humana imediata. Os comandos de instalação das toolchains serão redigidos somente depois que o Perfil A estiver aprovado; publicar comandos agora poderia sugerir uma combinação ainda não demonstrada.

## Onde a instalação deverá permanecer

- Toolchains e caches devem ficar nos diretórios do usuário Linux (`~/.rustup`, `~/.cargo`, `~/.risc0` e equivalentes documentados pela ferramenta), nunca versionados.
- Dependências de sistema ficam na distribuição WSL, registradas com versão e origem.
- Docker Engine já altera o host WSL e armazena imagem/cache fora do
  repositório; não é dependência vendorizada. O grupo `docker` é privilegiado.
- Nunca copiar `.env`, wallet, keypair ou credencial entre Windows e WSL.

## Arquivos de versão futuros

Nenhum destes arquivos deve ser criado antes do Perfil A comprovado:

- `rust-toolchain.toml` para o Rust host escolhido; a toolchain guest continuará registrada separadamente.
- `.anchorversion` para a versão do Anchor CLI.
- `Anchor.toml` somente com o workspace Anchor autorizado e versões demonstradas.
- `Cargo.toml` e `Cargo.lock` apenas quando as crates forem autorizadas; lockfiles devem ser versionados.
- arquivo de versão Node e `package.json` somente quando Node entrar no escopo real.
- registro explícito da versão Docker no documento de ambiente/gate determinístico.

## Rollback futuro

- Usar desinstaladores oficiais e o inventário produzido na instalação; não remover diretórios de forma ampla.
- Antes de remover WSL, Docker, Rust, RISC Zero, Agave ou Anchor, confirmar que nenhum outro projeto depende deles.
- A remoção de distribuição WSL é destrutiva e exige confirmação específica.
- Reversão de PATH/perfil deve remover apenas as linhas registradas durante o bootstrap.

## Checklist D1b0.1

- [x] WSL2 e Ubuntu 24.04 confirmados.
- [x] Arquitetura `x86_64` e filesystem ext4 confirmados.
- [x] Seis pacotes-base instalados e versionados.
- [x] Clone canônico criado em `~/src/vericode` na branch `main`.
- [x] AGENTS, CLAUDE e documentos D1 presentes no clone.
- [x] Nenhum Git global alterado; nenhum commit ou push realizado.
- [x] Nenhuma toolchain de produto, wallet, keypair, `.env`, Program ID ou scaffold criado.

## Checklist para liberar D1b

- [x] O tag leve `risc0-solana v3.0.0` e o commit
  `ee415935d04a948f27a346b563391900bdad6486` foram registrados.
- [x] Os gates Rust sem chave do `counter` passaram na raia do workflow,
  usando Rust pinado e locks preservados; build SBF e CPI seguem pendentes.
- [x] Os gates Rust sem chave passaram com Anchor `0.31.1` + Agave `2.1.0`
  e Rust host `1.85.0`; build SBF e CPI seguem pendentes.
- [x] A divergência `2.1.0` versus `2.3.9` tem decisão documentada: `2.1.0`
  no Perfil A candidato, `2.3.9` preservado como referência da CI.
- [x] Discriminadores, instruction data, account metas, ownership,
  seal/journal e serialização do exemplo têm evidência executável nas duas
  raias.
- [ ] CPI runtime e os testes negativos de `Job/mint/executor` do VeriCode têm
  evidência real.
- [x] Docker foi escolhido, instalado e validado; dois builds efetivos e o
  receipt do ELF determinístico passaram.
- [ ] Um único Perfil A substituiu o bloqueio na matriz.
- [x] Nenhuma wallet, keypair, `.env`, Program ID novo ou scaffold foi criado.

Enquanto qualquer item acima permanecer aberto, D1b não pode começar.
