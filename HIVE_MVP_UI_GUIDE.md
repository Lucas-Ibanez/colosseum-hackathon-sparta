# Hive MVP — Guia para construção da interface visual

> Arquivos relacionados: o sistema visual (paleta, tipografia, componentes e tokens) está em `DESIGN.md`; os assets de marca estão em `brand/` (veja `brand/MANIFEST.md`). Onde este guia fala em "arquivo de identidade", leia `DESIGN.md`.
> Nomenclatura: o produto agora se chama Hive (antes, Vericode). Só a interface nova usa o nome Hive. Comandos da CLI, nomes de programa, instruções, erros e caminhos exibidos na UI aparecem exatamente como existem no código, e o código não é renomeado.
> MVP construído: este guia foi escrito antes do MVP. Onde ele e `HIVE_MVP_UI_ADAPTATION.md` divergirem sobre fatos do MVP (por exemplo, o Verifier Router, que não está no caminho), vale a adaptação. A identidade do `DESIGN.md` não muda.

> Este arquivo orienta o agente de código (Claude Code) na criação da interface do MVP da Hive para o Hackathon da Colosseum.
> Ele define **o que a interface deve mostrar, como deve se comportar e como deve usar a identidade visual em termos de papéis e princípios**.
> Ele **não** define valores de identidade (cores, fontes, medidas, logo). Esses valores virão de um arquivo de identidade separado.

---

## 0. Como usar este arquivo

### 0.1 Precedência entre arquivos

1. **Este arquivo** governa estrutura, conteúdo, comportamento, semântica de estados e vocabulário.
2. **O arquivo de identidade** (fornecido separadamente) governa os valores visuais: paleta, tipografia, escala, raios, logo.
3. Se houver conflito sobre **o que um elemento significa ou promete**, este arquivo vence.
4. Se houver conflito sobre **como um elemento se parece**, o arquivo de identidade vence.

### 0.2 Antes de escrever qualquer código

1. **Inspecione o repositório.** Identifique stack, estrutura, CLI, API do worker, program Anchor e como o estado do job é lido.
2. **Adapte-se à stack existente.** Não introduza framework, biblioteca de UI pesada ou gerenciador de estado novo sem necessidade clara.
3. **Não invente endpoints, campos ou estados.** Os nomes de estados, contas, instruções e erros devem vir do código real do program e do worker, não deste arquivo. Os nomes usados aqui são conceituais.
4. Se um dado necessário não existir no backend, **não simule**. Veja a seção 9.

### 0.3 Como tratar a identidade ainda não fornecida

O arquivo de identidade pode chegar depois do esqueleto da interface. Portanto:

- Construa **todos** os valores visuais como **tokens semânticos** (variáveis CSS ou equivalente da stack), nomeados por **papel**, nunca por valor.
- Nunca escreva cor, família tipográfica, tamanho ou raio literal dentro de componentes. Componentes só referenciam tokens.
- Use valores provisórios **neutros e claramente temporários** nos tokens. Marque cada um com um comentário `TODO(identity)`.
- Trocar a identidade inteira deve exigir alterar **somente** o arquivo de tokens.

Papéis mínimos de token que a interface deve prever:

| Grupo | Papéis |
|---|---|
| Cor de marca | identidade principal, destaque de ação |
| Superfícies | fundo geral, superfície de conteúdo, superfície secundária, superfície de navegação, superfície elevada (modal, menu) |
| Texto | principal, secundário, sobre navegação, sobre ação de destaque |
| Bordas | divisória suave, borda funcional (campos, foco) |
| Estado de resultado | sucesso, falha, atenção, neutro/pendente (cada um com variante de fundo suave e de texto/ícone) |
| Tipografia | família da interface, família técnica (monoespaçada), e escala por papel (título, seção, corpo, legenda, métrica, dado técnico) |
| Forma | raio de campo/botão, raio de cartão, espessura de linha, espaçamento base |
| Modo | tokens organizados para permitir modo claro e escuro **por função**, não por inversão |

---

## 1. Princípio central

**A interface da Hive é uma interface de justificação, não de decisão.**

O veredito é produzido pelo guest dentro do zkVM, o vínculo é conferido pelo programa, e a proof é verificada pelo Verifier Router. A UI **exibe** o que aconteceu, **explica** por quê e **inicia operações reais**. Ela nunca decide.

O slogan da marca, "Confiança começa com um porquê", é o critério de qualidade:

> **Todo estado terminal exibido deve ter um "porquê" a um clique de distância: a evidência que o produziu.**

