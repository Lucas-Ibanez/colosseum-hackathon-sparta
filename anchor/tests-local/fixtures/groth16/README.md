# Fixtures Groth16 do VeriCode (D2d → D2c.1)

Dados **públicos**: selector, ImageID, journal, digest do journal e seal
Groth16. Não há chave, keypair ou segredo. Estes arquivos sozinhos **não**
são verificação on-chain. Eles só se tornam evidência quando verificados pelo
Verifier Router em teste (D2d em spike; D2e no programa).

## Formato

Um arquivo por cenário (`pass.txt`, `fail.txt`), com linhas `chave=hex` e
comentários `#`:

| Chave | Bytes | Conteúdo |
| --- | ---: | --- |
| `selector` | 4 | `73c457ba`, os 4 primeiros bytes do digest de `Groth16ReceiptVerifierParameters::default()` (`risc0-zkvm 3.0.3`) |
| `image_id` | 32 | ImageID do guest determinístico D1c2b: `4da06f90…fb1a` |
| `journal` | 165 | `JournalV1` (Job `[0x11; 32]`, artefato `(7,14)` ou `(7,15)`) |
| `journal_digest` | 32 | SHA-256 do journal (o valor que o Router recebe) |
| `seal` | 256 | seal Groth16 cru (`pi_a`, `pi_b`, `pi_c`), **sem** negar `pi_a`; o cliente nega ao montar o `Seal` |

## Proveniência

1. Receipts `Composite` reais do D1c2b.3i: PASS
   `5dcf89f5ff2417fe9615157857242e0bf8d8c17f8dd5676b521d927c89dd2d3f` e FAIL
   `3477a592c631afaad424f81c80fa2f97abf2dae23f6420fd24cd4733a671efb8`.
2. Compressão local no D2d:
   - `default_prover().compress(&ProverOpts::groth16(), …)`;
   - prover Docker `risczero/risc0-groth16-prover` amd64
     `sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331`.
3. As receipts Groth16 foram verificadas localmente contra o ImageID e
   aceitas pelo Verifier Router em `solana-program-test`. Ver
   `docs/d2d-groth16-router-spike-results.md`.

| Cenário | SHA-256 do seal | SHA-256 do journal |
| --- | --- | --- |
| PASS | `4373517058bdf048484d2f57913740df9c611e113e9a6b212f41d905a9f578c2` | `7c3f596eec21cafeef9aa7eab421582f97819ae7961aa1250e6526315b3f7de1` |
| FAIL | `93a12d12a8ed406a69b54c1b91aa49bd1f67d663723188dd7b03888e2d343ec5` | `29ff43b03973adf789f1426f31b3bbcd095ece94e3d984c92ec53be511127475` |

O teste `tests/groth16_fixtures.rs` confere esses hashes, o digest e o
vínculo do journal com o Job pelo `vericode-core`.
