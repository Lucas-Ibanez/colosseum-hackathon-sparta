# Fixtures Groth16 do VeriCode (D2d → D2c.1; D4b no D7)

Dados **públicos**: selector, ImageID, journal, digest do journal e seal
Groth16. Não há chave, keypair ou segredo. Estes arquivos sozinhos **não**
são verificação on-chain. Eles só se tornam evidência quando verificados em
teste: pelo Verifier Router (D2d em spike; D2e no programa) e, desde o D4a,
pelo verificador Groth16 chamado direto pelo programa.

Job `0x11` (`pass.txt`, `fail.txt`) é só para testes locais (fallback
rotulado). Em devnet, cada Job usa `job_id` aleatório e receipts novas (D4-4,
R-D2e R-03); os vetores de `d4b/` são dessas receipts de devnet.

## Formato

Um arquivo por cenário (`pass.txt`, `fail.txt`), com linhas `chave=hex` e
comentários `#`:

| Chave | Bytes | Conteúdo |
| --- | ---: | --- |
| `selector` | 4 | `73c457ba`, os 4 primeiros bytes do digest de `Groth16ReceiptVerifierParameters::default()` (`risc0-zkvm 3.0.3`) |
| `image_id` | 32 | ImageID do guest determinístico D1c2b: `4da06f90…fb1a` |
| `journal` | 165 | `JournalV1` (Job `[0x11; 32]`, artefato `(7,14)` ou `(7,15)`) |
| `journal_digest` | 32 | SHA-256 do journal (o valor que o verificador recebe) |
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
   aceitas pelo Verifier Router em `solana-program-test` (D2d). Desde o D4a,
   o escrow chama o verificador Groth16 direto, sem Router. Ver
   `docs/d2d-groth16-router-spike-results.md` e
   `docs/d4a-direct-verifier-results.md`.

| Cenário | SHA-256 do seal | SHA-256 do journal |
| --- | --- | --- |
| PASS | `4373517058bdf048484d2f57913740df9c611e113e9a6b212f41d905a9f578c2` | `7c3f596eec21cafeef9aa7eab421582f97819ae7961aa1250e6526315b3f7de1` |
| FAIL | `93a12d12a8ed406a69b54c1b91aa49bd1f67d663723188dd7b03888e2d343ec5` | `29ff43b03973adf789f1426f31b3bbcd095ece94e3d984c92ec53be511127475` |

O teste `tests/groth16_fixtures.rs` confere esses hashes, o digest e o
vínculo do journal com o Job pelo `vericode-core`.

## Receipts do D4b (`d4b/`, D7)

`d4b/S.txt`, `d4b/A.txt`, `d4b/A-fail.txt` e `d4b/B.txt` são os vetores
públicos das receipts que liquidaram (ou foram rejeitadas) em devnet no D4b
(`docs/d4b-devnet-results.md`), no mesmo formato acima. Foram gerados no D7
dos arquivos de `~/.local/share/vericode-spikes/d4/receipts-out/` (fora do
clone), depois de `sha256sum -c` da lista `d4/logs/receipts-out.sha256`
(28/28 OK). Atendem RD4A-07 (f) da R-D4a.

| Vetor | `job_id` de devnet | Artefato | Veredito | SHA-256 do journal | SHA-256 do seal |
| --- | --- | --- | --- | --- | --- |
| `S` | `fe6d25fe…0959b` | `(7,14)` | PASS | `20b3353c…b1` | `4f1131c2…8ef` |
| `A` | `3f0dd1c8…aec8a` | `(7,14)` | PASS | `a1060efe…f47` | `610704b1…cf0` |
| `A-fail` (A′) | `3f0dd1c8…aec8a` | `(7,15)`, nunca entregue | FAIL | `e04f62d9…63c` | `7c596225…036` |
| `B` | `5a25ae48…fc309` | `(7,15)` | FAIL | `e931596b…741` | `462c4855…63f` |

`tests/d4b_receipts.rs` confere o vínculo de cada vetor ao seu Job e
reproduz em processo, com o verificador real, as liquidações e os negativos
do D4b e a dupla liquidação (invariante 9). Os Jobs S, A e B já estão
consumidos em devnet: esses vetores não liquidam mais nada lá.
