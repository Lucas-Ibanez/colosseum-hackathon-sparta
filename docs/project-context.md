# Contexto do projeto — índice e reconciliação

Este arquivo organiza os documentos de contexto fornecidos pelo responsável
humano e os liga ao estado real do repositório. Leia-o no início de toda
tarefa, depois de `AGENTS.md`.

## Documentos de contexto

| Documento | Papel | Natureza |
| --- | --- | --- |
| [`docs/context/guia-mvp-agentes-de-codigo.md`](context/guia-mvp-agentes-de-codigo.md) | Missão, escopo imutável, arquitetura, contrato de dados, máquina de estados, invariantes, gates M0–M7, protocolo de agentes | Norma de produto (fornecida em 2026-10-04, preservada sem alteração) |
| [`docs/context/sequencia-mvp.md`](context/sequencia-mvp.md) | Cronograma D0–D12 com evidência de saída por dia; datas indicativas (prazo real de entrega: 11 out, decisão humana de 2026-10-05) | Plano, não evidência |
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
| `zk-host/` | `zkvm/host`; `prover/` (D7) | receipts locais reais PASS/FAIL `Composite` (D1c2b); compressão Groth16 por harness fora do clone (D2d, D4a); desde o D7, `vericode-prover` no repositório prova o guest admitido versionado (`prover/artifacts`, `e09ba8cf…`) e comprime para Groth16 pelo Docker local por digest |
| `anchor-program/` | `anchor/programs/vericode-escrow` (workspace `anchor/`, testes em `anchor/tests-local`) | D2c/D2b.1/D2e/D4a: `create_job` (termos da v1, mint Test USDC admitido, janela de prazo), `fund`, `deliver`, `release`/`refund_on_fail` com CPI direta ao verificador Groth16 de `risc0-solana v3.0.0` (sem Router desde o D4a), `refund_on_timeout`; destino = ATA canônica; testado em processo com o verificador real (rebuild e bytes de devnet); **implantado e finalizado em devnet** (D4b: `GZqbL2Tb…`, authority `none`) |
| CLI (guia §3 e §10: "CLI como fonte de verdade") | `cli/` (D7) | `vericode`: `check`, `job create/deliver/settle/refund-timeout/show` em devnet; instruções iguais byte a byte aos builders da suíte; Jobs P e T liquidados em devnet (D7); Jobs P′ e T′ e quatro negativos em ambiente limpo (D9) |
| `worker-api/` | inexistente | não iniciado |
| `frontend/` | inexistente | não iniciado |

Os nomes do guia descrevem responsabilidades; não renomear diretórios
existentes sem decisão registrada.

## Convenção de nomes de gate

Os IDs de gate usam o dia da sequência como prefixo e letras/números para
subdivisões: `D1c2b.3i` é uma subtarefa do dia D1; `D2a` é a primeira
subtarefa do dia D2. O dia é marco de escopo, não data de calendário.

Na prática, a série D2a…D2e manteve o prefixo D2 mesmo quando o conteúdo já
era do D3, D5 e D6. Por isso, **a partir do D4, o ID do gate é o dia do plano
cujo conteúdo ele entrega** (subtarefas com letra, como `D4a`; revisões
adversariais com prefixo `R-`). O gate seguinte ao D4 é o `D7`, não "D5".

### Gate → dia do plano

