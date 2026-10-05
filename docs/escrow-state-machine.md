# Máquina de estados do escrow — política pura (D2b, D2b.1; aplicada on-chain no D2e e no D4a)

**Política pura em `crates/vericode-core/src/escrow.rs`. Não custodia
fundos, não move tokens e não verifica prova.**

Este documento descreve a política do core: alinhada ao guia de produto
(`docs/context/guia-mvp-agentes-de-codigo.md` §5 e §7) no D2b e vinculada à
entrega do executor no D2b.1. O programa Anchor local aplica essa política em
todas as instruções (`docs/escrow-program.md`). Desde o D2e, `release` e
`refund_on_fail` on-chain só chamam o core depois de decodificar o journal e
só transferem depois que o verificador Groth16 aceita a prova por CPI, em
`solana-program-test` local. A CPI passava pelo Verifier Router no D2e e vai
direto ao verificador desde o D4a. Verificação em devnet: `STATUS: NÃO
VALIDADO` até o D4b.

## Pré-condição de prova

`release` e `refund_on_fail` recebem um `JournalV1` já decodificado. A
política **presume** que o adaptador verifica a receipt correspondente contra
o ImageID do Job. Este módulo não verifica receipt, seal, Groth16, Router ou
CPI e não deve ser citado como verificação ZK. No programa, essa verificação
é a CPI ao verificador Groth16 (D2e via Router; D4a direta) na mesma
instrução, depois do core e antes da transferência; um journal forjado é
barrado por ela.

## Tipos

| Tipo | Conteúdo | Validação |
| --- | --- | --- |
| `BuyerId`, `ExecutorId`, `MintId` | 32 bytes exatos, tipos distintos | identidade toda-zero rejeitada em `JobV1::new` |
| `Amount` | `u64` em unidades base do mint | zero rejeitado (`AmountError::Zero`) |
| `JobV1` | `job_id`, `buyer`, `executor`, `mint`, `amount`, `deadline_slot`, `spec_hash`, `harness_hash`, `image_id` | campos privados, somente leitura; `JobV1::new` rejeita identidade zero e `buyer == executor`; regras de criação em `JobV1::admit` |
| `EscrowState` | `Created`, `Funded`, `Delivered { artifact_hash }`, `Released { artifact_hash }`, `Refunded { reason }` | `Released` e `Refunded` são terminais; `Delivered` não |
| `RefundReason` | `Fail { artifact_hash }` ou `Timeout` | registra o motivo do refund |
| `Payout` | `recipient: PayoutRecipient`, `mint`, `amount` | sempre copiado do Job, nunca do pedido |
| `Settlement` | `state` terminal + `payout` | devem ser persistidos/transferidos atomicamente pelo programa |

O `artifact_hash` não faz parte dos termos do Job. Ele entra no estado por
`deliver`, compromisso único e assinado pelo executor, e toda liquidação por
veredito exige um journal para exatamente esse artefato.

**Divergência registrada (D2b.1):** o guia §5 diz que o programa registra o
`artifact_hash` "apenas na liquidação". A decisão humana D2b.1 substituiu essa
regra pelo compromisso de entrega. Motivo: R-D2 F-01. Com o guest atual,
qualquer pessoa gera um FAIL vinculado ao Job para um artefato arbitrário
(`docs/decisions.md`).

## Regras de criação (`JobV1::admit`)

`admit(admitted_image_id, current_slot)` é chamado pelo programa em
`create_job`, com o `Clock` e a constante `ADMITTED_IMAGE_ID_V1`. Não é
reexecutado quando os termos são reconstruídos da conta. Ordem:

1. `spec_hash == hash_restricted_spec(&RESTRICTED_SPEC_V1)`, senão
   `SpecNotAdmitted`;
2. `harness_hash == hash_harness_version(DETERMINISTIC_HARNESS_VERSION)`,
   senão `HarnessNotAdmitted`;
3. `image_id == admitted_image_id`, senão `ImageIdNotAdmitted`;
4. `current_slot + MIN_DEADLINE_WINDOW_SLOTS <= deadline_slot <=
   current_slot + MAX_DEADLINE_WINDOW_SLOTS`, senão `DeadlineOutOfWindow`.

