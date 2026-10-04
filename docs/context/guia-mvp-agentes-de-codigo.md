<!--
Fonte: documento fornecido pelo responsável humano em 2026-10-04
(`guia_mvp_para_agentes_de_codigo.md`). Conteúdo preservado sem alteração
abaixo deste comentário. Precedência, mapeamento para o repositório e
conflitos conhecidos: `docs/project-context.md`.
-->

# Guia do MVP para Agentes de Código

> Documento operacional. Use este arquivo para planejar, implementar, revisar e validar mudanças no MVP. Ele não é um pitch nem substitui a documentação oficial das dependências fixadas no repositório.

## 1. Missão do MVP

Construir uma demonstração end-to-end, reproduzível e honesta de **aceite verificável de uma tarefa determinística**:

1. o comprador cria um job e deposita um token de teste em escrow;
2. o executor entrega um artefato restrito;
3. um guest RISC Zero avalia os bytes desse artefato contra um avaliador determinístico previamente comprometido;
4. a receipt publica um journal com os compromissos e um veredito `Pass` ou `Fail`;
5. o programa Anchor só liquida o escrow depois de conferir a ligação entre job e journal e, no caminho forte, de obter verificação on-chain pelo Verifier Router;
6. `Pass` libera o valor ao executor; `Fail` ou timeout devolve o valor ao comprador.

O MVP serve a dois objetivos distintos:

- **Hackathon / viabilidade técnica:** provar que o fluxo pode ligar evidência criptográfica, estado on-chain e liquidação em Solana.
- **Aprendizado de produto:** gerar uma base para comparar, em pilotos futuros, evidência de alto assurance (zkVM) com mecanismos mais simples, como CI assinado e revisão humana.

Uma demo funcional **não** prova product-market fit. Não implementar funcionalidades de plataforma, marketplace ou reputação para compensar essa lacuna.

## 2. O que queremos provar — e o que não queremos provar

### Afirmação permitida

> Um avaliador determinístico previamente comprometido executou sobre o artefato vinculado ao job e produziu o veredito publicado; quando a verificação on-chain é bem-sucedida, esse veredito aciona o escrow sem decisão discricionária da interface.

### Fora da afirmação

O MVP não prova:

- que o código é universalmente correto, seguro ou livre de bugs;
- que a suíte/harness cobre todos os requisitos relevantes;
- autoria do artefato por um agente ou pessoa específica;
- qualidade arquitetural, manutenção ou valor econômico do trabalho;
- privacidade total dos inputs perante quem gera a prova;
- ausência de toda confiança residual;
- reputação Sybil-proof;
- suporte a qualquer repositório, patch ou linguagem.

Não usar, em código, README, UI ou demo, frases como “o código está correto”, “ninguém precisa confiar em ninguém”, “qualquer repositório” ou “ZK on-chain” se a receipt não tiver sido verificada on-chain antes da liquidação exibida.

## 3. Escopo imutável da v1

### Entra

- Uma classe única de job: artefato Rust pequeno **ou** formato restrito/serializável equivalente.
- Regra de negócio pura e determinística, sem rede, relógio, RPC ou I/O externo.
- Harness fixo para a v1, pré-comprometido e coerente com o guest/ImageID aceito.
- Receipts reais de `Pass` e `Fail`, verificadas localmente.
- Escrow Anchor em devnet, com token SPL mock `D-USDC`/`Test USDC`, vault controlado por PDA, prazo, release e refund.
- CLI como fonte de verdade; worker/API e UI apenas como camadas finas de orquestração e visualização.
- Pelo menos um caso adversarial demonstrável.

### Não entra

- Mainnet, dinheiro real, custódia comercial ou dados de clientes.
- Marketplace, catálogo de agentes, matching, chat, multiagente ou login sofisticado.
- Score, grafo anti-Sybil, token ou reputação como produto.
- Repositórios arbitrários, compilação de código arbitrário na zkVM ou múltiplas linguagens.
- Testes secretos bilaterais, fair exchange, execução confidencial perfeita ou MPC/TEE como requisito.
- LLM/Jev como juiz de uma condição de pagamento.
- Painel enterprise, indexador completo, múltiplos rails de pagamento ou abstrações prematuras.

Se o prazo apertar, cortar primeiro itens de interface e todas as funcionalidades acima. Nunca cortar receipt real, vínculo job–journal, escrow básico, cenário negativo e explicação do limite da prova.

## 4. Arquitetura obrigatória

