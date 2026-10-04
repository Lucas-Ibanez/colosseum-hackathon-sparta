# Programa `vericode_escrow` — especificação local (D2c, D2c.1, D2b.1)

**Programa Anchor local. Testado apenas em processo (`solana-program-test`);
nunca implantado em nenhum cluster. Router/CPI/devnet:
`STATUS: NÃO VALIDADO`.**

Fonte: `anchor/programs/vericode-escrow/src/lib.rs`. Política econômica:
`crates/vericode-core/src/escrow.rs` (ver `docs/escrow-state-machine.md`).

## Perfil

| Item | Valor |
| --- | --- |
| Anchor | `0.31.1` |
| Agave | `2.3.9` (platform-tools `v1.48`, `rustc 1.84.1-dev`) |
| Rust host | `1.89.0` |
| SPL Token, crate cliente | `spl-token 7.0.0` |
| SPL Token, programa executado nos testes | `spl_token-3.5.0.so`, embutido no `solana-program-test 2.3.9` |
| Program ID de localnet | `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH` |

Somente a chave pública está versionada; o keypair fica fora do
repositório. Isso não é deploy.

## Princípio

O programa não reimplementa a regra. Cada instrução reconstrói `JobV1` a
partir da conta e delega ao core as checagens econômicas:
- identidades, `buyer ≠ executor`, amount;
- termos admitidos da v1 e janela de prazo (`JobV1::admit`);
- depositante, mint e valor;
- entrega (deliverer, estado e prazo);
- estado, prazo, destinatário.

O programa mesmo verifica três grupos de coisas:
- **estrutura**, por constraints Anchor: PDA e seeds, owner do token
  program, tipo da conta, `Signer` e endereço do mint igual a `job.mint`;
- **duas pré-condições de custódia** sobre contas Solana, que o core não
  conhece: mint sem freeze authority (D2c.1) e executor diferente das PDAs do
  Job e do vault (D2b.1);
- **o valor do ImageID admitido** (`ADMITTED_IMAGE_ID_V1`) e o slot do
  `Clock`, passados ao core.

A transferência acontece depois do aceite do core, na mesma instrução que
grava o novo estado. Uma falha em qualquer ponto reverte a transação
inteira.

## Termos admitidos da v1 (D2b.1)

`create_job` mantém os parâmetros (IDL estável), mas só aceita:

| Termo | Valor admitido | Fonte |
| --- | --- | --- |
| `spec_hash` | `af642b56…b778` | `hash_restricted_spec(&RESTRICTED_SPEC_V1)`, calculado on-chain pelo core |
| `harness_hash` | `01124025…6b50` | `hash_harness_version(DETERMINISTIC_HARNESS_VERSION)`, calculado on-chain pelo core |
| `image_id` | `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a` | constante `ADMITTED_IMAGE_ID_V1` (guest D1c2b) |
| `deadline_slot` | `Clock.slot + 1_500 ≤ deadline_slot ≤ Clock.slot + 1_512_000` | `JobV1::admit` |
| `executor` | diferente da PDA do Job e da PDA do vault | programa |

O cálculo dos dois hashes eleva o `create_job` de 22.083 para 36.267 CU,
medido em processo (`docs/d2b1-delivery-binding-results.md`).

## Contas

| Conta | Endereço | Conteúdo |
| --- | --- | --- |
| `JobAccount` | PDA `["job", job_id]` | `version=1`, `bump`, `vault_bump`, `job_id`, `buyer`, `executor`, `mint`, `amount`, `deadline_slot`, `spec_hash`, `harness_hash`, `image_id`, `status` |
| vault | PDA `["vault", job]` | token account SPL do `mint` do Job; autoridade = PDA do Job; sem delegate nem close authority |

`JobAccount::INIT_SPACE = 276`, testado.

`job_id` é globalmente único **por implantação**: a segunda criação com o
mesmo `job_id` falha com `Custom(0)` (`AccountAlreadyInUse` do System
Program). Assim, nesta implantação, um journal vinculado a um `job_id` só
pode corresponder a um Job. O journal não contém Program ID nem cluster
(R-D2 F-15).

`status` (`EscrowStatus`) codifica o `EscrowState` do core. A tag Borsh faz
parte do layout, e variantes novas entram sempre no fim:

| Tag | `EscrowStatus` | `EscrowState` |
| ---: | --- | --- |
| 0 | `Created` | `Created` |
| 1 | `Funded` | `Funded` |
| 2 | `Released { artifact_hash }` | `Released { artifact_hash }` |
| 3 | `RefundedOnFail { artifact_hash }` | `Refunded { Fail { artifact_hash } }` |
| 4 | `RefundedOnTimeout` | `Refunded { Timeout }` |
| 5 | `Delivered { artifact_hash }` | `Delivered { artifact_hash }` (D2b.1) |

## Instruções

| Instrução | Signers | Contas | Regra (core) | Efeito |
| --- | --- | --- | --- | --- |
| `create_job(job_id, executor, amount, deadline_slot, spec_hash, harness_hash, image_id)` | buyer (payer) | buyer, mint (**sem freeze authority**), job (`init`), vault (`init`), token program, system program | `Amount::new`, `JobV1::new`, executor ≠ PDAs, `JobV1::admit(ADMITTED_IMAGE_ID_V1, Clock.slot)` | cria Job `Created` e vault vazio |
| `fund(amount)` | buyer | buyer, job, mint (`address = job.mint`), `buyer_token`, vault, token program | `JobV1::fund(estado, signer, mint, amount)` | `transfer_checked` de `job.amount` para o vault; `Funded` |
| `deliver(artifact_hash)` | executor | executor, job | `JobV1::deliver(estado, signer, artifact_hash, Clock.slot)` | `Delivered { artifact_hash }`; nenhum token move |
| `refund_on_timeout()` | nenhum (permissionless) | job, mint (`address = job.mint`), vault, `buyer_token`, token program | `JobV1::refund_on_timeout(estado, Clock.slot, buyer_token.owner, buyer_token.mint)` | `transfer_checked` de `job.amount` do vault ao buyer, assinado pela PDA; `RefundedOnTimeout`, a partir de `Funded` ou `Delivered` |

`artifact_hash` do `deliver` tem a semântica de `hash_restricted_artifact`, a
mesma do `artifact_hash` do journal. Os testes entregam o artefato `(7, 14)`
das fixtures Groth16 versionadas (Job `0x11`).

Não existem `release`, `refund_on_fail`, instruções administrativas, close
ou realloc. A IDL gerada (`anchor idl build`) contém exatamente as quatro
instruções acima. A IDL não é versionada; só seu hash está registrado.

## Erros

`VericodeEscrowError`, códigos `6000 + índice`. Os códigos são interface
pública: variantes novas entram sempre no fim, e `tests/layout.rs` confere
cada código por literal.

| Código | Erro | Origem |
| ---: | --- | --- |
| 6000 | `AmountZero` | `AmountError::Zero` |
| 6001–6003 | `ZeroBuyer`, `ZeroExecutor`, `ZeroMint` | `JobError::ZeroIdentity` |
| 6004 | `BuyerIsExecutor` | `JobError::BuyerIsExecutor` |
| 6005–6008 | `NotFunded`, `AlreadyFunded`, `AlreadyReleased`, `AlreadyRefunded` | `EscrowError` |
| 6009–6012 | `DepositorMismatch`, `RecipientMismatch`, `MintMismatch`, `AmountMismatch` | `EscrowError`; `MintMismatch` também da constraint `address = job.mint` |
| 6013–6018 | `Journal*Mismatch` | `EscrowError::Journal` (sem instrução que o produza ainda) |
| 6019–6022 | `VerdictNotPass`, `VerdictNotFail`, `DeadlineNotReached`, `DeadlinePassed` | `EscrowError` |
| 6023 | `UnsupportedAccountVersion` | conta com `version ≠ 1` |
| 6024 | `MintHasFreezeAuthority` | constraint de `create_job` (D2c.1) |
| 6025 | `NotDelivered` | `EscrowError::NotDelivered` (sem instrução que o produza ainda) |
| 6026 | `AlreadyDelivered` | `EscrowError::AlreadyDelivered` |
| 6027 | `DelivererMismatch` | `EscrowError::DelivererMismatch` |
| 6028–6030 | `SpecNotAdmitted`, `HarnessNotAdmitted`, `ImageIdNotAdmitted` | `JobError` (`admit`) |
| 6031 | `DeadlineOutOfWindow` | `JobError::DeadlineOutOfWindow` (`admit`) |
| 6032 | `ExecutorIsProgramAccount` | programa: executor igual à PDA do Job ou do vault |