### 1.1 Regras invioláveis

1. **Nenhum botão decide veredito ou destino de token.** Não existem "Aprovar", "Rejeitar", "Liberar pagamento" ou equivalentes discricionários.
2. A única ação do comprador após o depósito que depende de condição é **solicitar refund após o prazo**, e ela só fica habilitada quando a condição objetiva for verdadeira (prazo vencido, job não liquidado).
3. **Nenhum dado simulado** é exibido como real. Nenhuma assinatura, hash, saldo ou estado inventado.
4. **Nenhuma afirmação além do que foi provado.** Veja a seção 8.
5. **O que é lido da cadeia é rotulado como lido da cadeia.** Veja a seção 5.
6. **Fallback é rotulado como fallback.** Nunca chame de "verificado on-chain" algo que não passou pelo Router.

---

## 2. Quem usa esta interface

Projete para dois leitores, nesta ordem:

1. **Jurado do hackathon**, com poucos minutos, vendo vídeo ou clicando sozinho. Precisa perceber, sem explicação verbal, que algo **real** aconteceu: hashes, receipt, transação, dinheiro que se moveu, e **um caso que falha**.
2. **Design partner do ICP** (tech lead ou sócio de software house AI-native, ou operador de plataforma de jobs). Precisa reconhecer o próprio workflow: tarefa delimitada, critério de aceite automatizável, contrapartes distintas.

**Não projete para um operador diário.** Ele não existe ainda. Isso elimina listas longas, filtros, paginação e catálogos.

**A unidade de navegação é o job, não a lista.** A lista só existe para chegar ao job.

---

## 3. Escopo

### 3.1 Dentro do escopo

- Criar e financiar um job.
- Acompanhar o ciclo do job (estados).
- Mostrar compromissos versus o que foi observado no journal.
- Mostrar verificação local e verificação on-chain, separadas.
- Mostrar liquidação (Released ou Refunded) com a transação.
- Mostrar os cenários negativos com a mesma qualidade do caminho feliz.
- Mostrar o que a prova **não** demonstra.
- Mostrar o comando CLI equivalente de cada operação.
- Mostrar ambiente e versões.

### 3.2 Fora do escopo (não construir)

- Catálogo de agentes, perfis, reputação, score, ranking.
- Halo de confiança funcional, estados de confiança contextual (Unknown, Observed, Established, Restricted).
- Editor ou biblioteca de políticas.
- Multi-tenant, seletor de organização, gestão de usuários, login sofisticado.
- Histórico analítico, gráficos, métricas agregadas, indexador.
- Busca, filtros e paginação além do mínimo estritamente necessário.
- Animações decorativas de rede ou partículas.
- Painel enterprise, billing, múltiplos trilhos de pagamento.
- Marketplace, descoberta de executores.
- Responsividade mobile além de **não quebrar**.

### 3.3 Ordem de corte (se faltar tempo)

Corte nesta ordem, de cima para baixo:

1. Acabamento visual de segundo plano (modo alternativo de tema, microinterações).
2. Lista de jobs além do mínimo.
3. Elementos de identidade ornamentais (padrões, ilustrações).
4. Detalhes secundários de formulário.

**Nunca corte, para preservar aparência:** compromissos versus observado, separação local/on-chain, anatomia da transação, cenário negativo visível, limites da prova, rotulagem de fallback, links de Explorer funcionais.

---

## 4. Telas e componentes essenciais

### 4.1 Mapa mínimo de telas

| Tela | Função |
|---|---|
| Jobs (entrada) | Chegar rapidamente a um job. Lista mínima, poucos itens reais. |
| Novo job | Definir e financiar o compromisso. |
| Detalhe do job | **A tela central do produto.** Concentra estado, compromissos, verificação, liquidação, limites. |

Não crie telas adicionais de "Agentes", "Políticas" ou "Evidências" como seções autônomas. Navegação é promessa; não prometa o que o MVP não entrega.

### 4.2 Tela: Jobs (entrada)

- Lista **curta**, com jobs reais e verificáveis. Poucos itens bem escolhidos superam muitos rasos.
- Cada linha mostra no mínimo: identificador, descrição curta da tarefa, estado, valor, prazo.
- Estado sempre com **ícone + rótulo** (nunca só cor).
- A linha destacada indica **foco ou seleção**, nunca aprovação.
- Sem busca nem filtros, a menos que a lista passe de poucos itens reais.
- Ação primária visível: criar novo job.

