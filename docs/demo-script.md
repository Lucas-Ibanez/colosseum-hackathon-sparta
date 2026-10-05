# Roteiro da demo (D9)

Para quem vai apresentar ou gravar a demo do MVP. Escrito no D7; deve ser
ensaiado no D9 num ambiente limpo (clone novo, homes novas), seguindo os
comandos do `README.md`.

## Mensagem e limite (dizer no início e no fim)

- **Mensagem:** um avaliador determinístico previamente comprometido
  executou sobre o artefato entregue no Job e produziu o veredito publicado.
  A receipt dessa execução é **verificada em devnet por CPI ao verificador
  Groth16 imutável de risc0-solana v3.0.0**, na mesma instrução que libera ou
  devolve o Test USDC. Nenhuma interface ou pessoa decide o pagamento.
- **Limite (frase visível):** "A prova atesta a execução da regra fixa sobre
  este artefato, não a qualidade de um software." A regra v1 é trivial
  (`saída = 2 × entrada`) e serve para demonstrar o fluxo.
- **Não dizer:** "o código está correto", "trustless", "ninguém precisa
  confiar em ninguém", "qualquer repositório", "Verifier Router", "mainnet"
  ou "ZK on-chain" sem a frase do claim acima.

## Preparação (antes de gravar)

1. Clone limpo; build de `cli/` e `prover/` com `--locked` (README, seção
   "Reproduzir").
2. Keypairs de devnet do buyer e do executor fora do clone, modo `0600`. O
   buyer precisa de SOL de devnet e de Test USDC do mint admitido, enviado
   pela mint authority do projeto. O executor precisa de SOL para as taxas.
3. `RUN=~/vericode-run; mkdir -p $RUN`; `V=cli/target/release/vericode`;
   `P=prover/target/release/vericode-prover`.
4. **Pelo menos 11 minutos antes da cena 4**, crie o Job do timeout:
   `$V job create --buyer-keypair … --executor <EXECUTOR> --deadline-offset
   1560 --job-file $RUN/T.json`. O prazo mínimo é de 1.500 slots, ~10 min.
5. Feche builds e outras provas: a compressão Groth16 usa quase toda a
   memória do WSL.

## Cenas

| # | Cena | Comando | O que mostrar |
| --- | --- | --- | --- |
| 1 | Conferência pública (1 min) | `$V check` | escrow `GZqb…` com authority **`none`** e bytes `cdf6967f…`; verificador `THq1q…` imutável; mint com 6 decimais e sem freeze. Ninguém, nem o projeto, pode mudar as regras |
| 2 | Comprador cria e financia (1 min) | `$V job create --buyer-keypair … --executor <EXECUTOR> --deadline-offset 9000 --job-file $RUN/P.json` | `job_id` aleatório, termos v1 (spec, harness, ImageID), `create_job`+`fund` numa transação; link no Explorer; vault com 1 Test USDC e authority = PDA do Job |
| 3 | Executor prova (2–3 min, ao vivo ou acelerado e **rotulado**) | `$P prove <job_id> 21 42 $RUN/P` e `$P compress $RUN/P` | `Composite` → `Groth16`; `journal_equal_to_core=true`; `verdict=PASS`; `docker run --pull=never --network=none`. Enquanto isso, o Job está em "Proving"; não fingir sincronia |
| 4a | Negativo: prova de outro Job (1 min) | `$V job settle --job-id <job_id> --receipt <receipt de outro Job> --deliver --executor-keypair … --expect-error escrow:6014` | a transação aterrissa com `JournalJobIdMismatch`; Job, vault e saldos iguais; o `deliver` também reverteu |
| 4b | Negativo: prova adulterada (opcional, 1 min) | `$V job settle --job-id <job_id> --receipt $RUN/P --deliver --executor-keypair … --tamper-seal --expect-error verifier:6003` | o **verificador** rejeita (`PairingError`) dentro da mesma transação; nada move |
| 5 | Liquidação PASS (1 min) | `$V job settle --job-id <job_id> --receipt $RUN/P --deliver --executor-keypair …` | no Explorer, `Program THq1q… invoke [2]` com 99.541 CU **antes** da transferência; `Released`; executor +1 Test USDC |
| 6 | Dupla liquidação (opcional, 30 s) | o mesmo `settle` de novo, sem `--deliver`, com `--payer-keypair … --expect-error escrow:6007` | estado terminal recusa nova liquidação |
| 7 | Refund por timeout (1 min) | `$V job refund-timeout --job-id <T> --payer-keypair … --wait` | `RefundedOnTimeout`; buyer +1 Test USDC |
| 8 | Encerramento (30 s) | — | repetir a mensagem e o limite; mostrar a tabela do README |

