# JournalV1

**v1 congelado (decisão humana de 2026-10-05; registrado no D4a).**

O humano aprovou "congelar o JournalV1 como está hoje (165 bytes)". Desde o
D1c2a, nenhum byte, campo, offset ou regra de hash mudou, e o congelamento
não muda nenhum deles. Este documento é a especificação v1 do journal
público que o guest admitido publica e que o programa `vericode_escrow`
consome.

## Layout v1 (165 bytes)

`crates/vericode-core` codifica `JournalV1` em Borsh `0.10.4`, fixado por
versão exata e por `Cargo.lock`:

| Offset | Tamanho | Campo | Codificação |
| ---: | ---: | --- | --- |
| 0 | 4 | `schema_version` | `u32` little-endian; sempre `1` |
| 4 | 32 | `job_id` | bytes exatos |
| 36 | 32 | `spec_hash` | bytes exatos |
| 68 | 32 | `harness_hash` | bytes exatos |
| 100 | 32 | `artifact_hash` | bytes exatos |
| 132 | 32 | `image_id` | bytes exatos |
| 164 | 1 | `verdict` | ordinal Borsh: `PASS=0`, `FAIL=1` |

Regras de decodificação:
- exatamente 165 bytes; truncamento ou bytes excedentes são rejeitados;
- `schema_version` diferente de 1 é rejeitado;
- tag de verdict diferente de 0 e 1 é rejeitada.

No programa, qualquer uma dessas falhas é o erro 6034 `JournalMalformed`. Os
vetores literais PASS/FAIL estão testados em `crates/vericode-core/src/lib.rs`
e registrados em `docs/d1c2a-wire-harness-results.md`.

## Hashing canônico v1

Os três compromissos usam SHA-256 (`sha2 0.10.9`) sobre um domínio ASCII
terminado em NUL seguido dos bytes Borsh canônicos:

| Compromisso | Preimagem | Valor v1 |
| --- | --- | --- |
| `spec_hash` | `b"vericode:spec:v1\0" ‖ borsh(spec)` | `af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778` |
| `harness_hash` | `b"vericode:harness:v1\0" ‖ borsh(harness_version)` | `01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50` |
| `artifact_hash` | `b"vericode:artifact:v1\0" ‖ borsh(artifact)` | depende do artefato entregue |

- **Especificação fixa:** o registro `(schema_version=1, multiplier=2,
  max_input=1_000_000)`.
- **Harness:** versão `u32` igual a `1`.
- **Artefato de desenvolvimento:** exclusivamente o registro
  `(schema_version, input, claimed_output)`, três `u32` little-endian, 12
  bytes.
  - Um registro válido recebe `PASS` somente quando `claimed_output == input
    * 2`; uma alegação diferente recebe `FAIL` como valor normal.
  - Formato inválido, versão incompatível e entrada acima do limite são
    erros explícitos, não verdicts.

O guest calcula `artifact_hash` sobre os bytes que avaliou. O `job_id` e o
`image_id` do journal são cópia da entrada do provador (frame `job_id ‖
artefato ‖ image_id`). O vínculo com o guest vem da verificação da receipt
contra o `image_id` do Job (R-D2 F-12).

Essas definições não aceitam código, arquivos, repositórios, patches, rede,
relógio, RPC ou I/O externo e não conferem autoridade de pagamento.

## Vínculos da v1

| Vínculo | Valor | Onde é imposto |
| --- | --- | --- |
| Guest admitido | `ADMITTED_IMAGE_ID_V1 = 4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a` (ELF D1c2b preservado) | `create_job` só admite esse ImageID; a CPI de verificação usa o `image_id` do Job |
| Programa consumidor | `vericode_escrow`, program ID `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`, implantado em devnet e finalizado no D4b (upgrade authority `none`) | `release` e `refund_on_fail` decodificam exatamente estes 165 bytes |
| Digest verificado | `SHA-256` dos mesmos 165 bytes decodificados | CPI ao verificador Groth16 `THq1qFYQ…` com `(seal, job.image_id, digest)` (D4a) |
| Spec e harness | os valores v1 da tabela acima, calculados on-chain pelo core | `create_job` (6028/6029) e `validate_against` na liquidação |

O journal não contém program ID nem cluster (R-D2e R-03): a mesma prova
liquidaria um Job com o mesmo `job_id` em outra implantação. A mitigação da
v1 é operacional: `job_id` aleatório de 32 bytes por Job e receipts novas por
Job.

## Regra de mudança

