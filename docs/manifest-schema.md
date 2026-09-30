# JournalV1

**Draft v0 — sujeito ao gate de receipt local**

Este documento define os campos públicos mínimos e seus significados. Não define ainda a serialização final, layout binário, endianness ou ID de programa.

## Estado D1c1 — estrutura semântica implementada

A crate Rust pura `crates/vericode-core` implementa a estrutura semântica
inicial deste documento, sem dependências externas. `Hash32` contém exatamente
32 bytes; `JobId` e `ImageId` são tipos distintos sobre esse valor fixo;
`JournalV1` usa `schema_version: u32` internamente e contém todos os campos da
tabela abaixo. Essas escolhas descrevem a API Rust atual, não congelam bytes
públicos, layout, endianness, algoritmo de hash ou codificação.

`JournalV1::validate_against` compara, em ordem explícita, versão, Job,
especificação, harness, artefato e ImageID. Após todos os compromissos
coincidirem, a API retorna tanto `Verdict::Pass` quanto `Verdict::Fail` como
valores normais. `FAIL` não autoriza release.

O wire format continua **Draft v0 e NÃO CONGELADO**. D1c1 não implementa
serialização, guest, receipt VeriCode nem prova Groth16. A receipt local do
D1a.3 pertence ao `hello-world` upstream, contém journal inteiro `391` e não
foi demonstrada como Groth16 ou como journal do VeriCode; M1 permanece fora
de verde.

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

D1c1 não adiciona executor ou mint ao journal e não confere autoridade de
release à crate pura. Essa ligação continua responsabilidade futura do estado
do Job e do contrato Anchor, em gate separado.
