# D1c2b.3d — reconciliação offline do lock do guest

Data: 2026-10-02

## Decisão

**BLOQUEADO.** A reconciliação mecânica do lock foi aplicada e seu diff foi
aceito: somente `risc0-groth16`, `enum-ordinalize` e
`enum-ordinalize-derive` mudaram para as versões já comprovadas pelos dois
tags oficiais exatos. Entretanto, a validação `metadata/tree --locked
--offline` não concluiu porque a `CARGO_HOME` D1c2b contém as entradas do
índice, mas não contém os archives das três versões reconciliadas. O primeiro
archive solicitado foi `enum-ordinalize 4.3.0`; o Cargo recusou corretamente
qualquer tentativa de download em modo offline.

O lock reconciliado foi mantido porque não houve deriva não comprovada. Este
resultado não libera build, vendor, ELF, ImageID, host ou receipts.

## Preflight e isolamento

- clone: `/home/lucas/src/vericode`, filesystem Linux `ext4`;
- branch observada: `main`;
- HEAD observado: `398e9e0f7b902120749674ed78d8ecbe6b51d91a`;
- `git diff --check`: exit `0` antes da alteração;
- `/home/lucas/.rustup`: ausente;
- hash inicial de `zkvm/Cargo.lock`:
  `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
- hash inicial de `zkvm/methods/guest/Cargo.lock`:
  `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50`.

O working tree já continha somente documentação dos gates D1c2b anteriores,
modificada ou não rastreada. Ela foi preservada; não houve stash, reset,
checkout, descarte ou sobrescrita.

Todo comando Cargo deste gate recebeu explicitamente:

```text
CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo
RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup
RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0
PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin
RUSTUP_AUTO_UPDATE=0
CARGO_NET_OFFLINE=true
```

## Proveniência do conjunto reconciliado

Foram conferidos localmente, em checkouts detached e limpos, os commits:

- `risc0/risc0 v3.0.3`:
  `14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6`;
- `boundless-xyz/risc0-solana v3.0.0`:
  `ee415935d04a948f27a346b563391900bdad6486`.

O lock guest do `hello-world` do primeiro tag e o lock guest do `counter` do
segundo preservam `enum-ordinalize 4.3.0` e
`enum-ordinalize-derive 4.3.1`. O lock guest do `counter` também fixa
`risc0-groth16 3.0.2` com checksum publicado. As entradas já existentes no
índice isolado D1c2b confirmaram os mesmos checksums e requisitos.

Fontes exatas:

- [`risc0/risc0`, lock guest do `hello-world`](https://github.com/risc0/risc0/blob/14b5d588dd01cf4f7ba804d8bb0a61264e6ae2c6/examples/hello-world/methods/guest/Cargo.lock);
- [`risc0-solana`, lock guest do `counter`](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/zkvm/methods/guest/Cargo.lock);
- [`risc0-solana`, lock da workspace zkVM do `counter`](https://github.com/boundless-xyz/risc0-solana/blob/ee415935d04a948f27a346b563391900bdad6486/examples/counter/zkvm/Cargo.lock).

## Alteração exata do lock

Os três comandos abaixo foram executados com o ambiente isolado acima e
`--manifest-path /home/lucas/src/vericode/zkvm/methods/guest/Cargo.toml`:

```text
cargo +1.89.0 update --offline -p risc0-groth16@3.0.5 --precise 3.0.2
Downgrading risc0-groth16 v3.0.5 -> v3.0.2; exit 0

cargo +1.89.0 update --offline -p enum-ordinalize@4.4.2 --precise 4.3.0
Downgrading enum-ordinalize v4.4.2 -> v4.3.0; exit 0

cargo +1.89.0 update --offline \
  -p enum-ordinalize-derive@4.4.2 --precise 4.3.1