Erros estruturais (seeds, owner, signer, conta já existente) são do Anchor,
do System Program ou do Token Program. Os testados são:

| Código | Erro | Caso |
| ---: | --- | --- |
| 2006 | `ConstraintSeeds` | vault falso |
| 3007 | `AccountOwnedByWrongProgram` | mint Token-2022; Job forjado de outro owner |
| 3008 | `InvalidProgramId` | programa Token-2022 |
| 3010 | `AccountNotSigner` | executor nomeado sem assinatura |
| `0` | System Program `AccountAlreadyInUse` | `job_id` repetido |

## Invariantes do guia §7

| Invariante | Estado no D2b.1 |
| --- | --- |
| 1. vault controlado por PDA, sem chave privada | **testado**: Job e vault fora da curva; autoridade do vault = PDA do Job |
| 2. mint, buyer, executor, amount e prazo do Job | **testado**: em `fund` e `refund_on_timeout`; conta mint amarrada a `job.mint` (F-08); prazo dentro da janela |
| 3. `Pass` paga só o executor | política no core, vinculada à entrega; on-chain não implementado (sem `release`) |
| 4. `Fail` devolve só ao buyer | política no core, vinculada à entrega; on-chain não implementado (sem `refund_on_fail`) |
| 5. timeout só após o prazo | **testado**: antes e no slot do prazo (`Clock.slot` conferido) falha; `prazo + 1` devolve, de `Funded` e de `Delivered` |
| 6. estado e transferência atômicos | **testado localmente**: toda rejeição deixa Job, vault e saldos byte a byte iguais |
| 7. sem admin nem destino livre | nenhuma instrução administrativa; só o executor entrega; destino validado pelo core; **upgrade authority ainda não tratada** (F-04) |
| 8. journal de outro Job/spec/harness/ImageID | termos fora da v1 rejeitados na criação (**testado**); verificação do journal on-chain não aplicável ainda |
| 9. terminal impede dupla liquidação | **testado**: refund duplicado, fund e deliver após refund falham |
| 10. falha de CPI/verificação reverte | não aplicável ainda (sem Router/CPI) |

## Pendências e riscos conhecidos

- **Freeze authority do mint — resolvido no D2c.1:** `create_job` rejeita
  mints com freeze authority (erro 6024).
  - O programa SPL Token **executado nos testes** (`spl_token-3.5.0.so`, do
    `solana-program-test 2.3.9`) não permite adicionar freeze authority a um
    mint criado sem ela (`MintCannotFreeze`). Por isso, a checagem na criação
    basta e o vault nunca pode ser congelado.
  - O `spl-token 7.0.0` é o crate cliente usado para montar as instruções.
  - Errata D2b.1 (R-D2 F-10): o D2c.1 atribuía o comportamento ao "SPL Token
    7.0.0". Repetir a verificação em devnet contra o Tokenkeg implantado.
- **Mint aceito (F-05):** qualquer mint do SPL Token clássico sem freeze
  authority é aceito; Token-2022 é rejeitado (3007/3008, testado). A
  allowlist do Test USDC fica para o gate devnet.
- **Upgrade authority (F-04):** um programa atualizável é bypass
  administrativo; decidir no gate de deploy (programa imutável ou autoridade
  documentada).
- **ImageID admitido:** depende do ELF D1c2b preservado. O guest não foi
  reconstruído depois das mudanças no core, e `escrow.rs` faz parte da crate
  que o guest compila. Um rebuild exige recertificar o ImageID e atualizar
  `ADMITTED_IMAGE_ID_V1`.
- **Spec v1 trivial:** o executor escolhe a entrada, e qualquer `(n, 2n)`
  passa. O `deliver` impede que terceiros troquem o artefato, mas não torna
  a tarefa não trivial.
- **Squatting de `job_id` (F-09):** qualquer conta pode criar primeiro um Job
  com um `job_id` escolhido, dentro da janela; o comprador legítimo recebe
  `Custom(0)` e precisa usar outro. Não há perda de fundos.
- **Destino do refund (F-06):** quem dispara o refund escolhe a token
  account do buyer (delegate e close authority aceitos); ATA canônica no D2e.
- **Rent (F-13):** Job e vault nunca são fechados; o rent não é recuperado.
- **Feature `token_2022` do `anchor-spl`:** é exigida pelo código gerado de
  `init` de token account no Anchor 0.31.1. O programa usa somente o SPL Token
  clássico.
