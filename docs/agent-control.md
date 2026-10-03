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
  `f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226`
  após reconciliação D1c2b.3h; valor de baseline:
  `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`.
- Lock guest reconciliado: SHA-256
  `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`.

## Gate atual

`D1c2b.3j` — realizar auditoria final independente do marco: conferir
histórico, árvore, locks, evidências, hashes, receipts e fronteiras; concluir
somente se nenhuma tarefa obrigatória permanecer.

## Estado

`GO`

## Ações autorizadas

- Leitura e auditoria do clone, locks, caches e toolchains isoladas.
- Criar documentação e evidência estritamente pertencentes ao gate atual.
- O uso adicional de `lane-a/cargo` e `lane-b/cargo` foi autorizado para
  fechar o archive guest `risc0-groth16-3.0.2.crate`; depois, a retomada
  humana autorizou dez archives host exatos de `lane-a/cargo`. Essas
  autorizações já foram consumidas; nenhuma cópia adicional está autorizada
  por inferência.
- Usar somente como cache destino
  `/home/lucas/.local/share/vericode-spikes/d1c2b/cargo`.
- Preservar a cache D1c2b já fechada; nenhuma nova cópia de archive é
  necessária ou autorizada para o gate de receipts.
- Executar Cargo/Rust/RISC Zero somente com `CARGO_HOME`, `RUSTUP_HOME`,
  `RISC0_HOME`, `PATH`, `RUSTUP_AUTO_UPDATE=0` e `CARGO_NET_OFFLINE=true`
  explicitamente declarados conforme o protocolo D1c2b.
- Criar vendor e targets somente em diretórios temporários auditáveis.
- Fazer a correção mínima `no_std + alloc` no core puro necessária para o
  guest real, preservando versões, locks, serialização, hashing e semântica.
- Usar um dos métodos finais A/B byte a byte idênticos, SHA-256
  `e09ba8cf…78f5` e ImageID `4da06f90…fb1a`, para proving local real, sem dev
  mode, e verificar receipt, ImageID e journal esperado.
- Auditar somente leitura os receipts temporários D1c2b.3i e seus hashes;
  nenhum novo proving é necessário salvo diagnóstico novo.
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

- Os dois builds finais A/B terminaram e produziram ELF e método combinado
  idênticos; os artefatos permanecem efêmeros em `/tmp`.
- Receipts PASS/FAIL reais foram comprovados, mas permanecem efêmeros em
  `/tmp`; são `Composite`, não Groth16.
- O build upstream não expõe isolamento de rede Docker; Cargo deve permanecer
  explicitamente offline e a ausência de pull deve ser comprovada.
- O BuildKit reutilizou bytes antigos para um arquivo com mesmo
  caminho/tamanho/timestamp; o workaround limitado a timestamp foi
  reproduzido e os dois vendors finais foram comparados integralmente.
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Última auditoria

D1c2b.3i em 2026-10-03: host local sem dev mode executou PASS/FAIL e produziu
três receipts `Composite` de 221.540 bytes. PASS/FAIL verificaram contra
ImageID `4da06f90…fb1a`; três negativos foram rejeitados. Auditor temporário
independente desserializou os arquivos, excluiu `Fake`, repetiu verificação e
revalidou journals/negativos.

## Próxima transição permitida

Validar integralmente o relatório D1c2b.3i, diff, locks, segredos e ausência
de artefatos no clone; realizar auditoria somente leitura e criar commit
local limitado. Depois, auditar o marco inteiro a partir do Git limpo. Se
todas as condições persistentes estiverem sustentadas, atualizar este
controle para `CONCLUÍDO`, criar o commit final auditado e completar o Goal.

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
