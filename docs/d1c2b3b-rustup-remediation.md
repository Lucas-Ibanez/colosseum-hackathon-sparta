# D1c2b.3b.1 — remediação do Rustup padrão

Data da execução: 2026-10-02.

## Decisão

**GO para a remediação; D1c2b.3b ainda não foi reiniciado.**

O Rustup padrão criado involuntariamente foi removido. A toolchain permitida
continua disponível somente na home isolada D1a.3.

## Causa

Durante o preflight de D1c2b.3b, o proxy `cargo +1.89.0` foi consultado sem
`RUSTUP_HOME`. O Rustup padrão, que não tinha toolchains instaladas, iniciou a
sincronização de `1.89.0-x86_64-unknown-linux-gnu` e criou a instalação em
`/home/lucas/.rustup`.

A saída observada foi:

```text
no installed toolchains
info: syncing channel updates for 1.89.0-x86_64-unknown-linux-gnu
info: downloading 6 components
```

## Inventário antes

O root continha somente os quatro grupos autorizados:

| Caminho | Estado/tamanho observado |
| --- | ---: |
| `/home/lucas/.rustup/downloads` | diretório vazio, 0 bytes |
| `/home/lucas/.rustup/tmp` | diretório vazio, 0 bytes |
| `/home/lucas/.rustup/toolchains/1.89.0-x86_64-unknown-linux-gnu` | 1.204.302.937 bytes |
| `/home/lucas/.rustup/update-hashes/1.89.0-x86_64-unknown-linux-gnu` | 20 bytes |

Não havia `settings.toml`, outra toolchain ou arquivo de configuração do
Rustup no root. A busca nominal não encontrou credenciais; os únicos nomes
semelhantes encontrados estavam dentro da documentação HTML instalada pela
própria toolchain.

## Remoção

Foi usado somente o Rustup isolado, com o root acidental explicitamente
declarado e auto-update desabilitado:

```text
env RUSTUP_HOME=/home/lucas/.rustup RUSTUP_AUTO_UPDATE=0 \
  /home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin/rustup \
  toolchain uninstall 1.89.0-x86_64-unknown-linux-gnu
```

Resultado real: exit `0`, com `toolchain ... uninstalled`. Os quatro
diretórios restantes foram confirmados vazios e removidos com `rmdir`; o root
`/home/lucas/.rustup` ficou ausente.

## Validação depois

- `/home/lucas/.rustup`: ausente;
- toolchain acidental: ausente;
- home isolada D1a.3: preservada, com `1.89-x86_64-unknown-linux-gnu`,
  `1.89.0-x86_64-unknown-linux-gnu` e `risc0` listados pelo Rustup isolado;
- `/home/lucas/.cargo`: não foi alvo de qualquer comando de remoção e
  manteve os mesmos metadados de topo observados antes/depois;
- repositório: nenhum arquivo em `zkvm/` alterado;
- locks preservados:
  - `zkvm/Cargo.lock` —
    `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
  - `zkvm/methods/guest/Cargo.lock` —
    `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50`.

## Regra para a retomada

Todo comando Cargo/Rust da retomada deve declarar explicitamente, no mesmo
ambiente controlado, `CARGO_HOME`, `RUSTUP_HOME`, `RISC0_HOME` e `PATH` das
homes D1a.3/D1c2b. Consultas com `cargo +versão` sem esses valores são
proibidas.

Não houve build, Docker, guest, host, receipt, proving, operação Solana,
wallet, chave, `.env`, deploy, commit ou push nesta remediação.