### 4.3 Tela: Novo job (compromisso e financiamento)

Esta tela comunica que **os critérios foram fixados antes de qualquer trabalho acontecer**. Não é um formulário descartável.

Deve conter:

- Campos do compromisso: especificação, harness, `image_id` aceito, destinatário, valor, prazo.
- **Hash calculado e exibido na própria tela**, com a origem do arquivo ou conteúdo. O usuário deve ver o hash ser produzido.
- Marcação de que o compromisso **precede** o depósito.
- Endereço do **vault PDA** derivado, exibido antes do depósito (veja seção 6).
- Mint do token, identificado como token de teste.
- Prazo expresso em **slot** (ou conforme o program define), com o slot atual ao lado e uma tradução legível.
- Resumo do compromisso que, após o financiamento, torna-se **imutável**.

Regras:

- **Formulário único e denso**, não wizard de vários passos.
- Depois do estado de financiado, **não exiba controle de edição** de nenhum campo do compromisso.
- A ação de financiar é a ação primária da tela e usa o destaque de ação da identidade.
- Antes de qualquer transação, mostre claramente **o que será assinado** e o **comando CLI equivalente**.

### 4.4 Tela: Detalhe do job (a tela do produto)

Organize em blocos, **nesta ordem de prioridade visual**:

#### Bloco A — Estado e linha do tempo

- Estado atual em destaque, com ícone e rótulo.
- Estados já percorridos, o atual e os possíveis à frente.
- **`Failed` (erro operacional) deve ser visualmente distinto de `Refunded` (veredito econômico).** São coisas diferentes e a interface deve ensinar isso.
- Cada transição registrada com link para a transação correspondente, quando houver.
- Use nomes e ordem de estados **do program real**.

#### Bloco B — Compromissos versus observado

Componente mais importante da interface. Tabela campo a campo:

| Campo | Comprometido no job | Publicado no journal | Vínculo |
|---|---|---|---|

Campos conceituais (confirme os nomes reais no código): `job_id`, `spec_hash`, `harness_hash`, `image_id`, `artifact_hash`, versão do journal, `verdict`.

Regras:

- Cada linha tem resultado de vínculo explícito: **confere** ou **diverge**, com ícone e rótulo.
- Campos sem contrapartida de compromisso (como `artifact_hash`, calculado no guest) exibem "não aplicável", não ficam vazios.
- **Uma divergência deve ser visível de relance** e explicar a rejeição. Esse bloco é o que torna os cenários de job errado e de hash divergente compreensíveis.
- Hashes em fonte técnica, com cópia e acesso ao valor completo (veja seção 7).

#### Bloco C — Verificação (duas camadas separadas)

Separe visualmente, com títulos distintos:

1. **Verificação local**: receipt gerada, `image_id` conferido, journal decodificado, resultado da checagem local.
2. **Verificação on-chain**: chamada ao Verifier Router, resultado, assinatura da transação, link de Explorer.

Regras:

- Se a verificação on-chain **não** passou pelo Router (fallback), o bloco deve dizer **"fallback"** de forma explícita e inequívoca.
- Mostre versões relevantes (Router, `image_id` ativo).
- Mostre o estado **em andamento** de cada camada (veja 4.5).

#### Bloco D — Liquidação

- Resultado: Released, Refunded ou ainda não liquidado.
- **Anatomia da transação** (veja seção 6.4): instruções em ordem.
- Saldos antes e depois das contas envolvidas.
- Motivo da liquidação em linguagem clara: "veredito PASS verificado" ou "veredito FAIL verificado" ou "prazo expirado sem liquidação".

#### Bloco E — Limites da prova (seção nomeada, nunca rodapé cinza)

Seção própria, sempre presente, com texto curto e fixo comunicando em essência:

- O que a verificação demonstra: que o avaliador comprometido executou sobre o artefato entregue e produziu aquele veredito, vinculado àquele job.
- O que **não** demonstra: completude do harness, segurança geral do software, ausência de vulnerabilidades fora do modelo testado, adequação para qualquer outra finalidade.

Isso é diferenciação, não ressalva jurídica. Trate com a mesma importância visual dos outros blocos.

#### Bloco F — Reproduzir

- Para cada operação, o **comando CLI equivalente**, copiável.
- Comunica que a interface orquestra o fluxo real e oferece caminho de reprodução imediato.

### 4.5 Estado "Proving" (obrigatório)

Geração de prova não é instantânea. A interface **deve** ter um estado de espera honesto:

