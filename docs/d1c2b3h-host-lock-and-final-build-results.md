# D1c2b.3h — lock host compatível e builds finais determinísticos

Data: 2026-10-03

## Resultado

**GO** para produzir receipts VeriCode locais reais. O lock host/methods foi
reconciliado offline com o conjunto compatível comprovado pelo lock oficial de
`risc0-zkvm 3.0.3`; o host compilou, seus dois testes passaram e dois builds
independentes do guest final produziram bytes e ImageID idênticos.

O gate não executou proving, Solana, Anchor, wallet, validator, Router/CPI,
rede blockchain, deploy, front-end ou push. Router/CPI/devnet permanecem
`STATUS: NÃO VALIDADO`.

## Baseline e plano somente leitura

- HEAD: `78ac4202f670ce59c3acbcc8aad7ad4be22aa59a`;
- lock host anterior: `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
- lock guest preservado:
  `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
- árvore limpa antes do gate; `/home/lucas/.rustup` ausente.

A superfície desta sessão não oferece Plan Mode formal nem seleção dinâmica de
subagente. Antes de alterar `zkvm/`, foi executada uma etapa somente leitura:
comparação do lock quebrado, manifests cacheados e lock oficial da crate
`risc0-zkvm 3.0.3`; definição do conjunto exato; inventário dos archives; e
revisão das proibições. O executor permaneceu único e sequencial.

## Diagnóstico do lock host anterior

O build real do host com o lock anterior chegou a `risc0-zkvm 3.0.3` e falhou
com sete erros de API contra transitivas mais novas. Entre eles estavam a
incompatibilidade `dyn SyscallContext`, tipos divergentes em
`host_read`/`host_write` e o uso de `.first()` sobre o resultado de
`read_u32s`.

O lock oficial incluído no archive local de `risc0-zkvm 3.0.3` fixa o conjunto
compatível. A versão exata `risc0-zkvm-platform 2.2.0` não existe em nenhuma
fonte local inventariada, inclusive na imagem Docker fixada. Como os manifests
aceitam `^2.2.0`, a versão já cacheada `2.2.3` foi preservada; nenhuma rede foi
usada.

## Archives autorizados e copiados

A autorização humana retomou o Goal e permitiu usar somente
`/home/lucas/.local/share/vericode-spikes/d1a3/homes/lane-a/cargo` como fonte
adicional. Dez archives ausentes foram copiados individualmente com
`--no-clobber` para a cache D1c2b. Origem, destino, bytes e SHA-256 foram
verificados antes e depois:

| Crate | Bytes | SHA-256 |
| --- | ---: | --- |
| `risc0-binfmt 3.0.2` | 25.287 | `1c8f97f81bcdead4101bca06469ecef481a2695cd04e7e877b49dea56a7f6f2a` |
| `risc0-circuit-keccak 4.0.2` | 5.535.052 | `5f195f865ac1afdc21a172d7756fdcc21be18e13eb01d78d3d7f2b128fa881ba` |
| `risc0-circuit-keccak-sys 4.0.1` | 2.227.818 | `30a8f21cc053fe9892acebbe0ebe2610a5d79ad638cd17f2e5122cf0b3e7cd1a` |
| `risc0-circuit-recursion 4.0.2` | 116.428 | `dca8f15c8abc0fd8c097aa7459879110334d191c63dd51d4c28881c4a497279e` |
| `risc0-circuit-recursion-sys 4.0.1` | 798.590 | `f5f137bcd382520efcd982e4ee131da43f448b12ade979fe9d1fa92d4337dec0` |
| `risc0-circuit-rv32im 4.0.2` | 4.050.328 | `ae1b0689f4a270a2f247b04397ebb431b8f64fe5170e98ee4f9d71bd04825205` |
| `risc0-circuit-rv32im-sys 4.0.1` | 1.081.478 | `cb25f3935e53e89ca020224ad0c09de96cab89a215054c0cee290405074a5166` |
| `risc0-core 3.0.0` | 16.229 | `80f2723fedace48c6c5a505bd8f97ac4e1712bc4cb769083e10536d862b66987` |
| `risc0-zkos-v1compat 2.2.0` | 23.142 | `840c2228803557a8b7dc035a8f196516b6fd68c9dc6ac092f0c86241b5b1bafb` |
| `risc0-zkp 3.0.2` | 109.790 | `ffb6bf356f469bb8744f72a07a37134c5812c1d55d6271bba80e87bdb7a58c8e` |

