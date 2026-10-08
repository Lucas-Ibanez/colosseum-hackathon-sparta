# Roteiro da demo (D9; pela interface desde o D11–D12)

Para quem vai apresentar ou gravar a demo do MVP. Escrito no D7 e executado
no D9 num ambiente limpo (clone e targets novos, com toolchains isoladas
copiadas; não uma máquina nova), com as escritas W1–W8 em devnet
([`docs/d9-demo-results.md`](d9-demo-results.md)). O vídeo de reserva é
gravado pelo humano seguindo a seção [Gravação](#gravação).
O vídeo definitivo, em inglês, numa tomada contínua pela interface Hive, segue a
seção [Gravação pela interface](#gravação-pela-interface) (D11–D12; roteiro final em
inglês no D12a).

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

São as mesmas do [`docs/README.pt-BR.md`](README.pt-BR.md#frases-permitidas-congeladas-no-d9). Mudar a lista exige decisão
registrada em `docs/decisions.md`.

1. **Claim:** "A receipt Groth16 da Hive é verificada em devnet por CPI
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

### Frozen phrases in English (ratified R-UI)

Texto oficial em inglês das frases acima (decisão D-EN-1, entrada "R-UI" de
`docs/decisions.md`), igual à seção "Frozen phrases (English, ratified R-UI)" do
[`README.md`](../README.md). É o que se diz no vídeo; o português continua válido e
equivalente.

1. **Claim:** "Hive's Groth16 receipt is verified on devnet via CPI to the immutable
   Groth16 verifier of risc0-solana v3.0.0, in the same instruction that releases or
   refunds the Job's Test USDC." Always with the link of a transaction.
2. **Statement:** "A previously committed deterministic evaluator ran on the artifact
   delivered in the Job and produced the published verdict. The program moves the Test
   USDC only after checking the binding between journal, Job and delivery and verifying
   the proof."
3. **Limit:** "The proof attests to the execution of the fixed rule on this artifact,
   not to the quality of a piece of software. The v1 rule is trivial (output = 2 ×
   input) and serves to demonstrate the flow."
4. **No administrator:** "The escrow and the verifier are immutable (upgrade authority
   `none`). No one, not even the project, changes the rules or decides the payment;
   there is also no e-stop."
5. **Network:** "Devnet and Test USDC only; none of this exists on mainnet."
6. **Negatives:** "On devnet, a tampered proof, a journal from another Job, a refund
   before the deadline and a second settlement were rejected without moving funds."
   Always with the links.
7. **Reproduction:** "The flow was reproduced with this repository's CLI and prover in
   a clean environment: a fresh clone and fresh targets, with copied isolated
   toolchains, not a new machine." (D9)
8. **Proof:** "The proof is generated locally and compressed to Groth16 in a local
   Docker container, with no network access."

**Do not say:**
- "the code is correct";
- "trustless" or "nobody needs to trust anybody";
- "any repository";
- "Verifier Router" as the current path;
- "mainnet" or "real money";
- "ZK on-chain" without phrase 1;
- "audited", "private" or "new machine".

Also do not present an earlier run as live, nor show a receipt without saying which Job
it belongs to. Regra de operação (fora da lista congelada): o programa que rejeitou vem só
de `rejection` (C10-7), nunca de um código `escrow:6000`–`6003` (RD7-01).

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

## Gravação pela interface

Passo a passo do **vídeo definitivo, em inglês, numa tomada contínua pela interface
Hive** servida pelo worker local (D10) sobre os binários do D10a, com os mesmos Jobs do
início ao fim ("Vídeo de reserva" em `docs/decisions.md`). Roteiro da R-UI
([`docs/r-ui-review-results.md`](r-ui-review-results.md), seção 10), conferido no D12a
texto a texto contra a interface implementada, num ensaio completo sobre os executáveis
falsos ([`docs/d12a-results.md`](d12a-results.md)). As instruções estão em português; os
rótulos entre aspas são os da tela, em inglês, exatamente como aparecem.

- Jobs novos: **T** (prazo de 1.560 slots), **P** (`PASS`) e **F** (`FAIL`). Nunca
  reutilize um `job_id` consumido (S, A, B, C, P, T, P′, T′, `8ab4ee8d`, `8bce67f2`,
  `cd77e7bd`, `3e115ca8`, `104f9a21`, `bc334093`, T₃ `2c38ca66`, P₃ `26df9ef4`, F₃
  `7395a76d`). A interface só habilita os cenários adversariais em Jobs vistos desde a
  partida atual do worker (D-NEG); num Job antigo eles ficam desabilitados com "Available
  only for jobs created since the worker's current start."
- Falas: só as frases ratificadas em inglês (F1 a F8, seção "Frozen phrases in English"
  acima), lidas da tela ou indicadas pelo número (C-UI-7). O resto da narração só lê a
  tela. Diga de qual Job é a receipt do 6014. Nunca apresente uma execução anterior como
  ao vivo. F7 não se aplica a esta tomada.
- Tempos: os da prova vêm do ensaio em devnet do D11–D12; os demais, do ensaio do D12a.

### 1. Antes de gravar (fora da câmera, ~10 min)

1. **Commit e worker (C-UI-5).** O commit do D12a está feito e `git status --short` está
   vazio. Pare o worker em execução **pelo PID exato** (`ss -ltnp | grep 8710` mostra o
   PID; `kill <PID>`; nunca `pkill -f`) e inicie de novo, num terminal **fora da
   gravação**, para que o "Commit" da barra seja o `HEAD` atual:

   ```bash
   cd /home/lucas/src/vericode
   env -i HOME=~/.local/share/vericode-spikes/d10/home PATH=/usr/bin:/bin LANG=C.UTF-8 \
     python3 -B worker/vericode_worker.py --config ~/.local/share/vericode-spikes/d10/worker.json
   ```

   Saída esperada: hashes `e6cd4e29…`/`3f66e1c0…`/`2a8f75b8…`, `worker.buyer=EZgG…`,
   `worker.executor=EdB2…`, `check=ok`, `prover=LocalProver env=ok`,
   `worker.url=http://127.0.0.1:8710`, `worker.ui=http://127.0.0.1:8710/ui/` e a linha
   do token. **Esse terminal nunca aparece no vídeo.**
2. **Navegador.** Perfil limpo, sem extensões; recuse "salvar senha". Janela de 1440 px
   ou mais, zoom 100%. Abra **exatamente** `http://127.0.0.1:8710/ui/` (`localhost` dá
   421). Tela "Connect to the local worker": cole o token no campo "Worker token" (campo
   de senha) **fora da câmera** e clique em "Connect". Não recarregue a página durante a
   tomada: uma recarga volta à tela do token.
3. **Barra de status:** "Devnet", "Test USDC `9TE2…wV2F`", "`image_id` `4da06f90…fac0fb1a`",
   "Verifier `THq1…QUge`, immutable", "Escrow `GZqb…wkCH`, authority `none`", "Commit"
   igual a `git rev-parse --short HEAD`.
4. **Ritmo dos slots.** Em "New job", clique em "Reread the chain" e leia a estimativa
   ao lado do prazo (`~N min`, "estimate"). A ~0,24 s por slot, 1.560 slots dão ~6 min. Se
   der mais de 9 min, faça a cena 13 depois da 14.
5. **Memória (C-UI-5).** Feche builds, testes e outras provas; `MemAvailable` ≥ 3 GiB
   antes da cena 5 (`free -m`). No ensaio do D11–D12, o `compress` chegou a 337 MB livres
   no pico, sem exit 137.
6. **Orçamento (C-UI-6).** Cada tomada completa custa ao buyer 10.769.200 lamports (3
   rents + 5 taxas) e ao executor 25.000. Com o saldo do buyer no fim da R-UI
   (88.981.200), cabem **no máximo 8 tomadas**. Mais SOL de devnet exige um gate.
7. Opcional: abas do Explorer do escrow, do verificador e do mint (seção "Gravação",
   item 1).

### 2. Cenas, cliques e saída esperada

**Cena 1 — Environment (~30 s).** Tela "Jobs". Clique em "Details" na barra de status
(rodapé).

- "Environment details" mostra: "Escrow `vericode_escrow`" `GZqb…wkCH` com "upgrade
  authority `none`" e "SHA-256 of the bytes" `cdf6967f…`; "Groth16 verifier" `THq1…QUge`,
  "risc0-solana v3.0.0, immutable" (`34ae6e5c…`); "Test USDC" com "6 decimals, freeze
  authority `none`"; e, na própria tela, F4 e F5.
- Fala: **F4** e **F5**. Feche com Esc.

**Cena 2 — Job T (~15 s).** "New job" → no campo "Deadline, in slots from submission",
`1560` → "Fund job" → leia o diálogo → "Fund job".

- O diálogo "Fund job" mostra "`create_job` + `fund` in a single transaction, signed by
  the buyer in the local worker's CLI.", o programa, "Amount: 1.00 Test USDC from the
  buyer to the job vault." e o comando ("Command the worker runs").
- "Commitment summary": o `job_id` e o "Vault (job PDA)" assim que a CLI os imprime;
  depois "Job funded", a assinatura e "Read from chain at slot N".
- "Open job": "State" `Funded`, "Amount" 1.00 Test USDC, "Deadline" `slot N` com "1,560
  slots left at the last slot read".
- Mostre, sem comentar além do texto, "Limitations of this environment" no fim da página
  (P3).

**Cena 3 — Refund before the deadline em T (~20 s)**, logo em seguida: o envio exige pelo
menos 300 slots de margem.

- "Adversarial scenarios (demonstration)" → "Request refund before the deadline" →
  confirmar no diálogo ("Expected: the transaction is rejected (`--expect-error
  escrow:6021`).").
- Resultado: "Transaction rejected: deadline not reached yet", "Rejected by
  `vericode_escrow`: `DeadlineNotReached` (6021)", "The job state was not changed.", "Read
  before sending: slot …, deadline …, margin of … slots (minimum 300)." e o link.

**Cena 4 — Job P (~15 s).** "New job" → `9000` → "Fund job" → confirmar → "Open job".

- Aponte, em "Commitments and observed", `spec_hash`, `harness_hash` e `image_id` na
  coluna "Committed in job" (a tela "New job" os mostra como "Fixed by the program").

**Cena 5 — Proof of P (~3 min 20 s).** Em "Operations": "Input" `21`, "Claimed output"
`42` → "Generate proof" → confirmar.

- "Generating proof" com "Elapsed time" pelo relógio do worker e "Current step:
  `prove`", depois `compress`; o estado `Proving` aparece com "(local, off-chain)". No
  ensaio do D11–D12: 7,2 s de `prove` e 177,9 s de `compress`.
- Fala: **F8**. Se a edição acelerar a espera, rotule "accelerated" e não corte a tomada.
- Ao fim: "Proving time"; "Local verification" com `prove.receipt_type=Composite`,
  `compress.receipt_type=Groth16`, "Admitted `image_id`", a linha "`docker_run` line" e
  F8; "Commitments and observed" com o journal decodificado ("Matches").

**Cena 6 — Tampered proof em P (~20 s).** "Submit tampered proof" → confirmar.

- Resultado: "Transaction rejected: invalid proof", "Rejected by the Groth16 verifier:
  `PairingError` (6003)", "The job state was not changed."

**Cena 7 — PASS settlement de P (~25 s).** "Submit delivery and proof" → o diálogo diz
"`deliver` + `release` in a single transaction…" e "The receipt's journal says `PASS`." →
confirmar.

- Resultado: "Settled: Released", "Criteria met (`PASS`). Reason: verified PASS verdict."
  com "View on Explorer" e "Transaction anatomy".
- "On-chain verification": "Proof verified on-chain by the Groth16 verifier (direct
  CPI).", "Program" `THq1…QUge`, "Verifier compute units" 99,541 CU, "Result" "`Verify`
  succeeded", e **F1 ao lado do link** ("View on Explorer").
- "Transaction anatomy": "Instruction 1" `vericode_escrow` `Deliver`; "Instruction 2"
  `vericode_escrow` `Release`, com as CPIs "Groth16 verifier" `Verify` (99,541 CU) e "SPL
  Token" `TransferChecked`, numa só transação.
- "Balances before and after": "Job vault" 1.00 → 0.00 (−1.00); "Executor ATA" +1.00.
- Fala: **F1**, apontando o link; depois **F2**. Abra o link do Explorer e mostre
  `Program THq1… invoke [2]` antes da transferência do Token.

**Cena 8 — Settle again em P (~20 s).** "Settle again" → confirmar.

- Resultado: "Transaction rejected: job already settled", "Rejected by
  `vericode_escrow`: `AlreadyReleased` (6007)".

**Cena 9 — Job F (~15 s).** "New job" → `9000` → "Fund job" → confirmar → "Open job".

**Cena 10 — Another job's receipt em F (~25 s).** Antes da prova de F.

- Em "Receipt of job", confira que o selecionado é o `job_id` de **P** (mesmo prefixo do
  cabeçalho de P; a lista vem do Job mais recente para o mais antigo) → "Submit another
  job's receipt" → confirmar.
- Fala: diga que a receipt é do Job P, desta mesma gravação.
- Resultado: "Transaction rejected: binding mismatch", "Rejected by `vericode_escrow`:
  `JournalJobIdMismatch` (6014)", "The job state was not changed.", "Receipt of job …,
  another job of this worker." e a tabela com `job_id` em "Mismatch" e os demais campos em
  "Matches".

**Cena 11 — Proof of F (~2 min).** "Input" `7`, "Claimed output" `15` → "Generate
proof" → confirmar.

- Ao fim: "Usable receipt: `FAIL`." (no ensaio do D11–D12, 8,6 s + 91,4 s).

**Cena 12 — FAIL settlement de F (~25 s).** "Submit delivery and proof" → o diálogo diz
"`deliver` + `refund_on_fail` …" e "The receipt's journal says `FAIL`." → confirmar.

- Resultado: "Criteria not met (`FAIL`)", "A verified result, not an error: the Test USDC
  went back to the buyer. Reason: verified FAIL verdict."; "On-chain verification" com o
  verificador invocado (99,541 CU) e F1 ao lado do link; "Buyer ATA" +1.00.
- Fala: **F6**, com os resultados dos quatro negativos na tela (links).

**Cena 13 — Refund of T by deadline (~40 s).** "Jobs" → T (confira o prefixo) →
"Reread the chain".

- Antes do prazo, "Request refund" fica desabilitado com "Available from slot N. Reread
  the chain to check the current slot."
- Quando o último `job show` diz que o prazo passou, "Request refund" habilita →
  confirmar.
- Resultado: "Deadline passed without settlement", "The Test USDC went back to the buyer
  by an objective condition: deadline passed without settlement."; estado
  `RefundedOnTimeout`; "On-chain verification" diz "The refund-by-deadline transaction
  did not invoke the verifier (`verifier_invoked=false`)."

**Cena 14 — Closing (~40 s).** Role até "Limits of this proof" ("What this verification
shows" / "What it does not show") e leia **F2** e **F3**; termine com **F5** e a lista
"Jobs" (T, P e F).

Duração bruta: ~13 min (no ensaio do D11–D12, 11 min 16 s de U1 a U10).

### 3. Se algo falhar (plano B)

| Sintoma na tela | O que fazer |
| --- | --- |
| "Another operation is running in the worker" (409) | Esperar a operação terminar. |
| "The worker is waiting for a chain reread before any other operation" | "Reread the chain" no Job indicado. Se a releitura de reconciliação falhou, C e D continuam coerentes com a cadeia, com a nota "The reconciliation read of this operation failed; the state came from the chain reread." |
| "No confirmed rejection" | Parar a tomada; não repetir o negativo; registrar. |
| "Operational failure: no verdict" | A receipt nunca é usada. Fechar outras janelas e clicar em "Generate proof" uma vez; se falhar de novo, parar. |
| Prazo de T ainda não venceu na cena 13 | Fazer a cena 14 e voltar a T, sem cortar. |
| Tela do token no meio da tomada | Colar o token no campo mascarado, sem mostrar o terminal. Se o worker reiniciou, os Jobs da tomada ficam travados para os cenários adversariais: a tomada recomeça com Jobs novos (C-UI-6). |
| RPC de devnet fora do ar | Mostrar as tabelas de "Plano B: evidência já executada" abaixo, dizendo que são uma execução anterior (D11–D12 ou D9). |

## Demo técnica (corte de 2–3 min)

Lista de cortes do **vídeo de demo técnica** da submissão (D13), montado a partir da
tomada contínua da seção [Gravação pela interface](#gravação-pela-interface). A tomada
completa, sem cortes, fica publicada à parte como evidência ("Decisões humanas depois do
D12a" em `docs/decisions.md`). Instruções em português; rótulos de tela e narração em
inglês. Os tempos de cada cena da tomada só serão conhecidos depois da gravação (10/10):
esta lista é plano até lá.

### Regras do corte

- Só material da **mesma tomada contínua**, na ordem em que aconteceu. Nada de outra
  tomada, de ensaio ou de execução anterior. Se o Plano B for usado na gravação, a demo diz
  "earlier run" na tela, nunca o apresenta como desta tomada.
- **Todo trecho removido** leva um rótulo na tela, em caixa fixa, por pelo menos 2 s:
  `Cut: <o que saiu>`. **Toda aceleração** leva `Accelerated N×` durante todo o trecho,
  com o N real da edição. Nenhum rótulo diz "live" ou "real time".
- A frase 1 só aparece junto do link da transação de liquidação visível na tela ("View on
  Explorer" ou o Explorer aberto em `?cluster=devnet`).
- No 6014, a narração diz que a receipt é a **do Job P desta mesma tomada**.
- A frase 6 **não** é dita: o corte mostra só três dos quatro negativos (o 6007 sai).
- O último quadro mostra o link da tomada completa (`TODO(video)`).
- A narração é uma locução gravada na edição. Ela descreve o que a tela mostra e usa só as
  frases ratificadas (F1, F3, F4, F5 e F8) e fatos do código e da evidência, com a fonte
  indicada abaixo. Sem termos da lista "Do not say".

### Cortes (alvo 2:55)

| # | Cena da tomada | Duração no corte | Rótulo na tela |
| --- | --- | ---: | --- |
| 1 | cartão de abertura (sem tela da tomada) | 0:10 | "Hive technical demo. A labeled cut of one continuous take recorded on `TODO(video)`. Full take: `TODO(video)`" |
| 2 | 1 Environment ("Details" da barra de status) | 0:20 | — |
| 3 | 2 Job T (New job → Fund job → Open job) | 0:10 | `Cut: dialog reading` |
| 4 | 3 refund antes do prazo em T (6021) | 0:08 | — |
| 5 | 4 Job P (Fund job → Open job, "Commitments and observed") | 0:06 | `Cut: job creation of P` |
| 6 | 5 prova de P (`prove`, `compress`; ~3 min 20 s na tomada) | 0:20 | `Accelerated N×` |
| 7 | 6 prova adulterada em P (6003) | 0:09 | — |
| 8 | 7 liquidação PASS de P, anatomia, saldos e Explorer | 0:40 | — |
| 9 | 8 liquidar de novo em P (6007) | 0:00 | `Cut: settle again on P (rejected, 6007)` |
| 10 | 9–10 Job F e receipt de P em F (6014) | 0:14 | `Cut: job creation of F` |
| 11 | 11–12 prova de F e liquidação FAIL | 0:14 | `Accelerated N×` na prova |
| 12 | 13 refund de T por prazo | 0:09 | `Cut: waiting for the deadline` |
| 13 | 14 encerramento ("Limits of this proof") e link da tomada | 0:15 | "Full take: `TODO(video)`" |

Total: 2:55. Se passar de 3:00 depois da edição, encurte nesta ordem: as linhas 3, 5 e 12
(com o rótulo `Cut`). **Nunca corte** a linha 8 (liquidação verificada com o link), um
negativo e a frase 3.

### Narração (inglês, ~400 palavras com as frases ratificadas, ~2,4 palavras por segundo)

| # | Narração | Fonte |
| --- | --- | --- |
| 1 | "One continuous take through the Hive interface, served by a local worker over this repository's Rust CLI and prover. Every cut and acceleration is labeled." | `worker/README.md`; "Decisões humanas depois do D12a" |
| 2 | (na tela, o Anchor program `vericode_escrow` e o verificador) F4: "The escrow and the verifier are immutable (upgrade authority `none`). No one, not even the project, changes the rules or decides the payment; there is also no e-stop." F5: "Devnet and Test USDC only; none of this exists on mainnet." | F4, F5 |
| 3 | "In the Anchor program, the buyer creates and funds a Job in one transaction, `create_job` and `fund`. The Test USDC goes to a vault whose authority is the Job's PDA." | `docs/d11-ui-results.md` (U1); `docs/escrow-program.md` |
| 4 | "A refund before the deadline is rejected by the escrow: `DeadlineNotReached`, 6021. Nothing moves." | cena 3; `docs/d11-ui-results.md` (U2) |
| 5 | "A second Job, P. The spec, the harness and the image ID are fixed by the program." | cena 4; `HIVE_MVP_UI_ADAPTATION.md` §3.1 |
| 6 | "The executor proves locally. The RISC Zero guest evaluates the artifact, 21 and 42, against the fixed rule: output equals two times input. The Rust core it runs is `no_std`, with no Solana or RISC Zero dependency." F8: "The proof is generated locally and compressed to Groth16 in a local Docker container, with no network access." | `prover/README.md`; `docs/d1c2b3g1-core-no-std-results.md`; F8 |
| 7 | "The same proof with one bit flipped: the Groth16 verifier itself rejects it, `PairingError`, inside the same transaction." | cena 6; `docs/d11-ui-results.md` (U5) |
| 8 | "`deliver` and `release` go in a single transaction. Inside `release`, the escrow checks the journal against the Job and the delivery, calls the verifier by CPI, 99,541 compute units, and only then moves the Test USDC with `TransferChecked`." F1, apontando o link: "Hive's Groth16 receipt is verified on devnet via CPI to the immutable Groth16 verifier of risc0-solana v3.0.0, in the same instruction that releases or refunds the Job's Test USDC." "The escrow calls the verifier directly, not through the Verifier Router: the upstream Router on devnet is not initialized." | cena 7; `docs/d11-ui-results.md` (U6); `docs/d4a-direct-verifier-results.md`; F1 |
| 10 | "A third Job, F. We submit the receipt of Job P, from this same take. The escrow rejects it, `JournalJobIdMismatch`, 6014: the journal belongs to another Job." | cena 10; D-6014 |
| 11 | "F is proved with 7 and 15, which does not meet the rule. The `FAIL` verdict goes through the same verifier CPI, 99,541 compute units, and the Test USDC goes back to the buyer." | cenas 11–12; `docs/d11-ui-results.md` (U9) |
| 12 | "After the deadline, the refund of T needs no proof: `refund_on_timeout` returns the Test USDC to the buyer." | cena 13; `docs/d11-ui-results.md` (U10) |
| 13 | F3: "The proof attests to the execution of the fixed rule on this artifact, not to the quality of a piece of software. The v1 rule is trivial (output = 2 × input) and serves to demonstrate the flow." | F3 |

A linha 9 não tem narração (só o rótulo). "Verified on devnet" só aparece dentro da F1, dita
na linha 8 com o link na tela; nenhuma outra fala diz "verified on-chain".

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

**Execução anterior pela interface (D11–D12)** (2026-10-07, 13:17–13:29 -03:00,
ensaio U1–U10; [`docs/d11-ui-results.md`](d11-ui-results.md)). Cenas da seção
"Gravação pela interface" (numeração do roteiro do D12a; no ensaio, a receipt do 6014 era
a do Job `bc334093…`, aplicada em P₃):

| Cena | Caso | Transação |
| --- | --- | --- |
| 2 | Job T₃ criado e financiado | [`8AJzbbKf…`](https://explorer.solana.com/tx/8AJzbbKfaeKSpjRxzX2wTu1R4DxpujLgT7z2ZRsUHjeUGDMka8bAkyriXZJ7ZuCNTfY3zYzq3rQWr1P4yoABDNo?cluster=devnet) |
| 3 | Reembolso antes do prazo em T₃ → 6021 | [`4jDL97DG…`](https://explorer.solana.com/tx/4jDL97DGJRLQhB2m2NKK9kof85EFHSQMyPDPMmdhQ9mm6nkRKcbFF9HCuXcEiooWKKWomBCuhhTd2KbpYFAGiRHF?cluster=devnet) |
| 4 | Job P₃ criado e financiado | [`5vWoxx2m…`](https://explorer.solana.com/tx/5vWoxx2mBra2iP5TfBX9FeYUjt5fujMde7m7saWHpo1kGEksZbrwVy66L9X6DqMLqDDNbk6BR5P6UjXqtZK5gzT6?cluster=devnet) |
| 6 | Seal adulterado em P₃ → verificador 6003 | [`kTQdVojx…`](https://explorer.solana.com/tx/kTQdVojxjzJULzS97Jo5HBFRcroqnYVMaK9Crtx2LmyAY1pUhnBgh7v1Xs686wY3gvvxGz3aiN9URWhNgepRg39?cluster=devnet) |
| 7 | PASS de P₃: `deliver`+`release`, verificador invocado (99.541 CU) | [`2gB9djwM…`](https://explorer.solana.com/tx/2gB9djwM3WpApHLXYf5riF974mTK5W4UW7HCv1xJamLPu3NEwnPcCY4oT6zAwvbMkaKRcATf1LCQPyXP8rmpj3o?cluster=devnet) |
| 8 | Dupla liquidação em P₃ → 6007 | [`j2pmqFWu…`](https://explorer.solana.com/tx/j2pmqFWuTSe4H6isKyEFX3cBYmufEANP9Ue1p74tTUawXbiPSQ4KqckPW6WX3gnoAWsXE5XBdeVHEevpLa852ky?cluster=devnet) |
| 9 | Job F₃ criado e financiado | [`2jjiYDAo…`](https://explorer.solana.com/tx/2jjiYDAoCQTrF93LvTAFB3RHTB3ab86k2BZmRXnFv3FmgK2ps2Kf1U2vs4MeFMMo7iqDQ77TSqz9hjigaQMrFQbW?cluster=devnet) |
| 10 | Receipt do Job `bc334093…` em P₃ → 6014 | [`5CBFajsa…`](https://explorer.solana.com/tx/5CBFajsaFvYreCJkQwiPuR8rHWtpBKCULoVAeysUJGRwTDfF2CXYzeFkuit6azjBjqQkoif8TbYwxByv8Ckmo4en?cluster=devnet) |
| 12 | FAIL de F₃: `deliver`+`refund_on_fail`, verificador invocado (99.541 CU) | [`4jcugMyX…`](https://explorer.solana.com/tx/4jcugMyXqhAttG7cjcjWZnMiyZe3SosHC2cgphQ6rf48n5KfrWZ4TRfzR1ZioP5NwYE2qeo2zUDEcD4rJQDTCmSJ?cluster=devnet) |
| 13 | Refund por timeout de T₃ | [`5oTbFiTX…`](https://explorer.solana.com/tx/5oTbFiTXjkv8aBgTQPrKSqTShfNgVqSePZuP5x57GXxfEi6mTPMLfXFCWpnZSQzgcXbyqSjBHSJy65dEy5aAKVzi?cluster=devnet) |

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
