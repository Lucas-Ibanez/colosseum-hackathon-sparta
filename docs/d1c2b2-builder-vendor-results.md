# D1c2b.2 — bootstrap offline do registry do builder

Data da execução: 2026-10-02.

## Decisão

**BLOQUEADO.** O vendor temporário foi produzido integralmente em modo
offline e contém todas as 154 crates de registry do lock guest, incluindo
`borsh 0.10.4`, `borsh 1.8.1` e `risc0-zkvm 3.0.3`. A prova dentro do
builder, porém, não chegou ao Cargo: a primeira invocação de `docker run`
terminou com exit `127` e `/bin/sh: 0: Can't open cargo`.

Conforme a regra deste gate, não houve correção ou repetição da invocação e
`cargo metadata` não foi executado no container. Portanto ainda não há
evidência de que o comando de fetch passa dentro do builder, e D1c2b não pode
retomar.

## Preflight e integridade

- clone: `/home/lucas/src/vericode`, branch `main`;
- commit exportado:
  `398e9e0f7b902120749674ed78d8ecbe6b51d91a`;
- alterações iniciais preservadas: somente os três documentos modificados e
  o relatório não rastreado da tentativa D1c2b;
- staging: `/tmp/vericode-d1c2b2.wE4GqG/src`, criado por `mktemp -d` e
  preenchido exclusivamente por `git archive HEAD`;
- o staging não contém `.git` e não contém as mudanças documentais não
  commitadas do clone;
- SHA-256 do archive `HEAD.tar`:
  `e6cae5c8645f5899902c182885363a988ecc3b4d7562d0ce3334faa30a8b471f`.

Hashes aferidos no clone e no staging:

| Arquivo | SHA-256 |
| --- | --- |
| `zkvm/Cargo.lock` | `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1` |
| `zkvm/methods/guest/Cargo.lock` | `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50` |
| `zkvm/Cargo.toml` | `d09624eec5ecad29e2be51b4f0788f0b3bafaef46412f79023c7db6b5a716284` |
| `zkvm/host/Cargo.toml` | `08e58dcc1e6ab5c29790acbee7d874b865b2bcb8c7fd76d5b8ac2efb52b49536` |
| `zkvm/host/src/main.rs` | `33b07258a75aeb65bbc4bebcda82a219466689d3443b8ebb8a7258e53e0f4e8a` |
| `zkvm/methods/Cargo.toml` | `77a627064b517b480a27d5a0869c944f6ffe0249583c9197a574b1e1a9360b1c` |
| `zkvm/methods/build.rs` | `0edffb0f5f87b95c37b9533e956c45f1a537a763891bda167aa4297a9f4af28c` |
| `zkvm/methods/guest/Cargo.toml` | `a079b4fabcf78d83f7b17b5a5caea8ab77a7e5384ce47d94c9f818665daee54e` |
| `zkvm/methods/guest/src/main.rs` | `e3775fa098ca00a6a07a9a62557b7b977d71c3bef576f4f955a13488610d3bf1` |
| `zkvm/methods/src/lib.rs` | `d6ca30ca0af1b69da188e3b2af494212f072b7bd861e46875a096a7c90c83017` |

O lock guest contém 156 pacotes: 154 de
`registry+https://github.com/rust-lang/crates.io-index`, dois locais e zero
dependências Git.

## Mecanismo auditado

O fonte local oficial de `risc0-build 3.0.3`, arquivo `src/docker.rs`, cria um
contexto com `COPY . .` e executa primeiro:

```text
cargo +risc0 fetch --locked --target riscv32im-risc0-zkvm-elf \
  --manifest-path zkvm/methods/guest/Cargo.toml
```

Como `DockerOptions` não injeta a `CARGO_HOME` host, o mecanismo escolhido foi
colocar no contexto temporário, ao lado do manifesto guest, apenas o vendor e
a configuração de substituição emitida pelo próprio `cargo vendor`. Nada
disso foi criado no clone real.

