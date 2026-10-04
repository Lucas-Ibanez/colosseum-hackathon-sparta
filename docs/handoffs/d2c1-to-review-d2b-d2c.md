# R-D2 — revisão adversarial separada do D2b, D2c e D2c.1 (somente leitura)

## Identificação
- Gate: `R-D2` (revisão adversarial obrigatória do guia §11)
- Escopo:
  - D2b: política pura de escrow, commit `58838ae`;
  - D2c: programa Anchor local com custódia SPL, `a10f026`/`ec980e9`;
  - D2c.1: mint sem freeze authority e fixtures Groth16, commit
    `anchor: reject freezable mints and add Groth16 fixtures (D2c.1)`.
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`.
- Resultado esperado: relatório de achados e veredito para o D2e. **Nenhuma
  edição no repositório.**

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço max**. É a revisão de
  segurança do caminho de dinheiro, antes de adicionar o pagamento ao
  executor.
- **Sessão nova e separada**: não reutilizar o contexto da sessão que
  escreveu o código.
- **Somente leitura** (`CLAUDE.md`):
  - não criar, editar, mover ou apagar arquivos no clone;
  - não fazer commit, stash, reset ou checkout;
  - por isso, o protocolo de handoff é cumprido **apenas na resposta**, sem
    salvar arquivo.

## Leitura obrigatória (integral)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§2, §5, §7, §8, §11)
- `docs/escrow-state-machine.md`, `docs/escrow-program.md`,
  `docs/manifest-schema.md`
- Relatórios:
  - `docs/d2b-escrow-guide-alignment-results.md`;
  - `docs/d2c-anchor-local-escrow-results.md`;
  - `docs/d2c1-mint-freeze-and-fixtures-results.md`;
  - `docs/d2d-groth16-router-spike-results.md` (contexto do D2e).
- `docs/decisions.md` (D2a.1 a D2c.1)
- Código:
  - `crates/vericode-core/src/escrow.rs`;
  - `crates/vericode-core/src/lib.rs` (`JournalV1`, `validate_against`);
  - `anchor/programs/vericode-escrow/src/lib.rs`;
  - `anchor/tests-local/tests/escrow.rs`,
    `anchor/tests-local/tests/groth16_fixtures.rs`;
  - `anchor/tests-local/fixtures/groth16/*`;
  - `anchor/Cargo.toml`, `anchor/Anchor.toml` e os dois `Cargo.lock` de
    `anchor/`.
- Diff completo: `git diff 4d7e18f^..HEAD -- crates anchor`.

## Preflight (somente leitura)
- `pwd`; raiz Git; branch; `git log --oneline -12`.
- HEAD esperado: commit D2c.1 sobre `9a18f71`.
- `git status --short` deve estar vazio. **Se houver alterações não
  commitadas, parar e reportar**: o objeto da revisão precisa ser um commit.
- `git diff --check`.

## Checagem de reprodutibilidade (opcional, sem editar o clone)

É permitido executar os testes existentes, offline, com target **fora do
clone**. Não é permitido baixar nada.

Core (duas raias):

```bash
B=/home/lucas/.local/share/vericode-spikes/d1a3/homes; T=$(mktemp -d)
env -i HOME=$HOME CARGO_HOME=$B/lane-a/cargo RUSTUP_HOME=$B/lane-a/rustup \
  PATH=$B/lane-a/cargo/bin:/usr/bin:/bin RUSTUP_AUTO_UPDATE=0 CARGO_NET_OFFLINE=true \
  CARGO_TARGET_DIR=$T/a cargo +1.85.0 test --locked --offline      # esperado 36 passed
# repetir com lane-b e +1.89.0
```

Programa (Perfil A; `R=/home/lucas/.local/share/vericode-spikes/d2c`):

```bash
d2c() { L=lane-b; env -i HOME=$R/homes/$L/home CARGO_HOME=$R/homes/$L/cargo \
  RUSTUP_HOME=$R/homes/$L/rustup AVM_HOME=$R/homes/$L/avm \
  PATH=$R/homes/$L/cargo/bin:$R/tools/$L/solana-release/bin:/usr/bin:/bin \
  RUSTUP_AUTO_UPDATE=0 CARGO_NET_OFFLINE=true "$@"; }
O=$(mktemp -d); cp $R/keys/vericode_escrow-keypair.json $O/
(cd anchor && d2c env CARGO_TARGET_DIR=$T/sbf cargo-build-sbf \
  --manifest-path programs/vericode-escrow/Cargo.toml --sbf-out-dir $O -- --locked)
#   esperado vericode_escrow.so SHA-256 d66ac76bc8ef66488facc6473b7679eac2fa5e605fba409a6cd61acce20baaae
(cd anchor/tests-local && d2c env SBF_OUT_DIR=$O CARGO_TARGET_DIR=$T/tests \
  cargo +1.89.0 test --locked)
#   esperado escrow 12 passed, fixtures 2 passed
```

Provas de conceito de ataque podem ser escritas **somente** em diretório
temporário fora do clone (por exemplo, uma cópia de `anchor/tests-local` em
`$T`). O resultado deve ser citado no relatório; nada volta ao repositório.

## Checklist adversarial obrigatório

Para cada item, responder **achado** ou **verificado sem achado**, com
evidência em `arquivo:linha` ou na saída de um comando.

Guia §11:
1. Troca de Job/prova:
   - journal de outro Job, spec, harness ou ImageID;
   - unicidade de `job_id` (PDA `["job", job_id]`);
   - reutilização das fixtures em outro Job.
2. Replay e dupla liquidação em todas as combinações de estado terminal.
3. Autoridade fraca:
   - signer de `fund`;
   - instruções permissionless;
   - inexistência de admin e de bypass.
4. Destino de token controlável pelo chamador:
   - `buyer_token` de outro dono ou mint;
   - token account de outro programa;
   - delegate ou close authority na conta de destino;
   - Token-2022.
5. Timeout injusto: semântica `<=` / `>` no slot do prazo; `Clock` como fonte.
6. Mismatch de serialização ou hash: `JobAccount`/`EscrowStatus` ↔ `EscrowState`
   (ida e volta); `version`; tamanho `InitSpace`; códigos de erro estáveis.
7. `Fail` abortando em vez de ser valor normal.
8. Claims maiores que a implementação: README, `router-notes`,
   `escrow-program` e relatórios.

Específicos de Solana/Anchor:
9. Substituição de contas:
   - mint falso ou do Token-2022;
   - vault que não é a PDA;
   - `JobAccount` forjado ou de outro programa (discriminador e owner);
   - bump canônico.
10. CPI de token: seeds do signer da PDA; `transfer_checked` (mint e decimals);
    valor transferido igual a `job.amount`; doação extra ao vault.
11. Regra D2c.1: a constraint de freeze authority cobre todos os caminhos de
    criação? A premissa `MintCannotFreeze` vale para o SPL Token clássico
    usado? Existe outro meio de congelar ou fechar o vault?
12. Aritmética, overflow, `u64` e `deadline_slot` extremos (`0`, `u64::MAX`).
13. Lacunas de teste: invariante sem teste, teste que não exercita o caminho
    alegado, asserts fracos.
14. Supply chain e reprodutibilidade:
    - lock semeado do `counter`;
    - feature `token_2022` do `anchor-spl`;
    - Criterion e platform-tools sem digest;
    - fixtures em hex coerentes com os hashes.
15. Riscos já conhecidos (upgrade authority, squatting de `job_id`, rent,
    Router em devnet): confirmar a classificação ou reclassificar.

## Formato obrigatório do relatório (na resposta)

1. Preflight e comandos executados, com saídas reais resumidas.
2. Tabela de achados com: ID, severidade (crítico / alto / médio / baixo /
   informativo), componente, `arquivo:linha`, descrição, cenário de
   exploração, evidência (leitura ou PoC fora do clone), recomendação e se
   **bloqueia o D2e** (sim/não).
3. Checklist 1–15 com o resultado de cada item.
4. Veredito para prosseguir ao D2e: `APROVADO`, `APROVADO COM RESSALVAS`
   (listar condições) ou `REPROVADO`.
5. Texto proposto para registro, a ser colado por uma sessão com permissão de
   escrita:
   - entrada em `docs/decisions.md`;
   - linha em `docs/evidence.md`;
   - conteúdo de `docs/r-d2-adversarial-review-results.md`.
6. Prompt da próxima fase, segundo `docs/handoff-protocol.md`:
   - correções dos achados bloqueantes, se houver;
   - ou o D2e (`docs/handoffs/d2d-to-d2e.md`) com as ressalvas incorporadas.

## Proibições
- Editar, criar ou apagar qualquer arquivo no clone; commit, push, stash,
  reset ou checkout.
- Rede, instalação, Docker, devnet, deploy ou keypair novo.
- Ler, imprimir ou copiar conteúdo de keypairs. É permitido apenas copiar o
  arquivo do keypair do programa para o out-dir temporário do build, sem
  exibi-lo.
- Classificar como resolvido algo que não foi verificado; suavizar a
  severidade para não bloquear.

## Critério de pronto
- Todos os 15 itens respondidos com evidência.
- Veredito explícito.
- `git status --short` vazio ao final, comprovando que nada foi alterado.
