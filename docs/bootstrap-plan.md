# Plano de bootstrap controlado D1a.1

Nenhum comando de instalação deste documento foi executado.

## Estado do gate

**D1b BLOQUEADO — SEM PERFIL PRONTO PARA INSTALAÇÃO.**

A documentação oficial do Anchor 0.31.x recomenda Agave `2.1.0`, enquanto o workflow do tag `risc0-solana v3.0.0` instala Agave CLI `2.3.9` com Anchor CLI `0.31.1`. Os manifests e lockfiles desse tag registram o conjunto de crates usado, mas não demonstram que a referência funcione com CLI `2.1.0`. A divergência precisa ser resolvida por spike antes de instalar toolchains para o VeriCode.

Não executar as antigas etapas D1b de Rust, RISC Zero, Agave, Anchor, Node ou Docker enquanto este bloqueio estiver aberto.

## Pré-requisito separado: WSL2 + Ubuntu

O ambiente recomendado continua sendo WSL2 com Ubuntu 24.04 LTS x86_64 e checkout canônico no filesystem Linux, por exemplo `~/src/vericode`. Isso é preparação de plataforma; não valida nem autoriza a instalação das toolchains.

Diagnóstico read-only proposto:

```powershell
wsl --version
wsl --status
wsl --list --online
wsl --list --verbose
```

Se `Ubuntu-24.04` estiver disponível, a instalação abaixo só pode ocorrer em tarefa separada e após confirmação humana imediata:

```powershell
wsl --install -d Ubuntu-24.04
```

- **Origem:** [Microsoft WSL](https://learn.microsoft.com/windows/wsl/install).
- **Altera:** recursos do Windows, uma distribuição Linux e armazenamento local; pode exigir reinicialização.
- **Privilégio:** administrador pode ser necessário; a criação do usuário Linux é interativa.
- **Verificação:** `wsl --version`, `wsl --status`, `wsl --list --verbose`, `cat /etc/os-release` e `uname -m`.
- **Rollback:** `wsl --unregister Ubuntu-24.04` destrói os dados da distribuição; só pode ser executado após inventário, backup e confirmação explícita.

O aviso `unable to access .../.config/git/ignore: Permission denied` permanece não bloqueante. Não alterar configuração global do Git; o `.gitignore` do repositório é a proteção relevante.

## Spike obrigatório antes do bootstrap

O próximo gate deve usar um ambiente descartável, preservar os lockfiles do tag e não criar código de produto:

1. Resolver e registrar o commit da tag `boundless-xyz/risc0-solana v3.0.0`.
2. Inventariar as versões efetivas dos dois lockfiles do `counter` e do lockfile de `solana-verifier`.
3. Reproduzir primeiro a combinação declarada no workflow do tag: Anchor `0.31.1` + Agave CLI `2.3.9`. A action Rust usada pelo workflow aponta para `risc0/risc0@main`; o spike deve substituir essa referência flutuante por uma versão explicitamente registrada.
4. Repetir com o perfil oficialmente recomendado pelo Anchor: Anchor `0.31.1` + Agave CLI `2.1.0`.
5. Comparar build, testes, CPI, account metas, discriminadores e bytes de serialização sem atualizar dependências silenciosamente.
6. Registrar falhas como falhas. Não alterar código, manifestos ou locks para forçar sucesso sem uma decisão separada.

O exemplo oficial cria keypair e faz deploy local em seu workflow. Esses passos não estão autorizados nesta tarefa. O desenho do spike deverá isolar ou pedir autorização específica para qualquer ação futura que gere chaves.

## Ordem de instalação somente após desbloqueio

Quando o spike demonstrar um único Perfil A e a decisão for atualizada, a ordem proposta será:

1. WSL2 e Ubuntu 24.04 LTS, como pré-requisito separado.
2. Dependências nativas mínimas do Ubuntu, com versões efetivas registradas.
3. `rustup` e Rust host exato exigido pelo perfil.
4. `rzup`, `cargo-risczero` e toolchain guest exatos do mesmo conjunto RISC Zero.
5. Docker em versão exata **antes** do primeiro `cargo risczero build` e do gate de receipt.
6. Agave CLI exato demonstrado pelo spike.
7. AVM/Anchor CLI exatos e crates correspondentes.
8. Node e gerenciador somente quando existir front-end, cliente TS ou teste oficial que realmente os exija.

Cada etapa exigirá confirmação humana imediata. Os comandos de instalação das toolchains serão redigidos somente depois que o Perfil A estiver aprovado; publicar comandos agora poderia sugerir uma combinação ainda não demonstrada.

## Onde a instalação deverá permanecer

- Toolchains e caches devem ficar nos diretórios do usuário Linux (`~/.rustup`, `~/.cargo`, `~/.risc0` e equivalentes documentados pela ferramenta), nunca versionados.
- Dependências de sistema ficam na distribuição WSL, registradas com versão e origem.
- Docker altera o host/integração WSL e armazenamento de imagens; não é dependência vendorizada do repositório.
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

## Checklist para liberar D1b

- [ ] O tag e o commit exatos do `risc0-solana v3.0.0` foram registrados.
- [ ] O exemplo `counter` foi testado com a raia do workflow, usando Rust pinado e locks preservados.
- [ ] O exemplo foi testado com Anchor `0.31.1` + Agave `2.1.0`.
- [ ] Há decisão documentada sobre a divergência `2.1.0` versus `2.3.9`.
- [ ] CPI, discriminadores, account metas, seal/journal e serialização entre workspaces têm evidência real.
- [ ] A versão Docker foi escolhida para o primeiro `cargo risczero build`.
- [ ] Um único Perfil A substituiu o bloqueio na matriz.
- [ ] Nenhuma wallet, keypair, `.env`, Program ID ou scaffold foi criado.

Enquanto qualquer item acima permanecer aberto, D1b não pode começar.