```text
core-logic/       tipos canônicos, serialização, hash e evaluate()
zk-guest/         lê inputs, calcula artifact_hash, chama evaluate(), publica JournalV1
zk-host/          prepara inputs, gera/verifica receipt e decodifica journal
anchor-program/   custódia, invariantes, vínculo job–journal, Router e liquidação
worker-api/       orquestra proving e expõe estado; não decide economia
frontend/         inicia operações reais e exibe estado/evidências; não decide resultado
```

### Separação de ambientes

- `anchor-program` compila para **sBPF**.
- `zk-guest` executa como **ELF RISC-V de 32 bits** na zkVM.
- Portanto, nunca tentar executar uma instrução Anchor dentro da zkVM nem compartilhar binário entre os dois ambientes.
- O compartilhamento permitido é a crate Rust pura: tipos, serialização, hash, fixtures e `evaluate()`.

### Responsabilidades por módulo

| Módulo | Deve fazer | Não deve fazer |
|---|---|---|
| `core-logic` | Tipos canônicos, regra determinística, fixtures e testes | Rede, relógio, filesystem, RPC ou dependência de Solana/Anchor |
| `zk-guest` | Avaliar bytes, calcular hashes e publicar `JournalV1` | Decidir pagamento, usar `panic` para `Fail` ou esconder inputs/erros |
| `zk-host` | Preparar entradas, gerar receipt e verificar localmente | Ser tratado como fonte confiável apenas porque gerou a proof |
| `anchor-program` | Validar contas/estado, custodiar token, chamar Router e liquidar atomicamente | Rodar guest, confiar no front-end ou permitir destino de token livre |
| `worker-api` | Orquestrar proving e apresentar erros/estado assíncrono | Guardar segredo no cliente ou criar regra econômica nova |
| `frontend` | Mostrar job, hashes, receipt e links de transação | Decidir `Pass`, refund, destino de tokens ou simular verificação |

## 5. Contrato de dados e compromissos

### Dados fixados ao criar o job

O comprador cria e financia o job com, no mínimo:

```text
job_id              bytes canônicos e únicos
buyer               autoridade que financia
executor            destinatário fixo em caso de Pass
mint                mint SPL mock aceita pelo programa
amount              valor exato do escrow
deadline_slot       prazo on-chain
spec_hash           hash da especificação/critério aceito antes do depósito
harness_hash        hash do avaliador fixo da v1
expected_image_id   guest/versionamento admitido
status              estado inicial da máquina de estados
```

O comprador **não** fixa `artifact_hash` antes da entrega. O guest calcula esse hash sobre os bytes efetivamente avaliados e o programa o registra apenas na liquidação.

### `JournalV1` mínimo

O journal é um contrato de segurança, não um payload decorativo. Ele deve conter serialização canônica e versão explícita:

```text
schema_version
job_id
spec_hash
harness_hash
artifact_hash        calculado dentro do guest sobre os bytes avaliados
image_id ou versão admitida
verdict              Pass | Fail
```

Campos operacionais, como versão do compilador/executor e métricas estritamente necessárias, podem ser adicionados somente se tiverem consumidor e regra de compatibilidade definidos.

### Regras de serialização e hash

- Escolher **um** algoritmo de hash e uma codificação canônica para a v1.
- Preferir hash dos bytes crus do artefato e de estruturas binárias canônicas; não usar JSON livre como compromisso econômico.
- Versionar qualquer schema que seja desserializado on-chain.
- O guest calcula `artifact_hash`; o host não pode apenas declará-lo.
- O programa compara `job_id`, `spec_hash`, `harness_hash` e `ImageID` esperados com o journal/receipt aceito.
- Qualquer mismatch deve reverter antes de movimentar tokens.

## 6. Regra determinística e veredito

O núcleo deve expor uma função simples, testável e livre de ambiente externo:

```rust
evaluate(artifact, job_spec) -> Verdict
```

Critérios:

- mesma entrada + mesmo `JobSpec` produzem o mesmo `Verdict`;
- existem fixtures pequenas e explícitas para `Pass` e `Fail`;
- `Fail` é um resultado econômico normal: o guest publica `Verdict::Fail` e termina normalmente;
- `panic`, `assert` abortivo ou erro operacional não equivalem a `Fail` e não substituem a receipt válida de refund;
- o primeiro artefato deve ser fácil de explicar em 30 segundos e impossível de confundir com “execução de patch arbitrário”.

## 7. Máquina de estados e invariantes de escrow

### Estados

```text
Draft -> Funded -> Proving -> Submitted -> Released
                                \-> Refunded

Funded/Proving/Failed -> Refunded por timeout válido
Failed = erro operacional; não é Verdict::Fail
Released e Refunded são terminais
```

Estados de UI/worker podem ser mais detalhados, mas não podem criar transições econômicas fora do programa Anchor.

