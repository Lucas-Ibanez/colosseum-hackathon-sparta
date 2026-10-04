# D2b — alinhar a política pura de escrow ao guia de produto

## Identificação
- Gate: `D2b`  ·  Dia da sequência: D2/D3 (estados, create/fund/refund em
  teste local)  ·  Marcos do guia: M3 e preparação do M4
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gates anteriores: `D2a` (commit `4d7e18f`,
  `docs/d2a-core-escrow-policy-results.md`) e `D2a.1` (commit
  `docs: register MVP context and handoff protocol`, registro de contexto)

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço high** — política
  econômica e de segurança (release, refund, timeout, dupla liquidação).
- **Plan Mode obrigatório** (`CLAUDE.md`: política de escrow).

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§5, §7, §9 e §11 em especial)
- `docs/escrow-state-machine.md`, `docs/d2a-core-escrow-policy-results.md`
- `docs/decisions.md` (entradas D2a e D2a.1), `docs/manifest-schema.md`,
  `docs/architecture.md`, `docs/mvp-agent-operating-guide.md`
- `crates/vericode-core/src/lib.rs` e `crates/vericode-core/src/escrow.rs`

## Preflight
- `pwd`; raiz Git; branch; HEAD (esperado: commit D2a.1 sobre `4d7e18f`);
  `git status --short` (esperado: vazio); `git diff --check`.
- Preservar qualquer alteração existente; sem reset, checkout destrutivo,
  clean ou stash.

## Checagem da tarefa anterior
- `git log --oneline -3` deve mostrar o commit D2a.1, `4d7e18f` (D2a) e
  `a3eb9e0`.
- Existem `docs/context/guia-mvp-agentes-de-codigo.md`,
  `docs/context/sequencia-mvp.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md` e este arquivo.
- Antes de editar, reexecutar na raia A (comando abaixo): esperado
  `test result: ok. 37 passed; 0 failed`.
