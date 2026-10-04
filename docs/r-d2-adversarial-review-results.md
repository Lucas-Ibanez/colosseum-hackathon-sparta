# R-D2 — revisão adversarial do D2b, D2c e D2c.1

Data: 2026-10-04 · Revisor: Claude Code (Opus 5.5, esforço max), sessão
separada e somente leitura · HEAD revisado: `42b4f58` (D2c.1) sobre `9a18f71`.

> Registro feito por outra sessão, com permissão de escrita, a partir da
> resposta da revisão (seções 5a–5c). As seções 1 a 4 abaixo reproduzem o
> conteúdo da revisão; só as tabelas foram convertidas para markdown.

## Resultado

**REPROVADO para iniciar o D2e como especificado.** O código on-chain atual
não tem achado alto explorável. O bloqueio vem de três pontos:

1. a política D2b aceita FAIL de qualquer artefato (F-01);
2. os termos de verificação são escolhidos pelo buyer (F-02);
3. o selector do Router não está fixado no plano (F-03).

Escopo: D2b `58838ae`, D2c `a10f026`/`ec980e9`, D2c.1 `42b4f58`; diff
`git diff 4d7e18f^..HEAD -- crates anchor` (15 arquivos). Os PoCs rodaram numa
cópia de `anchor/tests-local` fora do clone (`tests/rd2_poc.rs`, SHA-256
`67060e22…`), não versionada e volátil (`/tmp`).

## 1. Preflight e comandos executados