- Mostre **tempo decorrido real**.
- Mostre a **etapa atual** se o worker a reportar.
- **Proibido:** barra de progresso falsa, estimativa inventada, spinner infinito sem informação, qualquer pré-computação disfarçada.
- Ao concluir, mostre o **tempo de prova real** como dado do job. Proof time é métrica do MVP e um ativo, não um defeito a esconder.

### 4.6 Cenários negativos como cidadãos de primeira classe

O caminho ruim tem **a mesma qualidade de apresentação** do caminho bom. A demonstração deve poder exibir um job que falha com a mesma clareza de um que passa.

Os cenários abaixo significam coisas diferentes e **não podem compartilhar o mesmo ícone e mensagem genéricos**:

| # | Cenário | Natureza | O que a UI deve comunicar |
|---|---|---|---|
| 1 | **FAIL válido** | A prova funcionou; o trabalho não passou. Refund. | Não é erro. É resultado verificado. |
| 2 | **Vínculo quebrado** (job trocado, hash divergente, `image_id` errado) | Transação rejeitada | Qual campo divergiu (Bloco B). Estado do job preservado. |
| 3 | **Replay / dupla liquidação** | Transação rejeitada por estado já terminal | Distinto do item 2: o vínculo pode estar correto, mas o job já foi liquidado. |
| 4 | **Proof incompatível** | Rejeição no Router | A verificação on-chain falhou; nenhuma liquidação ocorreu. |
| 5 | **`Failed` operacional** | Erro de geração ou de infraestrutura | Nem veredito nem fraude. Retentar ou aguardar o prazo. |
| + | **Timeout** | Prazo expirou sem liquidação | Refund disponível por condição objetiva. |

Regras:

- Rejeições on-chain mostram o **nome do erro do program** e a transação (ou simulação) que falhou, quando existirem. Isso prova que o contrato barra, e não a tela.
- **Ausência de evidência, resultado negativo e rejeição técnica não são equivalentes.** Não os agrupe visualmente.
- Rejeição não é "falha do usuário". Use tom neutro e informativo.

### 4.7 Rodapé/faixa de ambiente (fixo em todas as telas)

Sempre visível e discreto:

- Cluster (devnet).
- Token de teste (nome).
- `image_id` ativo.
- Versão do Verifier Router e do program.
- Identificador de commit/versão da aplicação.

Serve para honestidade, para responder "isso é mainnet?" e como sinal de rigor.

---

## 5. Fonte de verdade e proveniência dos dados

### 5.1 Regra

A interface **não é a fonte de verdade**. A CLI e a cadeia são.

- **Estado on-chain** (estado do job, saldo do vault, transações) deve ser **lido da cadeia via RPC**, não de banco ou cache da aplicação.
- **Dados produzidos fora da cadeia** (receipt local, journal decodificado, tempo de prova, logs do worker) vêm da API do worker.
- A UI **nunca deriva veredito**. Ela exibe o que o journal e a cadeia contêm.

### 5.2 Rótulo de proveniência

Todo dado relevante carrega, de forma discreta mas legível, sua origem. Crie um componente reutilizável (por exemplo, `ProvenanceBadge`) com variantes como:

- **On-chain**, com o slot da leitura ("lido da cadeia no slot N").
- **Off-chain / worker**.
- **Calculado localmente** (por exemplo, o hash calculado na tela de criação).

O rótulo de slot responde por si só à dúvida "isso é mock?". Não o omita.

### 5.3 Atualização

- Estados em andamento devem atualizar sem recarregar a página (polling ou subscription, conforme a stack).
- Indique quando um dado está sendo atualizado e quando falhou ao atualizar.
- Se a leitura on-chain falhar, **diga que falhou**. Não mostre o último valor como se fosse atual.

---

## 6. Presença de Solana

Solana deve estar presente como **estado verificável**, não como marca.

### 6.1 O que é específico e deve ser destacado

1. O **vault como PDA**, com custódia real.
2. A **máquina de estados** vivendo em conta on-chain.
3. A **CPI ao Verifier Router**, verificando a proof na própria transação.
4. A **liquidação atômica** de SPL token.

"Usamos USDC em Solana" não é diferencial. **A verificação da receipt dentro da transação é.** Priorize a visibilidade nesta ordem:

1. Verificação e liquidação na mesma transação.
2. Vault PDA e estado do job lidos da cadeia.
3. Saldos de token antes e depois.
4. Carteira conectada (menos importante).

### 6.2 Onde aparece