| Gate | Conteúdo | Dia do plano |
| --- | --- | --- |
| D1a.2, D1a.3 | protocolo e spikes de toolchain; nota de compatibilidade; mapa de contas e ABI do Router | D0 |
| D1b0.1 | ambiente WSL, pacotes-base e clone | D0 |
| D1c1, D1c2a | crate compartilhada: tipos, `JournalV1` (job, spec, harness, artefato, ImageID, veredito), Borsh, SHA-256, harness restrito | D1, D3 (campos do journal) |
| D1c2b, D1c2b.1–.3h | guest RISC Zero, caches, vendor e builds determinísticos (ImageID `4da06f90…`) | D1 (guest mínimo) |
| D1c2b.3i, .3j | receipts locais reais PASS/FAIL e auditoria | D2 (GATE 48H), D3 (provas PASS e FAIL) |
| D2a, D2b | política pura de escrow e estados, testada em Rust puro | D1 (estados), D2 (testes de estados) |
| D2a.1, D2a.2 | contexto de produto, handoff, autorizações, README honesto | processo; D1 (README) |
| D2c | Perfil A; programa Anchor local: create, fund, refund por timeout, vault PDA | D1 (skeleton), D2 (custódia SPL), D3 (create/fund/refund local) |
| D2d | Groth16 e Verifier Router em processo | D3 (API do Router), D5 (adaptador do Router) |
| D2c.1 | mint sem freeze authority; fixtures Groth16 | D3 (endurecimento) |
| R-D2 | revisão adversarial (REPROVADO para o D2e original) | guia §11 |
| D2b.1 | compromisso de entrega e termos admitidos da v1 | D3 (vínculo Job–artefato), D8 (parte, local) |
| D2e | `release`/`refund_on_fail` com CPI ao Router; prova errada rejeitada | D6, D8 (parte, local) |
| R-D2e | revisão adversarial (APROVADO COM RESSALVAS para o D4) | guia §11 |
| D4a | reconhecimento do Router em devnet (reprovado); CPI direta ao verificador imutável; mint admitido; `JournalV1` v1 congelado; PoCs do R-D2e na suíte; receipts novas | D4 (parte local) |
| R-D4a | revisão delta somente leitura do D4a (APROVADO COM RESSALVAS, CD1 a CD9) | guia §11 |
| D4b | deploy em devnet, smoke, finalização, Jobs PASS/FAIL/timeout e negativos no Explorer | D4 e parte de D7/D8 (em devnet) |
| D7 | CLI e prover reproduzíveis no repositório; Job P (PASS) e T (timeout) em devnet pela CLI; invariante 9 em devnet; README de entrega e roteiro da demo; RD4A-07 (a)(e)(f) | D7, D8 (dupla liquidação em devnet), D9 (README e roteiro) |
| R-D7 | revisão adversarial final do MVP, somente leitura (APROVADO COM RESSALVAS, CR1 a CR8) | guia §11 |
| D9 | demo em ambiente limpo (W1–W8 em devnet), congelamento dos claims, erratas da CR1, roteiro de gravação; vídeo gravado pelo humano | D9 |
| D10a (próximo) | endurecimento da CLI e do prover (RD7-01, 02, 03, 04, 07), com revisão delta curta | preparação de D10–D12 |
| D10–D12 | worker de prova e telas Buyer, Submit e Result (CR8) | D10–D12 |

Itens de produto e mercado do plano (inscrição, outreach, design partners,
pitch, telas, vídeo) não são rastreados neste repositório.

## Estado da sequência (evidência, 2026-10-05)

Data de calendário: 2026-10-05, que corresponde ao dia D9 do cronograma. O
projeto concluiu tecnicamente o **D3** e o caminho forte local dos **D5/D6**
(D2e concluído). O **D4** também está concluído:
- a parte local (D4a) foi aprovada com ressalvas pelo R-D4a;
- a parte em devnet (D4b) implantou e finalizou o escrow e liquidou Jobs
  PASS, FAIL e timeout com negativos.

O **D7** também está concluído: CLI (`cli/`) e prover (`prover/`) no
repositório, Job P liquidado por PASS e Job T por timeout em devnet pela
CLI, dupla liquidação rejeitada em devnet (6007/6008), README de entrega e
roteiro da demo (`docs/d7-cli-results.md`). A revisão adversarial final
**R-D7** aprovou com ressalvas (CR1 a CR8, `docs/r-d7-review-results.md`).

O **D9** também está concluído (`docs/d9-demo-results.md`):
- o fluxo rodou pela CLI e pelo prover num ambiente limpo (clone e targets
  novos, toolchains isoladas copiadas): P′ `Released` com o verificador
  invocado e T′ `RefundedOnTimeout`;
- negativos 6021, 6014, `verifier:6003` (seal adulterado, pela CLI) e 6007;
- claims congelados e erratas da CR1;
- seção "Gravação" pronta para o humano.

O próximo gate é o **D10a**.

