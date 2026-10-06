# Roteiro da demo (D9)

Para quem vai apresentar ou gravar a demo do MVP. Escrito no D7 e executado
no D9 num ambiente limpo (clone e targets novos, com toolchains isoladas
copiadas; não uma máquina nova), com as escritas W1–W8 em devnet
([`docs/d9-demo-results.md`](d9-demo-results.md)). O vídeo de reserva é
gravado pelo humano seguindo a seção [Gravação](#gravação).

## Mensagem e limite (dizer no início e no fim)

- **Mensagem:** um avaliador determinístico previamente comprometido
  executou sobre o artefato entregue no Job e produziu o veredito publicado.
  A receipt dessa execução é **verificada em devnet por CPI ao verificador
  Groth16 imutável de risc0-solana v3.0.0**, na mesma instrução que libera ou
  devolve o Test USDC. Nenhuma interface ou administrador decide o
  pagamento.
- **Limite (frase visível):** "A prova atesta a execução da regra fixa sobre
  este artefato, não a qualidade de um software." A regra v1 é trivial
  (`saída = 2 × entrada`) e serve para demonstrar o fluxo.

### Frases permitidas (congeladas no D9)

São as mesmas do [`README.md`](../README.md#frases-permitidas-congeladas-no-d9). Mudar a lista exige decisão
registrada em `docs/decisions.md`.

1. **Claim:** "A receipt Groth16 do VeriCode é verificada em devnet por CPI
   ao verificador Groth16 imutável de risc0-solana v3.0.0, na mesma
   instrução que libera ou devolve o Test USDC do Job." Sempre com o link de
   uma transação.
2. **Afirmação:** "Um avaliador determinístico previamente comprometido
   executou sobre o artefato entregue no Job e produziu o veredito publicado.
   O programa só move o Test USDC depois de conferir o vínculo entre journal,
   Job e entrega e de verificar a prova."
3. **Limite:** "A prova atesta a execução da regra fixa sobre este artefato,
   não a qualidade de um software. A regra v1 é trivial (saída = 2 × entrada)
   e serve para demonstrar o fluxo."
4. **Sem administrador:** "O escrow e o verificador são imutáveis (upgrade
   authority `none`). Ninguém, nem o projeto, muda as regras ou decide o
   pagamento; também não existe e-stop."
5. **Rede:** "Só devnet e Test USDC; nada disso existe em mainnet."
6. **Negativos:** "Em devnet, uma prova adulterada, um journal de outro Job,
   um reembolso antes do prazo e uma segunda liquidação foram rejeitados sem
   mover fundos." Sempre com os links.
7. **Reprodução:** "O fluxo foi reproduzido pela CLI e pelo prover deste
   repositório num ambiente limpo: clone e targets novos, com toolchains
   isoladas copiadas, não uma máquina nova."
8. **Prova:** "A prova é gerada localmente e comprimida para Groth16 num
   container Docker local, sem rede."

**Não dizer:**
- "o código está correto";
- "trustless" ou "ninguém precisa confiar em ninguém";
- "qualquer repositório";
- "Verifier Router" como caminho atual;
- "mainnet" ou "dinheiro real";
- "ZK on-chain" sem a frase 1;
- "auditado", "privado" ou "máquina nova".

Também não:
- apresentar uma execução anterior como ao vivo;
- mostrar uma receipt sem dizer de qual Job ela é;
- dizer qual programa rejeitou a partir de um `escrow:6000` a `6003`
  (RD7-01). Esses códigos nunca são usados na demo.

## Preparação (qualquer apresentador)

1. Clone limpo; build de `cli/` e `prover/` com `--locked` (README, seção
   "Reproduzir"). A imagem Groth16 tem de estar presente por digest, porque
   o prover nunca faz pull.
2. Keypairs de devnet do buyer e do executor fora do clone, modo `0600`. O
   buyer precisa de SOL de devnet e de Test USDC do mint admitido, enviado
   pela mint authority do projeto. O executor precisa de SOL para as taxas.
3. `RISC0_PROVER=local`, sem `BONSAI_*` nem `RISC0_DEV_MODE` (RD7-03). Os
   binários do D9 obedecem a essas variáveis. Desde o D10a, o prover recusa
   um ambiente diferente antes de provar.
4. Crie o Job do timeout no começo da gravação, com `--deadline-offset 1560`
   (o mínimo da CLI). No D9, 1.560 slots levaram **cerca de 6 min 20 s**
   (~0,24 s por slot), não os 10 min de uma estimativa a 0,4 s.
5. Feche builds e outras provas: no D9, a compressão Groth16 deixou só 117 MB
   de memória disponível, com o swap cheio.

## Cenas

| # | Cena | Comando | O que mostrar |
| --- | --- | --- | --- |
| 1 | Conferência pública | `check` | escrow `GZqb…` com authority **`none`** e bytes `cdf6967f…`; verificador `THq1q…` imutável; mint com 6 decimais e sem freeze. Ninguém, nem o projeto, pode mudar as regras |
| 2 | Job do timeout e reembolso antecipado | `job create … --deadline-offset 1560`; `job refund-timeout … --expect-error escrow:6021` | `create_job`+`fund` numa transação; reembolso antes do prazo → `DeadlineNotReached`, contas iguais |
| 3 | Comprador cria e financia | `job create … --deadline-offset 9000` | `job_id` aleatório, termos v1 (spec, harness, ImageID); vault com 1 Test USDC e authority = PDA do Job |
| 4 | Executor prova (ao vivo ou acelerado e **rotulado**) | `prove <job_id> 21 42 …`, `compress …` | `Composite` → `Groth16`; `journal_equal_to_core=true`; `verdict=PASS`; `docker run --pull=never --network=none`. Enquanto isso, o Job está em "Proving"; não fingir sincronia |
| 5a | Negativo: prova de outro Job | `job settle … --receipt <receipt de outro Job> --deliver … --expect-error escrow:6014` | `JournalJobIdMismatch`; Job, vault e saldos iguais; o `deliver` também reverteu. **Diga de qual Job é a receipt** (CR6). Um clone limpo não tem nenhuma: as fixtures do repositório são `.txt` em hex, não o diretório que a CLI lê. Use a receipt de um Job já liquidado, guardada fora do clone |
| 5b | Negativo: prova adulterada | `job settle … --deliver … --tamper-seal --expect-error verifier:6003` | o **verificador** rejeita (`PairingError`) dentro da mesma transação; nada move |
| 6 | Liquidação PASS | `job settle … --deliver …` | no Explorer, `Program THq1q… invoke [2]` com 99.541 CU **antes** da transferência; `Released`; executor +1 Test USDC |
| 7 | Dupla liquidação | o mesmo `settle`, sem `--deliver`, com `--payer-keypair … --expect-error escrow:6007` | estado terminal recusa nova liquidação |
| 8 | Refund por timeout | `job refund-timeout … --wait` | `RefundedOnTimeout`; buyer +1 Test USDC |
| 9 | Encerramento | — | repetir a mensagem e o limite; mostrar a tabela do README |

A cena 5b foi executada pela CLI em devnet no D9 (W5,
[`HXgnGUhh…`](https://explorer.solana.com/tx/HXgnGUhhB4zRgD8Bubs6ULCqpuhoqBSuim3Vbx1kz7V3bK2FFJG3ND2jcTtRFf6QxD9i1bpwKW4z32yFYYEofVr?cluster=devnet)).
Desde o D10a, a opção `--tamper-seal` tem teste em
`cli/tests/instructions.rs` (RD7-04): o seal difere só em `pi_c[10]` e é igual,
byte a byte, à mutação da suíte. O casamento do `--expect-error` também tem
teste, e a CLI do D10a não aceita mais `escrow:6000` a `6003` para uma falha
do verificador (RD7-01). A seção "Gravação" usa os binários do D9, anteriores
a essas correções; por isso os negativos da demo continuam os da CR2.

Se o tempo for curto, corte nesta ordem:
1. a cena 5b;
2. a cena 7;
3. a prova ao vivo da cena 4: mostre a saída gravada, dizendo que é do
   mesmo Job;
4. a cena 1.

**Nunca corte** a liquidação verificada (6), um negativo (5a) e a frase de
limite.

## Gravação

Passo a passo para o humano gravar o vídeo de reserva nesta máquina, com as
mesmas ferramentas do D9. Cada tomada cria Jobs novos em devnet; nunca
reutilize um `job_id` (S, A, B, C, P, T, P′ e T′ estão consumidos).

### 1. Antes de gravar (fora da câmera, ~3 min)

- Feche builds, testes e outras provas. Confira `free -m`: o D9 começou a
  prova com ~6,2 GB disponíveis.
- Abra no navegador, em abas, as páginas do
  [escrow](https://explorer.solana.com/address/GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH?cluster=devnet),
  do [verificador](https://explorer.solana.com/address/THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge?cluster=devnet)
  e do [mint](https://explorer.solana.com/address/9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F?cluster=devnet).
- Num terminal novo, cole o bloco abaixo. Ele não imprime nada nem lê
  conteúdo de chave: só define caminhos e funções. `env9.sh` usa só nomes
  `N9_*`/`n9_*` (nunca `R`, `B`, `D` ou `VC`).

```bash
source ~/.local/share/vericode-spikes/d9/bin/env9.sh
REC=~/.local/share/vericode-spikes/rec-$(date +%m%d-%H%M)    # uma pasta nova por tomada
(umask 077; mkdir -p $REC/jobs $REC/receipts $REC/logs)
vc() { env -i HOME=$N9_HOMES/sol PATH=/usr/bin:/bin $N9_V --log $REC/logs/cli-tx.jsonl "$@"; }   # CLI do D9
vp() { n9_zk $N9_P "$@"; }                                      # prover do D9, RISC0_PROVER=local
jid() { python3 -c 'import json,sys;print(json.load(open(sys.argv[1]))["job_id"])' "$1"; }
n9_zk env | grep -E '^(RISC0|BONSAI)'      # esperado: só RISC0_EXECUTOR=local e RISC0_PROVER=local
clear
```

As chaves (`$N9_KEYS`, em `d4/keys`, `0600`) aparecem só como caminho. A CLI
imprime só pubkeys.

**Não mostre na tela:**
- `cat`, `less` ou `ls -l` de `$N9_KEYS`;
- `env` ou `set` sem filtro;
- o histórico do shell.

### 2. Cenas, comandos e saída esperada

Os tempos são os do D9 (2026-10-05). Os comandos esperam o terminal
preparado acima.

**Cena 1 — conferência pública (~2 s).**

```bash
vc check
```

Saída esperada:

```text
check.escrow=GZqbL2Tb… upgrade_authority=none
check.escrow_program_data bytes=395064 sha256=cdf6967f…
check.verifier=THq1q… upgrade_authority=none
check.mint=9TE2V… decimals=6 freeze_authority=none
check=ok
```

Mostre também a aba do escrow no Explorer.

Fala: "Quando um agente entrega um trabalho, quem decide se ele recebe? No
VeriCode, o pagamento fica num escrow em devnet que só libera com a prova de
uma regra fixa. Este é o programa: a upgrade authority é `none`. Ninguém,
nem nós, muda as regras."

**Cena 2 — Job do timeout e reembolso antecipado (~12 s + ~6 s).**

```bash
vc job create --buyer-keypair $N9_KEYS/buyer.json --executor $N9_EXECUTOR --deadline-offset 1560 --job-file $REC/jobs/T.json
TJ=$(jid $REC/jobs/T.json)
vc job refund-timeout --job-id $TJ --payer-keypair $N9_KEYS/deployer.json --expect-error escrow:6021
```

Saída esperada:
- `[create+fund] PASS … err=null`, `job.status=Funded`, `job.vault=… amount=1000000`;
- `[refund_on_timeout] … PASS … Custom":6021`, com `DeadlineNotReached` e
  `watched accounts unchanged=true`.

Fala: "O comprador cria um Job e deposita 1 Test USDC na mesma transação.
Este primeiro Job vence em poucos minutos. Pedir o reembolso antes do prazo
falha: erro 6021, e nada se move."

**Cena 3 — o Job que vai ser cumprido (~10 s).**

```bash
vc job create --buyer-keypair $N9_KEYS/buyer.json --executor $N9_EXECUTOR --deadline-offset 9000 --job-file $REC/jobs/P.json
PJ=$(jid $REC/jobs/P.json); echo $PJ
```

Saída esperada: `create.job_id=…`; `job.terms_v1_admitted=true`;
`job.status=Funded`. Abra o link `explorer=` da transação.

Fala: "Segundo Job, este o executor vai cumprir. Os termos fixam a
especificação, o harness e a imagem do avaliador."

**Cena 4 — prova (~10 s + ~1 min 55 s).** Pode ser acelerada na edição,
com o rótulo "acelerado".

```bash
vp prove $PJ 21 42 $REC/receipts/P
vp compress $REC/receipts/P
```

Saída esperada:
- `prove.receipt_type=Composite`, `prove.journal_equal_to_core=true`,
  `prove.verdict=PASS`;
- `compress.receipt_type=Groth16`,
  `compress.docker_run=… docker run --pull=never --network=none … risc0-groth16-prover@sha256:7f173963…`,
  `compress.verdict=PASS`.

Exit 137 é falta de memória: repita uma vez, sozinho.

Fala: "O executor prova localmente. O guest RISC Zero avalia o artefato
(21, 42) com a regra fixa, saída igual a duas vezes a entrada, e publica o
veredito PASS. A prova é comprimida para Groth16 num container local, sem
rede. Enquanto isso, o Job está em Proving."

**Cena 5a — prova de outro Job (~8 s).** A receipt é a do **Job P′ do D9**
(`job_id 91ea6fcd…`), já liquidado. Diga isso na fala.

```bash
vc job settle --job-id $PJ --receipt $N9_ROOT/receipts/Pp --deliver --executor-keypair $N9_KEYS/executor.json --expect-error escrow:6014
```

Saída esperada:
- `precheck.expected_rejection=journal belongs to job_id 91ea6fcd… (6014)`;
- `PASS … {"InstructionError":[1,{"Custom":6014}]}` com
  `JournalJobIdMismatch`;
- `watched accounts unchanged=true`;
- `verifier_invoked=false`.

Fala: "Primeiro, uma prova válida, mas de outro Job: a do Job P′, de uma
execução anterior. O programa recusa com 6014, journal de outro Job, e a
entrega reverte junto. Nada se move."

**Cena 5b — prova adulterada (~5 s).**

```bash
vc job settle --job-id $PJ --receipt $REC/receipts/P --deliver --executor-keypair $N9_KEYS/executor.json --tamper-seal --expect-error verifier:6003
```

Saída esperada:
- `settle.tampered=pi_c[10]^=1`;
- `Program THq1q… invoke [2]`;
- `PairingError`;
- `Program THq1q… failed: custom program error: 0x1773`;
- `watched accounts unchanged=true`.

Fala: "Agora a prova certa, com um bit trocado. O próprio verificador Groth16
rejeita, dentro da mesma transação."

**Cena 6 — liquidação PASS (~9 s).**

```bash
vc job settle --job-id $PJ --receipt $REC/receipts/P --deliver --executor-keypair $N9_KEYS/executor.json
```

Saída esperada:
- `[deliver+release] PASS … err=null … verifier_invoked=true`;
- `Program THq1q… consumed 99541`;
- `job.status=Released artifact_hash=225384a7…`;
- `settle.recipient_amount_before=X after=X+1000000`.

Abra o link `explorer=`. Nos logs da transação, mostre `Program THq1q…
invoke [2]` **antes** da transferência do Token.

Fala: frase 1 (o claim), apontando para a transação.

**Cena 7 — dupla liquidação (~5 s).**

```bash
vc job settle --job-id $PJ --receipt $REC/receipts/P --payer-keypair $N9_KEYS/deployer.json --expect-error escrow:6007
```

Saída esperada: `AlreadyReleased`, `Custom":6007`,
`verifier_invoked=false`, `watched accounts unchanged=true`.

Fala: "Liquidar de novo falha: o Job já está encerrado."

**Cena 8 — reembolso por timeout.**

```bash
vc job refund-timeout --job-id $TJ --payer-keypair $N9_KEYS/buyer.json --wait
```

Se o prazo ainda não passou, a CLI mostra `refund.wait … slots_to_go=…` e
espera. No D9 a espera foi de 2 min 24 s; corte a espera na edição, rotulada.

Saída esperada:
- `[refund_on_timeout] PASS … err=null`;
- `job.status=RefundedOnTimeout`;
- `refund.buyer_amount_before=Y after=Y+1000000`.

Fala: "O primeiro Job venceu sem entrega. Qualquer um pode pedir o
reembolso, e o Test USDC volta ao comprador."

**Cena 9 — encerramento (30 s).** Mostre a tabela do README e diga a frase
3 (limite) e a frase 5 (só devnet e Test USDC).

Duração bruta esperada: ~10 min de terminal. O prazo de T vence ~6 min
depois da cena 2.

### 3. Se algo falhar

- **Simulação diferente do esperado:** a CLI não envia nada. Pare a tomada;
  não troque o código de erro, e nunca use `escrow:6000` a `6003`.
- **RPC fora do ar ou HTTP 429 persistente:** use o plano B, dizendo que é
  uma execução anterior.
- **`compress` com exit 137:** repita uma vez, sozinho. Se falhar de novo,
  use a saída do D9 rotulada "execução anterior".

## Plano B: evidência já executada

Se o RPC de devnet falhar durante a gravação, mostre as transações abaixo,
**dizendo que são de uma execução anterior** com os mesmos comandos.

**Execução anterior do D9** (2026-10-05, 23:36–23:43 -03:00, ambiente
limpo, CLI e prover deste repositório;
[`docs/d9-demo-results.md`](d9-demo-results.md)):

| Cena | Caso | Transação |
| --- | --- | --- |
| 2 | Job T′ criado e financiado | [`4eATVTWf…`](https://explorer.solana.com/tx/4eATVTWfUgo6EB7GbaEFttbNhKG7fbR2AtXcS3xxWbF46N8Z4sdCaqSryXbKJrCGXagv7cVVHcahLHekXPCQ9Mms?cluster=devnet) |
| 2 | Reembolso antes do prazo em T′ → 6021 | [`3BKnczeK…`](https://explorer.solana.com/tx/3BKnczeKPQd15iCkYfPbnEvDbYayRfbVV9Zokqn3xLyVehJuZSNffgDy1paFTxof8tpYfaN92N789LzB2w4DQTd6?cluster=devnet) |
| 3 | Job P′ criado e financiado | [`3ME9F9HA…`](https://explorer.solana.com/tx/3ME9F9HAUhutBG8j9GzCXjVSqK9kf2cyitYdRihW8pVPqMZ4YGR81DfahWAtHk9dz1z4X6VeMRPaEg9nqFSGE4Lr?cluster=devnet) |
| 5a | Receipt do Job P do D7 em P′ → 6014 | [`8Kk5UkX5…`](https://explorer.solana.com/tx/8Kk5UkX5XW8tssPMxNSCEBt71jiv4RriHLDtYqRzb1f1U5WLZfgQmotQTD1ZqecEbyqMmhWSjQqWpXxCJUubJLg?cluster=devnet) |
| 5b | Seal adulterado em P′ → verificador 6003 | [`HXgnGUhh…`](https://explorer.solana.com/tx/HXgnGUhhB4zRgD8Bubs6ULCqpuhoqBSuim3Vbx1kz7V3bK2FFJG3ND2jcTtRFf6QxD9i1bpwKW4z32yFYYEofVr?cluster=devnet) |
| 6 | PASS de P′: `deliver`+`release`, verificador invocado (99.541 CU) | [`5tjezXYh…`](https://explorer.solana.com/tx/5tjezXYhN361HHUiZcMcJQFXwViWc4cSUfdreHfokdXuB89LrLDt6WPE8KRLhHwNE7rx5nGAgxjdfod6r7pPvDs1?cluster=devnet) |
| 7 | Dupla liquidação em P′ → 6007 | [`5T9XE5Y3…`](https://explorer.solana.com/tx/5T9XE5Y3DzqKffoeFFzEhgqxsuzqMstpxtMCQikrZPe1RyP1KWvRmPNs9CFuke1yBgtwg5vVnGZVpwHWZr31Dfrs?cluster=devnet) |
| 8 | Refund por timeout de T′ | [`33ezPvow…`](https://explorer.solana.com/tx/33ezPvow48vSFHYS43i76pDJwkpyKTnDCN3Tdr1p7he8mbHjEpXvsBkt2MrMHKPUNW6AbqWE7oRJ7MriRZNxrjkh?cluster=devnet) |

**Execuções anteriores do D7 e do D4b** (`docs/d7-cli-results.md`,
`docs/d4b-devnet-results.md`):

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