`risc0-groth16 3.0.2` já estava na cache D1c2b, com SHA-256
`724285dc79604abfb2d40feaefe3e335420a6b293511661f77d6af62f1f5fae9`.
Nenhuma configuração, índice inteiro, diretório Git, binário, credencial ou
arquivo sem checksum foi copiado.

## Reconciliação mecânica do lock

O lock continua com 461 pacotes. Um parser independente comparou os blocos e
confirmou zero pacote adicional, zero pacote removido fora do conjunto e zero
mudança de aresta em pacote de versão invariável. As onze trocas são:

| Crate | Antes | Depois |
| --- | --- | --- |
| `risc0-binfmt` | 3.0.5 | 3.0.2 |
| `risc0-circuit-keccak` | 4.0.6 | 4.0.2 |
| `risc0-circuit-keccak-sys` | 4.0.3 | 4.0.1 |
| `risc0-circuit-recursion` | 4.0.5 | 4.0.2 |
| `risc0-circuit-recursion-sys` | 4.0.3 | 4.0.1 |
| `risc0-circuit-rv32im` | 4.0.5 | 4.0.2 |
| `risc0-circuit-rv32im-sys` | 4.0.3 | 4.0.1 |
| `risc0-core` | 3.0.2 | 3.0.0 |
| `risc0-groth16` | 3.0.5 | 3.0.2 |
| `risc0-zkos-v1compat` | 2.2.3 | 2.2.0 |
| `risc0-zkp` | 3.0.5 | 3.0.2 |

`risc0-zkvm 3.0.3`, `risc0-zkvm-platform 2.2.3`, manifests diretos e o lock
guest não mudaram. O novo SHA-256 do lock host é
`f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226`.

As atualizações foram ordenadas pelos consumidores. Uma tentativa inicial de
baixar `risc0-binfmt` antes de `circuit-keccak` falhou corretamente por
restrição `^3.0.5`; outra tentativa de baixar `risc0-core` antes de `zkp`
falhou por `^3.0.2`. Nenhuma dessas falhas alterou o conjunto final. Quatro
arestas `windows-sys` incidentalmente reescritas pelo Cargo foram restauradas;
o parser final confirmou que não restou mudança alheia.

## Validação offline do grafo

Todos os comandos Rust/Cargo/RISC Zero declararam explicitamente as homes
prescritas, `RUSTUP_AUTO_UPDATE=0` e `CARGO_NET_OFFLINE=true`.

- `cargo metadata --locked --offline`: exit `0`; 461 pacotes, 458 registry,
  dois membros de workspace e 461 nós resolvidos;
- `cargo tree --locked --offline`: exit `0`; SHA-256 da saída
  `00d092a274dc4b6a35ebbcaeeea9fafee4a865312791ef58d56e9d6624ead0d3`;
- onze árvores inversas, uma para cada crate reconciliada: exit `0` e cadeia
  até `risc0-zkvm`/host confirmada;
- `git diff --check`: exit `0` após a comparação estrutural.

## Vendor temporário final

O contexto A foi exportado do HEAD e recebeu somente o lock candidato e,
depois do diagnóstico de compilação, o helper host candidato. O vendor da
união dos locks foi gerado por `cargo vendor --locked --offline --sync`.

Auditoria integral do contexto A:

- 467 crates/checksums esperados e observados;
- 23.182 arquivos regulares e 499.440.487 bytes;
- 22.715 arquivos declarados relidos;
- zero arquivo ausente, extra ou divergente;
- hash canônico de paths:
  `53c89f6e7b1997ec256281e7b5115eccb5313c2086b77e435c1ced06efcfcf7f`;