Detalhes:
- Os dois hashes são calculados pelo core em tempo de execução, como fonte
  única. Se não puderem ser calculados, nada é admitido.
- `MIN_DEADLINE_WINDOW_SLOTS = 1_500` (cerca de 10 min a 400 ms/slot) e
  `MAX_DEADLINE_WINDOW_SLOTS = 1_512_000` (cerca de 7 dias).
- A comparação usa `deadline_slot.checked_sub(current_slot)`: não há
  overflow, e prazo no passado ou no próprio slot é rejeitado.

## Estados do guia × estados da política

| Guia §7 | Política | Natureza |
| --- | --- | --- |
| `Draft` | `Created` | econômico |
| `Funded` | `Funded` | econômico |
| — | `Delivered { artifact_hash }` | econômico, não terminal (D2b.1): compromisso de entrega do executor |
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
| `deliver(state, deliverer, artifact_hash, current_slot)` | `Funded` | deliverer = executor; `current_slot <= deadline_slot` | `Delivered { artifact_hash }` |
| `release(state, journal, current_slot, recipient, mint)` | `Delivered { h }` | `current_slot <= deadline_slot`; journal vinculado ao Job **e a `h`**; `Pass`; recipient = executor; mint do Job | `Released { h }` + payout ao executor |
| `refund_on_fail(state, journal, recipient, mint)` | `Delivered { h }` | journal vinculado ao Job **e a `h`**; `Fail`; recipient = buyer; mint do Job; qualquer slot | `Refunded { Fail { h } }` + payout ao buyer |
| `refund_on_timeout(state, current_slot, recipient, mint)` | `Funded` ou `Delivered` | `current_slot > deadline_slot`; recipient = buyer; mint do Job | `Refunded { Timeout }` + payout ao buyer |

"Journal vinculado" = `schema_version`, `job_id`, `spec_hash`,
`harness_hash`, `artifact_hash` e `image_id` iguais aos do Job e da entrega,
por reuso de `JournalV1::validate_against`. Um FAIL ou PASS de artefato não
entregue falha com `Journal(ArtifactHashMismatch)`.

Não existe outra forma de chegar a um estado terminal. Não existe parâmetro
de autoridade, administrador, override ou destino livre.

### Partição por estado e slot

| Estado | `current_slot <= deadline_slot` | `current_slot > deadline_slot` |
| --- | --- | --- |
| `Created` | nenhum destino | nenhum destino |
| `Funded` | nenhum destino (só `deliver`) | buyer (`Timeout`) |
| `Delivered { h }`, `h` avaliado `Pass` | executor (`release`) | buyer (`Timeout`) |
| `Delivered { h }`, `h` avaliado `Fail` | buyer (`refund_on_fail`) | buyer (`Fail` ou `Timeout`) |
| `Released`, `Refunded` | nenhum (terminal) | nenhum (terminal) |

**Afirmação testada:** em cada estado e slot, no máximo um destino é aceito.
- Teste: `at_most_one_destination_per_state_and_slot`, em
  `crates/vericode-core/src/escrow.rs`.
- Quantificado sobre:
  - os estados acima, incluindo `Delivered` de 42 artefatos PASS e FAIL;
  - os slots `0`, `prazo-1`, `prazo`, `prazo+1` e `u64::MAX`;
  - todas as partes e mints;
  - 44 journals **produzidos pelo harness determinístico**.
- Premissa: os journals vêm de receipts verificadas do guest admitido. O
  core não verifica receipts. Com um journal forjado (por exemplo, PASS e
  FAIL do mesmo artefato), a afirmação não vale até a verificação da prova
  (D2e).

## Rejeições por estado

