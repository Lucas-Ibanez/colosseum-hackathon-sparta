# R-D2e — revisão adversarial separada do D2b.1 e do D2e (somente leitura)

## Identificação
- Gate: `R-D2e` (revisão adversarial obrigatória do guia §11, antes de
  qualquer gate devnet)  ·  Marcos do guia: M3, M4 (local) e M5 (local)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Objeto da revisão:
  - D2b.1: `0e9838e` (core), `5736565` (anchor), `75a1971` (docs);
  - D2e: `2f10a8f` (`anchor: settle escrow through Verifier Router CPI (D2e)`)
    e `docs: record Router settlement (D2e)` (HEAD esperado).
- Relatórios:
  - `docs/d2b1-delivery-binding-results.md`;
  - `docs/d2e-router-settlement-results.md`.
- Resultado esperado: relatório de achados e veredito para o gate devnet
  (D4). **Nenhuma edição no repositório.**

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço max**. É a revisão de
  segurança do caminho de dinheiro completo (entrega, prova e pagamento)
  antes de qualquer rede.
- **Sessão nova e separada.** Não reutilizar o contexto da sessão que
  escreveu o código.
- **Somente leitura** (`CLAUDE.md`):
  - não criar, editar, mover ou apagar arquivos no clone;
  - não fazer commit, stash, reset ou checkout;
  - por isso, o protocolo de handoff é cumprido **apenas na resposta**, sem
    salvar arquivo.

## Leitura obrigatória (integral)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§2, §5, §7, §8, §9, §11)
- `docs/r-d2-adversarial-review-results.md`: base de comparação; conferir
  cada achado F-01 a F-15.
- `docs/escrow-state-machine.md`, `docs/escrow-program.md`,
  `docs/manifest-schema.md`, `docs/router-notes.md`