### Invariantes não negociáveis

1. O vault é uma token account controlada pelo PDA do job; não existe chave privada do vault.
2. O mint é validado; buyer, executor, amount e deadline são os do job armazenado.
3. Em `Pass` válido, tokens vão somente ao executor previamente registrado.
4. Em `Fail` válido, tokens voltam somente ao buyer do job.
5. Refund por timeout só ocorre após `deadline_slot`; antes disso deve falhar.
6. Mudança de status e transferência de tokens ocorrem na mesma instrução/transação atômica.
7. Não existe administrador capaz de sacar o vault ou escolher qualquer destino de token.
8. Uma receipt/journal de outro job, outro spec, outro harness ou outro ImageID não pode liquidar o job atual.
9. Replay e dupla liquidação falham porque estados terminais não aceitam nova settlement.
10. Falha de CPI/verificação reverte a instrução inteira; nenhum token move.

## 8. Integração RISC Zero e Verifier Router

Esta integração é o maior risco técnico. Não inferir API, contas ou formato de proof de exemplos desatualizados ou da branch `main`.

Antes de qualquer patch de Router, o agente deve identificar e registrar:

1. release/tag exata de `risc0-solana`;
2. versão compatível de RISC Zero e do guest;
3. exemplo oficial usado como referência;
4. formato Groth16/receipt esperado;
5. contas exigidas pela CPI e programa Router;
6. ambiente reproduzível: localnet e, se confirmado, devnet;
7. comando de build, geração, verificação e teste negativo.

### Caminho forte

O programa Anchor:

1. monta/valida o journal esperado a partir do `Job`;
2. chama a verificação compatível pelo Router;
3. só depois de retorno bem-sucedido transfere tokens e muda o status.

Teste a rejeição de proof, journal ou `ImageID` incompatível **antes** do happy path.

### Fallback honesto

Se o Router não fechar no ambiente disponível, preservar:

- receipt real verificada localmente;
- escrow em devnet com fluxo atestado explicitamente como centralizado/plataforma;
- diagnóstico reproduzível do bloqueio.

Nesse caso, não usar “ZK on-chain”, não apresentar a receipt local como autorização on-chain e não esconder a diferença na UI ou no vídeo.

## 9. Gates de aceite do Hackathon

Não avançar para a camada seguinte apenas porque o código parece plausível. Cada gate precisa de comando executado, resultado real e registro curto de evidência.

| Marco | Está pronto quando |
|---|---|
| **M0 — Escopo** | Artefato restrito, harness fixo, schema inicial e rota real do Router foram definidos; não há promessa de repo arbitrário. |
| **M1 — Core** | `core-logic` tem `evaluate()` determinística, fixtures `Pass`/`Fail` e testes locais rápidos. |
| **M2 — Receipt** | Há receipts reais de `Pass` e `Fail`, com journal decodificado e `ImageID` verificado localmente. |
| **M3 — Compromissos** | Job e journal vinculam job, spec, harness, artefato, versão e verdict; adulterações são rejeitadas. |
| **M4 — Dinheiro** | Escrow em devnet usa PDA e token mock; depósito, `Pass`/release, `Fail`/refund e timeout respeitam invariantes. |
| **M5 — Router** | Proof incompatível falha; proof válida libera via caminho on-chain. Se não houver integração, fallback está rotulado e não recebe claim ZK on-chain. |
| **M6 — Demo** | CLI reproduz o fluxo em ambiente limpo; UI é fina, mostra Explorer e inclui cenário negativo. |
| **M7 — Credibilidade** | README contém versões, comandos, hashes, limitações e evidências; claims coincidem com o que foi executado. |

### Matriz mínima de testes adversariais

- liberar sem proof ou em estado errado;
- usar proof/journal de outro job;
- trocar `spec_hash`, `harness_hash` ou `ImageID`;
- tentar `Pass` com destinatário diferente;
- tentar refund antes do prazo;
- tentar dupla liquidação;
- enviar `Fail` válido e verificar refund;
- submeter proof incompatível e verificar reversão sem movimento de tokens.

## 10. Interface, CLI e evidências de demo

### Prioridade

O fluxo por CLI deve funcionar antes da UI. A interface existe para tornar a evidência assistível, não para esconder etapas manuais nem introduzir regra de negócio.

### A UI mínima deve mostrar

- job, valor de teste, prazo e estado;
- `spec_hash`, `harness_hash`, `artifact_hash`, `ImageID` e veredito quando disponíveis;
- receipt/resultados que possam ser conferidos;
- transação de depósito e settlement/refund no Explorer;
- estados assíncronos claros: `Draft`, `Funded`, `Proving`, `Submitted`, `Released`, `Refunded` e `Failed`;
- frase visível de limitação: a prova atesta a execução definida, não qualidade geral do software.

