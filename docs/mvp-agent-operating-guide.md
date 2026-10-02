# Guia operacional do MVP para agentes

## Finalidade

Este documento traduz o objetivo do VeriCode em limites de produto, critérios
de aceite e perguntas que um agente deve responder antes de ampliar o sistema.
Ele complementa, mas não substitui:

- `AGENTS.md`, para regras obrigatórias de trabalho e segurança;
- `docs/architecture.md`, para fronteiras entre componentes;
- `docs/manifest-schema.md`, para o contrato de dados vigente;
- `docs/zkvm-notes.md` e `docs/router-notes.md`, para evidência das integrações;
- `docs/decisions.md` e `docs/evidence.md`, para decisões e resultados reais.

Este arquivo não registra o status dos gates. Consulte `docs/evidence.md` e os
relatórios do gate correspondente antes de afirmar que uma capacidade existe.

## Ordem de autoridade

Em caso de conflito, use esta precedência:

1. resultado reproduzido por comando e registrado no repositório;
2. documentação ou manifesto oficial da versão exata pinada;
3. decisão humana explícita registrada em `docs/decisions.md`;
4. contratos vigentes de arquitetura e schema;
5. este guia operacional;
6. hipótese, plano ou exemplo ainda não executado.

Se o comportamento observado contrariar um plano, preserve a falha, registre a
divergência e pare no gate. Não adapte a evidência para confirmar a expectativa.

## Missão e afirmação permitida

O MVP deve demonstrar aceite verificável de uma tarefa determinística sobre um
artefato serializado restrito:

1. um Job compromete previamente a especificação, o harness e a identidade do
   guest admitido;
2. o executor fornece o artefato restrito;
3. o guest avalia exatamente esses bytes e publica um `JournalV1` com os
   compromissos e `Verdict::Pass` ou `Verdict::Fail`;
4. a receipt é verificada localmente;
5. somente se Router/CPI forem realmente validados, o programa poderá usar a
   verificação on-chain como condição atômica de liquidação;
6. o escrow move Test USDC apenas segundo as autoridades e regras persistidas
   no Job.

A afirmação técnica máxima permitida é:

> Um avaliador determinístico previamente comprometido executou sobre o
> artefato vinculado ao Job e publicou o veredito correspondente. Quando a
> verificação on-chain for comprovada, esse veredito poderá condicionar a
> liquidação do escrow sem decisão discricionária da interface.

Mesmo com o fluxo completo, o MVP não prova correção universal do código,
cobertura completa dos requisitos, autoria, qualidade arquitetural,
manutenibilidade, privacidade total, ausência de toda confiança residual ou
product-market fit.

## Escopo da v1

### Incluído no alvo

- uma única classe de artefato serializado restrito;
- regra Rust pura, determinística e sem I/O externo;
- harness fixo e versionado;
- receipts locais reais para `PASS` e `FAIL`;
- vínculos explícitos entre Job, spec, harness, artefato, ImageID e verdict;
- escrow básico em Solana devnet com Test USDC, PDA, prazo, release e refund;
- fluxo reproduzível primeiro por CLI;
- pelo menos um cenário adversarial visível na demonstração;
- Router/CPI apenas se a integração for demonstrada com evidência real.

### Excluído

- repositórios ou patches arbitrários e múltiplas linguagens;
- execução de código arbitrário dentro do guest;
- avaliação com rede, relógio, RPC ou filesystem externo;
- mainnet, dinheiro real, custódia comercial ou dados de clientes;
- marketplace, matching, chat, reputação, score ou token de produto;
- LLM como juiz de uma condição de pagamento;
- testes secretos, fair exchange, TEE, MPC ou confidencialidade perfeita;
- painel enterprise, indexador completo ou múltiplos meios de pagamento.

Se houver pressão de prazo, reduza UI e orquestração antes de remover receipt
real, vínculo Job–journal, escrow mínimo, cenário negativo ou comunicação
honesta dos limites.

## Fronteiras da implementação