| Dia | Situação | Evidência / lacuna |
| --- | --- | --- |
| D0 | técnico concluído; administrativo fora do repositório | `docs/toolchain-matrix.md`, `docs/router-notes.md`, `docs/d1a3-spike-results.md`; inscrição, outreach e pitch não são rastreados aqui |
| D1 | concluído (técnico) | crate e guest (D1c1–D1c2b); manifesto em `docs/manifest-schema.md`; Anchor skeleton e Perfil A (D2c); README corrigido (D2a.2) |
| D2 | concluído (técnico) | GATE 48H atendido: receipts locais reais PASS/FAIL (`docs/d1c2b3i-local-receipts-results.md`); máquina de estados testada em Rust puro (D2b); custódia SPL em vault PDA testada em processo (D2c); vídeo fora do repositório |
| D3 | concluído (local) | journal com job_id, artifact, harness, versão e verdict; provas PASS/FAIL; create/fund/refund por timeout (D2c); compromisso de entrega e termos admitidos da v1 (D2b.1); `release`/`refund_on_fail` vinculados ao Job e à entrega, com prova verificada por CPI ao Router (D2e); tudo em `solana-program-test` |
| D4 | **concluído**: D4a (local, aprovado com ressalvas pelo R-D4a) e D4b (devnet) | Router upstream de devnet inutilizável; o escrow chama direto o verificador Groth16 imutável; mint admitido; `JournalV1` v1 congelado (`docs/d4a-direct-verifier-results.md`). D4b: escrow `GZqbL2Tb…` implantado com `cdf6967f…` e authority `none`; mint Test USDC criado; Jobs S (smoke), A `Released`, B `RefundedOnFail`, C `RefundedOnTimeout`; negativos e C7 no Explorer (`docs/d4b-devnet-results.md`) |
| D5/D6 | concluído localmente (D2d, D2e, D4a) | receipts Groth16 PASS/FAIL reais verificadas pelo Verifier Router em `solana-program-test` (D2d) e por CPI a partir do `vericode_escrow` (D2e via Router; D4a direto ao verificador, também com os bytes de devnet), com prova errada rejeitada antes do happy path; **em devnet desde o D4b**: liquidações PASS/FAIL verificadas por CPI ao verificador imutável e seals errados rejeitados (6000/6003) |
| D7 | **concluído** | `cli/` e `prover/` reproduzíveis com `--locked` (sem crate novo); Job P: `create_job`+`fund`, prova `(21,42)` nova, `deliver`+`release` com CPI ao verificador (`4oWhwZfU…`); Job T: 6021 antes do prazo e `RefundedOnTimeout` (`3fiNWgTW…`); journal de outro Job → 6014 (`docs/d7-cli-results.md`) |
| D8 | concluído em devnet (D4b + D7) | prova errada (6000/6003), journal de outro Job ou artefato (6014/6017), FAIL (refund), timeout antecipado (6021) e **dupla liquidação** (6007/6008, D7) |
| D9 | **concluído** (técnico); vídeo de reserva a gravar pelo humano | demo em ambiente limpo pela CLI (CR4 (a), não máquina nova): core 42/42 ×2, CLI 14/14, suíte 61/61, prover 4/4; P′ `Released` (`5tjezXYh…`, 99.541 CU no verificador), T′ `RefundedOnTimeout` (`33ezPvow…`), negativos 6021/6014/`verifier:6003`/6007; frases permitidas congeladas no `README.md` e no roteiro; seção "Gravação" em `docs/demo-script.md` (`docs/d9-demo-results.md`) |
| D10a | próximo | endurecimento da CLI e do prover (`docs/handoffs/d9-to-d10a.md`) |
| D10–D12 | não iniciado; planejado depois do D10a | worker e UI fina (CR8) |

### Caminho crítico e risco de prazo

- Prazo de entrega: **11/10** (decisão humana de 2026-10-05). As datas da
  sequência são indicativas; vale a ordem dos gates. O guia (§3) manda cortar
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
  4. revisão adversarial R-D2e concluída: **APROVADO COM RESSALVAS** para o
     D4, com condições C1 a C7
     (`docs/r-d2e-adversarial-review-results.md`);
  5. D4a concluído: Router upstream reprovado em devnet; CPI direta ao
     verificador Groth16 imutável e mint admitido (decisão humana), com
     `JournalV1` v1 congelado e PoCs do R-D2e na suíte
     (`docs/d4a-direct-verifier-results.md`). R-D4a concluído: **APROVADO
     COM RESSALVAS** para o D4b, com condições CD1 a CD9
     (`docs/r-d4a-review-results.md`). D4b concluído:
     - escrow implantado e finalizado em devnet;
     - Jobs PASS, FAIL e timeout e negativos no Explorer
       (`docs/d4b-devnet-results.md`);
     - D7 concluído: CLI e prover no repositório, Jobs P e T em devnet,
       invariante 9 em devnet, README e roteiro (`docs/d7-cli-results.md`);
     - R-D7 concluído: **APROVADO COM RESSALVAS** para o D9 e o D10–D12
       (`docs/r-d7-review-results.md`);
  6. D9 concluído: demo em ambiente limpo, claims congelados e roteiro de
     gravação (`docs/d9-demo-results.md`); vídeo de reserva gravado pelo
     humano, pela seção "Gravação" de `docs/demo-script.md`;
  7. D10a: endurecimento da CLI e do prover (RD7-01, 02, 03, 04, 07) e
     revisão delta curta;
  8. D10–D12: worker e telas finas (CR8); revisão curta da interface; vídeo
     definitivo e submissão até 11/10.
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