Nunca pré-gerar ou trocar receipt sem tornar explícito que ela pertence ao job exibido. Se proving for lento, exibir `Proving`; não fingir sincronia.

## 11. Protocolo obrigatório para agentes de código

### Antes de alterar código

1. Ler este guia, o estado atual do repositório e os arquivos do módulo alvo.
2. Para dependência incomum (Router, Groth16, Anchor/SPL, integração de carteira), consultar documentação oficial da **versão pinada**.
3. Produzir um plano curto com: versões assumidas, arquivos a mudar, risco, invariantes preservadas e comando de aceitação.
4. Se uma API, conta ou formato não puder ser confirmado, parar e relatar a lacuna. Não completar com pseudocódigo apresentado como integração funcional.

### Ao implementar

- Fazer uma mudança pequena, compilável e focada por vez.
- Manter um único owner por módulo enquanto uma mudança está em andamento.
- Não ampliar escopo para resolver um bloqueio técnico.
- Não inserir mock no caminho crítico para simular receipt, Router, transferência ou verificação.
- Não alterar o schema/journal silenciosamente; atualizar versão, testes e consumidores juntos.
- Não modificar decisões de produto, promessas de marketing, chaves, wallets ou configuração sensível sem decisão humana explícita.

### Ao concluir

Retornar sempre:

1. arquivos modificados;
2. resumo do diff e das invariantes preservadas;
3. comandos realmente executados;
4. saída resumida real de build/test/proof/transação;
5. falhas, hipóteses não confirmadas e próximo gate bloqueado, se houver.

“O código foi gerado” ou “deve funcionar” não é evidência de conclusão.

### Revisão adversarial obrigatória para mudanças críticas

Uma segunda pessoa/agente revisa especificamente:

- job/proof swap;
- replay;
- dupla liquidação;
- autoridade fraca;
- destino de token controlável pelo chamador;
- timeout injusto;
- mismatch de serialização/hash;
- `Fail` abortando em vez de produzir receipt;
- claim de produto maior que a implementação.

## 12. Segurança operacional

- Usar wallets efêmeras de devnet para buyer, executor e deployer/payer.
- Manter private keys, seed phrases, tokens, `.env` e dados sensíveis fora do repositório e fora de prompts de agentes.
- Nunca criar/usar mainnet ou dinheiro real no escopo do MVP.
- Registrar por gate: versões, `ImageID`, hashes, comando, resultado e links do Explorer.
- Fixar dependências e comandos reprodutíveis antes da gravação final; a branch `main` de dependências externas não é uma especificação.

## 13. Ordem de trabalho recomendada

1. Fixar artefato restrito, `JobSpec`, `JournalV1`, algoritmo de hash e versões.
2. Implementar/testar `core-logic` com fixtures `Pass` e `Fail`.
3. Gerar e verificar receipts locais reais para os dois vereditos.
4. Ligar commitments e criar testes de mismatch.
5. Implementar e testar escrow local; depois devnet com token mock e Explorer.
6. Integrar Router com teste negativo primeiro.
7. Reproduzir o caminho completo por CLI em ambiente limpo.
8. Só então adicionar worker/API e UI mínima sobre esse mesmo fluxo.
9. Congelar versões, schema e claims; executar a matriz adversarial e preparar demo/vídeo.

## 14. Critério final de entrega

O MVP está pronto para demonstração somente quando for possível executar, sem intervenção escondida:

```text
criar job -> financiar -> entregar artefato -> gerar receipt -> submeter/verificar
-> Pass: liberar Test USDC ao executor
ou
-> Fail/timeout: devolver Test USDC ao comprador
```

O repositório deve permitir que outra pessoa confira esse fluxo por comandos documentados e, quando o caminho forte estiver ativo, pelas transações correspondentes. A qualidade da demo é definida pela evidência reproduzível e pelos limites declarados, não pela quantidade de telas ou funcionalidades.

## 15. Referências de decisão

Este guia consolida os documentos internos de plano revisado do MVP, guia de estudo aplicado, tese geral e análise cruzada de mercado. Em caso de conflito:

1. evidência executada no repositório e na rede de teste;
2. documentação oficial da versão pinada;
3. invariantes e escopo deste guia;
4. decisões humanas registradas no projeto.

Quando houver conflito entre uma hipótese de documento e o comportamento da ferramenta oficial, registrar o diagnóstico e escalar; não improvisar uma integração.