| Momento | O que mostrar |
|---|---|
| Novo job | Program ID, vault PDA derivado, mint (de teste), prazo em slot com slot atual |
| Financiamento | Assinatura da transação, saldo do vault lido da cadeia |
| Detalhe | Endereço da conta do job, estado on-chain com slot de leitura |
| Verificação | Program ID e versão do Router, resultado, compute units consumidas |
| Liquidação | Anatomia da transação, saldos antes e depois |
| Negativos | Nome do erro do program, transação ou simulação que falhou |
| Timeout | Slot atual versus `deadline_slot` |
| Sempre | Faixa de ambiente com cluster e program IDs |

### 6.3 Regras de uso

- **Todo hash, endereço e assinatura é copiável**, com acesso ao valor completo.
- **Todo link de Explorer deve abrir e conferir.** Link quebrado destrói a credibilidade de todo o resto. Teste cada um.
- Use o cluster correto no link (devnet).
- **Solana não ganha identidade visual própria na interface.** Não use as cores nem gradientes da rede. A identidade da Hive permanece. O logo da rede só aparece onde for referência nominal, e conferindo as diretrizes de uso da marca.
- Na linguagem da UI, Solana é a **camada de consequência** ("Liquidação em Solana devnet"), não a identidade do produto. O domínio da Hive deve permanecer independente de trilho. Evite espalhar nomes de tipos Solana na camada de apresentação além do necessário.

### 6.4 Anatomia da transação (componente-chave)

Componente (por exemplo, `TransactionAnatomy`) que, para a transação de liquidação, lista **as instruções em ordem** de forma legível:

1. Chamada ao Verifier Router (CPI), com program ID.
2. Transferência de token.
3. Atualização de estado do job.

Mostre também: assinatura, slot, compute units, status. Isso torna visível a **atomicidade**, que é a garantia que o contrato oferece. É o momento mais forte da demonstração; dedique capricho a ele.

Use os nomes de instruções do **program real**.

### 6.5 Carteira e assinatura (decisão em aberto)

A CLI é a fonte de verdade e o front-end **não guarda chave privada**. Há duas possibilidades:

- **A)** Carteira no navegador assina a ação do comprador (mais convincente, mais risco de cronograma).
- **B)** A CLI assina e a UI exibe e orquestra.

**Não misture as duas.** Pergunte ao responsável qual adotar antes de implementar a assinatura. Enquanto isso, construa a UI de forma que a camada de assinatura seja **substituível**. Nunca aceite chave privada, seed ou segredo em campo de formulário, em storage do navegador ou no repositório.

---

## 7. Como usar a identidade visual (por papel, não por valor)

Esta seção diz **como** aplicar a identidade. Os valores vêm do arquivo de identidade.

### 7.1 Cor

- **Cor de identidade principal:** sustenta a estrutura (navegação, títulos, logotipo, superfícies de marca). Define o caráter geral da interface.
- **Cor de destaque de ação:** usada **com parcimônia** e **apenas** para ação primária, indicação de foco, seleção e detalhes da marca. Evite grandes áreas preenchidas com ela.
- **A cor de destaque de ação NUNCA representa resultado, aprovação ou sucesso.** Se "ação primária" e "entrega aprovada" compartilharem o mesmo significante, a interface passa a sugerir que aprova coisas. Isto é crítico.
- **Resultados têm cores próprias de status**, sempre acompanhadas de **ícone e rótulo de texto**. Nunca comunique estado só por cor.
- **Mesmo significado em todos os modos.** Em modo claro e escuro, um componente significa a mesma coisa. Ajuste as cores por função; não inverta mecanicamente.
- **Seleção ≠ resultado.** Uma linha destacada indica foco, não aprovação.
- Links seguem a regra de links da identidade (podem diferir entre modos).
- **Contraste:** valide texto, controles, bordas funcionais e estados de foco na interface implementada. Não presuma acessibilidade pela paleta.

### 7.2 Tipografia

- **Uma família principal** para a interface. Não misture famílias adicionais.
- **Uma família técnica (monoespaçada)**, restrita a conteúdo técnico: hashes, IDs de job, `image_id`, endereços, assinaturas, versões, comandos, código.
- **Não aplique a fonte técnica a números comuns.** Totais, métricas, datas e valores seguem a fonte principal. Só identificadores e sequências técnicas ganham tratamento monoespaçado.
- Hierarquia por **tamanho, peso, espaço e agrupamento**, sem recorrer a pesos extremos em todos os componentes.
- Caixa de frase nos rótulos. Maiúsculas só para siglas e códigos (API, ID, PASS, FAIL).
- **Não reduza a fonte para fazer o conteúdo caber.** Permita quebra, amplie o componente ou reorganize.
- Em tabelas, alinhe valores numéricos à direita, com casas decimais consistentes.
- Teste nomes longos, acentos do português e caracteres ambíguos (0/O, 1/I/l).