| Checagem | Resultado |
| --- | --- |
| `pwd` / raiz Git / branch | `/home/lucas/src/vericode` / mesma / `main` |
| HEAD | `42b4f58` (D2c.1) sobre `9a18f71` ✔ |
| `git status --short` (início/fim) | vazio / vazio; `--ignored` vazio |
| `git diff --check` | exit 0 |
| Locks | raiz `191802b2…`, host `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…`: todos inalterados |
| Perfil padrão | `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes |

Todos os comandos rodaram offline, com target e out-dir no scratchpad da
sessão (fora do clone):

| Comando | Saída real |
| --- | --- |
| core, lane-a, `cargo +1.85.0 test --locked --offline` | exit 0; 36 passed |
| core, lane-b, `cargo +1.89.0 test --locked --offline` | exit 0; 36 passed |
| `cargo-build-sbf … -- --locked` (Perfil A) | exit 0, 97 s; 13 warnings de macro do Anchor; `vericode_escrow.so` com 298.224 bytes, SHA-256 `d66ac76bc8ef66488facc6473b7679eac2fa5e605fba409a6cd61acce20baaae` (idêntico ao do D2c.1) |
| `anchor/tests-local`, `cargo +1.89.0 test --locked` | exit 0; escrow 12 passed, fixtures 2 passed |
| Fixtures por caminho independente (Python `hashlib`) | SHA-256 de seal e journal iguais aos registrados; digest == SHA-256(journal); campos decodificados (schema 1, job `0x11`, spec `af642b56…`, harness `01124025…`, artefato `d5aa9223…`/`343ad778…`, image `4da06f90…`, verdict 0/1); bytes iguais aos de `d2d/vectors`; `sha256sum -c` 17/17 OK |
| Locks de `anchor/` | 264/264 e 705/705 pacotes de registry com checksum; nenhuma fonte git |
| PoCs em cópia de `tests-local` fora do clone (paths absolutos, mesmo lock `be94760a…`; `tests/rd2_poc.rs` SHA-256 `67060e22…`) | exit 0; 10 passed |

Fonte pinado consultado (somente leitura):

- `anchor-spl 0.31.1` `token.rs:463-514`: owner de Mint/TokenAccount e id de
  Token = `spl_token::ID`.
- `anchor-syn 0.31.1` `constraints.rs:574-629`: o `init` do vault usa o campo
  `token_program`.
- `spl-token 7.0.0` `processor.rs:497-500, 743`.
- `solana-program-test 2.3.9` `programs.rs:20`: o programa SPL Token executado
  nos testes é `spl_token-3.5.0.so`.
- `verifier_router` em `ee415935`: `router/mod.rs:66-112, 123-156, 190,
  219-240` e `estop/mod.rs:84-96`.

| PoC | Resultado observado |
| --- | --- |
| 1 (core) | 4 artefatos FAIL arbitrários × slots {0, prazo}: `refund_on_fail` aceito em 8/8 casos nos quais o release do PASS também é aceito. As fixtures reais pass/fail liquidam o mesmo Job `0x11` |
| 2 | mint Token-2022 → 3007; programa Token-2022 → 3008; Job não é criado |
| 3 | `refund_on_timeout` com conta mint estranha → `0x3` do SPL Token (`MintMismatch`), não um erro do VeriCode |
| 4 | refund aceito numa conta do buyer com delegate antigo; o delegate dispara o refund e drena 1.000.000 |
| 5 | `Clock.slot == 1000` → 6021; slot 1001 → refund; Job com `deadline_slot = 0` criado, financiado e reembolsado sem janela de release |
| 6 | 7 unidades doadas continuam no vault depois do refund |
| 7 | freeze → `0x10` `MintCannotFreeze`; close → `0xb` (saldo); `set_authority(CloseAccount)` → `0x4`; fund com o vault como origem → `0x4`; Job continua `Created` |
| 8 | códigos literais 6000/6004/6009/6010/6011/6017/6021/6022/6023/6024 conferem com a documentação; `INIT_SPACE = 276`; ida e volta `EscrowStatus`↔`EscrowState` 5/5 |
| 9 | squatter cria o Job `0x11` com mint próprio, amount 1 e prazo `u64::MAX`; a criação legítima falha com `0x0` (`AccountAlreadyInUse`) |
| 10 | vault falso em fund/refund → 2006 `ConstraintSeeds`; Job de outro owner → 3007 |

## 2. Achados

| ID | Severidade | Componente | Local | Resumo | Evidência | Bloqueia D2e |
| --- | --- | --- | --- | --- | --- | --- |
| F-01 | Crítico (latente) | core D2b + plano D2e | `crates/vericode-core/src/escrow.rs:435-461, 497-508`; `zkvm/methods/guest/src/main.rs:17-47` | `refund_on_fail` aceita o FAIL de qualquer artefato, e qualquer pessoa consegue gerá-lo | PoC-1 | Sim |
| F-02 | Alto (latente) | programa + core | `anchor/programs/vericode-escrow/src/lib.rs:36-58`; `escrow.rs:272-308` | o buyer escolhe `image_id`/`spec_hash`/`harness_hash`, ou seja, o guest que decide | leitura | Sim |
| F-03 | Alto (latente) | plano D2e / Router | `docs/handoffs/d2d-to-d2e.md:98-100`; `router/mod.rs:66-112, 123-156` | selector não fixado: o dono do Router pode registrar um verificador que aceita tudo | leitura do fonte pinado | Sim (condição do D2e) |
| F-04 | Alto | deploy | `docs/escrow-program.md:107-109` | upgrade authority do escrow e de Router/verificador próprios é bypass administrativo | leitura | Não (bloqueia D4) |
| F-05 | Médio | programa | `lib.rs:156-161` | qualquer mint clássico sem freeze authority é aceito; não há allowlist | leitura; PoC-9 | Não (bloqueia D4) |
| F-06 | Baixo | programa | `lib.rs:116-126, 203-204` | quem dispara o refund escolhe o destino; delegate e close authority são aceitos | PoC-4 | Não (ressalva) |
| F-07 | Baixo | programa + core | `lib.rs:36-58`; `escrow.rs:270-271` | termos sem limite: prazo passado ou `u64::MAX`, executor igual ao PDA | PoC-5, PoC-9 | Não (ressalva) |
| F-08 | Baixo | programa | `lib.rs:124, 200` | conta mint do refund não é amarrada a `job.mint`; só o SPL Token barra | PoC-3 | Não (ressalva) |
| F-09 | Baixo | programa | `lib.rs:162-168` | squatting de `job_id` | PoC-9 | Não |
| F-10 | Baixo | docs | ver detalhe | claims imprecisos | grep + PoC-1/7 | Não (corrigir junto com F-01) |
| F-11 | Baixo | testes | ver detalhe | lacunas de teste | PoC-2, 3, 5, 8, 9, 10 | Não |
| F-12 | Info | plano D2e | `main.rs:28-35` | `journal.image_id` é autodeclarado pelo provador | leitura | Não (condição já no plano) |
| F-13 | Info | programa | n/a | doação ao vault e rent ficam presos | PoC-6 | Não |
| F-14 | Info | supply chain | n/a | platform-tools/Criterion sem digest; feature `token_2022` | comandos | Não |
| F-15 | Info | contrato de dados | `crates/vericode-core/src/lib.rs:386-394` | journal sem program ID/cluster: replay entre implantações | leitura | Não |

### F-01 — Crítico (latente): FAIL arbitrário reembolsa o buyer em qualquer slot

**Descrição:**
- O guest recebe `job_id`, artefato e `image_id` como entradas do provador
  (`main.rs:19, 29, 31`) e publica um journal "vinculado" a qualquer
  `job_id`.
- `bound_verdict` compara o `artifact_hash` do journal com ele mesmo
  (`escrow.rs:497-508`).
- `refund_on_fail` não exige nada que ligue o artefato à entrega do executor
  (`escrow.rs:435-461`).

**Cenário:**
- O buyer financia e o executor entrega `(7,14)` off-chain.
- O buyer, ou qualquer terceiro, prova `(7,15)` (ou qualquer registro errado)
  para o mesmo `job_id`/ImageID e chama `refund_on_fail` antes do release.
- Em todo slot ≤ prazo, os dois destinos são válidos e vence quem entra
  primeiro. Um griefer consegue cancelar todo Job financiado.
- O D2e especificado leva isso on-chain: `refund_on_fail` em qualquer slot e
  permissionless (`d2d-to-d2e.md:79-83, 124-125`).
- Não é explorável no HEAD, porque o programa não tem `refund_on_fail`
  (`lib.rs:9-12`).

**Contradiz:**
- `docs/escrow-state-machine.md:74`. A própria matriz do D2b aceita 2 releases
  e 3 refunds por FAIL a partir do mesmo `Funded`
  (`docs/d2b-escrow-guide-alignment-results.md:163-171`).
- A afirmação permitida do guia §2 ("artefato vinculado ao job").

**Recomendação (decisão humana em Plan Mode):**
- **(A), recomendado:** compromisso de entrega assinado pelo executor.
  - `deliver(artifact_hash)` em `Funded`, até o prazo, uma única vez →
    `Delivered { artifact_hash }`.
  - `release` e `refund_on_fail` exigem `journal.artifact_hash ==
    compromisso` (reativa o código 6017).
  - `refund_on_timeout` aceita `Funded` e `Delivered`.
  - Diverge da letra do guia §5 ("registra apenas na liquidação") e do D2a.1
    ("o executor não pré-registra"): registrar a divergência e o motivo.
- **(B):** `refund_on_fail` só com assinatura do executor (ele admite o
  FAIL); sem isso, só timeout.
- **(C):** remover `refund_on_fail`. Viola guia §9/M4; rejeitar.

### F-02 — Alto (latente): o buyer escolhe o guest que decide o veredito

**Descrição:** `create_job` aceita `image_id`, `spec_hash` e `harness_hash`
livres; `JobV1::new` só valida identidades (`escrow.rs:283-296`). O D2e vai
verificar a prova com `job.image_id`.

**Cenário:**
- O buyer registra o ImageID de um guest próprio que copia as entradas e
  sempre publica FAIL com spec e harness canônicos. Todo artefato entregue
  gera um FAIL válido, e o buyer é reembolsado.
- A correção (A) de F-01 sozinha não resolve: o buyer prova o artefato
  entregue com esse guest.
- Um `spec_hash` não canônico torna qualquer release impossível; só resta o
  timeout.

**Recomendação (decidir junto com F-01):**
- **(i), recomendado:** constantes da v1 no programa (ImageID
  `4da06f90…fb1a`, spec `af642b56…b778`, harness `01124025…6b50`);
  `create_job` rejeita outros valores. Coerente com "harness fixo para a v1"
  (guia §3). Exige atualizar as constantes quando o ImageID for
  recertificado.
- **(ii):** registrar que o executor e a CLI precisam conferir os termos
  on-chain, e que o programa não protege o executor contra termos escolhidos
  pelo buyer.

### F-03 — Alto (latente): selector do Router não fixado no plano do D2e

**Descrição:**
- `add_verifier` aceita qualquer programa executável cuja upgrade authority
  seja o PDA do Router, sob um selector novo (`router/mod.rs:66-112, 190`).
- `verify` encaminha pela `seal.selector` para qualquer entrada registrada
  (`router/mod.rs:123-156, 219-240`).
- O plano do D2e fixa só o Program ID do Router.

**Cenário:**
- O dono do Router (no D2d, chave de teste; em devnet, RISC Zero ou chave do
  projeto) implanta um "verificador" que sempre retorna Ok, passa a upgrade
  authority para o PDA do Router e o registra sob `deadbeef`.
- Um seal com esse selector libera qualquer Job. É bypass administrativo,
  contra o princípio 7 e a invariante 7.

**Recomendação (condição do D2e):**
- Constante `SELECTOR = 73c457ba`; exigir `seal.selector == SELECTOR` e que
  `verifier_entry` seja o PDA `["verifier", SELECTOR]` do Router fixado.
  Opcionalmente, fixar também o Program ID do verificador.
- Documentar a confiança residual:
  - o e-stop do dono (`estop/mod.rs:84-96`) bloqueia `release` e
    `refund_on_fail`, mas o timeout continua devolvendo ao buyer;
  - a upgrade authority do Router.

### F-04 — Alto: upgrade authority

Confirmado e reclassificado de "potencial" para alto em devnet: quem detém a
upgrade authority troca o código e saca os vaults.

**Recomendação:** no gate de deploy, implantar imutável depois dos testes, ou
nomear a autoridade como confiança explícita e não alegar "sem admin". Vale
também para Router e verificador próprios. Não afeta o D2e local.

### F-05 — Médio: mint sem allowlist

Qualquer mint clássico sem freeze authority é aceito, inclusive mint
controlado pelo buyer e wSOL (`lib.rs:156-161`). No PoC-9, o squatter usa o
próprio mint.

- **Cenário:** um Job "de 1.000.000" num token sem valor engana um executor
  ou uma UI que só mostra o valor.
- **Recomendação:** allowlist do Test USDC por cluster, ou CLI que confira e
  exiba o mint, antes do D4.

### F-06 — Baixo: destino escolhido por quem dispara o refund

`refund_on_timeout` é permissionless e aceita qualquer token account do buyer
com o mint do Job; o core só compara owner e mint. O mesmo vale para contas
wSOL com close authority, e o D2e repetiria o padrão para o executor.

- **Cenário (PoC-4):** um delegate antigo do buyer dispara o refund para a
  conta onde ainda tem allowance e a drena.
- **Recomendação:** nas três liquidações, exigir a ATA canônica da parte
  (`address = get_associated_token_address(parte, job.mint)`), ou delegate e
  close_authority vazios.

### F-07 — Baixo: termos sem limite

- `deadline_slot = 0` é aceito: o Job é financiado e reembolsado de imediato,
  sem janela de release (PoC-5).
- `u64::MAX` torna o timeout impossível (PoC-9). Depois da correção de F-01,
  isso prende fundos até o executor agir.
- O executor pode ser o PDA do próprio Job.

**Recomendação:**
- `Clock.slot < deadline_slot ≤ Clock.slot + JANELA_MAX`;
- executor diferente do PDA do Job e do vault;
- a CLI do executor confere a janela restante contra o tempo de prova Groth16
  (cerca de 90 s por cenário no D2d).

### F-08 — Baixo: conta mint do refund não amarrada

Em `RefundOnTimeout`, o core recebe `buyer_token.mint` (`lib.rs:124`); a conta
mint (`lib.rs:200`) só é barrada pelo `transfer_checked` do SPL Token (PoC-3:
`0x3`).

**Recomendação:** `#[account(address = job.mint)]` em `Fund`,
`RefundOnTimeout` e nas novas liquidações.

