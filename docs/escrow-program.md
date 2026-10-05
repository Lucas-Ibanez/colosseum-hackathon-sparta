# Programa `vericode_escrow` — especificação local (D2c, D2c.1, D2b.1, D2e)

**Programa Anchor local. Testado apenas em processo (`solana-program-test`);
nunca implantado em nenhum cluster.** Desde o D2e, `release` e
`refund_on_fail` só liquidam depois que o Verifier Router (`risc0-solana
v3.0.0`) aceita, por CPI na mesma instrução, a receipt Groth16 do journal
entregue.

Claim máximo: **verificado por CPI ao Verifier Router em
`solana-program-test` local**. Router em devnet, Program ID de rede e deploy:
`STATUS: NÃO VALIDADO`. Não é "ZK on-chain" em cluster.

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
| Verifier Router e verificador Groth16 | `risc0-solana v3.0.0`, commit `ee415935`; `.so` reconstruídos offline no Perfil A, idênticos ao D2d (`1b26b017…`, `dab6746d…`) |
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
- vínculo do journal ao Job e ao artefato entregue, veredito, estado, prazo,
  destinatário e mint.

O programa mesmo verifica:
- **estrutura**, por constraints Anchor: PDA e seeds, owner do token
  program, tipo da conta, `Signer`, endereço do mint igual a `job.mint` e
  endereços fixos do Router;
- **destino canônico** (D2e, R-D2 F-06): toda liquidação paga a associated
  token account (ATA) da parte paga para `job.mint`;
- **duas pré-condições de custódia** sobre contas Solana: mint sem freeze
  authority (D2c.1) e executor diferente das PDAs do Job e do vault (D2b.1);
- **a prova** (D2e): selector fixo e CPI ao Router;
- **o valor do ImageID admitido** (`ADMITTED_IMAGE_ID_V1`) e o slot do
  `Clock`, passados ao core.

A transferência acontece depois do aceite do core e da prova, na mesma
instrução que grava o novo estado. Uma falha em qualquer ponto, inclusive
dentro do Router ou do verificador, reverte a transação inteira.

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

## Verificação da prova (D2e)