### 7.3 Campos técnicos (hash, endereço, assinatura)

Na Hive, **o hash não é metadado, é o argumento**. Tratamento obrigatório:

- Fonte técnica.
- Truncamento apenas com **acesso ao valor completo** (expandir, tooltip acessível ou equivalente).
- **Botão de copiar** em cada um, com confirmação visível.
- Quando fizer sentido, link de Explorer adjacente.
- Seleção de texto habilitada.
- Quebra segura para não estourar o layout.

Crie um componente reutilizável (por exemplo, `HashField`) e use-o em toda parte.

### 7.4 Forma e superfície

- Conteúdo em **retângulos com cantos suaves**, com raios consistentes por tipo de componente (campo/botão, cartão).
- **Traço de espessura predominantemente uniforme.** Não varie espessura ou densidade de linha para sugerir confiança, nível ou score.
- **Preenchimentos planos.** Sem sombras dramáticas, brilhos, relevo, texturas ou degradês decorativos.
- Cápsulas apenas para funções específicas (por exemplo, filtros, se existirem).
- Respeite espaçamento generoso: a interface deve parecer serena, precisa e criteriosa.

### 7.5 O hexágono e a representação de partes (uso contido)

A identidade usa o hexágono como símbolo de **identidade de um agente**. No MVP, use-o **com muita contenção**:

- Se representar comprador e executor no detalhe do job, use uma marca hexagonal simples associada ao endereço e ao papel.
- **A representação da parte é idêntica em PASS e em FAIL, em Released e em Refunded.** O que muda é o rótulo do resultado da interação, **nunca a célula**. Uma célula que muda de cor com o resultado vira selo de aprovação universal, o que a marca proíbe expressamente.
- Setas indicam **direção de delegação** (quem contratou quem). Rótulo da tarefa explícito.
- A representação **não** deve sugerir perfil, reputação, nível ou competência.

### 7.6 Elementos da marca que NÃO entram na interface do MVP

| Elemento | Motivo |
|---|---|
| Halo de confiança (contexto) | Sugere que a Hive sabe em que o agente é bom. O MVP não produz perfis contextuais. |
| Padrão de favos como fundo | Compete com os hashes e dá ar decorativo a um produto cujo valor é rigor. |
| Mascote | Em telas que decidem dinheiro, é ruído. Fica para vídeo e materiais institucionais. |
| Espessura/densidade de linha como indicador de confiança | Sem critérios definidos, é over-promise. |
| Taxonomia Unknown / Observed / Established / Restricted | Pressupõe histórico. O MVP tem a máquina de estados do escrow, que é concreta. Não misture estado de job com estado de confiança. |
| Animações de partículas em conexões | Fora do escopo; não agregam evidência. |
| Selos, escudos, "verified badge" genérico | Violam o princípio de que verificado não significa seguro. |

Quando houver dúvida se um elemento da marca cabe, aplique o teste: **ele exibe evidência, inicia operação real, ou cria a impressão de uma decisão que a UI não toma?** Se for o terceiro caso, remova.

### 7.7 Logo

- Use a assinatura apropriada ao espaço, **sem modificá-la**, conforme o arquivo de identidade.
- Não use a abelha como selo de aprovação. Resultados de verificação sempre vêm com contexto e rótulos próprios.
- Respeite área de proteção e tamanhos mínimos do arquivo de identidade.

### 7.8 Modo claro e escuro

- Implemente **um** modo com acabamento total. Estruture os tokens para permitir o outro depois, sem retrabalho de componentes.
- Não gaste orçamento polindo os dois se isso competir com itens da seção 3.3 "nunca corte".

---

## 8. Vocabulário e microcopy

### 8.1 Princípio

A linguagem é **serena, precisa e criteriosa**. Explica, não proclama. Descreve **critérios verificados**, não qualidades gerais.

### 8.2 Preferir

- "Critérios atendidos" / "Critérios não atendidos" (descrevem apenas o que foi verificado).
- "Verificado contra os critérios comprometidos."
- "Vínculo confere" / "Vínculo diverge".
- "Receipt verificada localmente." / "Proof verificada no Verifier Router."
- "Liquidado: Released" / "Liquidado: Refunded".
- "Fallback: não verificado on-chain."
- "Esta verificação não demonstra…" (na seção de limites).

