# Máquina de estados do escrow — política pura D2b

**Política pura em `crates/vericode-core/src/escrow.rs`. Não é programa
Anchor, não custodia fundos, não move tokens e não verifica prova.**

Este documento descreve a política implementada no D2b, alinhada ao guia de
produto (`docs/context/guia-mvp-agentes-de-codigo.md` §5 e §7). Ela é a
referência que o futuro programa on-chain deve aplicar; até esse programa
existir e ser testado, as invariantes on-chain do guia continuam requisitos,
não capacidades. Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Pré-condição de prova

`release` e `refund_on_fail` recebem um `JournalV1` já decodificado. A
política **presume** que um adaptador futuro verificou a receipt
correspondente contra o ImageID do Job antes da chamada. Este módulo não
verifica receipt, seal, Groth16, Router ou CPI e não deve ser citado como
verificação ZK.

## Tipos

| Tipo | Conteúdo | Validação |
| --- | --- | --- |
| `BuyerId`, `ExecutorId`, `MintId` | 32 bytes exatos, tipos distintos | identidade toda-zero rejeitada em `JobV1::new` |
| `Amount` | `u64` em unidades base do mint | zero rejeitado (`AmountError::Zero`) |
| `JobV1` | `job_id`, `buyer`, `executor`, `mint`, `amount`, `deadline_slot`, `spec_hash`, `harness_hash`, `image_id` | campos privados, somente leitura; `buyer == executor` rejeitado; `deadline_slot` sem outra restrição |
| `EscrowState` | `Created`, `Funded`, `Released { artifact_hash }`, `Refunded { reason }` | `Released` e `Refunded` são terminais |
| `RefundReason` | `Fail { artifact_hash }` ou `Timeout` | registra o motivo do refund |
| `Payout` | `recipient: PayoutRecipient`, `mint`, `amount` | sempre copiado do Job, nunca do pedido |
| `Settlement` | `state` terminal + `payout` | devem ser persistidos/transferidos atomicamente pelo programa |

`JobV1` não tem serialização nem layout de conta: isso pertence ao gate
Anchor. O `artifact_hash` não faz parte dos termos: é registrado a partir do
journal na liquidação (guia §5).

## Estados do guia × estados da política

| Guia §7 | Política | Natureza |
| --- | --- | --- |
| `Draft` | `Created` | econômico |
| `Funded` | `Funded` | econômico |
| `Proving`, `Submitted` | — | worker/UI; a liquidação verifica e transfere numa única instrução |
| `Failed` (erro operacional) | — | worker/UI; não é `Verdict::Fail` e não muda o escrow |
| `Released` | `Released { artifact_hash }` | econômico, terminal |
| `Refunded` | `Refunded { Fail { artifact_hash } \| Timeout }` | econômico, terminal |

## Transições

As funções são métodos de `JobV1`; recebem o estado por valor e devolvem o
novo estado (ou `Settlement`) ou um erro explícito, sem efeito colateral. O
slot atual é parâmetro: o core não lê relógio.

| Transição | Origem | Exigências | Resultado |
| --- | --- | --- | --- |
| `fund(state, depositor, mint, amount)` | `Created` | buyer, mint e amount exatos | `Funded` |
| `release(state, journal, current_slot, recipient, mint)` | `Funded` | `current_slot <= deadline_slot`; journal vinculado; `Pass`; recipient = executor; mint do Job | `Released { artifact_hash }` + payout ao executor |
| `refund_on_fail(state, journal, recipient, mint)` | `Funded` | journal vinculado; `Fail`; recipient = buyer; mint do Job; qualquer slot | `Refunded { Fail { artifact_hash } }` + payout ao buyer |
| `refund_on_timeout(state, current_slot, recipient, mint)` | `Funded` | `current_slot > deadline_slot`; recipient = buyer; mint do Job | `Refunded { Timeout }` + payout ao buyer |

"Journal vinculado" = `schema_version`, `job_id`, `spec_hash`,
`harness_hash` e `image_id` iguais aos do Job, por reuso de
`JournalV1::validate_against`; o `artifact_hash` é registrado, não comparado.

Não existe outra forma de chegar a um estado terminal. Não existe parâmetro
de autoridade, administrador, override ou destino livre.

### Partição temporal

| Slot | Destinos possíveis |
| --- | --- |
| `current_slot <= deadline_slot` | executor (`Pass`) ou buyer (`Fail`) |
| `current_slot > deadline_slot` | somente buyer (`Fail` ou `Timeout`) |

Em nenhum slot dois destinos diferentes competem por liquidação.

## Rejeições por estado

| Estado | `fund` | `release` / `refund_on_fail` / `refund_on_timeout` |
| --- | --- | --- |
| `Created` | permitido | `NotFunded` |
| `Funded` | `AlreadyFunded` | permitido (sujeito às exigências) |
| `Released` | `AlreadyReleased` | `AlreadyReleased` |
| `Refunded` | `AlreadyRefunded` | `AlreadyRefunded` |

## Ordem de checagem

- `release`: estado → `DeadlinePassed` → vínculo do journal
  (`Journal(...)`) → `VerdictNotPass` → `RecipientMismatch` → `MintMismatch`.
- `refund_on_fail`: estado → vínculo → `VerdictNotFail` →
  `RecipientMismatch` → `MintMismatch`.
- `refund_on_timeout`: estado → `DeadlineNotReached` → `RecipientMismatch`
  → `MintMismatch`.

A ordem só determina qual erro é reportado quando há várias divergências;
todas as condições são obrigatórias.

## Mapeamento às invariantes do guia §7

| Invariante | Cobertura na política pura |
| --- | --- |
| 2. mint validado; buyer, executor, amount e deadline do Job | `fund`/liquidações comparam com o Job; `Payout` copiado do Job |
| 3. `Pass` paga só o executor | `release` exige recipient = executor; payout `Executor(job.executor)` |
| 4. `Fail` devolve só ao buyer | `refund_on_fail` exige recipient = buyer; payout `Buyer(job.buyer)` |
| 5. timeout só após `deadline_slot` | `DeadlineNotReached` até o slot do prazo, inclusive |
| 7. sem admin nem destino arbitrário | nenhum parâmetro de autoridade; matriz exaustiva de testes |
| 8. journal de outro job/spec/harness/ImageID falha | `Journal(...)` em `release` e `refund_on_fail` |
| 9. terminais impedem replay/dupla liquidação | `AlreadyReleased`, `AlreadyRefunded` |
| 1 (vault PDA), 6 (atomicidade), 10 (falha de CPI reverte) | **fora do core**; responsabilidade do programa Anchor/Router |

## Pendências fora deste gate

- quem assina/aciona cada liquidação on-chain (o destino já é fixo);
- layout, seeds e serialização do Job e do vault;
- verificação da receipt (Router/CPI ou fallback atestado rotulado).
