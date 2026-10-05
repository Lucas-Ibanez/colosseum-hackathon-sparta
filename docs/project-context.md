# Contexto do projeto — índice e reconciliação

Este arquivo organiza os documentos de contexto fornecidos pelo responsável
humano e os liga ao estado real do repositório. Leia-o no início de toda
tarefa, depois de `AGENTS.md`.

## Documentos de contexto

| Documento | Papel | Natureza |
| --- | --- | --- |
| [`docs/context/guia-mvp-agentes-de-codigo.md`](context/guia-mvp-agentes-de-codigo.md) | Missão, escopo imutável, arquitetura, contrato de dados, máquina de estados, invariantes, gates M0–M7, protocolo de agentes | Norma de produto (fornecida em 2026-10-04, preservada sem alteração) |
| [`docs/context/sequencia-mvp.md`](context/sequencia-mvp.md) | Cronograma D0–D12 com evidência de saída por dia; entrega por volta de 8 out | Plano, não evidência |
| [`docs/mvp-agent-operating-guide.md`](mvp-agent-operating-guide.md) | Tradução operacional anterior do mesmo objetivo; limites de claims e matriz adversarial | Guia operacional interno |
| [`docs/handoff-protocol.md`](handoff-protocol.md) | Padrão obrigatório de encerramento de tarefa e prompt da próxima fase | Protocolo de trabalho |
| `docs/agent-control.md` | Objetivo, gate e estado atuais | Controle vivo |
| `docs/decisions.md`, `docs/evidence.md`, relatórios `docs/d*-results.md` | Decisões humanas e evidência executada | Registro |

## Ordem de precedência

Em conflito, vale a primeira fonte aplicável:

1. evidência executada e registrada no repositório ou na rede de teste;
2. documentação oficial da versão pinada;
3. princípios não negociáveis de `AGENTS.md`;
4. escopo, contrato de dados e invariantes do guia de produto
   (`docs/context/guia-mvp-agentes-de-codigo.md`);
5. decisões humanas registradas em `docs/decisions.md` — uma decisão
   posterior só prevalece sobre o guia se registrar explicitamente a
   divergência e o motivo;
6. contratos de arquitetura e schema (`docs/architecture.md`,
   `docs/manifest-schema.md`, `docs/escrow-state-machine.md`);
7. guia operacional interno (`docs/mvp-agent-operating-guide.md`);
8. sequência (`docs/context/sequencia-mvp.md`) e planos ainda não executados.

A sequência orienta prioridade e prazo; ela nunca autoriza pular um gate de
evidência nem um escopo proibido.

## Mapeamento guia → repositório

| Módulo do guia | Caminho no repositório | Estado |
| --- | --- | --- |
| `core-logic/` | `crates/vericode-core` | tipos, Borsh, SHA-256, harness restrito, `JournalV1`, política pura de escrow alinhada ao guia (D2b) e vinculada à entrega do executor, com termos admitidos e janela de prazo (D2b.1) |
| `zk-guest/` | `zkvm/methods/guest` | guest real; dois builds determinísticos; ImageID `4da06f90…fb1a` (antes do D2a) |
| `zk-host/` | `zkvm/host` | receipts locais reais PASS/FAIL `Composite`; compressão Groth16 demonstrada por harness fora do clone (D2d) |
| `anchor-program/` | `anchor/programs/vericode-escrow` (workspace `anchor/`, testes em `anchor/tests-local`) | D2c/D2b.1/D2e: `create_job` (termos da v1, janela de prazo), `fund`, `deliver`, `release`/`refund_on_fail` com CPI ao Verifier Router, `refund_on_timeout`; destino = ATA canônica; testado em processo com Router e verificador reais; sem deploy |
| `worker-api/` | inexistente | não iniciado |
| `frontend/` | inexistente | não iniciado |

Os nomes do guia descrevem responsabilidades; não renomear diretórios
existentes sem decisão registrada.

## Convenção de nomes de gate

Os IDs de gate usam o dia da sequência como prefixo e letras/números para
subdivisões: `D1c2b.3i` é uma subtarefa do dia D1; `D2a` é a primeira
subtarefa do dia D2. O dia é marco de escopo, não data de calendário.

## Estado da sequência (evidência, 2026-10-04)

Data de calendário: 2026-10-04, que corresponde ao dia D8 do cronograma. O
projeto concluiu tecnicamente o **D3** e o caminho forte local dos **D5/D6**
(D2e concluído); D4 (devnet) não começou.