### 8.3 Evitar

Termos que prometem mais do que a prova entrega:

- "Aprovado", "Rejeitado" como veredito da UI.
- "Seguro", "Confiável", "Garantido", "Certificado".
- "Trustless" ou equivalentes.
- "Agente confiável", "agente verificado" como qualidade geral.
- "Reputação", "score", "nível de confiança".
- Qualquer frase que sugira que a Hive decide.

### 8.4 Mensagens de erro e rejeição

- Neutras e informativas. Digam **o que** divergiu e **o que** foi preservado ("O estado do job não foi alterado").
- Nunca culpem o usuário.
- Incluam o nome técnico do erro em campo secundário, com cópia.

### 8.5 Idioma

- Centralize **todas** as strings em um dicionário (preparado para i18n). Nada de texto literal espalhado em componentes.
- **Idioma padrão: decisão em aberto.** Recomendação: inglês como padrão (jurado internacional), com português como segunda opção. Pergunte ao responsável antes de fixar.
- Regras de marca: use a versão da assinatura do idioma correspondente e preserve sua redação oficial. Não reescreva o slogan.

---

## 9. Estados vazios, de carregamento, de erro e dados indisponíveis

**Toda** leitura de dados precisa ter quatro estados projetados: carregando, vazio, erro e sucesso.

- **Carregando:** informe o que está sendo carregado. Sem skeleton enganoso para dados que podem não existir.
- **Vazio:** explique o que falta e qual ação cria o dado. Sem ilustração decorativa.
- **Erro:** diga qual fonte falhou (RPC, worker, programa) e se o dado exibido é o último conhecido, com horário/slot.
- **Dado indisponível no backend:** mostre explicitamente "indisponível" e **registre no código um `TODO` identificando o campo ausente**. Nunca preencha com valor plausível.

**Dados de demonstração:** use **jobs reais**, gerados pelo fluxo real, com transações verificáveis. Dados fictícios ao lado de dados reais custam mais credibilidade do que o ganho estético. Se precisar de dados de apoio, rotule-os de forma inequívoca como exemplo, e jamais em telas que mostram hash, assinatura ou saldo.

---

## 10. Interação e acessibilidade mínimas

- Toda ação que cria transação mostra **antes**: o que será executado, o programa envolvido e o comando CLI equivalente.
- Toda ação em andamento mostra estado e permite entender se falhou.
- Ações destrutivas ou irreversíveis exigem confirmação clara (por exemplo, financiar).
- Foco de teclado visível e ordem de tabulação lógica.
- Estado nunca depende só de cor.
- Elementos interativos com área de clique adequada.
- Textos de ajuda curtos, no ponto de uso.
- Não esconda informação essencial em tooltips que só aparecem ao passar o mouse; o valor completo de hashes e endereços deve ser acessível por teclado e toque também.

---

## 11. Componentes reutilizáveis sugeridos

Nomes ilustrativos; ajuste à convenção do repositório.

| Componente | Responsabilidade |
|---|---|
| `HashField` | Exibe hash/endereço/assinatura em fonte técnica, com truncamento, valor completo, cópia e link opcional |
| `ProvenanceBadge` | Rótulo de origem do dado (on-chain com slot, worker, calculado localmente) |
| `StateTimeline` | Estados percorridos, atual e possíveis, com links de transação |
| `StatusLabel` | Ícone + rótulo + cor de status para estados e resultados |
| `CommitmentVsObserved` | Tabela comprometido × journal com vínculo por linha |
| `VerificationPanel` | Duas camadas separadas: local e on-chain, com rótulo de fallback |
| `TransactionAnatomy` | Instruções da transação em ordem, assinatura, slot, compute units |
| `BalanceDelta` | Saldos antes e depois das contas envolvidas |
| `ProvingProgress` | Tempo decorrido real e etapa atual, sem progresso falso |
| `ScenarioOutcome` | Apresentação distinta para cada um dos cenários da seção 4.6 |
| `LimitsOfProof` | Seção fixa do que a prova não demonstra |
| `CliEquivalent` | Comando copiável da operação |
| `EnvironmentBar` | Faixa fixa de cluster, token de teste, `image_id`, versões |
| `PartyMark` (opcional) | Marca hexagonal idêntica de comprador/executor, com papel e endereço |

---

## 12. Anti-padrões (não faça)

