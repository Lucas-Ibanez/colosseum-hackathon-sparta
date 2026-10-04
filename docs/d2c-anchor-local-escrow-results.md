# D2c — Perfil A e programa Anchor local com custódia SPL

Data: 2026-10-04

## Resultado

**CONCLUÍDO.**

1. **Perfil A escolhido por evidência:** raia B, com Anchor `0.31.1`, Agave
   `2.3.9` (platform-tools `v1.48`, `rustc 1.84.1-dev`) e Rust host `1.89.0`.
   - Ela compila SBF o `counter` oficial de `risc0-solana v3.0.0` com locks
     preservados e também o programa VeriCode.
   - A raia A (Agave `2.1.0`) não compila o `counter`.
2. **Programa `vericode_escrow` local:** `create_job`, `fund` e
   `refund_on_timeout`, com vault PDA de SPL Token e regra econômica
   delegada ao `vericode-core`.
   - 10/10 testes em processo (`solana-program-test 2.3.9`) contra o `.so`
     SBF.
   - Core 36/36 nas duas raias.

Não houve devnet, deploy, transação em rede pública, Router, CPI, release,
refund por `Fail`, Docker nem push. Router/CPI/devnet continuam
`STATUS: NÃO VALIDADO`.

## Preflight e checagem anterior

- HEAD `58838ae` (D2b); árvore limpa; `git diff --check` exit `0`.
- Hashes dos locks, de `escrow.rs` e de `lib.rs` iguais aos esperados.
- Core antes de editar: 36/36 em `+1.85.0` e `+1.89.0`.
- `~/.rustup` e `~/.cache/solana` estavam ausentes.
- **Divergência registrada:** o prompt de handoff afirmava que `~/.cargo`
  estaria ausente, mas ele existe desde 2026-09-29 21:34. É um cache de
  registry com 116 crates, já citado como existente e intocado no D1c2b.3b.
  - O erro estava no prompt, não no ambiente.
  - Decisão humana: prosseguir com snapshot (5.816 entradas, SHA-256
    `d9e12578…`).
  - Ao final, o snapshot foi recomparado por `cmp`: idêntico.
- **Também preexistente:** `~/.avm`, criado em 2026-09-29 20:23 (época do
  D1a.3). Está vazio, exceto por `.version`, e não foi tocado neste gate.

## Decisões confirmadas no Plan Mode

- Workspace separado `anchor/`, com lock próprio; os locks raiz e `zkvm/`
  ficaram intocados.
- Job PDA `["job", job_id]`, com `job_id` globalmente único; vault PDA
  `["vault", job]`.
- `refund_on_timeout` permissionless; o destino é fixo no buyer do Job.
- Testes com `solana-program-test` na versão exata do Agave escolhido
  (`=2.3.9`).
- Durante a execução, por decisão humana: aceitar o Criterion v2.3.3 baixado
  automaticamente pelo SDK Agave (ver Downloads).

## Ambiente isolado

Raiz `~/.local/share/vericode-spikes/d2c`. As homes `cargo`/`rustup`/`avm` e
as ferramentas Agave de cada raia são **cópias** de `d1a3`, para que nada do
D2c escreva nas instalações do D1a.3. Cada comando declarou:
- `env -i`;
- `HOME=d2c/homes/<raia>/home` (destino de `.cache/solana`);
- `CARGO_HOME`, `RUSTUP_HOME` e `AVM_HOME` isolados;
- `PATH` restrito;
- `RUSTUP_AUTO_UPDATE=0`;
- `CARGO_TARGET_DIR` em `d2c/targets`.

Versões observadas:

| Raia | Rust host | Agave | `cargo-build-sbf` | platform-tools | Anchor CLI |
| --- | --- | --- | --- | --- | --- |
| A | `1.85.0` | `2.1.0` (`c1080de4`) | `2.1.0` | `v1.43` (`rustc 1.79.0-dev`, `cargo 1.79.0`) | `0.31.1` |
| B | `1.89.0` | `2.3.9` (`47647df7`) | `2.3.9` | `v1.48` (`rustc 1.84.1-dev`, `cargo 1.84.0`) | `0.31.1` |

Comportamentos do `cargo-build-sbf`, confirmados no fonte dos commits exatos:
- grava as platform-tools em `$HOME/.cache/solana/<v>/platform-tools`;
- cria links simbólicos em `platform-tools-sdk/sbf/dependencies` da
  instalação Agave;