| Área | Responsabilidade | Proibição principal |
| --- | --- | --- |
| `crates/vericode-core` | Tipos canônicos, Borsh, hashes, fixtures e regra determinística | Solana, Anchor, RISC Zero, rede, relógio ou decisão de pagamento |
| `zkvm/methods/guest` | Ler o input restrito, chamar o core e publicar `JournalV1` | Duplicar a regra, decidir pagamento ou representar `FAIL` por panic |
| `zkvm/host` | Preparar input, gerar/verificar receipt e decodificar journal | Ser tratado como autoridade econômica por ter gerado a prova |
| futuro programa Anchor | Persistir Job, custodiar tokens, validar contas e liquidar | Reexecutar o harness, confiar no front-end ou aceitar destino livre |
| futuro Router/CPI | Verificar a prova no formato e deployment comprovados | Ser considerado disponível a partir de exemplo, tag ou claim externo |
| futura CLI/worker | Orquestrar operações e tornar falhas observáveis | Criar regra econômica ou substituir validação criptográfica |
| futuro front-end | Exibir estado, compromissos, receipt e transações reais | Decidir verdict, custodiar segredo ou simular sucesso assíncrono |

O core é a única implementação da regra. Guest e host devem consumir sua API e
seus vetores; Anchor e guest permanecem adaptadores separados para alvos
incompatíveis.

## Vínculos de segurança

### Estado mínimo esperado do Job

O desenho on-chain futuro precisa persistir, no mínimo:

- identidade canônica do Job;
- buyer e executor fixos;
- mint e amount do escrow;
- prazo on-chain;
- `spec_hash` e `harness_hash` acordados antes do depósito;
- `expected_image_id` admitido;
- estado da liquidação.

O nome, layout, seeds de PDA e tipos finais pertencem ao gate Anchor e não são
definidos por este documento.

### Journal público

O `JournalV1` vigente contém `schema_version`, `job_id`, `spec_hash`,
`harness_hash`, `artifact_hash`, `image_id` e `verdict`. O significado, layout
candidato e vetores estão em `docs/manifest-schema.md`.

Cada compromisso cobre uma pergunta diferente:

- `spec_hash`: o que deveria ser satisfeito;
- `harness_hash`: como a regra foi aplicada;
- `artifact_hash`: quais bytes foram avaliados;
- `image_id`: qual guest executou a avaliação.

Nenhum campo substitui outro. Qualquer mismatch deve ser rejeitado antes de
movimentar tokens.

### Decisões ainda abertas

Não assuma sem uma decisão registrada:

- se o comprador compromete previamente algum identificador de entrega ou se
  `artifact_hash` é aceito somente após a entrega;
- se um `Verdict::Fail` válido causa refund imediato ou apenas comprova a
  execução e aguarda outra regra de liquidação;
- a máquina de estados, as transições de timeout e quem pode acioná-las;
- o layout Anchor, seeds, account metas, formato de instrução e IDL;
- o tipo exato de receipt/seal aceito e as contas da CPI;
- qualquer deployment de Router em localnet ou devnet.

`Verdict::Fail` já é uma saída normal do domínio. Isso não define, por si só,
a política econômica de refund.

## Invariantes do futuro escrow

Estas invariantes são critérios de revisão para o gate Anchor:

1. O vault é uma token account controlada por PDA; não existe chave privada do
   vault.
2. Buyer, executor, mint, amount e prazo vêm do Job persistido.
3. Um `PASS` válido só pode pagar o executor registrado.
4. Um refund válido só pode devolver tokens ao buyer registrado.
5. Timeout antes do prazo deve falhar.
6. Mudança de estado e transferência devem ser atômicas.
7. Não existe admin bypass nem saque para destino arbitrário.
8. Evidência de outro Job, spec, harness ou ImageID deve falhar.
9. Estados terminais impedem replay e dupla liquidação.
10. Falha de verificação/CPI reverte a instrução sem mover tokens.

Até o programa existir e os testes passarem, essas invariantes são requisitos,
não capacidades implementadas.

## Classes de resultado

Agentes devem manter estes resultados distintos:

| Resultado | Significado |
| --- | --- |
| `Verdict::Pass` | O artefato válido satisfez a regra determinística |
| `Verdict::Fail` | O artefato válido foi avaliado e não satisfez a regra |
| Erro de input | Bytes, versão ou limites do artefato são inválidos |
| Erro operacional | Build, proving, rede, serviço ou integração falhou |
| Rejeição de segurança | Compromisso, autoridade, estado ou prova não coincide |