1. Botão que decide, aprova ou rejeita entrega.
2. Cor de destaque de ação usada como cor de sucesso.
3. Célula/marca da parte mudando de cor conforme o resultado.
4. Barra de progresso falsa ou valor de prova pré-computado disfarçado.
5. Estado on-chain lido de cache sem rótulo de proveniência.
6. Link de Explorer que não abre ou aponta para o cluster errado.
7. Hash truncado sem acesso ao valor completo.
8. Verificação por fallback exibida como "verificada on-chain".
9. Cenário negativo com ícone/mensagem genérico igual a todos os outros.
10. Limites da prova escondidos em rodapé de baixo contraste.
11. Valores de identidade (cor, fonte, medida) escritos dentro de componentes.
12. Navegação com seções que prometem produtos inexistentes.
13. Dados fictícios misturados a dados reais em telas de evidência.
14. Textos como "seguro", "confiável", "garantido", "aprovado".
15. Padrões decorativos de favos competindo com dados técnicos.
16. Chave privada, seed ou segredo em qualquer ponto do front-end.
17. Campo do compromisso editável após o financiamento.

---

## 13. Critérios de aceitação

A interface está pronta quando **todos** os itens abaixo forem verdadeiros:

1. Nenhum botão da UI decide veredito ou destino de token.
2. Todo estado terminal tem caminho de um clique até a evidência que o produziu.
3. Compromissos e observado aparecem lado a lado, com vínculo explícito por campo.
4. Verificação local e verificação on-chain estão visualmente separadas, e fallback é rotulado como fallback.
5. Os cenários negativos da seção 4.6 são distinguíveis entre si.
6. O que a prova não demonstra aparece como seção nomeada.
7. Cluster (devnet), token de teste, `image_id` e versões estão visíveis em qualquer tela.
8. A representação das partes é idêntica em PASS e FAIL.
9. A cor de destaque de ação não aparece como cor de resultado.
10. Nenhum elemento sugere perfil contextual, score ou reputação.
11. Toda ação tem comando CLI equivalente copiável.
12. Cada transação tem link de Explorer que abre e confere, no cluster correto.
13. O estado exibido do job é lido da cadeia, com o slot da leitura visível.
14. A transação de liquidação mostra suas instruções em ordem, incluindo a CPI ao Router.
15. Erros on-chain do program aparecem com nome e mapeiam para os cenários negativos.
16. Qualquer uso de fallback é rotulado como tal, sem a expressão "on-chain".
17. Nenhum valor de identidade está escrito dentro de componentes; tudo passa por tokens semânticos.
18. Todo dado exibido tem proveniência rotulada e nenhum dado simulado aparece como real.
19. Estado nunca é comunicado apenas por cor.
20. Todo hash, endereço e assinatura é copiável com valor completo acessível.

---

## 14. Decisões em aberto (pergunte ao responsável antes de assumir)

1. **Assinatura:** carteira no navegador (A) ou CLI (B)? Veja 6.5.
2. **Idioma padrão da interface.** Veja 8.5.
3. **Modo de tema priorizado** (claro ou escuro) para acabamento total.
4. **Quais jobs reais** serão usados na demonstração (um que passa e um adversarial que falha).
5. **Origem exata dos dados** de cada bloco do detalhe do job (RPC direto versus API do worker). Mapeie no repositório e proponha antes de implementar.
6. **Se o Verifier Router está no caminho reproduzível**, ou se há fallback ativo. Isso determina o texto do Bloco C.

---

## 15. Ordem de trabalho sugerida

1. Inspecionar o repositório e mapear: estados reais, instruções, erros do program, endpoints do worker, leitura de cadeia.
2. Criar o arquivo de **tokens semânticos** com valores provisórios marcados `TODO(identity)`.
3. Construir componentes base: `HashField`, `StatusLabel`, `ProvenanceBadge`, `EnvironmentBar`.
4. Construir o **Detalhe do job** com dados reais: Blocos A, B, C, D, E, F.
5. Construir **Novo job** (compromisso e financiamento).
6. Construir a lista mínima de **Jobs**.
7. Implementar `ProvingProgress` e os estados de espera honestos.
8. Implementar a apresentação distinta de cada **cenário negativo**.
9. Revisar vocabulário (seção 8) e centralizar strings.
10. Passar o checklist da seção 13 item por item e corrigir.
11. Aplicar o arquivo de identidade trocando **somente** os tokens, quando ele for fornecido.

Não polir acabamento antes de os itens 4 a 8 estarem funcionando com dados reais.
