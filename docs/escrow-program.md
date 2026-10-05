# Programa `vericode_escrow` — especificação local (D2c, D2c.1, D2b.1, D2e, D4a)

**Programa Anchor testado apenas em processo (`solana-program-test`); ainda
não implantado em nenhum cluster.** `release` e `refund_on_fail` só liquidam
depois que o verificador Groth16 de `risc0-solana v3.0.0` aceita, por CPI na
mesma instrução, a receipt do journal entregue. Desde o D4a a CPI vai
direto ao verificador, sem Verifier Router: o Router upstream em devnet não
está inicializado e nunca poderá registrar o verificador imutável
(`docs/d4a-direct-verifier-results.md`).

Claim máximo: **verificado por CPI ao verificador Groth16 de `risc0-solana
v3.0.0` em `solana-program-test` local**, inclusive com os bytes do
verificador implantado em devnet. Deploy do escrow e transações em devnet:
`STATUS: NÃO VALIDADO` até o D4b. Não é "ZK on-chain" em cluster.

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
| Associated Token Account, programa executado nos testes | `spl_associated_token_account-1.1.1.so`, embutido |
| Verificador Groth16 | `risc0-solana v3.0.0`, commit `ee415935`. Testes com o rebuild offline no Perfil A (`dab6746d…`, igual ao D2d) e com o dump do programa implantado em devnet (`34ae6e5c…`, 199.256 bytes), equivalentes estrutural e funcionalmente (R-D4a RD4A-05) |
| Verifier Router | não é usado desde o D4a |
| Program ID (localnet e devnet futuro) | `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH` |
| Mint admitido | Test USDC de devnet `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F` (D4a; a criar em devnet no D4b) |

Somente as chaves públicas estão versionadas; os keypairs ficam fora do
repositório. Isso não é deploy.

## Princípio

O programa não reimplementa a regra. Cada instrução reconstrói `JobV1` a
partir da conta e delega ao core as checagens econômicas:
- identidades, `buyer ≠ executor`, amount;
- termos admitidos da v1 e janela de prazo (`JobV1::admit`);
- depositante, mint e valor;
- entrega (deliverer, estado e prazo);
- vínculo do journal ao Job e ao artefato entregue, veredito, estado, prazo,
  destinatário e mint.

O programa mesmo verifica:
- **estrutura**, por constraints Anchor: PDA e seeds, owner do token
  program, tipo da conta, `Signer`, endereço do mint igual a `job.mint` e
  endereço fixo do verificador;
- **destino canônico** (D2e, R-D2 F-06): toda liquidação paga a associated
  token account (ATA) da parte paga para `job.mint`;
- **três pré-condições de custódia** sobre contas Solana: mint admitido
  (D4a, R-D2 F-05), mint sem freeze authority (D2c.1) e executor diferente
  das PDAs do Job e do vault (D2b.1);
- **a prova** (D2e, D4a): selector fixo e CPI ao verificador Groth16;
- **o valor do ImageID admitido** (`ADMITTED_IMAGE_ID_V1`) e o slot do
  `Clock`, passados ao core.

A transferência acontece depois do aceite do core e da prova, na mesma
instrução que grava o novo estado. Uma falha em qualquer ponto, inclusive
dentro do verificador, reverte a transação inteira.

## Termos admitidos da v1 (D2b.1)

`create_job` mantém os parâmetros (IDL estável), mas só aceita:

| Termo | Valor admitido | Fonte |
| --- | --- | --- |
| `spec_hash` | `af642b56…b778` | `hash_restricted_spec(&RESTRICTED_SPEC_V1)`, calculado on-chain pelo core |
| `harness_hash` | `01124025…6b50` | `hash_harness_version(DETERMINISTIC_HARNESS_VERSION)`, calculado on-chain pelo core |
| `image_id` | `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a` | constante `ADMITTED_IMAGE_ID_V1` (guest D1c2b) |
| `deadline_slot` | `Clock.slot + 1_500 ≤ deadline_slot ≤ Clock.slot + 1_512_000` | `JobV1::admit` |
| `executor` | diferente da PDA do Job e da PDA do vault | programa |
| `mint` | `ADMITTED_MINT` (Test USDC de devnet), sem freeze authority | programa (D4a; D2c.1) |

O cálculo dos dois hashes eleva o `create_job` de 22.083 para 36.267 CU,
medido em processo (`docs/d2b1-delivery-binding-results.md`).

## Verificação da prova (D2e, D4a)