### F-09 — Baixo: squatting de `job_id`

Confirmado como baixo: não há perda de fundos (PoC-9). Mas o `job_id` é o
único vínculo entre journal e Job.

**Recomendação:**
- CLI e worker conferem todos os termos on-chain antes de trabalhar.
- Opcional: `job_id = SHA-256(domínio ‖ program_id ‖ buyer ‖ nonce)`,
  conferido em `create_job`. Resolve também F-15.
- Não trocar as seeds por `["job", buyer, job_id]` sem vincular o buyer no
  journal: a unicidade global do `job_id` deixaria de existir e uma prova
  serviria para dois Jobs.

### F-10 — Baixo: claims imprecisos

| Local | Problema |
| --- | --- |
| `docs/escrow-state-machine.md:74` | "Em nenhum slot dois destinos diferentes competem": falso (PoC-1) |
| `docs/escrow-program.md:100-103`, `docs/d2c1-mint-freeze-and-fixtures-results.md:14-16`, `docs/decisions.md:767-768` | atribuem `MintCannotFreeze` ao "SPL Token 7.0.0". O 7.0.0 é o crate cliente; o programa executado é `spl_token-3.5.0.so` (`programs.rs:20`). O comportamento foi confirmado nele (PoC-7: `0x10`); repetir em devnet contra o Tokenkeg implantado |
| `docs/escrow-program.md:26-27` | "constraints validam somente estrutura": desde o D2c.1 existe uma pré-condição de custódia (freeze authority) |
| `docs/manifest-schema.md:98` | exige um `artifact_hash` "aceito/registrado para a entrega" antes da liquidação. Contradiz D2a.1/D2b, mas é exatamente a regra que falta (F-01) |
| `README.md:12`, `docs/architecture.md:26-28` | "artefato vinculado ao Job": sem compromisso de entrega, não existe "o" artefato do Job |
| `docs/escrow-program.md:38-39` | "só pode corresponder a um Job" vale por implantação (F-15) |

