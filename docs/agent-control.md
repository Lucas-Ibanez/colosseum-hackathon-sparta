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

`D1c2b.3e` — fechar offline o lock guest copiando para a cache destino
somente archives exigidos, ausentes, presentes na fonte local autorizada e
com SHA-256 igual ao checksum do lock; depois validar metadata, tree e
árvores inversas locked/offline.

## Estado

`GO`

## Ações autorizadas

- Leitura e auditoria do clone, locks, caches e toolchains isoladas.
- Criar documentação e evidência estritamente pertencentes ao gate atual.
- Usar somente como fonte de archives
  `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo`.
- Usar somente como cache destino
  `/home/lucas/.local/share/vericode-spikes/d1c2b/cargo`.
- Copiar somente archives `.crate` exigidos pelo lock guest atual, ausentes
  na cache destino e cujo SHA-256 coincida exatamente com o checksum do lock.
- Executar Cargo/Rust/RISC Zero somente com `CARGO_HOME`, `RUSTUP_HOME`,
  `RISC0_HOME`, `PATH`, `RUSTUP_AUTO_UPDATE=0` e `CARGO_NET_OFFLINE=true`
  explicitamente declarados conforme o protocolo D1c2b.
- Criar vendor e targets somente em diretórios temporários auditáveis.
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

- A cache destino ainda não demonstrou conter todos os archives do lock guest
  reconciliado.
- A compatibilidade integral do guest com Rust guest `1.88.0-dev` ainda
  depende de build VeriCode real após o fechamento offline.
- O vendor precisa ser recriado do lock final e auditado; hashes antigos não
  podem ser reutilizados como evidência do conjunto reconciliado.
- Ainda não existem dois ELF VeriCode, ImageID VeriCode ou receipts VeriCode
  PASS/FAIL comprovados.
- O build upstream não expõe isolamento de rede Docker; Cargo deve permanecer
  explicitamente offline e a ausência de pull deve ser comprovada.
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Última auditoria

Preflight somente leitura em 2026-10-02: raiz e Git corretos; branch `main`;
HEAD `0d55e44a69ed13325d8a33deea00840bde6540db`; árvore limpa;
`git diff --check` exit `0`; filesystem Linux; `/home/lucas/.rustup` ausente;
documentação obrigatória, relatórios `docs/d1*.md`, manifests e registros dos
locks relevantes lidos. Nenhuma mutação ocorreu durante o preflight.

## Próxima transição permitida

Se o controle persistente passar diff, higiene e auditoria, criar commit local
limitado. Depois executar apenas `D1c2b.3e`. Avançar para vendor/build somente
se metadata/tree e as árvores inversas passarem locked/offline. Qualquer
archive ausente, checksum divergente ou fonte fora da allowlist muda o estado
para `BLOQUEADO` ou `AGUARDANDO_AUTORIZAÇÃO`, conforme a causa.

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