- Hashes esperados: `Cargo.lock` `191802b234a6aa0f6bb9ce58a61963c377aed435a2baa6dec9576d13d8283b87`;
  `zkvm/Cargo.lock` `f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226`;
  `zkvm/methods/guest/Cargo.lock` `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
  `crates/vericode-core/src/escrow.rs` `f03110d31d1c79475b51e8e52d14ef9901bd5cbbe9ad989ffb44eaa37b2ad85e`.
- Se algo divergir: parar, registrar e reportar; não corrigir silenciosamente.

## Objetivo
Alinhar a política pura de escrow de `crates/vericode-core` ao guia de
produto: release ao executor em `Pass` válido, refund ao buyer em `Fail`
válido, refund por timeout somente após `deadline_slot`, e `artifact_hash`
registrado apenas na liquidação.

## Decisões já tomadas
- `artifact_hash` não é fixado pelo buyer nem pré-registrado pelo executor; a
  liquidação compara `schema_version`, `job_id`, `spec_hash`, `harness_hash`
  e `image_id` com o Job e registra o `artifact_hash` do journal no estado
  terminal — `docs/decisions.md` D2a.1; guia §5.
- `Fail` válido devolve tokens somente ao buyer — guia §7, invariante 4.
- Refund por timeout só ocorre após `deadline_slot`; antes falha — guia §7,
  invariante 5. "Após" = `current_slot > deadline_slot` (no próprio slot
  ainda falha); registrar essa leitura em `docs/decisions.md`.
- O core não lê relógio: `current_slot: u64` é parâmetro de entrada.
- `buyer == executor`, identidade toda-zero e amount zero continuam
  rejeitados — D2a.
- `Released` e `Refunded` são terminais; replay e dupla liquidação falham —
  guia §7, invariante 9.

## Decisões a confirmar no Plan Mode (perguntar ao humano)
1. **Mapeamento de estados.** Recomendação: estados de política
   `Created` (≙ `Draft`), `Funded`, `Released { artifact_hash }`,
   `Refunded { reason }`, com `reason` = `Fail { artifact_hash }` ou
   `Timeout`. `Proving`, `Submitted` e `Failed` são estados de worker/UI,
   sem transição econômica (guia §7 permite estados de UI mais detalhados).
2. **Release após o prazo.** O guia não define. Opções: (a) `Pass` válido
   pode liquidar enquanto o Job estiver `Funded`, mesmo após o prazo — vence
   a primeira liquidação; (b) release exige `current_slot <= deadline_slot`.
   Sem resposta: `AGUARDANDO_AUTORIZAÇÃO` e não implementar esse ponto.
3. **Refund por `Fail` após o prazo.** Mesma questão do item 2.

## Escopo autorizado
- `crates/vericode-core/src/escrow.rs` e, se o plano aprovado exigir, um
  acréscimo em `crates/vericode-core/src/lib.rs`, sem alterar os campos, o
  encoding ou `validate_against` de `JournalV1`.
- Criar `docs/d2b-escrow-guide-alignment-results.md` e
  `docs/handoffs/d2b-to-<próximo>.md`.
- Atualizar `docs/escrow-state-machine.md` (reescrever e remover a nota
  "Revisão pendente"), `docs/decisions.md`, `docs/evidence.md`,
  `docs/agent-control.md`, `docs/project-context.md` (estado da sequência e
  linhas resolvidas da tabela de conflitos) e, se necessário,
  `docs/architecture.md`.

## Fora de escopo / proibido
- `Cargo.toml`, locks, `zkvm/`, `JournalV1`, wire format,
  `docs/manifest-schema.md`, `docs/context/*` (fonte humana).
- Anchor, `Anchor.toml`, programas, IDL, Solana, SPL, validator, wallet,
  keypair, seed, `.env`, Program ID, airdrop, transação, deploy, Router, CPI,
  Docker, rede, instalação ou atualização de ferramentas.
- Verificação de receipt/seal/Groth16 ou qualquer alegação disso.
- Inventar regra para pontos não confirmados; mock ou stub como sucesso.
- Push.

## Implementação esperada
- `JobV1` ganha `deadline_slot: u64`; continua imutável.
- Remover o estado `Delivered` e `register_delivery`.
- Transições puras em `JobV1` (estado entra por valor, sai novo estado):
  - `fund` — sem mudança de regra;
  - release por `Pass` → `Released { artifact_hash }` com payout ao executor;
  - refund por `Fail` → `Refunded { Fail { artifact_hash } }` com payout ao
    buyer;
  - refund por timeout a partir de `Funded` com `current_slot > deadline_slot`
    → `Refunded { Timeout }` com payout ao buyer.
- O payout é sempre copiado do Job. Destinatário e mint informados pelo
  chamador só são validados. Nenhum parâmetro administrativo.
- Erros explícitos para cada rejeição, incluindo `DeadlineNotReached`,
  `VerdictNotPass`, `VerdictNotFail`, estados terminais distintos
  (`AlreadyReleased`, `AlreadyRefunded`) e divergência de destinatário/mint.
- Doc-comments mantêm a pré-condição: a receipt foi verificada por adaptador
  futuro; este módulo não verifica prova.

## Testes obrigatórios
- `Pass` válido → `Released` com payout ao executor, mint e amount do Job, e
  `artifact_hash` do journal registrado.
- `Fail` válido → `Refunded` com payout ao buyer; `Fail` não libera ao
  executor e `Pass` não gera refund por `Fail`.
- Timeout: `current_slot < deadline_slot` e `== deadline_slot` falham;
  `deadline_slot + 1` devolve ao buyer; timeout fora de `Funded` falha.
- Cada compromisso divergente (`job_id`, `spec_hash`, `harness_hash`,
  `image_id`) rejeitado no release **e** no refund por `Fail`.
- Journal com outro `artifact_hash` liquida normalmente e registra esse hash.
- Destinatário divergente (release ≠ executor; refund ≠ buyer) e mint
  divergente rejeitados.
- Job não funded rejeitado em release, refund por `Fail` e timeout.
- Dupla liquidação: release→release, release→refund, refund→release e
  refund→refund rejeitados.
- Ausência de bypass: matriz exaustiva estado × verdict × destinatário × mint
  × slot; só os casos elegíveis retornam `Ok`, sempre com payout do Job.
- Construção do Job (identidades, `buyer == executor`, amount zero) mantida.
- Os 20 testes D1c1/D1c2a continuam passando sem alteração.
- Comandos (offline, target fora do clone):

```bash
S=<scratchpad da sessão>; B=/home/lucas/.local/share/vericode-spikes/d1a3/homes
env -i HOME=$HOME CARGO_HOME=$B/lane-a/cargo RUSTUP_HOME=$B/lane-a/rustup \
  PATH=$B/lane-a/cargo/bin:/usr/bin:/bin RUSTUP_AUTO_UPDATE=0 \
  CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=$S/target-a \
  cargo +1.85.0 test --locked --offline
# repetir com lane-b e +1.89.0; depois `build` e `tree --locked --offline`
# nas duas raias e `cmp` das árvores
```

## Evidências exigidas
- `docs/d2b-escrow-guide-alignment-results.md` com preflight, checagem do D2a.1,
  diff, comandos e saídas reais nas duas raias, invariantes × testes,
  decisões confirmadas e pendentes, riscos e fronteiras.
- `sha256sum` dos três locks e de `Cargo.toml` (inalterados) e de
  `escrow.rs`/`lib.rs` finais.
- Linha D2b em `docs/evidence.md`; entrada D2b em `docs/decisions.md`.

## Critério de pronto
- Todos os testes obrigatórios passam com Rust `1.85.0` e `1.89.0`; sem
  warnings; árvores de dependência idênticas e sem Solana/Anchor/RISC Zero.
- Mapeamento de estados e regra de release após o prazo confirmados pelo
  humano, ou explicitamente em `AGUARDANDO_AUTORIZAÇÃO` sem implementação.
- `git diff --check` exit `0`; diff integral revisado; busca de segredos limpa.
- Invariantes 2, 3, 4, 5 (lógica), 7, 8 e 9 do guia §7 cobertas por teste.
  As invariantes 1, 6 e 10 continuam fora do core e declaradas como tais.

## Condições de parada
- Divergência na checagem anterior → `BLOQUEADO`.
- Decisão 2 ou 3 sem resposta → `AGUARDANDO_AUTORIZAÇÃO` para esse ponto.
- Necessidade de alterar `JournalV1`, wire format, locks ou dependências →
  parar e pedir decisão.
- Qualquer necessidade de rede, ferramenta nova, Anchor ou Docker → parar.

## Commit
- Um commit local autorizado ao final, somente se o critério de pronto for
  atendido: `core: align escrow policy with product guide (D2b)`. Usar a
  identidade dos commits anteriores via `git -c user.name=… -c user.email=…`,
  sem alterar a configuração do Git. Push proibido.
- Revisão adversarial separada (guia §11): recomendar ao humano uma sessão
  somente leitura (Opus 5.5, esforço max) sobre job/proof swap, replay, dupla
  liquidação, destino de token e timeout injusto. Se não for executada,
  registrá-la como pendente.

## Relatório final
1. arquivos modificados; 2. testes e saídas reais; 3. invariantes;
4. decisões pendentes; 5. riscos; 6. confirmação de que Anchor, Solana,
Router, wallet, Docker, rede e deploy não foram usados; 7. prompt da próxima
fase, segundo `docs/handoff-protocol.md`. O próximo gate provável é a decisão
humana do Perfil A Anchor/Agave (D1b), que bloqueia custódia SPL, devnet e
Router.