Ferramentas efetivas:

```text
cargo 1.89.0 (c24e10642 2025-06-23)
rustc 1.89.0 (29483883e 2025-08-04)
CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo
CARGO_NET_OFFLINE=true
```

Antes do vendor, este comando confirmou que a cache host satisfaz o lock:

```text
cargo +1.89.0 fetch --locked --offline \
  --manifest-path zkvm/methods/guest/Cargo.toml
exit 0
```

O vendor foi produzido no staging por:

```text
cargo +1.89.0 vendor --locked --offline \
  --manifest-path Cargo.toml vendor
exit 0
```

A configuração impressa pelo Cargo e copiada literalmente para
`zkvm/methods/guest/.cargo/config.toml` foi:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

SHA-256 da configuração:
`77e9219c27274120197571fd165cbe4121963b5ad3bc0b20b383c86ef0ce6c2b`.

## Inventário do vendor

| Medida | Resultado |
| --- | ---: |
| Diretórios de crate | 154 |
| Arquivos regulares | 5.906 |
| `.cargo-checksum.json` | 154 |
| Tamanho por `du -sb` | 113.740.872 bytes |

Não faltou `.cargo-checksum.json` em nenhuma crate. A lista ordenada de
arquivos foi definida por:

```text
find vendor -type f -printf '%P\n' | LC_ALL=C sort
```

Seu SHA-256 é
`b7fde08fbf2c21916868069c6045e753d26ef932b6be34f4e27979900b223bcc`.
O inventário de conteúdo, calculado ordenando os paths, aplicando SHA-256 a
cada arquivo e então aplicando SHA-256 ao fluxo completo, é:
`6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`.

## Prova no builder e falha real

Imagem local fixada, inspecionada sem pull:

```text
risczero/risc0-guest-builder:r0.1.88.0
sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3
```

Comando executado:

```text
docker run --rm --pull=never --network none \
  -e CARGO_NET_OFFLINE=true \
  -v /tmp/vericode-d1c2b2.wE4GqG/src:/src:ro \
  -w /src/zkvm/methods/guest \
  risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3 \
  cargo +risc0 fetch --locked --offline \
  --target riscv32im-risc0-zkvm-elf \
  --manifest-path /src/zkvm/methods/guest/Cargo.toml
```

Resultado literal:

```text
exit 127
/bin/sh: 0: Can't open cargo
```

A saída indica que a invocação não chegou ao executável Cargo. A causa exata
na configuração de entrypoint/CMD da imagem não foi investigada depois da
falha, porque este gate determinava parada imediata. Não houve segunda
tentativa e o comando `cargo metadata --locked --offline` dentro do container
ficou **NÃO EXECUTADO**.

## Limites preservados

- o staging foi montado em `/src` somente-leitura;
- `--network none`, `CARGO_NET_OFFLINE=true` e `--pull=never` foram usados;
- não houve rede de registry, Docker pull, imagem derivada, `docker build`,
  compilação, check, teste, ELF, ImageID, receipt ou proving;
- nenhum arquivo sob `zkvm/` no clone foi criado ou modificado;
- os dois locks preservaram seus hashes;
- nenhuma toolchain foi instalada, atualizada ou removida;
- nenhuma operação Solana, Anchor, Router/CPI, wallet, keypair, `.env`,
  validator, transação, deploy, commit ou push ocorreu;
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

## Riscos e próximo passo

O vendor está completo e auditado, mas seu consumo pelo builder ainda não foi
provado. Um gate posterior deve primeiro inspecionar por leitura a configuração
`Entrypoint`/`Cmd` da imagem fixa e autorizar uma única repetição com a forma de
invocação correta, mantendo o mesmo staging, digest, mount somente-leitura,
`--network none`, `--pull=never` e os locks. Só após fetch e metadata com exit
`0` D1c2b poderá retomar.