A cena 4b não foi executada pela CLI em devnet no D7 (a opção
`--tamper-seal` tem teste offline). O mesmo caso, seal com `pi_c[10] ^= 1`,
aterrissou no D4b pelo cliente fora do clone (`5UwKqSJs…`, abaixo). Ensaie a
cena antes de gravar.

Se o tempo for curto, corte nesta ordem: 4b, 6, a prova ao vivo da cena 3
(mostre a saída gravada, dizendo que é do mesmo Job) e a cena 1. **Nunca
corte** a liquidação verificada (5), um negativo (4a) e a frase de limite.

## Evidência já executada (links para a tela final ou plano B)

Se o RPC de devnet falhar durante a gravação, mostre as transações já
executadas, **dizendo que são de uma execução anterior** com os mesmos
comandos (`docs/d7-cli-results.md`):

| Caso | Transação |
| --- | --- |
| PASS do Job P (D7): `deliver`+`release`, verificador invocado | [`4oWhwZfU…`](https://explorer.solana.com/tx/4oWhwZfUzZhVhrTydrhdBx1TJxWVH2mdWdKwiUhtHtMnhmeJg3MprEshvp592hYMgsaiwwhyvsDHek1egS9zKM1L?cluster=devnet) |
| Journal de outro Job no Job P (D7) → 6014 | [`5eNRKgfH…`](https://explorer.solana.com/tx/5eNRKgfHii8gbWvJCNRTUehsT7Z46mXDC4Xorj2mfEfF4QjJ2nvFKwt3wmevJ5L81VXweaYpLzTxk3adKDAcBbJ7?cluster=devnet) |
| Seal adulterado no Job A (D4b) → verificador 6003 | [`5UwKqSJs…`](https://explorer.solana.com/tx/5UwKqSJsku7rvYCXWz96BYNDNo3DjA8Yn7YXC4eMj1SYMhiUUvDLBvTLwWQEeabffLe8GtVFLC6x8Z1KH1PvCsUU?cluster=devnet) |
| FAIL do Job B (D4b): `refund_on_fail` | [`2osG9m8J…`](https://explorer.solana.com/tx/2osG9m8JcE1CpAKt8JribtJCw8ffrdhBh6hYm5xKeLPptM9YLsugouMRH2KnmRXAymwAMvBABUmuZY6tkwHoVbP4?cluster=devnet) |
| Dupla liquidação (D7) → 6007 e 6008 | [`61de4rBk…`](https://explorer.solana.com/tx/61de4rBkBSN7UjqFrMed8a4KncQvRiBG4rNHJtqVA46B74tMUBoZ6stXVKx3RH2gMydCbRM4Ki9G7EXemgsqY7na?cluster=devnet), [`5vZ6TGRo…`](https://explorer.solana.com/tx/5vZ6TGRoF8hnNMCARsw9AdUkrQSQccavrNVxJLuXbiu1VH3rZ8wM1wfuk2DGpFGq1g4YqdHeysBFKdmi4Zag3pjG?cluster=devnet) |
| Reembolso antes do prazo no Job T (D7) → 6021 | [`8DmQFdjE…`](https://explorer.solana.com/tx/8DmQFdjEjRsVHTfU1DmgjA6GmXZs52G1fdcuCaKmkSV9qXR6eEKh8MiGpKAKv6xa7LDdNPzK7AS9fbP9DgiyX1S?cluster=devnet) |
| Refund por timeout do Job T (D7) | [`3fiNWgTW…`](https://explorer.solana.com/tx/3fiNWgTWdscCB8kRTE4gxacQgNXCZ36NHNhazzgUtRBiF7ub7Zs3NDBBZ7qwEdXGEy2HGUsZyyZX9sv7gxFsVZ7?cluster=devnet) |
| Finalização do escrow (authority `none`, D4b) | [`4AsofYxr…`](https://explorer.solana.com/tx/4AsofYxry7vjdpo2CQxSKSg9MdczuCB4GnRCLzz6ZeVFffPkWdedFBuBQDw58audRHJLcAQ1sGrN5G3t2CoW7eYH?cluster=devnet) |

## Perguntas prováveis

- **"Por que não o Verifier Router?"** O Router upstream de devnet não está
  inicializado e nunca poderá registrar esse verificador imutável. O escrow
  chama o mesmo verificador, com os mesmos argumentos, por CPI direta. O
  custo é não ter e-stop (`docs/d4a-direct-verifier-results.md`).
- **"Quem pode mudar o contrato?"** Ninguém: upgrade authority `none`. Uma
  correção exigiria um novo program ID.
- **"O executor não pode simplesmente escolher a entrada?"** Pode: a regra v1
  é trivial de propósito. O que o MVP mostra é o vínculo entre artefato
  entregue, prova e pagamento, não a dificuldade da tarefa.
- **"E a privacidade?"** O journal é público e o prover vê o artefato. O MVP
  não promete privacidade.
