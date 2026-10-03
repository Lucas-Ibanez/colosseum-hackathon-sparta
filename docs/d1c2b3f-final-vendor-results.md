# D1c2b.3f — vendor temporário do lock guest final

Data: 2026-10-02

## Decisão

**GO** para os dois builds independentes do guest em gate posterior.

Um staging novo foi exportado do commit auditado `3c505a8` e recebeu somente
um vendor na raiz e a configuração de substituição emitida pelo Cargo. O
vendor corresponde exatamente aos 154 pacotes de registry do lock guest
reconciliado. Todos os checksums de pacote e de arquivo foram verificados;
metadata e árvore Cargo passaram locked/offline e resolveram todos os pacotes
de registry por paths dentro do vendor.

Nenhum build, ELF, ImageID, receipt ou proving foi executado neste gate.

## Baseline e staging

- clone: `/home/lucas/src/vericode`, branch `main`;
- commit exportado:
  `3c505a8725323e6b8a96c59c2219e3b86f04342a`;
- staging: `/tmp/vericode-d1c2b3f.0b1Sjh/src`;
- archive do commit:
  `/tmp/vericode-d1c2b3f.0b1Sjh/HEAD.tar`;
- SHA-256 do archive:
  `03a0384f30fd3449fe19c271a78d23a2e95c2a622ea6ea7a551d1c8e5c443546`;
- o staging não contém `.git`;
- `/home/lucas/.rustup` permaneceu ausente;
- `git status --short`, `git diff --check` e `git show --check HEAD`
  passaram antes da exportação.

Os dez manifests, locks e fontes críticos comparados entre clone e staging
foram byte a byte equivalentes por SHA-256. Locks preservados:

| Arquivo | SHA-256 |
| --- | --- |
| `zkvm/Cargo.lock` | `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1` |
| `zkvm/methods/guest/Cargo.lock` | `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa` |

## Geração offline

O comando efetivo, executado na raiz do staging, foi:

```text
env \
  CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo \
  RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup \
  RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0 \
  PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin \
  RUSTUP_AUTO_UPDATE=0 \
  CARGO_NET_OFFLINE=true \
  cargo +1.89.0 vendor --locked --offline \
  --manifest-path zkvm/methods/guest/Cargo.toml vendor
```

Resultado: exit `0`. A saída listou explicitamente as versões reconciliadas
`risc0-groth16 3.0.2`, `enum-ordinalize 4.3.0` e
`enum-ordinalize-derive 4.3.1`.

A configuração literal emitida pelo Cargo foi criada somente como
`/tmp/vericode-d1c2b3f.0b1Sjh/src/.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

SHA-256 da configuração:
`77e9219c27274120197571fd165cbe4121963b5ad3bc0b20b383c86ef0ce6c2b`.

## Inventário e checksums

| Medida | Resultado |
| --- | ---: |
| Diretórios de crate | 154 |
| `.cargo-checksum.json` | 154 |
| Arquivos regulares | 5.904 |
| Arquivos de conteúdo declarados | 5.750 |
| Tamanho por `du -sb` | 113.733.052 bytes |
| Tipo diferente de arquivo/diretório | 0 |
| Arquivos com nome sensível | 0 |

Um parser independente associou cada checksum de pacote do lock a exatamente
um diretório do vendor e ao `name`/`version` de seu `Cargo.toml`:

```text
expected=154
crate_dirs=154
checksum_files=154
missing_lock_checksums=0
extra_checksums=0
duplicate_checksums=0
manifest_identity_mismatches=0
```

Para cada uma das 154 crates, todos os paths de `files` do
`.cargo-checksum.json` foram relidos e comparados por SHA-256. Resultado:
5.750 arquivos verificados, zero ausente, zero divergente e zero arquivo
regular extra não listado.

Inventários canônicos novos, sem reutilizar o vendor do lock anterior:

- SHA-256 dos paths relativos ordenados por `LC_ALL=C`:
  `dc242b5e026102f57118c53212ab89d990e9dd4aead6518d12efe46023535d7a`;
- SHA-256 do fluxo de `sha256sum` dos arquivos em ordem de path relativo:
  `3217344ba8c0e5c842323183993efdc24c0e6cd8a4c0d914fef2aa324578da05`.

## Configurações internas auditadas

Três arquivos `.cargo/config.toml` fazem parte do conteúdo upstream
checksummed das crates `ruint-macro`, `bytemuck` e `ruint`. Os dois arquivos
`ruint*` definem somente `rustdocflags` para KaTeX; o arquivo de `bytemuck`
define somente um alias de documentação. Nenhum contém `source`, registry,
substituição, rede, proxy, token ou credencial. Como estão dentro dos
diretórios das dependências, não substituem a configuração raiz do staging.

Não foram encontrados `.pem`, `.key`, keypair, credential, `.env`,
`id_rsa`, `id_ed25519`, `credentials` ou `credentials.toml` no vendor.

## Consumo locked/offline

`cargo metadata --locked --offline --format-version 1`, com todas as homes e
flags obrigatórias explícitas, foi consumido por um parser somente leitura.
A primeira apresentação falhou no parser Python por quoting de um `f-string`;
o pipeline terminou antes de permitir atribuir um exit confiável ao Cargo.
Após corrigir exclusivamente o parser, a mesma consulta terminou com exit
`0`:

```text
packages=156
registry=154
registry_paths_in_vendor=154
outside_vendor=0
root=.../zkvm/methods/guest#vericode-guest@0.1.0
```

`cargo tree --locked --offline` também terminou com exit `0` e mostrou a
cadeia final, incluindo `risc0-groth16 3.0.2`, `enum-ordinalize 4.3.0`,
derive `4.3.1` e `syn 2.0.119`.

## Limites e riscos

- O vendor existe somente em `/tmp`; não foi copiado para o clone nem
  versionado.
- Ainda não há dois builds independentes, ELF, comparação byte a byte,
  ImageID ou receipt VeriCode.
- O consumo no builder será exercitado pelo gate de build; este gate prova o
  vendor e sua resolução host locked/offline, não compilação guest.
- Não houve rede, Docker, pull, instalação, atualização, dev mode, Bonsai,
  Solana, Anchor, wallet, validator, Router/CPI, deploy, front-end ou push.
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

A próxima transição permitida é criar dois contextos e targets independentes,
usar a imagem local fixada por digest sem pull e executar dois builds reais do
guest. Cada build deve permanecer offline, produzir seu próprio ELF e não
pode avançar para receipts antes da comparação byte a byte, tamanho, SHA-256
e ImageID.
