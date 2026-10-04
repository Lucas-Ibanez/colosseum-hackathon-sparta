# Protocolo de handoff entre tarefas

## Regra

Toda tarefa concluída, inclusive as somente documentais, termina com um
**prompt completo para a próxima fase**:

1. salvo em `docs/handoffs/<gate-concluído>-to-<próximo-gate>.md` e incluído
   no commit da tarefa, quando houver commit autorizado;
2. reproduzido integralmente na mensagem final ao humano.

Se a tarefa terminou bloqueada (`BLOQUEADO` ou `AGUARDANDO_AUTORIZAÇÃO`), o
prompt descreve a decisão humana necessária e a tarefa que ela libera; nunca
propõe contornar o bloqueio.

O prompt deve funcionar sozinho para um agente sem memória da sessão anterior,
mas não deve copiar documentos inteiros: ele referencia arquivos e diz quais
ler integralmente.

## Escolha de modelo e esforço

O agente recomenda; o humano seleciona o modelo e o esforço na ferramenta. O
agente não deve alegar que trocou de modelo.

| Classe da tarefa | Modelo recomendado | Esforço |
| --- | --- | --- |
| Registro documental, índices, tabelas de evidência | Sonnet 5.5 | medium |
| Lógica pura do core sem efeito econômico | Sonnet 5.5 ou Opus 5.5 | high |
| Política econômica, escrow, schema, journal, hashing | Opus 5.5 | high |
| Anchor, SPL, Router/CPI, devnet, autoridade e contas | Opus 5.5 | xhigh |
| Auditoria adversarial final de um marco | Opus 5.5, sessão separada e somente leitura | max |

## Modelo de prompt

Copie a estrutura abaixo, preenchendo todas as seções. Seções sem conteúdo
recebem "não se aplica" com o motivo.

```markdown
# <GATE> — <título curto>

## Identificação
- Gate: <ID>  ·  Dia da sequência: <Dn>  ·  Marcos do guia: <M1…M7>
- Repositório: /home/lucas/src/vericode (WSL), branch main
- Gate anterior: <ID>, commit <hash curto>, relatório <arquivo>

## Modelo e modo
- Modelo/esforço recomendados: <modelo>, <esforço> — <motivo>
- Plan Mode: <obrigatório/opcional> — <motivo>

## Leitura obrigatória (integral, antes de editar)
- AGENTS.md, CLAUDE.md, docs/project-context.md, docs/handoff-protocol.md
- docs/agent-control.md
- <documentos e código específicos do gate>

## Preflight
- pwd; raiz Git; branch; HEAD (esperado <hash>); git status --short (esperado <estado>);
  git diff --check
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou stash.

## Checagem da tarefa anterior
- Commits esperados: <lista>
- Reexecutar: <comandos exatos>; saída esperada: <ex.: 37 passed>
- Hashes esperados: <locks/arquivos>
- Se divergir: parar, registrar e reportar; não corrigir silenciosamente.

## Objetivo
<uma frase>

## Decisões já tomadas
- <decisão> — fonte: <arquivo/entrada>

## Decisões a confirmar ou pendentes
- <decisão> — se não confirmada no Plan Mode: AGUARDANDO_AUTORIZAÇÃO

## Escopo autorizado
- <arquivos que podem ser criados/alterados>

## Fora de escopo / proibido
- <lista explícita, incluindo rede, instalação, wallet, deploy etc.>

## Implementação esperada
- <tipos, funções, estados, documentos>

## Testes obrigatórios
- <um item por comportamento e cenário adversarial>
- Comandos: <comandos exatos com ambiente isolado>

## Evidências exigidas
- Relatório: docs/<gate>-results.md com comandos e saídas reais
- Atualizar: docs/decisions.md, docs/evidence.md, docs/agent-control.md
  e docs/project-context.md (estado da sequência), quando aplicável

## Critério de pronto
- <condições verificáveis>
- git diff --check exit 0; diff integral revisado; busca de segredos limpa

## Condições de parada
- <quando parar em BLOQUEADO / AGUARDANDO_AUTORIZAÇÃO>

## Commit
- <autorizado ou não; mensagem sugerida; push sempre proibido salvo autorização>

## Relatório final
1. arquivos modificados; 2. testes e saídas reais; 3. invariantes;
4. decisões pendentes; 5. riscos; 6. confirmação de fronteiras;
7. prompt da próxima fase, segundo docs/handoff-protocol.md
```

## Checklist antes de entregar o prompt

- O HEAD e as saídas esperadas citados no prompt são os reais após o commit.
- Toda decisão citada aponta para o arquivo ou entrada que a registra.
- Nenhum segredo, keypair, seed ou valor de `.env` aparece no prompt.
- O prompt não alega capacidade ainda não evidenciada (Groth16, Router, CPI,
  devnet, "ZK on-chain").