README e `router-notes` não alegam verificação ZK on-chain, e os status estão
corretos (`docs/router-notes.md:10-23`; tabela por rede).

### F-11 — Baixo: lacunas de teste

| Lacuna | Situação |
| --- | --- |
| T1: rejeição de Token-2022 alegada sem teste | PoC-2 confirma o comportamento |
| T2: `tests/escrow.rs:477-498` não confere `Clock.slot == DEADLINE` depois do warp | PoC-5 confirma |
| T3: `tests/escrow.rs:392` só usa `is_err()` | PoC-9 mostra o erro real `0x0` |
| T4: códigos esperados derivados do próprio enum (`tests/escrow.rs:71-73`); uma reordenação passaria despercebida | PoC-8 usa literais |
| T5: sem teste de mint estranho, vault falso ou Job forjado | PoC-3 e PoC-10 |
| T6: ramo `UnsupportedAccountVersion` (`lib.rs:230-233`) sem teste | n/a |
| T7: no core, `transitions_do_not_change_the_job_terms` (`escrow.rs:1008-1027`) é trivial (`&self` sobre tipo `Copy`) | o oráculo da matriz (`escrow.rs:870-947`) codifica a política de F-01 |
| T8: sem teste de ida e volta de `EscrowStatus` nem de `INIT_SPACE` | PoC-8 |

