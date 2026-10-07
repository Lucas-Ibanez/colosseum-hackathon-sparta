# VeriCode — regras compartilhadas

## Escopo do MVP

O VeriCode verifica somente um artefato serializado restrito com regra determinística e harness fixo. O fluxo alvo usa escrow de Test USDC em Solana devnet e, quando comprovadamente viável, uma receipt RISC Zero.

<!-- hive-ui-core:start -->
## Marca e nomenclatura

O produto agora se chama **Hive**; até recentemente se chamava **Vericode**. O MVP (program, worker e CLI) está completo; falta o front-end e sua integração. Só o **design da interface nova** segue a identidade Hive. O código existente **não** foi renomeado e **não deve** ser.

- **Nunca renomeie** o que existe: pastas, pacotes e crates, nomes de programa e de instrução, seeds de PDA, variáveis, variáveis de ambiente, binário e comandos da CLI, nome do repositório. Sem busca e substituição global, nem "por consistência". Nomes on-chain e seeds não podem mudar.
- **Mudanças em program, worker ou CLI** só por pedido explícito. A interface se adapta ao backend, não o contrário.
- **Na interface nova**, a marca é Hive: logotipo, `<title>`, textos institucionais, nome do produto em títulos e mensagens.
- **Identificadores técnicos exibidos** (comando da CLI, nome de programa, instrução, erro, caminho, pacote) aparecem **exatamente como existem no código**, mesmo contendo o nome legado. A UI não inventa nomes nem comandos.
- **Código novo de design** (tokens como `--hive-*`, componente `BrandMark`, tema) usa Hive. Nomes de pasta e de pacote do front-end seguem a convenção do repositório existente; em caso de dúvida, pergunte.
- Identificadores legados principais (referência; não alterar):
  - diretório do clone: `vericode` (`/home/lucas/src/vericode`);
  - crates e pacotes: `vericode-core`, `vericode-escrow`, `vericode-cli`, `vericode-prover`, `vericode-host`, `vericode-methods`, `vericode-guest`;
  - programa Anchor: `vericode_escrow` (program ID `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`) e o tipo de erro `VericodeEscrowError`;
  - binários da CLI: `vericode` e `vericode-prover`;
  - worker: `worker/vericode_worker.py` e o header HTTP `X-VeriCode-Token`;
  - variáveis de ambiente com prefixo `VERICODE_` (`VERICODE_REAL_DOCKER`, `VERICODE_DOCKER_SHIM_LOG`, `VERICODE_GUEST_ELF`, `VERICODE_GUEST_ID`);
  - artefato do guest admitido: `prover/artifacts/vericode-guest.bin`;
  - raiz de trabalho fora do clone: `~/.local/share/vericode-spikes/`.

## UI/UX: leitura obrigatória antes de agir

Antes de qualquer tarefa de UI/UX, **leia inteiros** os arquivos abaixo. Não responda de memória e não use trechos nem resumos.

1. `HIVE_MVP_UI_ADAPTATION.md`: adaptação do guia ao MVP construído (o que se mantém, o que muda e o que fica de fora; fontes de dados; stack). O guia foi escrito antes do MVP.
2. `HIVE_MVP_UI_GUIDE.md`: comportamento, escopo, semântica de estados e vocabulário (o que cada elemento significa ou promete).
3. `DESIGN.md`: sistema visual (tokens, tipografia, componentes, números, labels, movimento, acessibilidade).
4. `brand/MANIFEST.md`: qual arquivo de marca usar e como. Abra antes de usar qualquer logotipo, símbolo ou ícone (não é importado).

Contam como UI/UX: telas, componentes, estilos (CSS, Tailwind, tokens, temas), layout, textos de interface e mensagens de erro, ícones, estados vazio/carregando/erro, acessibilidade e qualquer uso da marca.

**Checkpoint obrigatório.** A primeira linha da resposta a uma tarefa de UI/UX é: `UI checkpoint: li HIVE_MVP_UI_ADAPTATION.md, HIVE_MVP_UI_GUIDE.md e DESIGN.md [e brand/MANIFEST.md]. Regras que mais pesam aqui: ...` (3 a 5 regras). Se algum arquivo não puder ser lido, pare e avise.

**Precedência.** Sobre fatos do MVP construído (estados, instruções, erros, programas, fontes de dados, verificação), vale o `HIVE_MVP_UI_ADAPTATION.md`, abaixo só da evidência e do código. Sobre o que um elemento significa ou promete, vale o `HIVE_MVP_UI_GUIDE.md` no que a adaptação não contrariar. Sobre como ele se parece, vale o `DESIGN.md`, que a adaptação não altera. Se dois deles entrarem em conflito fora dessas regras, pare e reporte.

