# Programa `vericode_escrow` — especificação local (D2c)

**Programa Anchor local. Testado apenas em processo (`solana-program-test`);
nunca implantado em nenhum cluster. Router/CPI/devnet:
`STATUS: NÃO VALIDADO`.**

Fonte: `anchor/programs/vericode-escrow/src/lib.rs`. Política econômica:
`crates/vericode-core/src/escrow.rs` (ver `docs/escrow-state-machine.md`).

## Perfil

Anchor `0.31.1`, Agave `2.3.9` (platform-tools `v1.48`, `rustc 1.84.1-dev`),
Rust host `1.89.0`, SPL Token clássico (`spl-token 7.0.0`). Program ID de
localnet `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`. Somente a chave
pública está versionada; o keypair fica fora do repositório. Isso não é
deploy.

## Princípio

O programa não reimplementa a regra. Cada instrução reconstrói `JobV1` a
partir da conta e delega ao core as checagens econômicas:
- identidades, `buyer ≠ executor`, amount;
- depositante, mint e valor;
- estado, prazo, destinatário.

As constraints Anchor validam somente estrutura: PDA e seeds, owner do token
program, tipo da conta e `Signer`. A transferência acontece depois do aceite
do core, na mesma instrução que grava o novo estado. Uma falha em qualquer
ponto reverte a transação inteira.

## Contas

| Conta | Endereço | Conteúdo |
| --- | --- | --- |
| `JobAccount` | PDA `["job", job_id]` | `version=1`, `bump`, `vault_bump`, `job_id`, `buyer`, `executor`, `mint`, `amount`, `deadline_slot`, `spec_hash`, `harness_hash`, `image_id`, `status` |
| vault | PDA `["vault", job]` | token account SPL do `mint` do Job; autoridade = PDA do Job; sem delegate nem close authority |

`job_id` é globalmente único: a segunda criação com o mesmo `job_id` falha.
Assim, um journal vinculado a um `job_id` só pode corresponder a um Job.

`status` (`EscrowStatus`) codifica o `EscrowState` do core:

| `EscrowStatus` | `EscrowState` |
| --- | --- |
| `Created` | `Created` |
| `Funded` | `Funded` |
| `Released { artifact_hash }` | `Released { artifact_hash }` |
| `RefundedOnFail { artifact_hash }` | `Refunded { Fail { artifact_hash } }` |
| `RefundedOnTimeout` | `Refunded { Timeout }` |

## Instruções

| Instrução | Signers | Contas | Regra (core) | Efeito |
| --- | --- | --- | --- | --- |
| `create_job(job_id, executor, amount, deadline_slot, spec_hash, harness_hash, image_id)` | buyer (payer) | buyer, mint (**sem freeze authority**, D2c.1), job (`init`), vault (`init`), token program, system program | `Amount::new`, `JobV1::new` | cria Job `Created` e vault vazio |
| `fund(amount)` | buyer | buyer, job, mint, `buyer_token`, vault, token program | `JobV1::fund(estado, signer, mint, amount)` | `transfer_checked` de `job.amount` para o vault; `Funded` |
| `refund_on_timeout()` | nenhum (permissionless) | job, mint, vault, `buyer_token`, token program | `JobV1::refund_on_timeout(estado, Clock.slot, buyer_token.owner, buyer_token.mint)` | `transfer_checked` de `job.amount` do vault ao buyer, assinado pela PDA; `RefundedOnTimeout` |

Não existem `release`, `refund_on_fail`, instruções administrativas, close
ou realloc. A IDL gerada (`anchor idl build`) contém exatamente as três
instruções acima. A IDL não é versionada; só seu hash está registrado.

## Erros

`VericodeEscrowError`, códigos `6000 + índice`:

| Código | Erro | Origem |
| ---: | --- | --- |
| 6000 | `AmountZero` | `AmountError::Zero` |
| 6001–6003 | `ZeroBuyer`, `ZeroExecutor`, `ZeroMint` | `JobError::ZeroIdentity` |
| 6004 | `BuyerIsExecutor` | `JobError::BuyerIsExecutor` |
| 6005–6008 | `NotFunded`, `AlreadyFunded`, `AlreadyReleased`, `AlreadyRefunded` | `EscrowError` |
| 6009–6012 | `DepositorMismatch`, `RecipientMismatch`, `MintMismatch`, `AmountMismatch` | `EscrowError` |
| 6013–6018 | `Journal*Mismatch` | `EscrowError::Journal` (sem uso neste gate) |
| 6019–6022 | `VerdictNotPass`, `VerdictNotFail`, `DeadlineNotReached`, `DeadlinePassed` | `EscrowError` |
| 6023 | `UnsupportedAccountVersion` | conta com `version ≠ 1` |
| 6024 | `MintHasFreezeAuthority` | constraint de `create_job` (D2c.1): mint com freeze authority |

Erros estruturais (seeds, owner, conta já existente) são do Anchor, do System
Program ou do Token Program.

## Invariantes do guia §7

| Invariante | Estado no D2c |
| --- | --- |
| 1. vault controlado por PDA, sem chave privada | **testado**: Job e vault fora da curva; autoridade do vault = PDA do Job |
| 2. mint, buyer, executor, amount e prazo do Job | **testado** em `fund` e `refund_on_timeout` |
| 3. `Pass` paga só o executor | não implementado (sem `release`) |
| 4. `Fail` devolve só ao buyer | não implementado (sem `refund_on_fail`) |
| 5. timeout só após o prazo | **testado**: antes e no slot do prazo falha; `prazo + 1` devolve |
| 6. estado e transferência atômicos | **testado localmente**: toda rejeição deixa Job, vault e saldos byte a byte iguais |
| 7. sem admin nem destino livre | nenhuma instrução administrativa; destino validado pelo core; **upgrade authority ainda não tratada** |
| 8. journal de outro Job/spec/harness/ImageID | não aplicável ainda (sem instrução com journal) |
| 9. terminal impede dupla liquidação | **testado**: refund duplicado e fund após refund falham |
| 10. falha de CPI/verificação reverte | não aplicável ainda (sem Router/CPI) |

## Pendências e riscos conhecidos

- **Freeze authority do mint — resolvido no D2c.1:** `create_job` rejeita
  mints com freeze authority (erro 6024). O SPL Token `7.0.0` não permite
  adicionar freeze authority a um mint criado sem ela (`MintCannotFreeze`),
  então a checagem na criação basta e o vault nunca pode ser congelado.
  Ambos os comportamentos estão testados.
- **Mint aceito:** qualquer mint do SPL Token clássico sem freeze authority é
  aceito; Token-2022 é rejeitado pelo tipo `Account<Mint>`. A allowlist do
  Test USDC fica para o gate devnet.
- **Upgrade authority:** um programa atualizável é um bypass administrativo
  em potencial; decidir no gate de deploy (programa imutável ou autoridade
  documentada).
- **Squatting de `job_id`:** qualquer conta pode criar primeiro um Job com um
  `job_id` escolhido; o comprador legítimo precisa usar outro. Não há perda de
  fundos.
- **Rent:** Job e vault nunca são fechados; o rent não é recuperado.
- **Feature `token_2022` do `anchor-spl`:** é exigida pelo código gerado de
  `init` de token account no Anchor 0.31.1. O programa usa somente o SPL Token
  clássico.
