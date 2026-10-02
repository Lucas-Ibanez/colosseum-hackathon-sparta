# D1c2b.2a — validação do vendor dentro do builder

Data da execução: 2026-10-02.

## Decisão

**GO para o bootstrap do builder; D1c2b pode retomar posteriormente.**

O vendor temporário do D1c2b.2 foi consumido com sucesso pela imagem RISC Zero
fixada, sem rede e sem pull. O fetch passou quando o comando foi entregue ao
entrypoint `/bin/sh` por `-c`. O metadata offline também passou. Nenhuma
compilação ou geração de artefato ocorreu.

## Preflight e staging

- HEAD: `398e9e0f7b902120749674ed78d8ecbe6b51d91a`;
- branch: `main`;
- `zkvm/Cargo.lock`:
  `c55eecfa196a5db6cd79a153a586c68a9c688ec9c2a2ea98c56a3d9f2c18ced1`;
- `zkvm/methods/guest/Cargo.lock`:
  `bb00f8e71f1f1e969e27803fbcacd103caa00f2397ac8043d465b3feb1835a50`;
- staging reutilizado: `/tmp/vericode-d1c2b2.wE4GqG/src`;
- vendor: 154 crates, 154 `.cargo-checksum.json`, 113.740.872 bytes;
- inventário de conteúdo:
  `6d4478599d837bc4c1549d952d80e11d1886b09f5c7d33be75d66cb806cdf335`;
- inventário de paths:
  `b7fde08fbf2c21916868069c6045e753d26ef932b6be34f4e27979900b223bcc`;
- configuração Cargo:
  `77e9219c27274120197571fd165bce4121963b5ad3bc0b20b383c86ef0ce6c2b`.

O staging não contém `.git`; o clone real não recebeu `vendor/` nem
`.cargo/` sob `zkvm/`.

## Imagem e comandos efetivos

O JSON bruto da imagem confirmou `Entrypoint=["/bin/sh"]`. O campo `Cmd` não
está presente na configuração e é efetivamente `null`.

Imagem usada:

```text
risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3
```

Fetch, executado uma única vez:

```text
docker run --rm --pull=never --network none \
  -e CARGO_NET_OFFLINE=true \
  -v /tmp/vericode-d1c2b2.wE4GqG/src:/src:ro \
  -w /src/zkvm/methods/guest \
  risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3 \
  -c 'cargo +risc0 fetch --locked --offline --target riscv32im-risc0-zkvm-elf --manifest-path /src/zkvm/methods/guest/Cargo.toml'
```

Saída real: exit `0`, stdout/stderr vazio.

Metadata, executado uma única vez após o fetch:

```text
docker run --rm --pull=never --network none \
  -e CARGO_NET_OFFLINE=true \
  -v /tmp/vericode-d1c2b2.wE4GqG/src:/src:ro \
  -w /src/zkvm/methods/guest \
  risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3 \
  -c 'cargo +risc0 metadata --locked --offline --manifest-path /src/zkvm/methods/guest/Cargo.toml'
```

Saída real: exit `0`, com o aviso:

```text
warning: please specify `--format-version` flag explicitly to avoid compatibility problems
```

O JSON emitido confirmou `version: 1`, root
`path+file:///src/zkvm/methods/guest#vericode-guest@0.1.0`,
`workspace_root=/src/zkvm/methods/guest` e fontes vendorizadas sob
`/src/zkvm/methods/guest/vendor`. A saída JSON completa excedeu o limite de
transporte da ferramenta e foi truncada somente na apresentação; o exit code
real foi `0`.

## Limites preservados

- não houve `cargo build`, `cargo check`, `cargo test`, `cargo risczero build`
  ou proving;
- não foram gerados ELF, ImageID, receipt, seal ou prova;
- `CARGO_NET_OFFLINE=true`, `--network none`, `--pull=never` e mount `/src:ro`;
- nenhum Docker pull, imagem derivada, instalação ou atualização;
- nenhum arquivo do clone sob `zkvm/` foi alterado;
- nenhuma wallet, keypair, `.env`, Program ID, validator, transação, deploy,
  Solana, Anchor, Router/CPI ou rede blockchain;
- Router/CPI/devnet continuam `STATUS: NÃO VALIDADO`.

## Próximo gate

D1c2b pode retomar pelo build do guest, mantendo locks, cache isolada, vendor,
configuração temporária e digest do builder. Este resultado prova somente
resolução offline e metadata dentro do container; não prova compilação, ELF,
ImageID, receipt ou proving VeriCode.
