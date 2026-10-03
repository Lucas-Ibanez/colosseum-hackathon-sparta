# D1c2b.3e — fechamento offline da cache do lock guest

Data: 2026-10-02

## Decisão

**GO.** O inventário completo confirmou 154 pacotes de registry no lock guest
reconciliado. A primeira execução levou a cache destino de 151 para 153
archives válidos e parou, corretamente, porque `risc0-groth16 3.0.2` não
existia na única fonte então permitida.

Após autorização humana explícita, as caches locais D1a.3 `lane-a/cargo` e
`lane-b/cargo` foram inspecionadas somente para o archive restante. Três
cópias candidatas eram byte a byte idênticas e tinham o SHA-256 exato do lock.
Uma única cópia de `lane-a` foi feita para o destino. A auditoria integral
passou com 154/154 archives válidos e zero divergência; `cargo metadata`,
`cargo tree` e quatro árvores inversas terminaram com exit `0`, sempre locked
e offline. Nenhuma rede foi usada.

## Baseline e limites

- clone: `/home/lucas/src/vericode`, filesystem Linux, branch `main`;
- HEAD inicial: `9714e18` (`docs: add D1c2b autonomous control`);
- working tree inicial: limpo; `git diff --check` exit `0`;
- `/home/lucas/.rustup`: ausente;
- lock host/methods preservado:
  `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
- lock guest preservado:
  `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
- fonte inicial:
  `/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo`;
- fontes adicionais autorizadas somente para localizar o archive restante:
  `/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/cargo` e
  `/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-b/cargo`;
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
| `risc0-groth16-3.0.2.crate` | 40.149 | `0644` | `724285dc79604abfb2d40feaefe3e335420a6b293511661f77d6af62f1f5fae9` |

Não foram copiados configuração Cargo, índice, diretório Git, binário,
credencial, token, diretório completo ou arquivo sem checksum correspondente.
O Cargo extraiu localmente os dois archives na cache destino durante a
validação; isso não envolveu outra fonte nem rede.

A auditoria após as duas cópias iniciais encontrou 154 exigidos, 153 presentes
com checksum válido, um ausente e zero divergências. Na retomada autorizada,
os três candidatos de `risc0-groth16-3.0.2.crate` — dois em `lane-a` e um em
`lane-b` — tinham 40.149 bytes, eram idênticos por `cmp` e tinham SHA-256
`724285dc79604abfb2d40feaefe3e335420a6b293511661f77d6af62f1f5fae9`.
A origem escolhida foi o índice `index.crates.io-1949cf8c6b5b557f` de
`lane-a`, correspondente ao índice do destino. `cp --no-clobber
--preserve=mode,timestamps` copiou somente esse arquivo. `cmp` origem/destino
passou, e o inventário integral final resultou em 154 presentes e válidos,
zero ausentes e zero divergências.

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

### Retomada após autorização da fonte adicional

Depois da cópia validada de `risc0-groth16-3.0.2.crate`, os comandos foram
repetidos no mesmo ambiente explícito:

| Comando | Exit | Resultado |
| --- | ---: | --- |
| `cargo metadata --locked --offline --format-version 1` | 0 | grafo completo resolvido pela cache local |
| `cargo tree --locked --offline` | 0 | árvore completa emitida; `risc0-groth16 3.0.2` e enums reconciliados presentes |
| `cargo tree ... --invert risc0-groth16@3.0.2` | 0 | `risc0-groth16 -> risc0-zkvm -> vericode-guest` |
| `cargo tree ... --invert enum-ordinalize@4.3.0` | 0 | caminho via `educe` e arkworks até o guest |
| `cargo tree ... --invert enum-ordinalize-derive@4.3.1` | 0 | derive ligado ao enum e ao mesmo caminho do guest |
| `cargo tree ... --invert syn@2.0.119` | 0 | consumidores reconciliados, inclusive o derive `4.3.1`, visíveis |

O Cargo extraiu o archive validado na própria cache isolada. Nenhum arquivo do
clone foi criado ou alterado pelos comandos; `git status --short --ignored`
permaneceu sem saída.

## Riscos e próxima transição

- O fechamento offline do lock está concluído, mas ainda não prova
  compatibilidade de compilação com o Rust guest `1.88.0-dev`.
- Vendor final, build A/B, ELF, ImageID, host e receipts ainda não foram
  executados nesta retomada.
- Os hashes de vendors antigos pertencem ao lock anterior e não podem ser
  reutilizados.
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

A próxima transição permitida é produzir, em diretório temporário, um vendor
novo e auditável derivado do lock reconciliado. Somente após seu inventário,
checksums e auditoria passarem poderão começar os dois builds independentes.