- hash canônico de conteúdo:
  `5b527e443adc70a56fd57133321d108a9954a76398b73fe797baefd00c83b6fa`.

O contexto B foi criado de um segundo `git archive` do mesmo HEAD, cujo
SHA-256 repetiu `a1c77299384023af008121f61e2e668b57d5abad6c0ac3d65ab4a2ee951fb393`,
e recebeu somente os mesmos dois arquivos candidatos. Seu vendor foi gerado
independentemente. A comparação de todos os paths e SHA-256 dos arquivos A/B
passou; o inventário agregado comum foi
`0be9ba1306dfefb5770462b826f32373f556c442288419c6e0df074fd3332274`.

### Anomalia diagnosticada do BuildKit

No primeiro build, o arquivo em disco
`vendor/risc0-groth16/Cargo.toml.orig` tinha o hash correto 3.0.2
`f2eec571…`, mas o `COPY` do BuildKit entregou o hash `b61c0fa1…`, que foi
identificado como o mesmo arquivo da versão 3.0.5. Um Dockerfile mínimo com
`--no-cache` reproduziu a troca. As versões têm caminho, tamanho de 1.978
bytes e timestamp histórico iguais.

Atualizar somente o timestamp do arquivo temporário com `touch` preservou o
SHA-256 esperado e fez o Dockerfile mínimo copiar os bytes corretos. A mesma
invalidação de metadado foi aplicada aos vendors A/B antes dos builds. Não
houve edição de conteúdo, e a igualdade integral dos vendors permaneceu
comprovada.

## Compatibilidade do host

Com o grafo reconciliado, o host chegou ao próprio código e revelou um erro
de tipo: `Digest::as_bytes()` retorna `&[u8]`, não `&[u8; 32]`. A alteração
limitada usa a API existente `AsRef<[u8; 32]>`, sem `unwrap`, `panic`, mudança
de journal, schema ou semântica de verdict.

Depois da correção:

```text
Finished `release` profile [optimized]
running 2 tests
test tests::guest_input_is_the_fixed_public_frame ... ok
test tests::expected_pass_and_fail_are_distinct_exact_journals ... ok
test result: ok. 2 passed; 0 failed
```

## Dois builds finais independentes

Os contextos A/B possuem exports, vendors, nonces e targets diferentes. Os
dois builds reais de `vericode-methods` terminaram com exit `0` usando a
imagem local fixada:

`risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3`.

| Propriedade | A | B | Comparação |
| --- | --- | --- | --- |
| ELF guest, bytes | 147.876 | 147.876 | `cmp` exit `0` |
| ELF guest, SHA-256 | `63fac491fdd8935141850b3a3423934c28e988f2db0746bbaa23726740215408` | igual | idêntico |
| Método combinado, bytes | 180.300 | 180.300 | `cmp` exit `0` |
| Método combinado, SHA-256 | `e09ba8cf16f7e36cb92e10656b7574c598f3e88fe089f02bc747a4bb00c078f5` | igual | idêntico |
| Array emitido | `[2423234637, 2313975258, 3460548992, 3295247617, 3690819715, 1747430131, 2019375005, 452706554]` | igual | idêntico |
| ImageID `r0vm` | `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a` | igual | idêntico |

O ELF permanece ELF32 little-endian, RISC-V estático, com entry point
`0x2058e8`. Ele difere em quatro bytes de tamanho do artefato do gate 3g
porque a união de locks reconciliada atribui paths de vendor diferentes a
versões duplicadas; por isso os dois builds finais foram refeitos e a
evidência antiga não foi reutilizada como artefato de receipts.

## Riscos e transição

- Os vendors, targets, ELF e métodos são auditáveis, mas efêmeros em `/tmp`.
- A API upstream de build não expõe `--network none`; Cargo permaneceu
  explicitamente offline, a imagem estava presente por digest e nenhum pull
  foi solicitado.
- Ainda não há receipt VeriCode PASS/FAIL. Esse é o próximo gate obrigatório.
- `Verdict::Fail` continua saída normal; o gate seguinte deve distingui-lo de
  erro operacional e verificar ImageID/journal e negativos reais.
- Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.
