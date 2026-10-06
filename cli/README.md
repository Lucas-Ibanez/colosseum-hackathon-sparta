# `vericode` — cliente de devnet do escrow VeriCode

Cliente de linha de comando (gate D7) que cria, financia, entrega, liquida e
reembolsa Jobs do programa `vericode_escrow`, implantado e **imutável** em
Solana devnet. Ele usa as receipts Groth16 de [`vericode-prover`](../prover/README.md).

A CLI não decide nada econômico: toda regra é imposta pelo programa (e, nele,
pelo `vericode-core`). Ela confere antes de enviar, para que um erro não custe
taxa, e mostra o link do Explorer de cada transação.

## Constantes (iguais às do programa; testadas em `tests/instructions.rs`)

| Item | Valor |
| --- | --- |
| Escrow | `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH` (ProgramData `B7s9JJVy…`, 395.064 B `cdf6967f…`, upgrade authority `none`) |
| Verificador Groth16 | `THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge` (`risc0-solana v3.0.0`, imutável) |
| Mint admitido | Test USDC `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F` (6 decimais, sem freeze) |
| Termos v1 | spec `af642b56…b778`, harness `01124025…6b50` (calculados pelo core), ImageID `4da06f90…fb1a` |
| Selector | `73c457ba` |
| Janela de prazo | `slot + 1.500 ≤ prazo ≤ slot + 1.512.000`; a CLI exige `--deadline-offset` ∈ [1.560, 1.512.000] (60 slots de margem para a transação aterrissar) |
| IDL | discriminadores e contas da IDL do D4a (`e8ce2c20…`), montados à mão; nenhuma dependência de Anchor no binário |

## Build e testes (a partir da raiz de um clone)

```bash
cargo +1.89.0 build --locked --release --manifest-path cli/Cargo.toml
cargo +1.89.0 test  --locked --manifest-path cli/Cargo.toml
```

O workspace e o lock são próprios (`cli/Cargo.lock`, semeado de
`anchor/tests-local/Cargo.lock`, sem fonte git). As dependências de teste são
as da suíte do programa: os testes comparam, byte a byte, cada instrução da
CLI com os builders de `anchor/tests-local/tests/common/mod.rs`, e os
decoders com Anchor e `spl_token`. Os testes não usam rede nem keypair.

## Comandos

Opções globais: `--rpc-url URL` (padrão `https://api.devnet.solana.com`) e
`--log ARQUIVO.jsonl` (cada transação, com simulação, logs, CU e fee;
criado com modo `0600`). A CLI recusa qualquer cluster cujo genesis não seja
o de devnet (`EtWTRABZ…`).

```bash
V=cli/target/release/vericode
RUN=~/vericode-run; mkdir -p $RUN          # receipts e registros fora do clone

$V check                                    # escrow, ProgramData, verificador, mint (só leitura)

# buyer: cria e financia na MESMA transação (create_job + fund)
$V job create --buyer-keypair ~/keys/buyer.json --executor <PUBKEY_DO_EXECUTOR> \
   --amount 1000000 --deadline-offset 9000 --job-file $RUN/job.json

# executor: prova com o prover (ver prover/README.md), depois entrega e libera
# na mesma transação (deliver + release); FAIL vira refund_on_fail ao buyer
$V job settle --job-id <JOB_ID_HEX> --receipt $RUN/P --deliver --executor-keypair ~/keys/executor.json

# alternativa em duas etapas
$V job deliver --executor-keypair ~/keys/executor.json --job-id <JOB_ID_HEX> --artifact 21,42
$V job settle --job-id <JOB_ID_HEX> --receipt $RUN/P --payer-keypair ~/keys/qualquer.json

# qualquer pagador: reembolso depois do prazo (--wait espera o slot)
$V job refund-timeout --job-id <JOB_ID_HEX> --payer-keypair ~/keys/buyer.json --wait

$V job show --job-id <JOB_ID_HEX>           # Job, vault, ATAs e saldos (só leitura)
```

### Cenários negativos (demonstração)

`--expect-error PROGRAMA:CÓDIGO` (`escrow`, `verifier`, `token` ou `system`)
envia de propósito uma transação que o programa deve rejeitar:
1. as conferências da CLI que falhariam viram `precheck.expected_rejection=…`;
2. a simulação precisa mostrar `Program <id> failed: custom program error:
   0x…` com `Custom(CÓDIGO)`, senão nada é enviado;
3. a transação vai com `skipPreflight`, aterrissa com erro, e Job, vault e as
   duas ATAs têm de estar byte a byte iguais antes e depois.