Panic, timeout do prover, erro de build ou falha de CPI nunca podem ser
convertidos em `Verdict::Fail`. Da mesma forma, `FAIL` não deve abortar o guest.

## Níveis de evidência e linguagem pública

| Evidência alcançada | Claim permitido |
| --- | --- |
| Testes do core | Regra e vetores passaram localmente |
| Guest compilado + receipt local verificada | RISC Zero executou o guest e a receipt foi verificada localmente |
| Formato Groth16 comprovado | A receipt/prova possui o formato demonstrado pelo comando registrado |
| Router/CPI com teste negativo e positivo | A prova foi verificada on-chain no ambiente e deployment registrados |
| Fallback atestado | A plataforma autorizou a liquidação; não é verificação ZK on-chain |

Não usar “o código está correto”, “trustless”, “qualquer repositório” ou “ZK
on-chain” além do nível de evidência realmente atingido.

## Matriz adversarial mínima

Cada camada futura deve acrescentar testes negativos antes de promover seu
gate. A matriz end-to-end mínima inclui:

- journal ou receipt de outro Job;
- `spec_hash`, `harness_hash`, `artifact_hash` ou `image_id` alterado;
- schema, tamanho ou ordinal de verdict inválido;
- `FAIL` produzido normalmente pelo guest;
- tentativa de release sem prova ou em estado inválido;
- executor, buyer, mint, amount ou destino divergente;
- refund antes do prazo;
- replay ou dupla liquidação;
- proof/seal incompatível e falha de CPI sem movimento de tokens.

Testes do exemplo upstream não substituem testes das invariantes VeriCode.

## Sequência de construção

1. Validar core, artefato restrito, hashes, wire format e vetores.
2. Compilar o guest real e produzir/verificar receipts locais de `PASS` e
   `FAIL`, sem dev mode.
3. Rejeitar adulterações de journal, ImageID e compromissos no host.
4. Especificar a política econômica e a máquina de estados antes do Anchor.
5. Implementar/testar escrow local, sem Router inicialmente.
6. Validar token mock e fluxo em devnet com evidência no Explorer.
7. Integrar Router/CPI com teste negativo antes do happy path.
8. Reproduzir o fluxo completo por CLI em ambiente limpo.
9. Adicionar worker/API e UI fina somente sobre operações reais já
   reproduzíveis.

O status de cada item deve vir dos relatórios de evidência, não desta lista.

## Protocolo de alteração

Antes de uma mudança:

- leia `AGENTS.md` e os documentos específicos do componente;
- confirme versões, locks e estado do working tree;
- declare arquivos, invariantes, riscos e comando de aceite;
- consulte somente fontes oficiais da versão pinada para integrações críticas;
- pare se API, contas, serialização ou deployment não puderem ser confirmados.

Ao implementar:

- faça uma mudança pequena e compilável;
- não duplique regra de negócio nem amplie o escopo para contornar bloqueio;
- não substitua receipt, Router, transferência ou verificação por mock;
- versione e documente alterações de schema junto com testes e consumidores;
- mantenha targets, caches, credenciais e segredos fora do Git.

Ao concluir, registre diff, comandos, saída real, evidência, falhas e riscos
abertos. “Deve funcionar” não é resultado de gate.

Mudanças em schema, hashing, guest, escrow ou CPI pedem revisão adversarial
separada, com foco em troca de Job/proof, replay, autoridade, destino de token,
timeout, serialização e claims maiores que a implementação.

## CLI, interface e demonstração

A CLI é a primeira superfície reproduzível. Worker e UI devem permanecer
camadas finas e mostrar, quando existirem:

- Job, valor de teste, prazo e estado;
- hashes, ImageID e verdict;
- receipt e resultado de verificação;
- transações reais e links do Explorer;
- progresso assíncrono e falhas sem simular sincronia;
- aviso de que a prova atesta a execução definida, não a qualidade geral do
  software.

O MVP só está pronto para demonstração quando outra pessoa puder reproduzir,
por comandos documentados, o fluxo real:

```text
criar Job -> financiar -> entregar artefato -> gerar e verificar receipt
-> PASS: liquidar segundo a política aprovada
ou
-> FAIL/timeout: seguir a política de refund aprovada
```

Se Router/CPI não estiverem validados, o fluxo alternativo deve ser rotulado
como fallback atestado e não pode ser apresentado como verificação ZK on-chain.