| Dia | Situação | Evidência / lacuna |
| --- | --- | --- |
| D0 | técnico concluído; administrativo fora do repositório | `docs/toolchain-matrix.md`, `docs/router-notes.md`, `docs/d1a3-spike-results.md`; inscrição, outreach e pitch não são rastreados aqui |
| D1 | concluído (técnico) | crate e guest (D1c1–D1c2b); manifesto em `docs/manifest-schema.md`; Anchor skeleton e Perfil A (D2c); README corrigido (D2a.2) |
| D2 | concluído (técnico) | GATE 48H atendido: receipts locais reais PASS/FAIL (`docs/d1c2b3i-local-receipts-results.md`); máquina de estados testada em Rust puro (D2b); custódia SPL em vault PDA testada em processo (D2c); vídeo fora do repositório |
| D3 | concluído (local) | journal com job_id, artifact, harness, versão e verdict; provas PASS/FAIL; create/fund/refund por timeout (D2c); compromisso de entrega e termos admitidos da v1 (D2b.1); `release`/`refund_on_fail` vinculados ao Job e à entrega, com prova verificada por CPI ao Router (D2e); tudo em `solana-program-test` |
| D4 | não iniciado | escrow em devnet com Test USDC e Explorer |
| D5/D6 | concluído localmente (D2d, D2e) | receipts Groth16 PASS/FAIL reais verificadas pelo Verifier Router em `solana-program-test` (D2d) e por CPI a partir do `vericode_escrow`, com prova errada rejeitada antes do happy path (D2e); **falta** Router em devnet |
| D7–D12 | não iniciados | CLI E2E, estados ruins on-chain, worker e UI |

### Caminho crítico e risco de prazo

- O atraso é de cerca de seis dias de calendário. O guia (§3) manda cortar
  primeiro interface e extras, nunca receipt real, vínculo Job–journal,
  escrow básico, cenário negativo ou explicação de limites.
- Perfil A escolhido no D2c: Anchor `0.31.1` + Agave `2.3.9` + Rust
  `1.89.0`, em homes isoladas.
- Caminho forte comprovado em processo no D2d (Groth16 + Router). O D2c.1
  fechou o risco de freeze authority do mint e versionou as fixtures Groth16.
  Próximos passos:
  1. revisão adversarial R-D2 concluída: **REPROVADO** para o D2e
     (`docs/r-d2-adversarial-review-results.md`);
  2. D2b.1 concluído: liquidação vinculada ao artefato entregue e aos termos
     admitidos da v1 (`docs/d2b1-delivery-binding-results.md`);
  3. D2e concluído: `release`/`refund_on_fail` a partir de `Delivered`, com
     CPI ao Router e selector fixado (`docs/d2e-router-settlement-results.md`);
  4. revisão adversarial separada de D2b.1 + D2e
     (`docs/handoffs/d2e-to-r-d2e.md`);
  5. devnet, que exige confirmar o Program ID e o dono do Router ou implantar
     um Router próprio (a decidir), além de F-04 e F-05.
- Wallets de devnet: o agente está autorizado a criar keypairs efêmeros de
  devnet/localnet (D2a.2), sob as restrições do princípio 9 de `AGENTS.md`.

## Conflitos conhecidos e resolução

| Tema | Guia de produto | Repositório | Resolução |
| --- | --- | --- | --- |
| `artifact_hash` | registrado pelo programa apenas na liquidação; comparar job, spec, harness, ImageID | D2b registrava só na liquidação; o R-D2 (F-01) mostrou que isso permite refund por FAIL de artefato arbitrário | **resolvido (D2b.1), divergência decidida pelo humano:** compromisso de entrega assinado pelo executor (`deliver`) antes da liquidação; testado no core e no programa local |
| Refund em `Fail` | `Fail` válido devolve ao buyer | D2b: `refund_on_fail`, em qualquer slot | **resolvido (D2b)** |
| Timeout | refund somente após `deadline_slot`; antes falha | D2b: `current_slot > deadline_slot`, slot como entrada | **resolvido (D2b)** |
| Estados | `Draft, Funded, Proving, Submitted, Released, Refunded, Failed` | D2b: `Created, Funded, Released, Refunded`; `Proving/Submitted/Failed` são worker/UI; D2b.1 acrescenta `Delivered` (econômico, não terminal) | **resolvido (decisão humana D2b; `Delivered` por D2b.1)** |
| Release após o prazo | não definido | D2b: `Pass` só até `deadline_slot`, inclusive | **resolvido (decisão humana D2b)** |
| Wallets devnet | usar wallets efêmeras | `AGENTS.md` §9 proibia o agente de criar keypair | **resolvido (D2a.2):** agente pode criar keypairs efêmeros só de devnet/localnet, fora do clone, `0600`, sem exibir segredo; §9 atualizado |
| Perfil Anchor/Agave | não fixa versões | D2c: Anchor `0.31.1` + Agave `2.3.9` + Rust `1.89.0` (Agave `2.1.0` falha no `counter`) | **resolvido (D2c, autoridade D2a.2)** |
| Precedência | evidência > docs oficiais > guia > decisões | guia operacional antigo colocava decisões acima dos contratos | unificada acima |