- Relatórios D2b.1 e D2e; `docs/decisions.md` (R-D2, "Decisões humanas para
  o D2b.1", D2b.1, D2e)
- Código:
  - `crates/vericode-core/src/escrow.rs` e `crates/vericode-core/src/lib.rs`
    (`JournalV1`, `validate_against`);
  - `anchor/programs/vericode-escrow/src/lib.rs`;
  - `anchor/tests-local/tests/{common/mod.rs,escrow.rs,settlement.rs,layout.rs,groth16_fixtures.rs}`;
  - `anchor/tests-local/fixtures/groth16/*`.
- Fonte pinado do Router, somente leitura:
  - `~/.local/share/vericode-spikes/d2c/staging/lane-b/risc0-solana` (commit
    `ee415935`);
  - `solana-verifier/programs/{verifier_router,groth_16_verifier}` e
    `solana-ownable`.
- Diff completo: `git diff 12529b4..HEAD -- crates anchor`.

## Preflight (somente leitura)
- `pwd`; raiz Git; branch; `git log --oneline -8`; HEAD esperado = commit
  `docs: record Router settlement (D2e)`.
- `git status --short` deve estar vazio. **Se houver alterações não
  commitadas, parar e reportar.**
- `git diff --check`.
- `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes; snapshot da
  listagem de `~/.cargo` (`d9e12578…`).

## Checagem de reprodutibilidade (opcional, sem editar o clone, offline)

Helper `~/.local/share/vericode-spikes/d2e/bin/env.sh`: `core_lane`, `d2c`,
`sbf_build`, `prog_tests`. Antes, ajustar `D` para um diretório próprio da
revisão, fora do clone.

| Comando | Esperado |
| --- | --- |
| Core `core_lane lane-a 1.85.0 test --locked --offline` e `lane-b 1.89.0` | 42 passed cada |
| `sbf_build <out>` | `vericode_escrow.so` 398.504 bytes, `6457aecf471e6d2cb38796fd7dc4442572001b3025fdf8330cde29b93ca9ca96` |
| `verifier_router.so` e `groth_16_verifier.so` no mesmo out-dir | `1b26b017…` e `dab6746d…`; copiar de `d2d/out/sbf` ou reconstruir offline (comando no relatório D2e) |
| `prog_tests <out> --no-fail-fast` | escrow 25, settlement 16, layout 6, fixtures 2 |
| `anchor-0.31.1 idl build -p vericode_escrow` (binário em `d2c/homes/lane-b/avm/bin`) | 6 instruções, 36 erros |

Provas de conceito de ataque podem ser escritas **somente** em diretório fora
do clone, por exemplo uma cópia de `anchor/tests-local` com paths absolutos e
o mesmo lock. Nada volta ao repositório. Citar o SHA-256 do arquivo de PoC.

## Checklist adversarial obrigatório

Para cada item, responder **achado** ou **verificado sem achado**, com
evidência em `arquivo:linha` ou na saída de um comando.

Guia §11:
1. **Troca de Job, prova ou artefato:**
   - FAIL ou PASS de artefato não entregue, com seal válido;
   - journal de outro Job, spec, harness ou ImageID;
   - replay de prova entre implantações (F-15);
   - squatting de `job_id` combinado com `deliver`.
2. **Replay e dupla liquidação:** todas as combinações de estado terminal ×
   6 instruções.
3. **Autoridade fraca:**
   - `deliver` só pelo executor;
   - liquidações permissionless;
   - poderes do dono do Router (e-stop, `add_verifier` sob outro selector,
     transferência de ownership);
   - upgrade authorities (F-04);
   - inexistência de admin.
4. **Destino controlável:**
   - derivação da ATA (seeds, token program, ATA program ID);
   - ATA com delegate ou close authority;
   - troca de owner da ATA (SPL Token clássico);
   - conta Token-2022;
   - ATA inexistente.
5. **Timeout injusto e prazos:**
   - janela de criação;
   - `deliver` no prazo sem tempo para provar;
   - executor que entrega e nunca prova;
   - release no slot do prazo;
   - e-stop × timeout.
6. **Serialização e hash:**
   - `RouterSeal` × `Seal` do Router;
   - os 332 bytes da CPI;
   - digest sobre exatamente os bytes decodificados;
   - `image_id` da CPI vindo do Job (F-12);
   - PDAs constantes;
   - discriminadores;
   - códigos 6000–6035 e tags 0–5.
7. **`Fail` abortando** em vez de ser valor normal (guest e core).
8. **Claims maiores que a implementação:** README, `router-notes`,
   `escrow-program`, `architecture`, relatórios; a frase "verificado por CPI
   ao Router em `solana-program-test` local".

Específicos de Solana/Anchor/Router:

9. **Substituição de contas:**
   - Router, router PDA, entrada e verificador;
   - entrada `estopped`;
   - verificador não executável;
   - ordem e flags das contas na CPI;
   - `system_program`;
   - vault e Job forjados.
10. **CPI:**
    - profundidade (escrow → Router → verificador);
    - reentrância;
    - lista de `AccountInfo` passada ao `invoke`;
    - efeito de erro do Router ou do verificador (reverte tudo?).
11. **Limites:**
    - tamanho da transação (journal, seal, 10 contas e compute budget)
      contra 1.232 bytes;
    - CU (135–142 k) contra o limite padrão;
    - variação pela busca do bump da ATA.
12. **Aritmética e overflow:** janela (`checked_sub`), prazos extremos,
    `overflow-checks`.
13. **Lacunas de teste:**
    - contas do Router montadas no genesis em vez de
      `initialize`/`add_verifier`;
    - IDL sem o endereço de `verifier_entry`/`verifier_program`;
    - F-12 não distinguível por teste;
    - asserts fracos.
14. **Supply chain e reprodutibilidade:**
    - rebuild offline do Router e do verificador;
    - `INITIAL_OWNER` de teste embutido no `.so` do Router;
    - staging limpo;
    - fixtures;
    - locks com checksum.
15. **Riscos conhecidos:** F-04, F-05, F-09, F-13, F-14, F-15, Router em
    devnet, ImageID não recertificado, spec v1 trivial. Confirmar ou
    reclassificar.

## Formato obrigatório do relatório (na resposta)

1. Preflight e comandos executados, com saídas reais resumidas.
2. Tabela de achados com:
   - ID, severidade (crítico / alto / médio / baixo / informativo),
     componente, `arquivo:linha`;
   - descrição, cenário de exploração, evidência (leitura ou PoC fora do
     clone), recomendação;
   - se **bloqueia o gate devnet** (sim/não).
3. Situação de cada achado do R-D2 (F-01 a F-15): resolvido, parcial ou
   aberto.
4. Checklist 1–15 com o resultado de cada item.
5. Veredito para o gate devnet (D4): `APROVADO`, `APROVADO COM RESSALVAS`
   (listar condições) ou `REPROVADO`.
6. Texto proposto para registro, a ser colado por uma sessão com permissão
   de escrita:
   - entrada em `docs/decisions.md`;
   - linha em `docs/evidence.md`;
   - conteúdo de `docs/r-d2e-adversarial-review-results.md`.
7. Prompt da próxima fase, segundo `docs/handoff-protocol.md`:
   - correções dos achados bloqueantes, se houver;
   - ou o gate devnet D4, com Test USDC, keypairs efêmeros (D2a.2), deploy,
     Explorer, decisão sobre o Router em devnet, F-04 e F-05.

## Proibições
- Editar, criar ou apagar qualquer arquivo no clone; commit, push, stash,
  reset ou checkout.
- Rede, instalação, Docker, devnet, deploy ou keypair novo.
- Ler, imprimir ou copiar conteúdo de keypairs. É permitido apenas copiar
  arquivos de keypair de programa para out-dirs temporários de build, sem
  exibi-los.
- Classificar como resolvido algo que não foi verificado; suavizar a
  severidade para não bloquear.

## Critério de pronto
- Todos os 15 itens respondidos com evidência; F-01 a F-15 reavaliados.
- Veredito explícito.
- `git status --short` vazio ao final, comprovando que nada foi alterado.