Qualquer mudança de byte, campo, offset, ordem, domínio de hash, especificação
ou harness exige, juntos:
1. novo `schema_version`;
2. novo guest e novo ImageID, recertificados;
3. nova admissão no programa (`ADMITTED_IMAGE_ID_*`, termos admitidos);
4. como o escrow terá a upgrade authority finalizada, um novo program ID;
5. atualização deste documento e registro em `docs/decisions.md`.

## Nomes históricos no código

O core ainda usa os nomes do D1c2a: `JOURNAL_V1_CANDIDATE_WIRE_SIZE`,
`JournalV1::encode_candidate`/`decode_candidate`, e comentários que dizem
"local candidate". Esses nomes designam o formato v1 congelado descrito aqui.
Renomeá-los seria mudança de código no core e no guest, fora do escopo do
congelamento.

## Base D1c1 — estrutura semântica

`Hash32` contém exatamente 32 bytes; `JobId` e `ImageId` são tipos distintos
sobre esse valor fixo; `JournalV1` usa `schema_version: u32` internamente e
contém todos os campos da tabela abaixo.

`JournalV1::validate_against` compara, em ordem explícita, versão, Job,
especificação, harness, artefato e ImageID. Depois que todos os compromissos
coincidem, a API retorna tanto `Verdict::Pass` quanto `Verdict::Fail` como
valores normais. `FAIL` não autoriza release.

| Campo | Tipo conceitual | Vínculo e fraude evitada |
| --- | --- | --- |
| `schema_version` | inteiro sem sinal/versionador | Impede interpretar bytes com um schema diferente ou aceitar mudanças silenciosas de significado. |
| `job_id` | identificador canônico do Job | Impede reutilizar uma prova válida de outro Job. |
| `spec_hash` | digest criptográfico da especificação canônica | Impede provar requisitos diferentes dos acordados. |
| `harness_hash` | digest criptográfico do harness fixo | Impede trocar a regra/ambiente de avaliação mantendo a mesma especificação. |
| `artifact_hash` | digest criptográfico dos bytes canônicos do artefato restrito | Impede apresentar ou liquidar um artefato diferente do efetivamente avaliado. |
| `image_id` | identificador criptográfico do guest RISC Zero esperado | Impede aceitar execução produzida por outro guest. Não é ID de programa Solana. |
| `verdict` | enum fechado `PASS` ou `FAIL` | Torna ambos os resultados saídas normais e verificáveis; evita representar falha por ausência de receipt ou panic. |

## Distinção entre compromissos

- `spec_hash`: compromete o que deve ser satisfeito.
- `harness_hash`: compromete como a regra determinística é aplicada.
- `artifact_hash`: compromete exatamente o que foi avaliado.
- `image_id`: compromete qual binário guest executou a avaliação.

Nenhum desses valores substitui outro. O mesmo artefato pode ser avaliado por
especificações ou harnesses diferentes; o mesmo guest pode executar
avaliações de Jobs diferentes.

## Validação pelo contrato

Antes de release ou de refund por `FAIL`, o contrato compara
`schema_version`, `job_id`, `spec_hash`, `harness_hash` e `image_id` com os
valores do Job. O `artifact_hash` deve coincidir com o compromisso de entrega
registrado pelo executor daquele Job. Somente `verdict = PASS` habilita
release; `FAIL` é válido como prova de execução, mas não autoriza pagamento
ao executor.

Compromisso de entrega (D2b.1):
- a instrução `deliver(artifact_hash)`:
  - é assinada pelo executor do Job;
  - é aceita uma única vez, a partir de `Funded` e até `deadline_slot`;
  - grava `Delivered { artifact_hash }`;
- `artifact_hash` tem a semântica de `hash_restricted_artifact`, a mesma
  deste journal;
- `release` e `refund_on_fail` exigem `Delivered { h }` e um journal com
  `artifact_hash == h`; senão, `ArtifactHashMismatch` (6017);
- divergência do guia §5 ("registrado apenas na liquidação"), decidida pelo
  humano (`docs/decisions.md`, D2b.1).

Executor, mint e destino do pagamento ficam no estado do Job, não no journal,
e são validados pelo contrato: destino = ATA canônica da parte paga; mint =
o Test USDC admitido (D4a).

Verificação da prova (D4a): o programa chama por CPI o verificador Groth16 de
`risc0-solana v3.0.0` (`THq1qFYQ…`, imutável em devnet) com o `image_id` do
Job e o SHA-256 destes 165 bytes, sem Verifier Router. Ver
`docs/escrow-program.md`.