**Invariantes (as mais violadas):**
- Nenhum botão decide veredito nem destino de token. A UI exibe evidência; não julga.
- Nenhum dado simulado exibido como real. Dado sem fonte aparece como "Indisponível" com `TODO(data)`.
- Cor, fonte e medida só por tokens do `DESIGN.md`, nunca literais em componentes.
- O âmbar nunca é cor de resultado. Estado nunca é comunicado só por cor.
- Logotipo, símbolo e ícones só dos arquivos de `brand/`. Nunca redesenhar, traçar nem redigitar a marca. Os SVGs atuais são provisórios.
- Sem halo, selo, mascote, padrão de favos nem pontuação de confiança nas telas do produto.
- Fallback é rotulado como fallback; nunca como "verificado on-chain".

**Skill.** Se a skill `frontend-design` estiver instalada, use-a para a execução (hierarquia, espaçamento, restrição, autocrítica). O `DESIGN.md` é o briefing e vence qualquer escolha estética da skill.

**Stack.** A interface é HTML, CSS e JavaScript sem framework nem dependência, servida pelo worker a partir de `worker/static/`. As ferramentas de desenvolvimento (lint do `DESIGN.md`, ícones e capturas com Playwright) ficam em `worker/ui-tools/`, com o Node isolado de `~/.local/share/vericode-spikes/ui/env-ui.sh`. Detalhes em `HIVE_MVP_UI_ADAPTATION.md` (seção 7) e em `worker/ui-tools/README.md`.

Os arquivos acima só mudam por pedido explícito. Depois de editar o `DESIGN.md`, rode `source ~/.local/share/vericode-spikes/ui/env-ui.sh && npm --prefix worker/ui-tools run lint:design` (equivale a `design.md lint DESIGN.md`) e deixe sem erros.
<!-- hive-ui-core:end -->

## Princípios não negociáveis

1. O MVP não verifica repositórios arbitrários nem patches gerais; verifica um artefato serializado restrito por regra determinística e harness fixo.
2. A lógica de negócio vive em uma crate Rust pura, sem dependência de Solana, Anchor ou RISC Zero.
3. O guest RISC Zero e Anchor compilam para alvos incompatíveis; código compartilhado não depende de nenhum dos dois ambientes.
4. `Verdict::Fail` é saída normal e verificável do guest, nunca `panic` ou `assert`.
5. A prova expõe somente um journal/manifest público contendo, no mínimo: `schema_version`, `job_id`, `spec_hash`, `harness_hash`, `artifact_hash`, `image_id` e `verdict`.
6. O contrato só libera fundos para o executor e mint definidos no Job, após prova e journal correspondentes ao Job.
7. Não existe admin bypass.
8. Sem prova Router/CPI realmente validada, não alegar verificação ZK on-chain. Atestado de plataforma é fallback explicitamente nomeado, nunca prova ZK.
9. Nunca imprimir, versionar ou solicitar seed phrase, keypair, chave privada ou `.env` real. Exceção autorizada (decisão D2a.2): o agente pode criar keypairs efêmeros **somente para devnet/localnet** (buyer, executor, payer/deployer, mint de Test USDC), em gate autorizado, armazenados fora do clone com permissão `0600`, sem exibir seed phrase ou chave privada, publicando apenas chaves públicas; nunca mainnet ou dinheiro real.

## Documentos de contexto

- `docs/project-context.md`: índice, precedência, mapeamento guia →
  repositório, estado da sequência e conflitos conhecidos. Ler no início de
  toda tarefa.
- `docs/context/guia-mvp-agentes-de-codigo.md`: norma de produto (missão,
  escopo, contrato de dados, máquina de estados, invariantes, gates M0–M7).
- `docs/context/sequencia-mvp.md`: cronograma D0–D12; é plano, não evidência.
- Em conflito, aplicar a ordem de precedência de `docs/project-context.md`.

## Protocolo de trabalho

- Antes de qualquer tarefa automatizada, ler `docs/agent-control.md` e
  `docs/project-context.md`.
- Executar uma tarefa pequena por vez: planejar, alterar, testar, registrar evidência real e revisar o diff.
- Não substituir falha por mock nem apresentar dev mode, stub ou fallback como sucesso real.
- Não alterar schema ou journal sem atualizar explicitamente a documentação e registrar a decisão.
- Segredos nunca entram no Git; usar somente nomes de variáveis e exemplos vazios.
- Antes de alterar zkVM, Router, schema ou contrato, ler `docs/architecture.md`, `docs/manifest-schema.md`, `docs/zkvm-notes.md`, `docs/router-notes.md` e `docs/decisions.md`.
- Antes de alterar escrow, política econômica, CLI/UI ou claims públicos, ler `docs/context/guia-mvp-agentes-de-codigo.md`, `docs/escrow-state-machine.md` e `docs/mvp-agent-operating-guide.md`.

## Definição de pronto

Entregar diff revisado, comando executado, saída real, evidência registrada e riscos abertos. Falhas permanecem visíveis.

Toda tarefa concluída termina com o prompt completo da próxima fase, conforme `docs/handoff-protocol.md`: salvo em `docs/handoffs/` e reproduzido na mensagem final.
