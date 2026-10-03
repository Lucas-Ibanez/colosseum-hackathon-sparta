# D1c2b.3j — auditoria final independente

Data: 2026-10-03

## Veredicto

**APROVADO — D1c2b CONCLUÍDO.** A auditoria final, executada como etapa
separada e somente leitura a partir do commit `d866e73`, recertificou todas as
condições do objetivo persistente. Nenhuma tarefa obrigatória permanece.

O trabalho para aqui. Solana, Anchor, wallet, validator, Router/CPI, rede
blockchain, deploy, front-end e push não foram iniciados. Router/CPI/devnet
permanecem `STATUS: NÃO VALIDADO`.

## Git e ambiente

- `pwd` e raiz Git: `/home/lucas/src/vericode`;
- filesystem: Linux `ext2/ext3` reportado pelo `statfs` do WSL;
- branch: `main`;
- HEAD auditado: `d866e73a237ffc4e0c4547120deee5bf78c5c526`;
- `git status --short`: vazio;
- `git diff --check`: exit `0`;
- `/home/lucas/.rustup`: ausente;
- nenhum target, vendor, receipt, `.env`, keypair, `.pem` ou `.key` foi
  rastreado.

O histórico D1c2b após o baseline `0d55e44` contém oito commits locais,
sequenciais e sem reescrita:

```text
9714e18 docs: add D1c2b autonomous control
eb7ec48 docs: record incomplete offline lock closure
3c505a8 docs: close guest lock cache offline
1525377 docs: audit final guest vendor
be23e01 core: support no_std guest build
78ac420 docs: record deterministic guest builds
dc41168 zkvm: reconcile host lock for receipts
d866e73 docs: record local VeriCode receipts
```

Além de documentação/controle, o diff do marco modifica somente
`AGENTS.md`, `crates/vericode-core`, `zkvm/Cargo.lock` e o helper tipado do
host. `programs/`, `docs/manifest-schema.md` e `docs/router-notes.md`
permaneceram byte a byte inalterados desde o baseline.

## Locks e fechamento offline

- lock guest:
  `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
- lock host:
  `f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226`.

Com as homes prescritas e rede desabilitada, a auditoria repetiu:

| Grafo | Comando | Resultado | SHA-256 da saída |
| --- | --- | --- | --- |
| host | `cargo metadata --locked --offline` | exit `0` | `05364c2b7dcfb80ee3cd859fbd0e9e67f44fb86b7a00984e719137a4c57e131e` |
| host | `cargo tree --locked --offline` | exit `0` | `21c59b99a78a224c767f8f08f183cbae8b22a1c9af90525530a3bed28e76691b` |
| guest | `cargo metadata --locked --offline` | exit `0` | `aaf877ca17b0139707f720713c0ff881ce63baede23cbe81dddd79334e8ef7de` |
| guest | `cargo tree --locked --offline` | exit `0` | `caab3faeaf475e75d27189dbaa124d7f63881ad38c1a8323b3f2e6f01a0b370f` |

O fechamento guest 154/154 e os onze archives host autorizados permanecem
documentados com checksums completos nos relatórios 3e e 3h. Nenhuma fonte
nova foi usada nesta auditoria.

## Vendor auditável

Os dois vendors finais independentes ainda existem somente em `/tmp`. A
auditoria releu todos os paths e SHA-256:

- A/B: 467 crates/checksums cada;
- A/B: 23.182 arquivos cada;
- inventário agregado de todos os arquivos, incluindo ignorados internos:
  `73218059ad35a15c492f8aa8a403423f6f60f7342b2f1a84423b0431441cc8f1`;
- comparação integral A/B: exit `0`.

O parser detalhado do gate 3h já havia validado 22.715 arquivos declarados,
zero ausência, extra ou divergência. A igualdade integral A/B recertificada
confirma que a invalidação apenas de timestamp usada contra o cache obsoleto
do BuildKit não alterou conteúdo.

## Builds independentes e determinismo

A auditoria repetiu `cmp`, tamanho, SHA-256 e dois cálculos `r0vm --id`:

| Propriedade | A | B | Resultado |
| --- | --- | --- | --- |
| ELF guest | 147.876 bytes; `63fac491fdd8935141850b3a3423934c28e988f2db0746bbaa23726740215408` | igual | byte a byte idêntico |
| Método combinado | 180.300 bytes; `e09ba8cf16f7e36cb92e10656b7574c598f3e88fe089f02bc747a4bb00c078f5` | igual | byte a byte idêntico |
| ImageID `r0vm` | `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a` | igual | idêntico |

Os dois testes host foram executados novamente a partir do binário de teste
validado: 2 passaram, zero falhou.

## Receipts e negativos

Os arquivos permanecem em `/tmp/vericode-d1c2b3i-receipts.x2zrL1`:

| Receipt | Tipo | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| PASS | `Composite` | 221.540 | `5dcf89f5ff2417fe9615157857242e0bf8d8c17f8dd5676b521d927c89dd2d3f` |
| FAIL | `Composite` | 221.540 | `3477a592c631afaad424f81c80fa2f97abf2dae23f6420fd24cd4733a671efb8` |
| wrong-image | `Composite` | 221.540 | `189bd609311800f51b7b3e38c016e8028130d16b392a6d00915a146e14d08d04` |

O auditor temporário independente foi executado novamente, com dev/Bonsai
ausentes e prover/executor locais. Ele desserializou os três arquivos,
confirmou que nenhum é `Fake`, verificou todos contra o ImageID final e
rejeitou:

- verificação do PASS contra ImageID incorreto;
- journal com ImageID fornecido incorreto;
- journal PASS contra Job ID divergente.

PASS e FAIL mantêm journals de 165 bytes. `Verdict::Fail` é saída normal e
verificável, não panic ou erro operacional.

## Matriz de conclusão

| Condição obrigatória | Evidência | Estado |
| --- | --- | --- |
| fechamento offline do lock guest | cache 154/154; metadata/tree e inversas | concluída |
| vendor temporário auditável | parsers de checksums; A/B 467 crates e 23.182 arquivos iguais | concluída |
| dois builds independentes | exports, vendors, nonces e targets distintos | concluída |
| comparação ELF/tamanho/SHA/ImageID | `cmp`, `stat`, `sha256sum`, dois `r0vm` | concluída |
| receipts PASS e FAIL reais | `Composite`, não Fake, `Receipt::verify` | concluída |
| testes negativos | ImageID de verificação, ImageID do journal e Job ID rejeitados | concluída |
| evidências registradas | relatórios 3e–3j, decisões, notes e tabela de evidências | concluída |
| auditoria final independente | este gate somente leitura | concluída |
| parar antes dos escopos proibidos | diff/histórico e fronteiras auditados | concluída |

## Limitações residuais não bloqueantes

- Artefatos e receipts são locais e efêmeros em `/tmp`.
- Os receipts são `Composite`, não Groth16.
- A API upstream do build guest não expõe `--network none`; Cargo foi
  explicitamente offline e a imagem local permaneceu fixada por digest.
- O comportamento de cache obsoleto do BuildKit foi reproduzido e mitigado
  somente por metadado, com conteúdo A/B novamente comparado.
- Nenhuma dessas limitações deixa tarefa obrigatória dentro do objetivo D1c2b
  ou autoriza alegação on-chain.

## Encerramento

O estado permitido é `CONCLUÍDO`. Não há próxima transição dentro de D1c2b.
Qualquer trabalho em Solana, Anchor, wallet, validator, Router/CPI, blockchain,
deploy, front-end, rede ou push exige nova autoridade e objetivo separado.