Downgrading enum-ordinalize-derive v4.4.2 -> v4.3.1; exit 0
```

O diff completo por package é:

| Package | Antes | Depois | Checksum anterior | Checksum final |
| --- | ---: | ---: | --- | --- |
| `risc0-groth16` | `3.0.5` | `3.0.2` | `b0ca702ea7d0162766defe7ed6a79bda4a747ad9e2684000a6edd14df0a6d1f3` | `724285dc79604abfb2d40feaefe3e335420a6b293511661f77d6af62f1f5fae9` |
| `enum-ordinalize` | `4.4.2` | `4.3.0` | `89dd01549b09589510cf0647475075d12071456586d70f5c75c98ae2a5537677` | `fea0dcfa4e54eeb516fe454635a95753ddd39acda650ce703031c6973e315dd5` |
| `enum-ordinalize-derive` | `4.4.2` | `4.3.1` | `a65863d15a4ce2888bd2f0f543cc963d3879c3a022c8ee43f6141d479a3ac815` | `0d28318a75d4aead5c4db25382e8ef717932d0346600cacae6357eb5941bc5ff` |

Não houve package adicional ou removido. A única mudança além de
versão/checksum foi, dentro do bloco de `enum-ordinalize-derive`, a referência
`syn 3.0.6 -> syn 2.0.119`. Ela pertence ao fechamento comprovado: o manifesto
publicado de `4.3.1` exige `syn ^2`, e ambos os locks oficiais também resolvem
esse derive contra `syn 2.x`. `syn 2.0.119` já existia no lock VeriCode; sua
própria entrada não foi adicionada nem alterada.

Os pins diretos permanecem intactos:

- `risc0-zkvm = "=3.0.3"` no manifesto guest e `3.0.3` no lock guest;
- `risc0-build = "=3.0.3"` no manifesto methods e `3.0.3` no lock
  host/methods.

## Hashes antes e depois

| Arquivo | Antes | Depois |
| --- | --- | --- |
| `zkvm/Cargo.lock` | `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1` | `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1` |
| `zkvm/methods/guest/Cargo.lock` | `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50` | `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa` |

O lock host/methods permaneceu byte a byte inalterado.

## Validação locked/offline

A `CARGO_HOME` D1c2b contém entradas de índice para as três versões finais,
mas não contém os arquivos:

```text
risc0-groth16-3.0.2.crate
enum-ordinalize-4.3.0.crate
enum-ordinalize-derive-4.3.1.crate
```

Por isso, a validação não pôde materializar os manifests de registry. O
comando real:

```text
cargo +1.89.0 metadata --locked --offline --format-version 1 \
  --manifest-path /home/lucas/src/vericode/zkvm/methods/guest/Cargo.toml
```

terminou com exit `101`:

```text
error: failed to download `enum-ordinalize v4.3.0`

Caused by:
  attempting to make an HTTP request, but --offline was specified
```

`cargo tree --locked --offline` e as três árvores inversas para
`risc0-groth16@3.0.2`, `enum-ordinalize@4.3.0` e
`enum-ordinalize-derive@4.3.1` também terminaram com exit `101` e a mesma
mensagem. Não houve tentativa de rede; o erro descreve a operação que seria
necessária fora do modo offline.

Os archives exatos existem em caches D1a.3 previamente auditados, mas não
foram copiados porque este gate autorizou modificar somente o lock guest e a
documentação. Misturar homes ou semear a cache sem autorização explícita
violaria o isolamento definido.

## Limites e riscos abertos

- não houve build, check, teste, vendor, Docker, host, proving ou receipt;
- não houve instalação, atualização de toolchain, rede ou modificação de
  configuração global;
- nenhum manifesto, fonte, core, schema, arquitetura ou lock host foi
  modificado;
- `/home/lucas/.rustup` permaneceu ausente;
- não houve Solana, Anchor, Router, CPI, wallet, keypair, `.env`, validator,
  transação, deploy, commit ou push;
- o lock final tem proveniência comprovada, mas sua resolução completa ainda
  não foi validada pela `CARGO_HOME` prescrita;
- build guest, ELF, ImageID e receipts VeriCode continuam pendentes;
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Próximo gate recomendado

Autorizar um bootstrap offline mínimo da `CARGO_HOME` D1c2b, copiando somente
os três archives públicos exatos já auditados nos caches D1a.3, após conferir
seus SHA-256 contra os checksums finais acima. Em seguida, repetir
`metadata/tree --locked --offline` e as árvores inversas. Somente após esses
comandos passarem deve-se recriar o vendor e retomar os dois builds
determinísticos.
