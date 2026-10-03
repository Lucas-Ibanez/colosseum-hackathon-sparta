# D1c2b.3i — receipts VeriCode locais reais

Data: 2026-10-03

## Resultado

**GO** para auditoria final do marco D1c2b. O host final executou os cenários
PASS e FAIL, produziu três receipts locais reais do tipo `Composite`, verificou
os receipts contra o ImageID final e rejeitou os três casos negativos
exigidos. Uma segunda ferramenta temporária desserializou e reverificou os
arquivos persistidos de forma independente.

Não houve dev mode, Bonsai, rede, Solana, Anchor, wallet, validator,
Router/CPI, blockchain, deploy, front-end ou push. Router/CPI/devnet
permanecem `STATUS: NÃO VALIDADO`.

## Baseline e ambiente

- HEAD: `dc41168b808e039cf36943a18df99d74fcb677ce`;
- árvore inicial e final do gate: limpa;
- lock host: `f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226`;
- lock guest: `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
- método combinado: 180.300 bytes, SHA-256
  `e09ba8cf16f7e36cb92e10656b7574c598f3e88fe089f02bc747a4bb00c078f5`;
- ImageID final:
  `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a`;
- binário host temporário: 100.497.528 bytes, SHA-256
  `80448c830828e047c1dbbfcd04a1bece136c0546b1d47fedcf4f2fb04f1c08bc`.

Todo comando Rust/RISC Zero declarou as homes prescritas,
`RUSTUP_AUTO_UPDATE=0` e `CARGO_NET_OFFLINE=true`. O ambiente removeu
explicitamente `RISC0_DEV_MODE`, `BONSAI_API_URL` e `BONSAI_API_KEY`, e fixou
`RISC0_PROVER=local` e `RISC0_EXECUTOR=local`.

O manifest host ativa simultaneamente as features `prove` e
`disable-dev-mode` de `risc0-zkvm 3.0.3`. A fonte cacheada confirma que
`RISC0_PROVER=local` seleciona `LocalProver` em processo e que
`disable-dev-mode` impede receipts falsos mesmo se a variável dev fosse
acidentalmente configurada.

## Execução PASS e FAIL

O comando `vericode-host execute` terminou com exit `0`, reportou
`elf_bytes=180300` e o ImageID final. Os dois journals foram comparados pelo
host byte a byte com os vetores locais esperados e têm 165 bytes:

| Cenário | Artifact hash | ImageID | Verdict |
| --- | --- | --- | --- |
| PASS | `9223d6d2a1ba23bd73ca325b411c75027a739c285a95ff63b963442a224c` | `4da06f90…fb1a` | `00` (`Verdict::Pass`) |
| FAIL | `343ad7781fe806e047dd7f965e2a41713ce0f668f4d88ebf351d21ef88ee55c6` | `4da06f90…fb1a` | `01` (`Verdict::Fail`) |

Os dois compartilham schema `1`, Job ID `[0x11; 32]`, spec hash
`af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778`
e harness hash
`01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50d5aa`.
`Verdict::Fail` foi saída normal do guest e do host, nunca panic ou erro
operacional.

## Proving e arquivos persistidos

O comando `vericode-host prove <diretório-temporário>` terminou com exit `0`
em 21,9 s. Antes de gravar PASS/FAIL, o próprio host chamou `Receipt::verify`
com o ImageID final e validou journal/verdict. O caso `wrong-image` também
prova o mesmo guest real, mas recebe um ImageID público adulterado para testar
o vínculo do journal.

| Arquivo temporário | Tipo | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `pass.receipt` | `Composite` | 221.540 | `5dcf89f5ff2417fe9615157857242e0bf8d8c17f8dd5676b521d927c89dd2d3f` |
| `fail.receipt` | `Composite` | 221.540 | `3477a592c631afaad424f81c80fa2f97abf2dae23f6420fd24cd4733a671efb8` |
| `wrong-image.receipt` | `Composite` | 221.540 | `189bd609311800f51b7b3e38c016e8028130d16b392a6d00915a146e14d08d04` |

PASS e FAIL são arquivos distintos por `cmp`. Todos permaneceram somente em
`/tmp/vericode-d1c2b3i-receipts.x2zrL1`; nenhum receipt ou binário entrou no
clone.

## Testes negativos reais

O host produziu e confirmou:

```text
negative.wrong_verify_image=rejected
negative.supplied_wrong_journal_image=rejected
negative.different_job_id=rejected
```

- o receipt PASS não verifica contra um ImageID com o primeiro byte invertido;
- o receipt `wrong-image` verifica criptograficamente contra o guest correto,
  mas seu journal é rejeitado por `ImageIdMismatch` contra os compromissos do
  Job;
- o journal PASS é rejeitado por `JobIdMismatch` contra Job ID `[0x22; 32]`.

## Auditoria independente dos arquivos

Um example criado somente no staging temporário, não no clone, leu os três
arquivos com `fs::read`, desserializou cada `Receipt` por `bincode`, rejeitou
explicitamente `InnerReceipt::Fake`, chamou novamente `Receipt::verify` com o
ImageID final e revalidou tamanho e campos dos journals.

Resultado:

```text
audit.pass=verified,type=Composite,receipt_bytes=221540,journal_bytes=165
audit.fail=verified,type=Composite,receipt_bytes=221540,journal_bytes=165
audit.wrong-image=verified,type=Composite,receipt_bytes=221540,journal_bytes=165
audit.negative.wrong_verify_image=rejected
audit.negative.supplied_wrong_journal_image=rejected
audit.negative.different_job_id=rejected
```

O auditor também confirmou que PASS/FAIL compartilham os compromissos
esperados, diferem no artifact hash/verdict, e que `wrong-image` altera apenas
o ImageID público em relação ao journal PASS.

## Riscos e transição

- Receipts e binários são evidência local real, porém efêmera em `/tmp`.
- O receipt produzido é `Composite`, não Groth16; nenhuma alegação on-chain é
  feita.
- Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.
- Resta somente a auditoria final independente do marco: conferir histórico,
  árvore limpa, locks, evidências e fronteiras; atualizar o controle para
  `CONCLUÍDO` e criar o commit local final antes de completar o Goal.