| Estado | `fund` | `deliver` | `release` / `refund_on_fail` | `refund_on_timeout` |
| --- | --- | --- | --- | --- |
| `Created` | permitido | `NotFunded` | `NotFunded` | `NotFunded` |
| `Funded` | `AlreadyFunded` | permitido (sujeito às exigências) | `NotDelivered` | permitido (sujeito às exigências) |
| `Delivered` | `AlreadyFunded` | `AlreadyDelivered` | permitido (sujeito às exigências) | permitido (sujeito às exigências) |
| `Released` | `AlreadyReleased` | `AlreadyReleased` | `AlreadyReleased` | `AlreadyReleased` |
| `Refunded` | `AlreadyRefunded` | `AlreadyRefunded` | `AlreadyRefunded` | `AlreadyRefunded` |

## Ordem de checagem

| Operação | Ordem |
| --- | --- |
| `admit` | spec → harness → image → janela |
| `deliver` | estado → `DelivererMismatch` → `DeadlinePassed` |
| `release` | estado → `DeadlinePassed` → vínculo do journal (`Journal(...)`) → `VerdictNotPass` → `RecipientMismatch` → `MintMismatch` |
| `refund_on_fail` | estado → vínculo → `VerdictNotFail` → `RecipientMismatch` → `MintMismatch` |
| `refund_on_timeout` | estado → `DeadlineNotReached` → `RecipientMismatch` → `MintMismatch` |

A ordem só determina qual erro é reportado quando há várias divergências;
todas as condições são obrigatórias. Por exemplo, um PASS de artefato não
entregue após o prazo é reportado como `DeadlinePassed`.

## Mapeamento às invariantes do guia §7

| Invariante | Cobertura na política pura |
| --- | --- |
| 2. mint validado; buyer, executor, amount e deadline do Job | `fund`/liquidações comparam com o Job; `Payout` copiado do Job; prazo dentro da janela (`admit`) |
| 3. `Pass` paga só o executor | `release` exige recipient = executor e o artefato entregue; payout `Executor(job.executor)` |
| 4. `Fail` devolve só ao buyer | `refund_on_fail` exige recipient = buyer e o artefato entregue; payout `Buyer(job.buyer)` |
| 5. timeout só após `deadline_slot` | `DeadlineNotReached` até o slot do prazo, inclusive; vale para `Funded` e `Delivered` |
| 7. sem admin nem destino arbitrário | nenhum parâmetro de autoridade; só o executor entrega; matriz com oráculo explícito |
| 8. journal de outro job/spec/harness/ImageID falha | `Journal(...)` em `release` e `refund_on_fail`; também de outro artefato (D2b.1); termos fora da v1 rejeitados na criação |
| 9. terminais impedem replay/dupla liquidação | `AlreadyReleased`, `AlreadyRefunded` em todas as 5 operações |
| 1 (vault PDA), 6 (atomicidade), 10 (falha de CPI reverte) | **fora do core**; responsabilidade do programa Anchor e do verificador |

## Testes

`crates/vericode-core/src/escrow.rs`, 22 testes (core total 42):
- `no_input_combination_bypasses_the_policy`: oráculo independente. Lista
  explícita das 30 chamadas aceitas sobre 7 estados × 5 journals × 3 slots ×
  3 partes × 2 mints, comparada por igualdade de conjunto.
- `refund_on_fail_rejects_a_fail_of_an_undelivered_artifact`: inverte o
  R-D2 PoC-1.
- `release_rejects_a_pass_of_an_undelivered_artifact`.
- `admission_accepts_only_the_v1_terms` e `admission_bounds_the_deadline_window`.
- Detalhes e testes de mutação em `docs/d2b1-delivery-binding-results.md`.

## Pendências fora deste gate

- `release`/`refund_on_fail` on-chain com verificação pelo Router e destino
  canônico (ATA): **feitos no D2e**, em `solana-program-test` local
  (`docs/d2e-router-settlement-results.md`).
- Allowlist do mint (F-05) e CPI direta ao verificador imutável: **feitos
  no D4a** (`docs/d4a-direct-verifier-results.md`).
- Upgrade authority do escrow (F-04) e transações em devnet: D4b.
- A regra de `fund` fora da janela (R-D2e R-02) continua no core; a
  mitigação do MVP é `create_job`+`fund` na mesma transação.