- executa `rustup toolchain link solana`;
- cria `<programa>-keypair.json` no `--sbf-out-dir` quando ele não existe.

Todos esses efeitos ficaram dentro de `d2c/`.

## Downloads

| Item | Origem | Bytes | SHA-256 observado | Digest oficial |
| --- | --- | ---: | --- | --- |
| platform-tools `v1.48` | `github.com/anza-xyz/platform-tools/releases/download/v1.48/platform-tools-linux-x86_64.tar.bz2` | 516.334.851 | `a9d3157f937d5f6faf4321a35a8035e0f827114a663b0150db492e893ec27597` | API da release: `digest: null`; tamanho igual ao da API |
| platform-tools `v1.43` | `…/releases/download/v1.43/platform-tools-linux-x86_64.tar.bz2` | 393.244.174 | `7a497174a0484bf8eee2d0a5a03a48f91ea0b5d1ba2d4055f37ef975a64ca8f6` | `digest: null`; tamanho igual ao da API |
| Criterion `v2.3.3` | `github.com/Snaipe/Criterion/releases/download/v2.3.3/criterion-v2.3.3-linux-x86_64.tar.bz2`, baixado automaticamente pelo `install.sh` oficial do SDK Agave 2.3.9 | 677.858 | tarball removido pelo script; 30 arquivos extraídos com SHA-256 agregado `c2f1f7b8faf4677d38bf6b0110a433c0915a471fc345c458708bbb0ee1102903` | não verificado |
| crates | `index.crates.io` / `static.crates.io` | — | checksums do `Cargo.lock` | lock |

Detalhes:
- As platform-tools foram baixadas por `curl` e extraídas como faz
  `install_if_missing`, para que o `cargo-build-sbf` não baixasse nada. A
  ausência de digest publicado é risco preservado, como no Agave `2.1.0`.
- **O Criterion estava fora da lista de rede autorizada.** O script o instala
  incondicionalmente quando ausente. O gate parou, a autorização humana foi
  concedida, e a raia A recebeu uma cópia local, sem nova rede.

## Fase 1 — Perfil A pelo `counter` oficial

O staging foi feito por `git clone --no-hardlinks` local dos checkouts D1a.3:
- HEAD `ee415935d04a948f27a346b563391900bdad6486`;
- locks antes e depois: `54949aa8…`, `49004c7c…`, `ab03b523…`;
- `git status` limpo ao final.

Comando:

```text
cargo-build-sbf --manifest-path programs/solana-counter/Cargo.toml \
  --sbf-out-dir <d2c/out/<raia>/counter> -- --locked
```

| Raia | Tentativa | Exit | Resultado |
| --- | --- | ---: | --- |
| B | offline | `1` | `no matching package named borsh`: o Cargo `1.84.0` usa o diretório de registry `index.crates.io-6f17d22bba15001f` (hash anterior ao Cargo 1.85), e a cache B só tinha `1949cf8c…` |
| B | crates.io (autorizado), `--locked` | `0` | 1m24s; `solana_counter.so` 254.520 bytes, SHA-256 `1f1e49e77476379273672d443cc79061fee438580fb5412858337c240c593f48`; locks inalterados |
| A | offline (cache com `6f17d22b…`) | `1` | 13 erros em `risc0-zkvm-platform 2.2.1`: `expected identifier, found keyword unsafe` em `#[cfg_attr(…, unsafe(no_mangle))]`; o `rustc 1.79.0-dev` das platform-tools `v1.43` não aceita essa sintaxe |

O keypair `solana_counter-keypair.json`, criado pelo `cargo-build-sbf` no
out-dir B, ficou fora do clone com `0600`.

### Programa VeriCode nas duas raias

| Raia | Exit | `.so` |
| --- | ---: | --- |
| B | `0` (1m15s) | `vericode_escrow.so` 297.440 bytes, SHA-256 `04cc2a845eea1b2e10e313601d1b51887f3f069f3e80c802651d499ddfd0ae56` |
| A (informativo) | `0` (1m43s) | 283.592 bytes, SHA-256 `5911e92d213b5557a44eaadd2d00a2f14f4066811d0e7323a4954183a510034a` |

### Decisão

