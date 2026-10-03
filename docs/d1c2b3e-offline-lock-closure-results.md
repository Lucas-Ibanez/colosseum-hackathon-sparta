# D1c2b.3e — fechamento offline da cache do lock guest

Data: 2026-10-02

## Decisão

**AGUARDANDO_AUTORIZAÇÃO.** O inventário completo confirmou 154 pacotes de
registry no lock guest reconciliado. A cache destino já continha 151 archives
válidos. Dos três ausentes, a única cache-fonte autorizada continha, com
checksum correto, somente `enum-ordinalize 4.3.0` e
`enum-ordinalize-derive 4.3.1`; ambos foram copiados individualmente e
revalidados no destino. O archive `risc0-groth16 3.0.2` não existe em nenhum
caminho da raiz-fonte permitida.

A cache destino passou a conter 153 dos 154 archives exigidos. `cargo
metadata --locked --offline`, `cargo tree --locked --offline` e as três
árvores inversas continuam com exit `101`, agora exclusivamente porque
`risc0-groth16 3.0.2` exigiria download e o Cargo recusou acesso em modo
offline. Nenhuma rede ou fonte alternativa foi usada.

## Baseline e limites

- clone: `/home/lucas/src/vericode`, filesystem Linux, branch `main`;
- HEAD inicial: `9714e18` (`docs: add D1c2b autonomous control`);
- working tree inicial: limpo; `git diff --check` exit `0`;
- `/home/lucas/.rustup`: ausente;
- lock host/methods preservado:
  `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
- lock guest preservado:
  `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
- fonte exclusiva:
  `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo`;
- destino:
  `/home/lucas/.local/share/vericode-spikes/d1c2b/cargo`;
- nenhum arquivo do clone sob `zkvm/`, core, schema ou Router foi alterado.

## Inventário completo antes da cópia

Um parser somente leitura percorreu todos os registros `[[package]]` do lock
guest e, para cada pacote de registry, comparou o archive destino e seu
SHA-256 com o checksum do lock. Para cada ausência, procurou somente o nome e
versão exatos na cache-fonte autorizada e conferiu o SHA-256.

| Medida | Resultado |
| --- | ---: |
| Pacotes registry exigidos | 154 |
| Archives destino presentes e válidos | 151 |
| Archives destino ausentes | 3 |
| Checksum divergente no destino | 0 |
| Ausentes com fonte presente e válida | 2 |
| Ausentes também na fonte permitida | 1 |
| Checksum divergente na fonte | 0 |

Ausências observadas:

| Archive | Checksum do lock | Estado na fonte permitida |
| --- | --- | --- |
| `enum-ordinalize-4.3.0.crate` | `fea0dcfa4e54eeb516fe454635a95753ddd39acda650ce703031c6973e315dd5` | presente; SHA-256 idêntico |
| `enum-ordinalize-derive-4.3.1.crate` | `0d28318a75d4aead5c4db25382e8ef717932d0346600cacae6357eb5941bc5ff` | presente; SHA-256 idêntico |
| `risc0-groth16-3.0.2.crate` | `724285dc79604abfb2d40feaefe3e335420a6b293511661f77d6af62f1f5fae9` | ausente em toda a raiz-fonte |

A busca por `risc0-groth16-3.0.2.crate` e por diretório extraído equivalente
em toda a raiz-fonte não produziu saída.

## Cópias executadas

Foram usados dois comandos `cp --no-clobber --preserve=mode,timestamps` com
origem e destino absolutos e explícitos. Nenhum glob ou diretório foi
copiado. O aviso de portabilidade de `-n` não alterou o resultado.

| Archive copiado | Bytes | Modo | SHA-256 observado no destino |
| --- | ---: | ---: | --- |
| `enum-ordinalize-4.3.0.crate` | 3.922 | `0644` | `fea0dcfa4e54eeb516fe454635a95753ddd39acda650ce703031c6973e315dd5` |
| `enum-ordinalize-derive-4.3.1.crate` | 7.957 | `0644` | `0d28318a75d4aead5c4db25382e8ef717932d0346600cacae6357eb5941bc5ff` |

Não foram copiados configuração Cargo, índice, diretório Git, binário,
credencial, token, diretório completo ou arquivo sem checksum correspondente.
O Cargo extraiu localmente os dois archives na cache destino durante a
validação; isso não envolveu outra fonte nem rede.

A auditoria pós-cópia repetiu o inventário integral: 154 exigidos, 153
presentes com checksum válido, um ausente, zero divergências no destino e
nenhuma cópia elegível restante na fonte permitida.

## Ambiente Cargo obrigatório

Todos os comandos Cargo receberam explicitamente:

```text
CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo
RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup
RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0
PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin
RUSTUP_AUTO_UPDATE=0
CARGO_NET_OFFLINE=true
```

O manifesto foi sempre
`/home/lucas/src/vericode/zkvm/methods/guest/Cargo.toml`, com `--locked` e
`--offline`.

## Validação locked/offline

A primeira rodada dos cinco comandos encontrou `Read-only file system` ao
tentar extrair `enum-ordinalize 4.3.0` em `CARGO_HOME/registry/src`. Essa
saída foi classificada como restrição do sandbox, não como falha do lock.
Somente `metadata` foi repetido com autorização de escrita local na cache,
sem ampliar rede ou fonte. Ele extraiu os dois archives válidos e avançou até
o bloqueio real:

```text
error: failed to download `risc0-groth16 v3.0.2`

Caused by:
  attempting to make an HTTP request, but --offline was specified
```

Resultados após essa extração:

| Comando | Exit | Resultado |
| --- | ---: | --- |
| `cargo metadata --locked --offline --format-version 1` | 101 | `risc0-groth16 3.0.2` ausente |
| `cargo tree --locked --offline` | 101 | mesma ausência |
| `cargo tree ... --invert risc0-groth16@3.0.2` | 101 | mesma ausência |
| `cargo tree ... --invert enum-ordinalize@4.3.0` | 101 | mesma ausência antes de construir a árvore completa |
| `cargo tree ... --invert enum-ordinalize-derive@4.3.1` | 101 | mesma ausência antes de construir a árvore completa |

Não houve tentativa efetiva de HTTP: `CARGO_NET_OFFLINE=true` e `--offline`
permaneceram ativos, e a mensagem registra a operação que seria necessária.

## Riscos e próxima autoridade necessária

- O fechamento permanece incompleto em 153/154 archives; metadata/tree ainda
  não validam o lock final.
- Vendor, build A/B, ELF, ImageID, host e receipts não podem começar.
- Usar outra cache local ou baixar o archive excede a fonte exclusiva deste
  gate e exige autorização humana explícita.
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

A transição mínima proposta é autorizar a inspeção e uso de uma cache Cargo
D1a.3 adicional somente para
`risc0-groth16-3.0.2.crate`, copiando-o apenas se seu SHA-256 for exatamente
`724285dc79604abfb2d40feaefe3e335420a6b293511661f77d6af62f1f5fae9`.
Sem essa autoridade, o estado permanece `AGUARDANDO_AUTORIZAÇÃO`.
