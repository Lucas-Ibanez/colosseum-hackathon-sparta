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

## Protocolo de trabalho

- Antes de qualquer trabalho D1 automatizado, ler `docs/agent-control.md`.
- Executar uma tarefa pequena por vez: planejar, alterar, testar, registrar evidência real e revisar o diff.
- Não substituir falha por mock nem apresentar dev mode, stub ou fallback como sucesso real.
- Não alterar schema ou journal sem atualizar explicitamente a documentação e registrar a decisão.
- Segredos nunca entram no Git; usar somente nomes de variáveis e exemplos vazios.
- Antes de alterar zkVM, Router, schema ou contrato, ler `docs/architecture.md`, `docs/manifest-schema.md`, `docs/zkvm-notes.md`, `docs/router-notes.md` e `docs/decisions.md`.
- Antes de alterar escrow, política econômica, CLI/UI ou claims públicos, ler `docs/mvp-agent-operating-guide.md`.

## Definição de pronto

Entregar diff revisado, comando executado, saída real, evidência registrada e riscos abertos. Falhas permanecem visíveis.