**Perfil A = raia B** (Anchor `0.31.1` + Agave `2.3.9` + platform-tools
`v1.48` + Rust host `1.89.0`).
- É a única raia que compila SBF a dependência do Router (`counter` com locks
  preservados) e o programa VeriCode.
- Coincide com a CI do `risc0-solana v3.0.0`.
- A raia A falha justamente no caminho do Router. Como a verificação on-chain
  é o caminho forte do produto, a recomendação geral do Anchor (`2.1.0`) não
  prevalece.

## Fase 2 — workspace `anchor/`

### Program ID

Keypair criado com:

```text
solana-keygen new --no-bip39-passphrase --silent --outfile d2c/keys/vericode_escrow-keypair.json
```

- permissão `0600`; nenhuma saída exibida;
- `solana-keygen pubkey`: `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`;
- apenas a chave pública está em `declare_id!` e `Anchor.toml`.

### Resolução do lock do programa

1. **Resolução livre** com o Cargo das platform-tools e
   `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback`. Resultado
   inutilizável, registrado como evidência:
   - trouxe `anchor-attribute-*`/`anchor-syn 0.31.2`, fora do perfil;
   - trouxe `blake3 1.8.7`, que puxa `digest 0.11`/`cpufeatures 0.3.1`
     edition 2024;
   - trouxe `toml_edit 0.25`, que exige Rust 1.85;
   - `cargo tree` falhou: `feature edition2024 is required` (Cargo 1.84.0).
2. **Lock semeado com o lock oficial do `counter`** (`49004c7c…`, recém
   compilado com essas platform-tools). O Cargo `1.84.0` acrescentou só
   `anchor-spl 0.31.1`, `spl-token 7.0.0` e `vericode-core`, todos
   compatíveis com Rust 1.84.1.
3. O primeiro build falhou: `could not find token_2022 / token_interface in
   anchor_spl`.
   - O código gerado por `init` + `token::mint` no Anchor 0.31.1 chama
     `anchor_spl::token_interface::initialize_account3`
     (`lang/syn/src/codegen/accounts/constraints.rs`).
   - Correção: feature `token_2022` do mesmo `anchor-spl =0.31.1`.
   - Entraram 45 pacotes (`spl-token-2022 6.0.0`, `solana-zk-sdk 2.3.13` etc.),
     todos compatíveis com Rust 1.84.1.
   - O programa continua aceitando apenas o SPL Token clássico.
4. Build offline falhou porque um archive novo (`either`) não estava em cache.
   Repetido com crates.io: exit `0`.

O **core compilou para SBF sem alteração**.

Locks finais:
- `anchor/Cargo.lock`: 266 pacotes, SHA-256
  `19a1db26a33c51bf6e4818a69c9ddf587bdebce72a6d6615fe697d9b9dfa6765`, com
  `anchor-*` 0.31.1, `solana-program 2.3.0`, `borsh 0.10.4`/`1.5.7` e
  `sha2 0.10.9`;
- `anchor/tests-local/Cargo.lock`: workspace separado, porque dependências de
  teste do Agave 2.3 não podem entrar no grafo lido pelo Cargo 1.84. Foi
  semeado com o lock do programa e resolvido com o host `1.89.0`. Tem 708
  pacotes, SHA-256
  `be94760a8b49161c24ba630a862e91da1d441afa2d6105cdafc0097b439ea377`.

### Warnings

- Build SBF: 13 warnings, todos da expansão de macros do Anchor 0.31.1 (`cfg`
  `anchor-debug`/`custom-heap`/`custom-panic` não declarados e `realloc`
  depreciado).
- Testes: o único warning do nosso código (módulo `system_program`
  depreciado) foi corrigido; a execução final não tem warning em
  `tests/escrow.rs`.

## Testes em processo

```text
SBF_OUT_DIR=d2c/out/lane-b/vericode cargo +1.89.0 test --locked   (em anchor/tests-local)
```

Exit `0`, `test result: ok. 10 passed; 0 failed`:

```text
create_job_persists_terms_and_an_empty_pda_vault ... ok
create_job_rejects_invalid_terms_without_creating_accounts ... ok
create_job_rejects_a_duplicate_job_id ... ok
fund_moves_exactly_the_job_amount_into_the_vault ... ok
fund_rejections_move_no_tokens_and_keep_the_state ... ok
vault_is_controlled_by_the_job_pda_without_private_key ... ok
timeout_refund_fails_before_and_at_the_deadline ... ok
timeout_refund_after_the_deadline_returns_exactly_the_amount_to_the_buyer ... ok
timeout_refund_rejects_other_recipients_and_mints ... ok
timeout_refund_requires_funding_and_settles_once ... ok
```