`--tamper-seal` (só com `--expect-error`) inverte um bit de `pi_c`, para ver
o verificador rejeitar a prova (`verifier:6003`).

Limites conhecidos do modo negativo (R-D7; correção prevista no D10a):
- **RD7-01:** quando a CPI ao verificador falha, o log também traz `Program
  GZqb… failed` com o mesmo código. Os códigos 6000 a 6003 do verificador
  coincidem com os do escrow, então `escrow:6000` a `escrow:6003` podem
  passar com uma falha que foi do verificador. Use só `escrow:6014`,
  `escrow:6007`/`6008`, `escrow:6021` e `verifier:6003`.
- **RD7-04:** os testes de `cli/` cobrem só o parse de `--expect-error`; o
  casamento com a transação e o `--tamper-seal` não têm teste unitário. Em
  devnet, o D9 exercitou os dois pela CLI: `escrow:6021`, `escrow:6014`,
  `--tamper-seal` com `verifier:6003` e `escrow:6007`
  ([`docs/d9-demo-results.md`](../docs/d9-demo-results.md)).
- **RD7-08:** um negativo `escrow:6021` enviado perto do prazo pode virar
  reembolso real; a CLI então reporta `UNEXPECTED` (exit 1). Envie-o com pelo
  menos 60 slots de folga.

```bash
# journal de outro Job → 6014, antes da CPI (diga de qual Job é a receipt)
$V job settle --job-id <P> --receipt $RUN/A --deliver --executor-keypair … --expect-error escrow:6014
# seal adulterado → o verificador rejeita dentro da mesma transação
$V job settle --job-id <P> --receipt $RUN/P --deliver --executor-keypair … --tamper-seal --expect-error verifier:6003
# dupla liquidação (invariante 9) → 6007 / 6008
$V job settle --job-id <A> --receipt $RUN/A --payer-keypair … --expect-error escrow:6007
# reembolso antes do prazo → 6021
$V job refund-timeout --job-id <T> --payer-keypair … --expect-error escrow:6021
```

## Conferências antes de cada operação

- cluster devnet (genesis); escrow executável, com ProgramData `B7s9JJVy…` e
  upgrade authority `none`;
- mint `9TE2V…`: SPL Token clássico, 82 bytes, inicializado, **6 decimais e
  sem freeze authority** (o programa não confere decimais; a CLI confere);
- termos da v1 recalculados pelo `vericode-core` e, num Job existente, campos
  iguais a eles;
- `job_id` novo de 32 bytes do gerador aleatório do sistema operacional
  (`getrandom`), diferente de `0x11…`, com PDA livre;
- prazo com margem; saldo de Test USDC e de SOL (rent do Job e do vault +
  taxa) suficientes, senão a CLI informa a pubkey e o valor;
- receipt: journal de exatamente 165 bytes decodificado pelo core, `job_id`
  igual ao Job, spec/harness/ImageID iguais aos termos, `journal_digest` =
  SHA-256(journal), selector `73c457ba`, `pi_a` negado pela CLI (o prover
  grava o seal cru);
- estado: `Funded` para entregar; `Delivered` com o mesmo `artifact_hash`
  para liquidar; prazo vencido para o reembolso por timeout;
- destino = ATA canônica da parte paga; se não existir, a CLI acrescenta
  `CreateIdempotent` do programa ATA na mesma transação.

## Envio

`getLatestBlockhash` → `simulateTransaction` (com verificação de assinatura) →
`sendTransaction` → reenvio da mesma transação a cada ~6 s →
`getSignatureStatuses` até `confirmed` ou expirar o blockhash →
`getTransaction` (slot, CU, fee, logs). O RPC público limita rajadas (HTTP
429): as chamadas são espaçadas em pelo menos 250 ms e repetidas com backoff.
Sem compute budget nem priority fee.

## Chaves

- Só keypairs de **devnet**, passados por caminho (`--buyer-keypair`,
  `--executor-keypair`, `--payer-keypair`).
- A CLI recusa arquivos dentro de qualquer work tree Git (este repositório
  incluído) e arquivos com permissão para grupo ou outros (use `0600`).
- Ela nunca imprime conteúdo de chave; só pubkeys. Erros de leitura não
  mostram o arquivo.

## Limites

- Só devnet e Test USDC. Quem cria Jobs precisa de Test USDC do mint
  admitido, cuja mint authority é a chave de deployer do projeto.
- `job_id` e Jobs são públicos; o rent do Job e do vault (~0,0036 SOL) fica
  preso, porque não há `close`.
- A prova atesta a execução da regra fixa (`saída = 2 × entrada`) sobre o
  artefato entregue, não a qualidade de um software.