### F-12 a F-15 — Informativos

- **F-12:** `journal.image_id` é cópia da entrada do provador
  (`main.rs:28-35`). Só o claim da receipt vincula a imagem. O D2e deve
  calcular `verify(seal, job.image_id, SHA-256(bytes))` exatamente sobre os
  165 bytes decodificados.
- **F-13:** tokens doados ficam presos (PoC-6) e o rent nunca é recuperado.
  Aceitável no MVP; qualquer close/sweep futuro é superfície nova.
- **F-14:** platform-tools e Criterion continuam sem digest oficial; locks com
  100% de checksum e sem git; o lock semeado do `counter` se mantém consistente
  com `--locked`. A feature `token_2022` traz `spl-token-2022 6.0.0` e
  `solana-zk-sdk 2.3.13` para o grafo do programa; em runtime, só o
  `InitializeAccount3` é endereçado ao programa clássico. O `.so` foi
  reproduzido byte a byte pela terceira vez.
- **F-15:** `JournalV1` não contém program ID nem cluster. Hoje é irrelevante,
  porque F-01 e F-02 já permitem forjar. Depois da correção (A), quem copiar
  compromisso e prova de outra implantação com o mesmo `job_id` recebe sem
  conhecer o artefato. Mitigação: a derivação de `job_id` de F-09.

## 3. Checklist 1–15

1. **Troca de Job/prova: ACHADO** (F-01, F-02, F-15). Sem achado no resto:
   - journal de outro job, spec, harness ou ImageID é rejeitado
     (`escrow.rs:497-508`; testes em `escrow.rs:748-770`);
   - `job_id` é único pela PDA (`lib.rs:162-168`; PoC-9);
   - as fixtures só valem para o Job `0x11`; em outro Job falham com
     `JobIdMismatch`. Mas qualquer pessoa gera vetores novos (F-01).
2. **Replay e dupla liquidação: sem achado.**
   - core: 3 estados terminais × 4 operações (`escrow.rs:848-868`);
   - programa: refund→refund e refund→fund (`tests/escrow.rs:553-586`);
   - combinações com `release`/`refund_on_fail` on-chain não se aplicam no
     HEAD e são obrigatórias no D2e.
