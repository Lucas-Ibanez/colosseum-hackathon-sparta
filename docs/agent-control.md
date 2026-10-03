# Controle autônomo D1c2b

## Objetivo atual

Concluir exclusivamente o marco D1c2b do VeriCode com evidência real:
validar o fechamento offline do lock guest; produzir vendor temporário
auditável; obter dois builds independentes e determinísticos do guest;
comparar ELF byte a byte, tamanho, SHA-256 e ImageID; executar receipts
VeriCode locais reais para PASS e FAIL e os testes negativos; registrar
evidências; realizar auditoria final independente; parar antes de Solana,
Anchor, wallet, validator, Router/CPI, rede blockchain, deploy, front-end ou
push.

## Baseline

- Raiz: `/home/lucas/src/vericode`.
- Branch: `main`.
- HEAD de baseline: `0d55e44a69ed13325d8a33deea00840bde6540db`.
- Filesystem Linux observado: `ext2/ext3` pelo `statfs` do WSL.
- Árvore inicial: limpa; `git diff --check` com exit `0`.
- `/home/lucas/.rustup`: ausente.
- Lock host/methods: SHA-256
  `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`.
- Lock guest reconciliado: SHA-256
  `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`.

## Gate atual

`D1c2b.3g.1` — validar e auditar a correção mínima que torna
`vericode-core` compatível com o guest `no_std`, sem alterar schema, wire,
hashing, locks ou código em `zkvm/`; somente depois retomar os dois builds.

## Estado

`GO`

## Ações autorizadas

- Leitura e auditoria do clone, locks, caches e toolchains isoladas.
- Criar documentação e evidência estritamente pertencentes ao gate atual.
- O uso adicional de `lane-a/cargo` e `lane-b/cargo` foi autorizado e
  consumido somente para fechar o archive `risc0-groth16-3.0.2.crate`;
  nenhuma cópia adicional está autorizada por inferência.
- Usar somente como cache destino
  `/home/lucas/.local/share/vericode-spikes/d1c2b/cargo`.
- Copiar somente archives `.crate` exigidos pelo lock guest atual, ausentes
  na cache destino e cujo SHA-256 coincida exatamente com o checksum do lock.
- Executar Cargo/Rust/RISC Zero somente com `CARGO_HOME`, `RUSTUP_HOME`,
  `RISC0_HOME`, `PATH`, `RUSTUP_AUTO_UPDATE=0` e `CARGO_NET_OFFLINE=true`
  explicitamente declarados conforme o protocolo D1c2b.
- Criar vendor e targets somente em diretórios temporários auditáveis.
- Fazer a correção mínima `no_std + alloc` no core puro necessária para o
  guest real, preservando versões, locks, serialização, hashing e semântica.
- Criar commit local após validação integral e auditoria somente leitura de
  cada gate.

## Ações proibidas

- Acesso à rede sem nova autorização humana.
- Instalar, atualizar ou remover ferramentas; Docker pull; imagem não fixada.
- Alterar ou regenerar locks fora de gate explícito.
- Copiar configurações Cargo, índices, diretórios Git, binários, credenciais,
  tokens, diretórios completos ou qualquer arquivo sem checksum correspondente.
- Criar wallet, seed, keypair, chave privada, `.env` ou Program ID.
- Solana, Anchor, validator, airdrop, transação, deploy, Router/CPI, rede
  blockchain, front-end ou push.
- Alterar schema, Journal, escrow, política econômica ou claims públicos.
- Mock, dev mode, stub ou fallback apresentado como sucesso real.
- Operação destrutiva, rollback, stash, rebase, amend ou reescrita de histórico.
- Duas escritas simultâneas no mesmo clone.

## Evidências exigidas

- Inventário completo do fechamento do lock e lista exata dos archives
  inicialmente ausentes.
- Para cada cópia: origem, destino, versão e SHA-256 esperado/observado.
- `cargo metadata --locked --offline`, `cargo tree --locked --offline` e
  árvores inversas das crates reconciliadas com exit real.
- Vendor temporário com inventário de paths, conteúdo, checksums e ausência
  de segredos/configuração indevida.
- Dois builds independentes efetivos; ELF comparado por `cmp`, tamanho,
  SHA-256 e ImageID calculado/emitido por mecanismos independentes aplicáveis.
- Receipts locais reais, sem dev mode, para PASS e FAIL, com verificação do
  ImageID e journal esperados.
- Testes negativos exigidos pelo host, preservando erro operacional distinto
  de `Verdict::Fail`.
- Diff integral, comandos, saídas reais, riscos, `git diff --check`, busca de
  segredos/artefatos e auditoria independente somente leitura por gate.
- Router/CPI/devnet mantidos como `STATUS: NÃO VALIDADO`.

## Riscos abertos

- A correção `no_std` compilou um ELF guest real em probe isolado, mas ainda
  depende de dois builds oficiais a partir do commit auditado.
- O build requer um vendor da união dos locks host e guest: o vendor apenas
  guest não contém as dependências do build script host. A união auditada
  contém 461 pacotes registry.
- O lock host/methods atualmente resolve transitivas `risc0-circuit-* 4.0.5`
  incompatíveis com APIs de `risc0-zkvm 3.0.3`; isso não impede compilar o
  pacote isolado `vericode-methods`, mas pode bloquear receipts posteriores.
- A reexecução dos testes host do core ficou bloqueada offline pelo archive
  ausente `cfg-if 1.0.3`; nenhum lock ou fonte foi ampliado.
- Ainda não existem dois ELF VeriCode, ImageID VeriCode ou receipts VeriCode
  PASS/FAIL comprovados.
- O build upstream não expõe isolamento de rede Docker; Cargo deve permanecer
  explicitamente offline e a ausência de pull deve ser comprovada.
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Última auditoria

D1c2b.3g.1 em 2026-10-02: o diff limitado do core foi reproduzido em staging
e compilado com a toolchain guest real, imagem local fixada por digest,
`--pull=never`, `--network none`, Cargo locked/offline e flags oficiais do
builder. O probe gerou ELF32 RISC-V de 147.880 bytes, SHA-256 `3fc668…c42d`.
As tentativas anteriores e seus diagnósticos permanecem registradas; não há
ImageID ou alegação de build final.

## Próxima transição permitida

Validar integralmente diff, locks, segredos e artefatos; realizar auditoria
somente leitura e criar commit local limitado para D1c2b.3g.1. Depois, criar
dois contextos temporários independentes a partir desse commit, reproduzir e
auditar em cada um o vendor da união dos locks, usar targets separados e
compilar somente `vericode-methods`. Executar A e somente depois B. Não
iniciar receipts antes da comparação integral dos ELF e ImageIDs.

## Modelo e esforço do próximo gate

- Papel requerido: análise de dependências/locks e segurança, equivalente a
  GPT-5.6 Sol com esforço `high`.
- Limitação da superfície: seleção dinâmica de modelo e criação de subagente
  com modelo específico não estão disponíveis. O controlador seguirá
  sequencialmente com o modelo efetivo desta sessão e esforço operacional
  `high`; nenhuma delegação inexistente será simulada.
- Executor único: este controlador.
- Auditoria: etapa separada, independente e somente leitura após a conclusão
  do executor e antes de qualquer transição ou commit.