| Constante | Valor | Papel |
| --- | --- | --- |
| `VERIFIER_ROUTER_ID` | `6JvFfBrvCcWgANKh1Eae9xDq4RC6cfJuBcf71rp2k9Y7` | único programa chamado para verificar (`declare_id` upstream) |
| `ROUTER_PDA` | `4Sh5ofCzLmmCg1zQraXRoPL2oB88bDZJEEbsfjXaauZT` | PDA `["router"]` do Router |
| `GROTH16_SELECTOR` | `73c457ba` | selector do verificador Groth16 (`risc0-zkvm 3.0.3`) |
| `GROTH16_VERIFIER_ENTRY` | `4Z7ok78xEh7vYmtHSnzF1Tobtm4sozZfHoQfUBgfG8pi` | PDA `["verifier", 73c457ba]` do Router |
| `GROTH16_VERIFIER_ID` | `THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge` | verificador Groth16 da mesma release |
| `ROUTER_VERIFY_DISCRIMINATOR` | `85a18d3078c65896` | `sha256("global:verify")[..8]` |
| `ATA_PROGRAM_ID` | `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | deriva o destino canônico |

As PDAs são constantes pré-calculadas. `tests/layout.rs` confere cada uma
contra `find_program_address`, além do discriminador e do selector das
fixtures.

A CPI é manual, sem dependência nova. Os dados têm 332 bytes:
`discriminador ‖ selector ‖ pi_a ‖ pi_b ‖ pi_c ‖ job.image_id ‖
SHA-256(journal)`. As contas são `[router PDA, verifier entry, verifier
program, system program]`, nenhuma writable ou signer. É o mesmo layout
confirmado no D1a.3 e no fonte pinado. O argumento `RouterSeal` tem o layout
Borsh do `Seal` do Router, com `pi_a` já negado pelo cliente.

Ordem em `release` e `refund_on_fail` (helper `settle_with_proof`):
1. `JournalV1::decode_candidate` sobre exatamente 165 bytes
   (`JournalMalformed`);
2. core `release`/`refund_on_fail`: estado, prazo, vínculo ao Job **e ao
   artefato entregue**, veredito, destinatário e mint;
3. destino = ATA canônica da parte paga (`DestinationNotCanonical`);
4. `seal.selector == GROTH16_SELECTOR` (`UnexpectedSelector`);
5. `SHA-256` dos **mesmos** 165 bytes;
6. CPI `verify(seal, job.image_id, digest)`; o `image_id` vem do Job, nunca
   do chamador nem do journal (F-12);
7. `transfer_checked` de `job.amount` do vault, assinado pela PDA;
8. estado terminal.

Os passos 1 a 4 rejeitam **antes** da CPI; os testes conferem nos logs que o
Router não foi chamado.

**Confiança residual** (registrada, não eliminada):
- **Dono do Router:** pode dar e-stop no selector `73c457ba`. Isso congela a
  liquidação por veredito (testado), mas o timeout continua devolvendo ao
  buyer.
- **Selector já registrado:** o dono não consegue apontá-lo para outro
  verificador (`add_verifier` faz `init` da entrada). Um verificador sob
  outro selector nunca é aceito pelo escrow (F-03).
- **Upgrade authorities:** a do Router pode trocar o código do Router. A do
  verificador é a PDA do Router, e o Router pinado não tem `invoke_signed`,
  então o verificador só muda depois de um upgrade do Router (errata R-D2e
  R-04b). O escrow também é upgradeable enquanto sua autoridade não for
  finalizada (F-04).
- **E-stop como alavanca de liveness (R-D2e R-06):** o e-stop é
  irreversível. Com o selector fixado, um e-stop de `73c457ba` encerra para
  sempre a liquidação por veredito daquela implantação; sobra o timeout.
- **Router em devnet:** não confirmado; o Program ID fixado é o upstream.

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
| `release(journal, seal)` | nenhum (permissionless) | job, mint (`address = job.mint`), vault, `recipient_token` (ATA do executor), token program, Router, router PDA, verifier entry, verificador, system program | `JobV1::release(estado, journal, Clock.slot, dono, mint)` + prova | `transfer_checked` de `job.amount` do vault ao executor; `Released { h }` |
| `refund_on_fail(journal, seal)` | nenhum (permissionless) | as mesmas, com `recipient_token` = ATA do buyer | `JobV1::refund_on_fail(estado, journal, dono, mint)` + prova | `transfer_checked` ao buyer; `RefundedOnFail { h }`, em qualquer slot |
| `refund_on_timeout()` | nenhum (permissionless) | job, mint (`address = job.mint`), vault, `buyer_token` (ATA do buyer), token program | `JobV1::refund_on_timeout(estado, Clock.slot, dono, mint)` | `transfer_checked` ao buyer; `RefundedOnTimeout`, a partir de `Funded` ou `Delivered` |

`artifact_hash` do `deliver` tem a semântica de `hash_restricted_artifact`, a
mesma do `artifact_hash` do journal. Os testes entregam os artefatos das
fixtures Groth16 versionadas (Job `0x11`: `(7,14)` PASS e `(7,15)` FAIL).

Medições em processo:
- `release` e `refund_on_fail` consomem de 135 k a 145 k CU, dos quais
  110.701 são do Router e 99.541 do verificador (dentro dos do Router). O
  D2e mediu até 142 k; o R-D2e observou 144.345 em `refund_on_fail`
  (errata R-04c).
- A variação vem da busca do bump da ATA, que depende das chaves.
- Cabem no limite padrão de 200 k CU por instrução, testado sem instrução de
  compute budget.

Não existem instruções administrativas, close ou realloc. A IDL gerada
(`anchor idl build`) contém exatamente as seis instruções acima e não é
versionada; só seu hash está registrado.

A IDL mostra os endereços fixos de `router_program` e `router`, mas não os
de `verifier_entry` e `verifier_program`. O motivo é o nome das constantes:
o anchor-syn 0.31.1 só resolve constantes cujo nome usa `[A-Z_]`
(`idl/accounts.rs:163-171`), e `GROTH16_*` contém dígitos (errata R-D2e
R-04d). Clientes devem usar a tabela de constantes acima.

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

Erros de outros programas testados:

| Código | Origem | Caso |
| ---: | --- | --- |
| 2006 | Anchor `ConstraintSeeds` | vault falso |
| 2012 | Anchor `ConstraintAddress` | Router, router PDA, verifier entry ou verificador falsos |
| 3007 | Anchor `AccountOwnedByWrongProgram` | mint Token-2022; Job forjado de outro owner |
| 3008 | Anchor `InvalidProgramId` | programa Token-2022 |
| 3010 | Anchor `AccountNotSigner` | executor nomeado sem assinatura |
| `0` | System Program `AccountAlreadyInUse` | `job_id` repetido |
| 6000 | verificador Groth16 `VerificationError` | seal válido de outro journal |
| 6003 | verificador Groth16 `PairingError` | seal adulterado |
| 6001 | Router `SelectorDeactivated` | entrada em e-stop |

Os códigos 6000+ de programas diferentes se sobrepõem. Os testes identificam
o programa que falhou pelos logs de uma simulação da mesma transação.

## Invariantes do guia §7

| Invariante | Estado no D2e |
| --- | --- |
| 1. vault controlado por PDA, sem chave privada | **testado**: Job e vault fora da curva; autoridade do vault = PDA do Job |
| 2. mint, buyer, executor, amount e prazo do Job | **testado** em todas as instruções; conta mint amarrada a `job.mint`; prazo dentro da janela |
| 3. `Pass` paga só o executor | **testado**: `release` paga exatamente `amount` à ATA do executor; outros destinos → 6010/6011/6035 |
| 4. `Fail` devolve só ao buyer | **testado**: `refund_on_fail` antes e depois do prazo, à ATA do buyer |
| 5. timeout só após o prazo | **testado**: antes e no slot do prazo falha; `prazo + 1` devolve, de `Funded` e de `Delivered` |
| 6. estado e transferência atômicos | **testado localmente**: toda rejeição, inclusive no verificador, deixa Job, vault e saldos byte a byte iguais |
| 7. sem admin nem destino livre | nenhuma instrução administrativa; destino = ATA canônica; selector e Router fixos; **upgrade authority ainda não tratada** (F-04) |
| 8. journal de outro Job/spec/harness/ImageID | termos fora da v1 rejeitados na criação; journal de outro artefato → 6017 antes da CPI; prova verificada contra `job.image_id` |
| 9. terminal impede dupla liquidação | **testado**: release→release/refund/timeout/deliver, refund→release, timeout→release |
| 10. falha de CPI/verificação reverte | **testado**: seal adulterado, seal de outro journal e e-stop revertem sem movimento |

## Pendências e riscos conhecidos

- **Freeze authority do mint — resolvido no D2c.1:** `create_job` rejeita
  mints com freeze authority (erro 6024).
  - O programa SPL Token executado nos testes (`spl_token-3.5.0.so`) não
    permite adicionar freeze authority depois (`MintCannotFreeze`).
  - O `spl-token 7.0.0` é o crate cliente.
  - Repetir a verificação em devnet contra o Tokenkeg implantado.
- **Mint aceito (F-05):** qualquer mint do SPL Token clássico sem freeze
  authority é aceito; Token-2022 é rejeitado (3007/3008, testado). A
  allowlist do Test USDC fica para o gate devnet.
- **Upgrade authority (F-04):** um programa atualizável é bypass
  administrativo; decidir no gate de deploy (programa imutável ou autoridade
  documentada). Vale também para Router e verificador próprios.
- **Router em devnet:** Program ID, dono e entrada `73c457ba` não confirmados
  (`STATUS: NÃO VALIDADO`). As contas do Router nos testes são montadas no
  genesis com o layout do fonte pinado; o `add_verifier` real foi exercitado
  no D2d.
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
