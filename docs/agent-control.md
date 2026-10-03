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

`D1c2b.3j` — auditoria final independente concluída; histórico, árvore,
locks, evidências, hashes, receipts e fronteiras recertificados.

## Estado

`CONCLUÍDO`

## Ações autorizadas

- Validar o diff documental final, criar um commit local limitado, confirmar
  Git limpo e completar o Goal.
- Leitura final do clone e das evidências temporárias somente para confirmar
  a conclusão.
- Nenhuma nova cópia, alteração de código/lock, build ou execução de proving
  está autorizada sob este objetivo concluído.

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

Todas as evidências abaixo estão satisfeitas e recertificadas no relatório
D1c2b.3j:

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

- Nenhuma tarefa obrigatória permanece dentro do objetivo D1c2b. Os itens a
  seguir são limitações residuais, não autorização de trabalho adicional.
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

D1c2b.3j em 2026-10-03: Git/ambiente e fronteiras limpos; locks e quatro
comandos metadata/tree recertificados offline; vendors A/B 467/467 e 23.182
arquivos integralmente iguais; ELF/método/ImageID A/B idênticos; 2/2 testes
host; receipts PASS/FAIL/wrong-image reverificados, não Fake, e três negativos
rejeitados. Matriz final sem tarefa obrigatória restante.

## Próxima transição permitida

Nenhuma transição dentro de D1c2b. Validar o diff documental final, criar o
commit local limitado, confirmar Git limpo e completar o Goal. Parar antes de
qualquer escopo proibido; trabalho posterior exige nova autoridade e objetivo.

## Modelo e esforço do próximo gate

- Não há próximo gate dentro deste objetivo.
- Limitação da superfície: seleção dinâmica de modelo e criação de subagente
  com modelo específico não estão disponíveis. O controlador seguirá
  sem simular delegação inexistente.
- Executor único e auditoria separada somente leitura foram mantidos até a
  conclusão.