3. **Autoridade fraca: ACHADO** (F-03, F-04, F-06). Sem achado no resto:
   - `fund` exige o buyer como signer (core; teste com erro 6009);
   - timeout permissionless com destino fixo;
   - nenhuma instrução administrativa (IDL com 3 instruções).
4. **Destino controlável: ACHADO baixo** (F-06). Sem achado no resto:
   - outro dono → 6010; outro mint → 6011 (`tests/escrow.rs:523-551`);
   - conta de outro programa: `Account<TokenAccount>` exige o owner do SPL
     Token clássico (`token.rs:463-466`);
   - Token-2022 → 3007/3008 (PoC-2).
5. **Timeout injusto: sem achado na semântica.**
   - `<=` → `DeadlineNotReached` (`escrow.rs:474`); `>` → `DeadlinePassed`
     (`escrow.rs:410`);
   - fonte `Clock::get()` (`lib.rs:122`);
   - PoC-5 mede o slot 1000 → 6021 e o slot 1001 → refund aceito.
   - A injustiça econômica real está em F-01 e F-07.
6. **Serialização e hash: sem achado.**
   - `from_core`/`to_core` bijetivos, com match exaustivo (`lib.rs:260-298`;
     PoC-8);
   - `version` conferido (`lib.rs:230-233`);
   - `INIT_SPACE = 276`;
   - códigos 6000–6024 estáveis.
   - Regra futura: novas variantes de status e novos erros entram sempre no
     fim.
7. **`Fail` abortando: sem achado.**
   - `Verdict::Fail` é `Ok` no core (`crates/vericode-core/src/lib.rs:615-619`);
   - o guest só usa `env::exit(2..4)` para erro de harness
     (`main.rs:37, 41, 44`).
8. **Claims: ACHADO baixo** (F-10).
9. **Substituição de contas: sem achado.**
   - 3007 e 3008 (PoC-2); 2006 e 3007 (PoC-10);
   - discriminador conferido por `Account<JobAccount>`;
   - bump canônico gravado no init (`lib.rs:62-63`) e reutilizado
     (`lib.rs:186, 191, 198, 201`).
   - Observação: F-08.
10. **CPI de token: sem achado.**
    - seeds `[b"job", job_id, bump]` (`lib.rs:128-141`);
    - `transfer_checked` com o mint e os decimals da conta Mint;
    - valor = `job.amount` (`lib.rs:104, 142`).
    - Doação: F-13.
11. **Regra D2c.1: sem achado** (ver F-10, linha do `MintCannotFreeze`).
    - A constraint está no único caminho de criação do vault
      (`lib.rs:158-178`).
    - A premissa vale no binário executado (3.5.0) e no fonte 7.0.0.
    - `close` exige saldo zero e owner/close authority; `set_authority` exige
      o PDA (PoC-7); o programa nunca assina outra CPI.
12. **Aritmética: sem achado de overflow.** Só há comparações;
    `overflow-checks = true` (`anchor/Cargo.toml:6-9`). Prazos extremos: F-07.
13. **Lacunas de teste: ACHADO baixo** (F-11).
14. **Supply chain: sem achado novo**; o risco já conhecido continua (F-14).
    Fixtures coerentes por verificação independente.
15. **Riscos conhecidos:**
    - upgrade authority → alto, bloqueia D4 (F-04);
    - squatting → baixo, confirmado, com a ressalva sobre seeds (F-09);
    - rent → informativo (F-13);
    - Router em devnet → `NÃO VALIDADO`, agravado por F-03.

## 4. Veredito

**REPROVADO** para iniciar o D2e como especificado em
`docs/handoffs/d2d-to-d2e.md`.

Condições para liberar o D2e:
- F-01 e F-02 decididos pelo humano e implementados num gate de correção
  (D2b.1), com testes que invertem o PoC-1.
- O prompt do D2e reescrito com F-03 (selector `73c457ba` fixado) como
  requisito.

Ressalvas a incorporar no D2e (não bloqueiam): F-06, F-07, F-08, F-11 e F-12.

Antes do D4: F-04, F-05 e confirmação do Router em devnet.

## Fronteiras

- Sem rede, instalação, Docker, devnet, deploy, keypair novo, commit ou push.
- Único uso de keypair: cópia do arquivo do programa para o out-dir temporário
  do build, sem leitura nem exibição.
- `git status --short` e `--ignored` vazios ao final; `~/.rustup`,
  `~/.cache/solana` e `~/.config/solana` ausentes.
