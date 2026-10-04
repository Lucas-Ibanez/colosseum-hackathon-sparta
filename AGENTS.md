# VeriCode — regras compartilhadas

## Escopo do MVP

O VeriCode verifica somente um artefato serializado restrito com regra determinística e harness fixo. O fluxo alvo usa escrow de Test USDC em Solana devnet e, quando comprovadamente viável, uma receipt RISC Zero.

## Princípios não negociáveis

1. O MVP não verifica repositórios arbitrários nem patches gerais; verifica um artefato serializado restrito por regra determinística e harness fixo.
2. A lógica de negócio vive em uma crate Rust pura, sem dependência de Solana, Anchor ou RISC Zero.
3. O guest RISC Zero e Anchor compilam para alvos incompatíveis; código compartilhado não depende de nenhum dos dois ambientes.
4. `Verdict::Fail` é saída normal e verificável do guest, nunca `panic` ou `assert`.
5. A prova expõe somente um journal/manifest público contendo, no mínimo: `schema_version`, `job_id`, `spec_hash`, `harness_hash`, `artifact_hash`, `image_id` e `verdict`.
6. O contrato só libera fundos para o executor e mint definidos no Job, após prova e journal correspondentes ao Job.
7. Não existe admin bypass.
8. Sem prova Router/CPI realmente validada, não alegar verificação ZK on-chain. Atestado de plataforma é fallback explicitamente nomeado, nunca prova ZK.
9. Nunca criar, imprimir, versionar ou solicitar seed phrase, keypair, chave privada ou `.env` real.

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
