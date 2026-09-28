# JournalV1

**Draft v0 — sujeito ao gate de receipt local**

Este documento define os campos públicos mínimos e seus significados. Não define ainda a serialização final, layout binário, endianness ou ID de programa.

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

Nenhum desses valores substitui outro. O mesmo artefato pode ser avaliado por especificações ou harnesses diferentes; o mesmo guest pode executar avaliações de Jobs diferentes.

## Validação pelo contrato

Antes de release, o contrato deve comparar `schema_version`, `job_id`, `spec_hash`, `harness_hash` e `image_id` com os valores autorizados pelo Job. O `artifact_hash` deve coincidir com o compromisso aceito/registrado para a entrega daquele Job antes da liquidação. Somente `verdict = PASS` pode habilitar release; `FAIL` é válido como prova de execução, mas não autoriza pagamento ao executor.

Executor, mint e destino do pagamento permanecem definidos no estado do Job e devem ser validados pelo contrato. Mudanças neste schema exigem atualização deste documento e registro em `docs/decisions.md`.
