# R-D2e — revisão adversarial do D2b.1 e do D2e

Data: 2026-10-05 · Revisor: Claude Code (Opus 5.5, esforço max), sessão
separada e somente leitura · HEAD revisado: `c0aba7d` (D2e docs) sobre
`2f10a8f`, `75a1971`, `5736565` e `0e9838e`.

> Registro feito por outra sessão, com permissão de escrita, a partir da
> resposta da revisão. As seções 1 a 5 abaixo reproduzem a revisão; só as
> tabelas foram convertidas para markdown.

## Resultado

**APROVADO COM RESSALVAS para o D4.**
- Não há achado crítico ou alto no código de D2b.1/D2e.
- F-01, F-02, F-03, F-06, F-08, F-10, F-11 e F-12 estão resolvidos; F-07
  ficou parcial.
- O deploy depende de três decisões humanas tomadas antes dele: o Router em
  devnet (achado novo R-01, médio), F-04 e F-05.
- As condições C1 a C7 valem antes de qualquer deploy.
- Nenhum arquivo do clone foi alterado pela revisão: `git status --short`
  vazio no início e no fim, e `--ignored` também vazio.

## 1. Preflight e comandos executados

| Checagem | Resultado real |
| --- | --- |
| `pwd` / raiz / branch | `/home/lucas/src/vericode` / mesma / `main` |
| HEAD | `c0aba7d docs: record Router settlement (D2e)`, sobre `2f10a8f`, `75a1971`, `5736565`, `0e9838e` e `12529b4` ✔ |
| `git status --short` (início/fim) | vazio / vazio; `--ignored` vazio |
| `git diff --check` | exit 0 |
| Perfil padrão | `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes; `~/.cargo` `d9e12578…` (5.816 entradas), `~/.avm` `7d29f7f8…` e `~/.docker` `6046f67f…` iguais no início e no fim |

Os comandos rodaram offline, com o helper copiado para
`~/.local/share/vericode-spikes/rd2e/bin/env.sh` (`D` próprio da revisão).
Targets e out-dirs ficaram fora do clone.

| Comando | Saída real |
| --- | --- |
| `core_lane lane-a 1.85.0 test --locked --offline` | exit 0; 42 passed |
| `core_lane lane-b 1.89.0 test --locked --offline` | exit 0; 42 passed |
| `sbf_build rd2e` (target novo) | exit 0, 82 s, 15 warnings de macro; `vericode_escrow.so` com 398.504 bytes, `6457aecf471e6d2cb38796fd7dc4442572001b3025fdf8330cde29b93ca9ca96` (idêntico) |
| Rebuild offline de `groth_16_verifier` e `verifier_router` (staging `ee415935`, `INITIAL_OWNER=8pS2iUcJ…`, keypairs de programa do D2d copiados sem leitura) | `dab6746d…` e `1b26b017…` (idênticos); staging com 0 arquivos alterados; lock `54949aa8…` |
| `prog_tests rd2e --no-fail-fast` | exit 0; escrow 25, settlement 16, layout 6, fixtures 2 |
| `anchor-0.31.1 idl build -p vericode_escrow` | exit 0; 21.779 bytes, `37a3028a…` (idêntico); 6 instruções e 36 erros (6000–6035); sem endereço para `verifier_entry` e `verifier_program` |
| CU (`settlement_fits_the_default_compute_limit`, 3 execuções) | release 135.516 / 135.516 / 141.516; `refund_on_fail` 136.845 / 144.345 / 136.845 |
| Fixtures por caminho independente (Python `hashlib`) | digest == SHA-256(journal); schema 1, Job `0x11`, spec, harness, artefato `(7,14)`/`(7,15)`, image `4da06f90…`, verdict 0/1 e selector `73c457ba` conferem |
| Locks | raiz `191802b2…` 29/29, `zkvm` `f5236689…` 458/458, guest `1116acef…` 154/154, `anchor/` `19a1db26…` 264/264, `tests-local` `be94760a…` 705/705: todos com checksum, nenhuma fonte git, inalterados |
| Busca de segredos no diff `12529b4..HEAD`; arquivos versionados | sem ocorrências; nenhum keypair versionado; `.env.example` só com nomes |
| Bytes embutidos (Python) | `.so` do Router: owner de teste `8pS2…` 1×, ID do Router 1×, ID do verificador 0×. `.so` do verificador: o próprio ID 1× |

**PoCs.** Rodaram numa cópia de `anchor/tests-local` fora do clone, em
`~/.local/share/vericode-spikes/rd2e/poc/tests-local`, com paths absolutos e
o mesmo lock `be94760a…`.
- Arquivo: `tests/rd2e_poc.rs`, SHA-256
  `a8aced162549275fc4a99dd2ebff979c7290d3825ca0b3aef01412c174389232`.
- Resultado: `--locked --offline` → exit 0, 8 passed.
- O PoC-8 usa uma variante do escrow em `rd2e/variant`. Só o `declare_id`
  difere (`PHLeiLZtbV2RLnXbDwU5EChFQySB8aPxqKZE4mCLyxQ`, derivado de um
  SHA-256, sem keypair). O `.so` da variante é `aee1caea…`.
- Nada voltou ao repositório. O diretório é persistente (não está em `/tmp`).

| PoC | Resultado observado |
| --- | --- |
| 1 | O SPL clássico aceita `SetAuthority(AccountOwner)` na ATA do executor. O release para o endereço canônico, agora com outro owner, falha com 6010 sem mover nada; com o owner restaurado, paga 1.000.000. O buyer que troca o owner da própria ATA recebe 6010 no timeout |
| 2 | ATA do executor inexistente → 3012; um terceiro cria a ATA → release paga. ATA do buyer fechada → 3012; recriada → o timeout devolve |
| 3 | Com `Clock.slot == 5000` (o prazo): timeout → 6021; release aceito; deliver+release numa única transação aceitos → `Released` |
| 4 | fund no slot 5001 (> prazo) aceito → `Funded`; deliver → 6022; refund imediato. fund no slot 4999 deixa 2 slots ao executor |
| 5 | Dados de release = 437 bytes, 10 contas. Transações: release 937 B; +CU limit 977 B; deliver+release (2 signers) 1.078 B; CU+deliver+release 1.118 B (limite 1.232). A maior foi enviada e aceita |
| 6 | 3 terminais × {fund, deliver, release, `refund_on_fail`, `refund_on_timeout`, `create_job` com o mesmo id}: 18/18 rejeitados (6007/6008; `Custom(0)` no `create_job`), com snapshot igual |
| 7 | Em release: Job forjado → 3007; vault falso → 2006; conta Token-2022 do executor → 3007; system program trocado → 3008; Job read-only → 2000 |
| 8 | A mesma (journal, seal) e o `h` copiado do Job `0x11` da implantação A liquidam o Job `0x11` da implantação B (`PHLeiL…`). O executor B recebe 1.000.000 sem conhecer o artefato |

## 2. Achados

| ID | Sev. | Componente | Local | Resumo | Evidência | Bloqueia D4 |
| --- | --- | --- | --- | --- | --- | --- |
| R-01 | Médio | integração Router/devnet | `anchor/programs/vericode-escrow/src/lib.rs:49-68`; Router `lib.rs:32,36`; verificador `lib.rs:32`; anchor-syn `codegen/program/entry.rs:51-53` | constantes do Router = endereços upstream; os `.so` testados só executam nesses endereços | leitura; bytes no `.so` | Sim, condição C1: bloqueia o deploy, não o início do D4 |
| R-02 | Baixo | core + programa | `crates/vericode-core/src/escrow.rs:429-455`; `lib.rs:148-177` | `fund` não respeita a janela do prazo (resíduo do F-07) | PoC-4 | Não (ressalva C5) |
| R-03 | Baixo (reclassifica F-15) | contrato de dados | `crates/vericode-core/src/lib.rs:386-394`; fixtures `0x11` | replay da prova entre implantações confirmado | PoC-8 | Não (ressalva C4) |
| R-04 | Baixo | docs | `docs/router-notes.md:140-158`; `docs/escrow-program.md:119-121,167`; `docs/d2e-router-settlement-results.md:157-160` | claims desatualizados ou imprecisos | leitura; CU | Não |
| R-05 | Baixo | testes | `anchor/tests-local/tests/settlement.rs` | lacunas da suíte; o comportamento está correto | PoC-1 a 8 | Não; ver C1 |
| R-06 | Info | confiança no Router | Router `estop/mod.rs:84-107,133-170` | e-stop irreversível como alavanca de liveness | leitura; teste existente | Não |
| R-07 | Info | limites | PoC-5; CU | margens de tamanho e CU | PoC-5; medição | Não |

### R-01 — Router e verificador fixados nos endereços upstream

**Descrição:**
- `VERIFIER_ROUTER_ID`, `ROUTER_PDA`, `GROTH16_VERIFIER_ENTRY` e
  `GROTH16_VERIFIER_ID` derivam dos `declare_id` upstream.
- O Anchor rejeita `program_id ≠ ID` (`DeclaredProgramIdMismatch`), então os
  `.so` testados só rodam em `6JvFf…`/`THq1q…`.
- O projeto não tem esses keypairs: os keypairs de programa do D2d têm outros
  endereços.
- O Router testado embute o `INITIAL_OWNER` de teste. Um Router upstream
  implantado embute outro owner, então seu hash nunca será `1b26b017…`.

**Cenário:** o D4 implanta o escrow atual e um Router próprio num endereço
novo.
- Toda liquidação por veredito falha; só o timeout funciona.
- Ou as quatro constantes são trocadas às pressas, mantendo a upgrade
  authority do Router próprio. Isso é bypass administrativo (F-04,
  princípio 7).

**Recomendação:** decisão humana no início do D4:
- (a) Router upstream verificado por RPC somente leitura; ou
- (b) fork próprio com novos `declare_id`/`INITIAL_OWNER`, verificador sob a
  PDA do Router e Router finalizado, com troca das constantes e de
  `tests/layout.rs` sob Plan Mode e revisão delta separada antes do deploy;
  ou
- (c) sem Router em devnet e sem claim de verificação on-chain.

### R-02 — funding fora da janela

**Descrição:**
- A janela de 1.500 slots só é imposta em `create_job`; `fund` não lê o
  `Clock`.
- Um Job pode ficar `Funded` depois do prazo (refund imediato) ou com 2 slots
  restantes.
- O R-D2 PoC-5 tinha esse mesmo sintoma ("financiado e reembolsado sem janela
  de release"), por outro caminho.

**Cenário:** um executor que começou antes do funding, ou que não confere a
janela, perde a chance; o timeout devolve ao buyer. Não há perda de fundos.

**Mitigação existente:** o executor pode provar antes e enviar deliver+release
atômicos (PoC-3/5).

**Recomendação:**
- (i) `fund` exige `Clock.slot + MIN_DEADLINE_WINDOW_SLOTS ≤ deadline_slot`,
  em gate próprio; ou
- (ii) no D4, `create_job`+`fund` na mesma transação e o executor confere a
  janela restante.

### R-03 — replay entre implantações (F-15 → baixo)

**Descrição:** o journal não contém program ID. As fixtures públicas liquidam
qualquer Job `0x11` em qualquer implantação ou cluster.

**Cenário:** na devnet, uma demo com o Job `0x11` pode ser bloqueada por
squatting (F-09). Um buyer que reutilize um `job_id` já usado paga sem
receber o artefato.

**Recomendação:**
- no D4, `job_id` aleatório de 32 bytes por Job, nunca `0x11`, e receipts
  novas por Job;
- depois do MVP, `job_id = SHA-256(domínio ‖ program_id ‖ buyer ‖ nonce)`
  conferido em `create_job`, o que também resolve F-09.

### R-04 — claims

- (a) `router-notes.md:144` ainda diz "não implementar CPI antes de confirmar
  Program ID…". Os itens `[ ] provar CPI runtime` e `[ ] testar Job, mint e
  executor` (`:155-158`) também estão desatualizados depois do D2e.
- (b) `escrow-program.md:119-121` diz que a upgrade authority do verificador
  "pode trocar o código". O Router pinado não tem `invoke_signed`, então o
  verificador só muda depois de um upgrade do Router.
- (c) CU: o documentado é 135–140 k para `refund_on_fail`, e a revisão
  observou 144.345.
- (d) A IDL omite dois endereços porque o anchor-syn só aceita constantes
  `[A-Z_]` (`idl/accounts.rs:163-171`) e `GROTH16_*` contém dígitos. Não é uma
  limitação genérica do gerador.

**Recomendação:** errata nos docs do D4; renomear as constantes ou documentar
os dois endereços para clientes.

### R-05 — lacunas de teste

- A suíte não cobre os casos dos PoCs 1 a 8: matriz terminal × instrução
  completa, substituições em `SettleWithProof`, liquidação no slot do prazo,
  troca de owner e ATA inexistente, funding fora da janela, tamanho de
  transação e replay.
- Falta teste do ciclo `initialize`/`add_verifier` com o verificador
  upgradeable sob a PDA do Router; as contas são montadas no genesis.
- O F-12 não é distinguível por teste: uma mutação para
  `decoded.image_id()` sobrevive, por construção.

**Recomendação:** incorporar os PoCs 1 a 7. Se o D4 usar Router próprio, o
teste de ciclo vira obrigatório (C1).

### R-06 — e-stop como alavanca de liveness

- O dono do Router pode dar e-stop, de forma irreversível, perto do prazo e
  forçar o timeout ao buyer.
- Com o selector fixado, um e-stop de `73c457ba` encerra para sempre a
  liquidação por veredito daquela implantação.
- Qualquer pessoa com uma prova de exploração também consegue disparar.
- Isso afeta liveness e censura, não a custódia. Documentar e pesar no D4-1.

### R-07 — margens

- Tamanho: a maior transação fica 114 bytes abaixo do limite;
  `SetComputeUnitPrice` acrescenta cerca de 12.
- CU: a busca do bump da ATA custa cerca de 1.500 CU por tentativa. Passar de
  200 k exigiria cerca de 43 tentativas extras, probabilidade da ordem de
  2⁻⁴³.
- CPI: usa 3 de 5 níveis.

## 3. Situação de F-01 a F-15

| Achado | Situação | Evidência |
| --- | --- | --- |
| F-01 | Resolvido | `lib.rs:262-311` → core `escrow.rs:504-508, 591-606`; `settlement.rs:131-146` (6017 antes do Router, seals reais); reproduzido |
| F-02 | Resolvido | `escrow.rs:408-426`; `lib.rs:123-125`; testes 6028–6030 |
| F-03 | Resolvido | `lib.rs:292-295, 466-479`; entrada criada por `init` (`router/mod.rs:77-87`), sem instrução de close; testes 6033/2012. Resíduos: R-06 e F-04 |
| F-04 | Aberto, alto, bloqueia D4, ampliado por R-01 | escrow, Router e verificador próprios |
| F-05 | Aberto, médio, bloqueia D4 | `lib.rs:409-412` |
| F-06 | Resolvido | `lib.rs:241, 291, 316-331`; 6035; PoC-1 (troca de owner não desvia). Resíduo: delegate ou close authority postos pela própria parte |
| F-07 | Parcial | janela e executor ≠ PDAs resolvidos (`escrow.rs:418-425`; `lib.rs:119-122`); resíduo R-02 |
| F-08 | Resolvido | `address = job.mint` em `lib.rs:439, 459, 487`; testes 6011 |
| F-09 | Aberto (baixo) | inalterado; com R-03 |
| F-10 | Resolvido (itens listados); novos itens em R-04 | erratas conferidas em escrow-state-machine, escrow-program, manifest-schema, README e architecture |
| F-11 | Resolvido (T1–T8); novas lacunas em R-05 | testes existentes |
| F-12 | Resolvido no código, sem teste discriminante | `lib.rs:298` (`job.image_id`; digest dos mesmos 165 bytes) |
| F-13 | Aberto (informativo) | inalterado |
| F-14 | Aberto (informativo) | platform-tools e Criterion sem digest; builds reproduzidos de novo |
| F-15 | Aberto, reclassificado para baixo (R-03) | PoC-8 |

## 4. Checklist 1–15

1. **Troca de Job, prova ou artefato: ACHADO R-03.** Sem achado no resto:
   - PASS ou FAIL de artefato não entregue, com seal válido → 6017 antes da
     CPI;
   - journal de outro Job, spec, harness ou ImageID → `validate_against`
     (core `lib.rs:476-501`) e termos fixos na criação;
   - prova de outro guest: a CPI usa `job.image_id`, e o claim é recalculado
     pelo verificador (`groth_16_verifier lib.rs:104-110`);
   - squatting + deliver: só o executor gravado no Job do squatter entrega;
     nenhum fundo de terceiro em risco. O executor precisa conferir buyer,
     mint e amount (F-05/F-09).
2. **Replay e dupla liquidação: verificado sem achado.** 18/18 combinações
   on-chain (PoC-6); core 3×5 (`escrow.rs:1019`).
3. **Autoridade fraca: ACHADO** (F-04 aberto; R-01, R-06). Sem achado no
   resto:
   - `deliver`: `Signer` + `DelivererMismatch` (`lib.rs:448-453`;
     `escrow.rs:476`); testes 6027/3010;
   - liquidações permissionless, com destino fixo;
   - dono do Router:
     - e-stop testado; o timeout continua devolvendo;
     - `add_verifier` sob outro selector é inútil (6033);
     - a entrada `73c457ba` não pode ser re-apontada nem fechada;
     - a transferência de ownership só move esses poderes;
   - o Router não tem `invoke_signed`;
   - nenhuma instrução administrativa: a IDL tem 6 instruções.
4. **Destino controlável: verificado sem achado.**
   - Seeds `[parte, Tokenkeg, mint]` sob `ATokenGP…` (`lib.rs:321-324`),
     confirmadas pelo programa ATA real nos testes
     (`common/mod.rs:451-468`).
   - Troca de owner → 6010 (PoC-1).
   - Token-2022 → 3007 (PoC-7).
   - ATA inexistente → 3012, e qualquer pessoa pode criá-la (PoC-2).
   - Delegate ou close authority na ATA: resíduo documentado, escolha da
     própria parte.
5. **Timeout e prazos: ACHADO R-02.** Sem achado no resto:
   - janela de criação: testes 6031;
   - executor sem tempo para provar: a prova não depende do estado on-chain,
     então ele prova antes e faz deliver+release atômicos (PoC-3);
   - executor que entrega e nunca prova: o timeout devolve de `Delivered`, no
     máximo cerca de 7 dias depois;
   - release no slot do prazo: aceito, e o timeout é rejeitado nesse slot
     (PoC-3);
   - e-stop × timeout: testado.
6. **Serialização e hash: verificado sem achado.**
   - `RouterSeal` tem o Borsh de `Seal{selector, Proof}` (Router
     `lib.rs:42-46`; `layout.rs:205-221`).
   - 332 bytes = 8+4+64+128+64+32+32 (`lib.rs:342-349`); `85a18d30…` =
     `sha256("global:verify")[..8]` (Python).
   - Digest sobre o mesmo slice de exatamente 165 bytes (`lib.rs:271-272,
     298`).
   - PDAs conferidas por `find_program_address` (`layout.rs:171-203`).
   - Discriminadores de conta: `bc2e1e36…` e `66f7949e…` (Python).
   - 6000–6035 e tags 0–5 por literal; mapeamento exaustivo
     (`lib.rs:684-732`).
7. **`Fail` abortando: verificado sem achado.**
   - `Verdict::Fail` é `Ok` no core (`lib.rs:595-633`).
   - O guest só sai com `env::exit(2|3|4)` em erro de harness ou codec
     (`main.rs:37,41,44`).
   - O verificador só aceita claim com `Halted(0)` (`lib.rs:109-110`), então
     um erro nunca vira veredito liquidável.
8. **Claims: ACHADO R-04.**
   - A frase "verificado por CPI ao Verifier Router em `solana-program-test`
     local" é precisa: os logs mostram a CPI, há negativos dentro do
     verificador, e o estado do Router injetado no genesis já está declarado.
   - Nenhum documento alega ZK on-chain.
9. **Substituição de contas: verificado sem achado.**
   - 2012 para as quatro contas do Router.
   - Entrada estopped → Router 6001.
   - Verificador não executável: endereço fixo e `executable` exigido pelo
     Router (`router/mod.rs:148-152`), por leitura.
   - Ordem e flags da CPI iguais às de `Verify` (`lib.rs:352-357` ×
     `router/mod.rs:125-156`): 4 contas, todas readonly, sem signer.
   - System program → 3008; Job forjado → 3007; vault falso → 2006; Job
     read-only → 2000 (PoC-7).
10. **CPI: verificado sem achado.**
    - Profundidade 3 de 5.
    - Reentrância impossível: o runtime proíbe A→B→A, e o Job não é passado
      à CPI.
    - O `invoke` recebe as 4 contas mais o programa do Router
      (`lib.rs:360-369`).
    - Erro no Router ou no verificador aborta a transação inteira: snapshots
      iguais em 6003/6000/6001.
11. **Limites: verificado sem achado (R-07).** 1.118/1.232 bytes; 135–144 k
    CU, abaixo de 200 k.
12. **Aritmética: verificado sem achado.** Só há comparações, `checked_sub`
    (`escrow.rs:418`) e `checked_mul`; `overflow-checks = true`; prazos
    extremos são rejeitados na criação.
13. **Lacunas de teste: ACHADO R-05**, mais R-04d (IDL).
14. **Supply chain: sem achado novo** (F-14 aberto).
    - Os três `.so` foram reproduzidos byte a byte, offline.
    - `INITIAL_OWNER` de teste embutido no Router: o `.so` não é implantável
      como está (R-01).
    - Staging limpo; fixtures e locks conferidos.
15. **Riscos conhecidos:**
    - F-04: alto, bloqueia D4.
    - F-05: médio, bloqueia D4.
    - F-09: baixo, confirmado.
    - F-13 e F-14: informativos.
    - F-15: baixo (R-03).
    - Router em devnet: `NÃO VALIDADO`, agravado por R-01.
    - ImageID não recertificado: confirmado. A semântica do guest não mudou
      desde o D1c2b (o único diff em `lib.rs` é `pub mod escrow;`, em
      `4d7e18f`), mas um rebuild pode gerar outro ImageID. O D4 deve provar
      com o ELF preservado.
    - Spec v1 trivial: confirmado e declarado.

## 5. Veredito para o D4

**APROVADO COM RESSALVAS.** Condições, todas antes de qualquer deploy ou
transação apresentada como evidência:

- **C1 (R-01):** decisão humana registrada sobre o Router em devnet:
  - upstream verificado: contas, upgrade authorities, owner, entrada
    `73c457ba`, dump do verificador contra `dab6746d…`; ou
  - Router próprio: fork com novos IDs, verificador sob a PDA do Router,
    Router finalizado, constantes e `layout.rs` atualizados, teste de ciclo
    completo e revisão delta separada antes do deploy; ou
  - nenhum Router, sem claim.
- **C2 (F-04):** escrow (e Router ou verificador próprios) com upgrade
  authority final antes das transações de evidência, ou autoridade
  documentada como confiança explícita, sem dizer "sem admin".
- **C3 (F-05):** allowlist do mint no programa (com revisão delta) ou
  checagem e exibição do mint pelo script, com limitação declarada.
- **C4 (R-03/F-09):** `job_id` aleatório por Job, nunca `0x11`; receipts
  novas por Job; fixtures só como fallback rotulado.
- **C5 (R-02):** `create_job`+`fund` na mesma transação, ou checagem da
  janela restante pelo executor.
- **C6:** "verificado on-chain em devnet" só depois de uma transação devnet
  com CPI ao Router bem-sucedida, com link do Explorer. Nunca "ZK on-chain"
  fora disso.
- **C7:** repetir em devnet `MintCannotFreeze` contra o Tokenkeg, tamanhos e
  CU.

Não bloqueiam: R-04 (errata nos docs do D4) e R-05 (incorporar os PoCs à
suíte).

## Artefatos fora do clone (não versionados, persistentes)

- `~/.local/share/vericode-spikes/rd2e/` com `bin/env.sh`, `logs/` e `out/`:
  `rd2e`, `variant`, `router-rebuild` e `idl`.
- PoC `poc/tests-local/tests/rd2e_poc.rs`, SHA-256
  `a8aced162549275fc4a99dd2ebff979c7290d3825ca0b3aef01412c174389232`, com o
  lock `be94760a…`.
- Variante `variant/`: só o `declare_id` difere; `.so` `aee1caea…`.

## Fronteiras

- Sem rede, instalação, Docker, devnet, deploy, keypair novo, commit ou push.
- Keypairs de programa (escrow do `d2c`; Router e verificador do D2d) foram
  apenas copiados para out-dirs `0600`, sem leitura nem exibição.
- `git status --short` e `--ignored` vazios ao final. Perfil padrão ausente.
  `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…` e `~/.docker` `6046f67f…`
  inalterados.