| Constante | Valor | Papel |
| --- | --- | --- |
| `GROTH16_VERIFIER_ID` | `THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge` | único programa chamado para verificar (`declare_id` upstream); em devnet, upgrade authority `None` |
| `GROTH16_SELECTOR` | `73c457ba` | parâmetros do verificador Groth16 (`risc0-zkvm 3.0.3`); conferido no seal antes da CPI |
| `VERIFY_DISCRIMINATOR` | `85a18d3078c65896` | `sha256("global:verify")[..8]` |
| `ADMITTED_MINT` | `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F` | único mint aceito por `create_job` (D4a) |
| `ATA_PROGRAM_ID` | `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | deriva o destino canônico |

`tests/layout.rs` confere cada valor, o discriminador e o selector das
fixtures.

A CPI é manual, sem dependência nova. Os dados têm 328 bytes:
`discriminador ‖ pi_a ‖ pi_b ‖ pi_c ‖ job.image_id ‖ SHA-256(journal)`. A
única conta é o system program, readonly e sem signer, como em `VerifyProof`
do fonte pinado. É exatamente a chamada que o Verifier Router faria ao
verificador depois de checar a entrada. O formato foi aceito pelo verificador
implantado em devnet em simulação (`docs/d4a-direct-verifier-results.md`).
O argumento `Groth16Seal` tem o layout Borsh do `Seal` upstream (`selector`
mais a prova), com `pi_a` já negado pelo cliente.

Ordem em `release` e `refund_on_fail` (helper `settle_with_proof`):
1. `JournalV1::decode_candidate` sobre exatamente 165 bytes
   (`JournalMalformed`);
2. core `release`/`refund_on_fail`: estado, prazo, vínculo ao Job **e ao
   artefato entregue**, veredito, destinatário e mint;
3. destino = ATA canônica da parte paga (`DestinationNotCanonical`);
4. `seal.selector == GROTH16_SELECTOR` (`UnexpectedSelector`): checagem
   de formato do seal, não vínculo criptográfico; o verificador nunca vê o
   selector (R-D4a RD4A-04);
5. `SHA-256` dos **mesmos** 165 bytes;
6. CPI `verify(proof, job.image_id, digest)` ao verificador fixo; o
   `image_id` vem do Job, nunca do chamador nem do journal (F-12);
7. `transfer_checked` de `job.amount` do vault, assinado pela PDA;
8. estado terminal.

Os passos 1 a 4 rejeitam **antes** da CPI; os testes conferem nos logs que o
verificador não foi chamado.

**Confiança residual** (registrada, não eliminada):
- **Verificador imutável de terceiros:** o `THq1q…` de devnet foi implantado
  pela chave upstream `7uAQqk…` e finalizado (upgrade authority `None`).
  Ninguém pode trocar, pausar ou fechar esse código. Os bytes implantados
  diferem do rebuild local `dab6746d…` só por artefatos do build em macOS
  (caminhos, 3 `TypeId` e ordem de funções). A equivalência é estrutural e
  funcional: VK, control root, identity control ID e tags são idênticos nos
  dois binários (R-D4a RD4A-05), e as simulações em devnet e a suíte local
  rodada com o dump dão os mesmos resultados.
- **Sem e-stop:** sem Router, não existe freio contra um bug de soundness do
  verificador Groth16 de `risc0-zkvm 3.0`. O fonte upstream recomenda o
  Router por isso. Como o escrow também será finalizado, um bug desse tipo
  exigiria um novo program ID. Isso é aceito para o MVP em devnet com Test
  USDC (decisão humana D4a).
- **Sem dono de Router:** o R-D2e R-06 (e-stop como alavanca de liveness) e
  a confiança no `add_verifier` deixam de existir.
- **Upgrade authority do escrow:** o escrow é upgradeable até a finalização
  prevista no D4b, depois do smoke run (F-04).
- **Mint admitido:** a autoridade de mint do Test USDC (deployer de devnet)
  pode emitir mais Test USDC; isso não afeta a custódia de um Job.

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
| `create_job(job_id, executor, amount, deadline_slot, spec_hash, harness_hash, image_id)` | buyer (payer) | buyer, mint (**`ADMITTED_MINT`, sem freeze authority**), job (`init`), vault (`init`), token program, system program | `Amount::new`, `JobV1::new`, executor ≠ PDAs, `JobV1::admit(ADMITTED_IMAGE_ID_V1, Clock.slot)` | cria Job `Created` e vault vazio |
| `fund(amount)` | buyer | buyer, job, mint (`address = job.mint`), `buyer_token`, vault, token program | `JobV1::fund(estado, signer, mint, amount)` | `transfer_checked` de `job.amount` para o vault; `Funded` |
| `deliver(artifact_hash)` | executor | executor, job | `JobV1::deliver(estado, signer, artifact_hash, Clock.slot)` | `Delivered { artifact_hash }`; nenhum token move |
| `release(journal, seal)` | nenhum (permissionless) | job, mint (`address = job.mint`), vault, `recipient_token` (ATA do executor), token program, verificador, system program (7 contas, D4a) | `JobV1::release(estado, journal, Clock.slot, dono, mint)` + prova | `transfer_checked` de `job.amount` do vault ao executor; `Released { h }` |
| `refund_on_fail(journal, seal)` | nenhum (permissionless) | as mesmas, com `recipient_token` = ATA do buyer | `JobV1::refund_on_fail(estado, journal, dono, mint)` + prova | `transfer_checked` ao buyer; `RefundedOnFail { h }`, em qualquer slot |
| `refund_on_timeout()` | nenhum (permissionless) | job, mint (`address = job.mint`), vault, `buyer_token` (ATA do buyer), token program | `JobV1::refund_on_timeout(estado, Clock.slot, dono, mint)` | `transfer_checked` ao buyer; `RefundedOnTimeout`, a partir de `Funded` ou `Delivered` |

`artifact_hash` do `deliver` tem a semântica de `hash_restricted_artifact`, a
mesma do `artifact_hash` do journal. Os testes entregam os artefatos das
fixtures Groth16 versionadas (Job `0x11`: `(7,14)` PASS e `(7,15)` FAIL).

Medições em processo (D4a):
- `release` consumiu 121.885 CU e `refund_on_fail`, 123.214–126.214, nas
  duas execuções da suíte (verificador do rebuild e bytes de devnet). Desses,
  99.541 são do verificador. No D2e, com o Router, eram 135 k a 145 k.
- A variação vem da busca do bump da ATA, que depende das chaves.
- Cabem no limite padrão de 200 k CU por instrução, testado sem instrução de
  compute budget.
- Tamanhos (R-D2e PoC-5, agora `tests/regressions.rs`): release 838 bytes;
  com `SetComputeUnitLimit`, 878; deliver+release, 979; os três juntos,
  1.019, com 213 bytes de margem para o limite de 1.232.

Não existem instruções administrativas, close ou realloc. A IDL gerada
(`anchor idl build`) contém exatamente as seis instruções acima e não é
versionada; só seu hash está registrado.

A IDL mostra o endereço fixo do mint de `create_job` (`ADMITTED_MINT`), mas
não o de `verifier_program`. O motivo é o nome da constante: o anchor-syn
0.31.1 só resolve constantes cujo nome usa `[A-Z_]`
(`idl/accounts.rs:163-171`), e `GROTH16_VERIFIER_ID` contém dígitos (errata
R-D2e R-04d). Clientes devem usar a tabela de constantes acima.

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
| 6013–6018 | `Journal*Mismatch` | `EscrowError::Journal` (vínculo do journal; 6017 = artefato não entregue) |
| 6019–6022 | `VerdictNotPass`, `VerdictNotFail`, `DeadlineNotReached`, `DeadlinePassed` | `EscrowError` |
| 6023 | `UnsupportedAccountVersion` | conta com `version ≠ 1` |
| 6024 | `MintHasFreezeAuthority` | constraint de `create_job` (D2c.1) |
| 6025 | `NotDelivered` | `EscrowError::NotDelivered` |
| 6026 | `AlreadyDelivered` | `EscrowError::AlreadyDelivered` |
| 6027 | `DelivererMismatch` | `EscrowError::DelivererMismatch` |
| 6028–6030 | `SpecNotAdmitted`, `HarnessNotAdmitted`, `ImageIdNotAdmitted` | `JobError` (`admit`) |
| 6031 | `DeadlineOutOfWindow` | `JobError::DeadlineOutOfWindow` (`admit`) |
| 6032 | `ExecutorIsProgramAccount` | programa: executor igual à PDA do Job ou do vault |
| 6033 | `UnexpectedSelector` | programa (D2e, F-03): selector ≠ `73c457ba` |
| 6034 | `JournalMalformed` | programa (D2e): journal fora do wire format de 165 bytes |
| 6035 | `DestinationNotCanonical` | programa (D2e, F-06): destino ≠ ATA canônica |
| 6036 | `MintNotAdmitted` | constraint de `create_job` (D4a, F-05): mint ≠ `ADMITTED_MINT`; checada depois da regra de freeze (6024) |

Erros de outros programas testados:

| Código | Origem | Caso |
| ---: | --- | --- |
| 2006 | Anchor `ConstraintSeeds` | vault falso |
| 2012 | Anchor `ConstraintAddress` | verificador substituído (SPL Token, system program ou conta qualquer) |
| 3012 | Anchor `AccountNotInitialized` | ATA de destino inexistente (R-D2e PoC-2) |
| 2000 | Anchor `ConstraintMut` | Job passado como read-only (R-D2e PoC-7) |
| 3007 | Anchor `AccountOwnedByWrongProgram` | mint Token-2022; Job forjado de outro owner |
| 3008 | Anchor `InvalidProgramId` | programa Token-2022 |
| 3010 | Anchor `AccountNotSigner` | executor nomeado sem assinatura |
| `0` | System Program `AccountAlreadyInUse` | `job_id` repetido |
| 6000 | verificador Groth16 `VerificationError` | seal válido de outro journal |
| 6003 | verificador Groth16 `PairingError` | seal adulterado |

Os códigos 6000+ de programas diferentes se sobrepõem. Os testes identificam
o programa que falhou pelos logs de uma simulação da mesma transação.

## Invariantes do guia §7

| Invariante | Estado no D4a |
| --- | --- |
| 1. vault controlado por PDA, sem chave privada | **testado**: Job e vault fora da curva; autoridade do vault = PDA do Job |
| 2. mint, buyer, executor, amount e prazo do Job | **testado** em todas as instruções; só o Test USDC admitido; conta mint amarrada a `job.mint`; prazo dentro da janela |
| 3. `Pass` paga só o executor | **testado**: `release` paga exatamente `amount` à ATA do executor; outros destinos → 6010/6011/6035 |
| 4. `Fail` devolve só ao buyer | **testado**: `refund_on_fail` antes e depois do prazo, à ATA do buyer |
| 5. timeout só após o prazo | **testado**: antes e no slot do prazo falha; `prazo + 1` devolve, de `Funded` e de `Delivered` |
| 6. estado e transferência atômicos | **testado localmente**: toda rejeição, inclusive no verificador, deixa Job, vault e saldos byte a byte iguais |
| 7. sem admin nem destino livre | nenhuma instrução administrativa; destino = ATA canônica; selector e verificador fixos; verificador imutável em devnet; **upgrade authority do escrow a finalizar no D4b** (F-04) |
| 8. journal de outro Job/spec/harness/ImageID | termos fora da v1 rejeitados na criação; journal de outro artefato → 6017 antes da CPI; prova verificada contra `job.image_id` |
| 9. terminal impede dupla liquidação | **testado**: release→release/refund/timeout/deliver, refund→release, timeout→release |
| 10. falha de CPI/verificação reverte | **testado**: seal adulterado e seal de outro journal revertem sem movimento, com o rebuild local e com os bytes de devnet do verificador |

## Pendências e riscos conhecidos

- **Freeze authority do mint — resolvido no D2c.1:** `create_job` rejeita
  mints com freeze authority (erro 6024).
  - O programa SPL Token executado nos testes (`spl_token-3.5.0.so`) não
    permite adicionar freeze authority depois (`MintCannotFreeze`).
  - O `spl-token 7.0.0` é o crate cliente.
  - Repetir a verificação em devnet contra o Tokenkeg implantado (C7, D4b).
- **Mint aceito (F-05) — resolvido no D4a:** só `ADMITTED_MINT` (6036,
  testado); Token-2022 é rejeitado (3007/3008, testado). O mint ainda será
  criado em devnet no D4b, com 6 decimais e sem freeze authority.
- **Upgrade authority (F-04):** um programa atualizável é bypass
  administrativo. O D4b finaliza a do escrow depois do smoke run. O
  verificador de devnet já é imutável.
- **Router em devnet — descartado no D4a:** o Router upstream está implantado
  e imutável, mas não inicializado (sem PDA `["router"]` nem entrada
  `73c457ba`), e o verificador imutável nunca poderá ser registrado nele. O
  escrow chama o verificador direto.
- **Sem e-stop:** ver "Confiança residual".
- **ATA da própria parte com delegate:** o destino é fixo, mas um delegate
  aprovado pela própria parte na ATA continua com allowance. É escolha da
  parte, não do chamador da liquidação.
- **ATA inexistente:** a liquidação falha até alguém criar a ATA (qualquer
  conta pode criá-la). A CLI deve criá-la de forma idempotente antes.
- **ImageID admitido:** depende do ELF D1c2b preservado; o guest não foi
  reconstruído depois das mudanças no core.
- **Spec v1 trivial:** o executor escolhe a entrada, e qualquer `(n, 2n)`
  passa.
- **Squatting de `job_id` (F-09)** e **rent (F-13):** inalterados.
- **Feature `token_2022` do `anchor-spl`:** exigida pelo código gerado de
  `init` no Anchor 0.31.1; o programa usa só o SPL Token clássico.
