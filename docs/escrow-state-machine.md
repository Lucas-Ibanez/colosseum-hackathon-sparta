# Máquina de estados do escrow — política pura D2a

**Política pura em `crates/vericode-core/src/escrow.rs`. Não é programa
Anchor, não custodia fundos, não move tokens e não verifica prova.**

> **Revisão pendente (D2a.1, 2026-10-04):** este documento descreve a
> implementação D2a. O guia de produto prevalece em três pontos que o D2b
> deve implementar: `artifact_hash` registrado apenas na liquidação (sem
> pré-registro `Delivered`), refund ao buyer em `Fail` válido e refund por
> timeout somente após `deadline_slot`. Ver `docs/project-context.md`.

Este documento descreve a política implementada no D2a. Ela é a referência
que um futuro programa on-chain deverá aplicar; até esse programa existir e
ser testado, as invariantes on-chain do guia operacional continuam requisitos,
não capacidades. Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Pré-condição de prova

`check_release` e `release` recebem um `JournalV1` já decodificado. A política
**presume** que um adaptador futuro verificou a receipt correspondente contra
o ImageID do Job antes da chamada. Este módulo não verifica receipt, seal,
Groth16, Router ou CPI e não deve ser citado como verificação ZK.

## Tipos

| Tipo | Conteúdo | Validação |
| --- | --- | --- |
| `BuyerId`, `ExecutorId`, `MintId` | 32 bytes exatos, tipos distintos | identidade toda-zero rejeitada em `JobV1::new` |
| `Amount` | `u64` em unidades base do mint | zero rejeitado (`AmountError::Zero`) |
| `JobV1` | `job_id`, `buyer`, `executor`, `mint`, `amount`, `spec_hash`, `harness_hash`, `image_id` | campos privados, somente leitura; `buyer == executor` rejeitado |
| `EscrowState` | `Created`, `Funded`, `Delivered { artifact_hash }`, `Released` | `Released` é o único estado terminal |
| `Payout` | `recipient`, `mint`, `amount` | sempre copiado do Job, nunca do pedido |

`JobV1` não possui serialização nem layout de conta: isso pertence ao gate
Anchor. Não há campo de prazo porque a política de prazo está pendente.

## Transições

As funções são métodos de `JobV1`; recebem o estado por valor e devolvem o
novo estado ou um erro explícito, sem efeito colateral.

| Transição | Origem aceita | Exigências | Destino |
| --- | --- | --- | --- |
| `fund(state, depositor, mint, amount)` | `Created` | `depositor == buyer`, `mint == mint`, `amount == amount` exatos | `Funded` |
| `register_delivery(state, submitter, artifact_hash)` | `Funded` | `submitter == executor`; registro único | `Delivered { artifact_hash }` |
| `check_release(state, journal, recipient, mint)` | `Delivered` | ver ordem abaixo | `Payout` (sem mudar estado) |
| `release(state, journal, recipient, mint)` | `Delivered` | igual a `check_release` | `Released` + `Payout` |

Não existe outra forma de chegar a `Released`. Não existe parâmetro de
autoridade, administrador, override ou destino livre.

## Rejeições por estado

| Estado | `fund` | `register_delivery` | `check_release`/`release` |
| --- | --- | --- | --- |
| `Created` | permitido | `NotFunded` | `NotFunded` |
| `Funded` | `AlreadyFunded` | permitido | `DeliveryNotRegistered` |
| `Delivered` | `AlreadyFunded` | `DeliveryAlreadyRegistered` | permitido |
| `Released` | `TerminalState` | `TerminalState` | `AlreadyReleased` |

## Ordem de checagem do release

1. estado (`NotFunded`, `DeliveryNotRegistered`, `AlreadyReleased`);
2. compromissos do journal por `JournalV1::validate_against` contra
   `JobV1::expected_commitments(artifact_hash registrado)`, na ordem
   `schema_version`, `job_id`, `spec_hash`, `harness_hash`, `artifact_hash`,
   `image_id` → `EscrowError::Journal(JournalValidationError::…)`;
3. `Verdict::Pass`; `Verdict::Fail` válido → `VerdictNotPass`;
4. `recipient == executor` do Job → senão `RecipientMismatch`;
5. `mint == mint` do Job → senão `MintMismatch`.

A ordem só determina qual erro é reportado quando há várias divergências;
todas as condições são obrigatórias.

## Mapeamento às invariantes do guia

| Invariante (`docs/mvp-agent-operating-guide.md`) | Cobertura na política pura |
| --- | --- |
| 2. buyer, executor, mint e amount vêm do Job | `fund` e `release` comparam com o Job; `Payout` é copiado do Job |
| 3. PASS válido só paga o executor registrado | `VerdictNotPass`, `RecipientMismatch`; `Payout.recipient = executor` |
| 7. sem admin bypass nem destino arbitrário | nenhum parâmetro de autoridade; matriz exaustiva de testes |
| 8. evidência de outro Job/spec/harness/ImageID falha | `Journal(...)` para cada compromisso, incluindo artefato registrado |
| 9. estado terminal impede replay/dupla liquidação | `AlreadyReleased`, `TerminalState` |
| 1, 4, 5, 6, 10 (PDA, refund, timeout, atomicidade, CPI) | **não cobertas**; dependem de decisão pendente ou do gate Anchor/Router |

## Decisões pendentes — `AGUARDANDO_AUTORIZAÇÃO`

- política de refund e estado terminal correspondente;
- prazo, timeout, quem aciona cada transição temporal e se release após o
  prazo é permitido;
- efeito econômico de um `Verdict::Fail` válido;
- quem pode acionar `release` on-chain (a política não modela chamador; o
  destino é fixo no executor);
- layout, seeds e serialização do Job on-chain.

Consequência conhecida: com a política atual, um Job em `Delivered` cujo
artefato registrado produz `FAIL` não tem transição de saída até que o refund
seja decidido. Isso é deliberado: a regra não foi inventada.