Os logs do runtime confirmam erros Anchor com códigos exatos, por exemplo
`DepositorMismatch` 6009, `AlreadyFunded` 6006 e `MintMismatch` 6011.

| Exigência do gate | Teste |
| --- | --- |
| `create_job` válido | `create_job_persists_terms_and_an_empty_pda_vault` |
| termos inválidos (executor zero, `buyer == executor`, amount zero) | `create_job_rejects_invalid_terms_without_creating_accounts` |
| `job_id` duplicado | `create_job_rejects_a_duplicate_job_id` |
| `fund` exato | `fund_moves_exactly_the_job_amount_into_the_vault` |
| mint, signer, amount divergentes e duplo funding sem movimento | `fund_rejections_move_no_tokens_and_keep_the_state` |
| invariante 1 (PDA sem chave privada) | `vault_is_controlled_by_the_job_pda_without_private_key` |
| timeout antes e no slot do prazo | `timeout_refund_fails_before_and_at_the_deadline` |
| timeout após o prazo, permissionless, valor exato | `timeout_refund_after_the_deadline_returns_exactly_the_amount_to_the_buyer` |
| refund para outro dono/mint | `timeout_refund_rejects_other_recipients_and_mints` |
| Job não financiado; refund duplicado; fund após refund (invariante 9) | `timeout_refund_requires_funding_and_settles_once` |
| estado e saldo inalterados em toda rejeição (invariante 6, parte local) | snapshots byte a byte nos testes de rejeição |

### IDL

```text
anchor-0.31.1 idl build -p vericode_escrow -o d2c/out/idl/vericode_escrow.json
```

- Exit `0`; 11.899 bytes; SHA-256
  `9177e5962a80404c301716d9125fd26636231e0e67b97b5df97333de3e2ee9b9`.
- Instruções: exatamente `create_job`, `fund` e `refund_on_timeout`.
- `refund_on_timeout` não tem signer; contas: `JobAccount`; 24 erros
  (6000–6023).
- A IDL não foi versionada.

## Core e locks após o gate

- Core: 36/36 em `+1.85.0` e `+1.89.0`, com `anchor/` presente.
- Inalterados:
  - `Cargo.lock` `191802b2…3b87`;
  - `zkvm/Cargo.lock` `f5236689…e226`;
  - guest `1116acef…dbfa`;
  - `crates/vericode-core/Cargo.toml` `2d7845c7…`;
  - `lib.rs` `c51a810f…`;
  - `escrow.rs` `8ea483d2…`.

## Fronteiras e higiene

- Keypairs só em `d2c/keys` e `d2c/out/*`, todos `0600`.
- Nenhum keypair, `.json`, `target/` ou `.env` no clone.
- `git status --short --ignored` só mostrava os arquivos previstos.
- `~/.cargo` idêntico ao snapshot; `~/.rustup`, `~/.cache/solana` e
  `~/.config/solana` ausentes.
- Checkouts D1a.3 limpos.
- `.global-cache` das homes D1a.3 atualizado às 12:00:14–15, pelas
  reexecuções prescritas do core; nenhuma build do D2c usou essas homes.
- `git diff --check` exit `0`; a busca de segredos só encontrou o nome de um
  teste.
- Commit do código: `a10f026`.

## Riscos abertos

- Platform-tools e Criterion sem digest oficial publicado; só o tamanho foi
  verificado contra a API.
- Freeze authority do mint pode congelar o vault; mint aceito ainda é
  qualquer SPL Token clássico.
- Upgrade authority do futuro deploy é um bypass administrativo em potencial.
- Squatting de `job_id`; rent de Job/vault não recuperado.
- Dependência da feature `token_2022` do `anchor-spl` (exigência do Anchor
  0.31.1).
- `release`, `refund_on_fail` e verificação de prova não existem.
- Revisões adversariais separadas do D2b e do D2c: **PENDENTES**.
- ImageID `4da06f90…fb1a` não recertificado após as mudanças no core.
- Router/CPI/devnet: `STATUS: NÃO VALIDADO`.
