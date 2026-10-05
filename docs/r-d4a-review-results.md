# R-D4a — revisão delta do D4a (CPI direta ao verificador imutável e mint admitido)

Data: 2026-10-05 · Revisor: Claude Code (Opus 5.5, esforço max), sessão
separada e somente leitura · HEAD revisado: `24364ae` (docs D4a) sobre
`fb4bfba` (código D4a).

> Registro feito por outra sessão, com permissão de escrita, a partir da
> resposta da revisão. As seções 1 a 5 reproduzem as seções 1 a 5 dessa
> resposta, sem mudança de conteúdo; só a formatação (tabelas e listas) foi
> refeita em markdown.

## Resultado

**APROVADO COM RESSALVAS para o D4b**, com as condições CD1 a CD9 (seção 5).
- Nenhum achado crítico ou alto; um médio operacional (RD4A-01): a CLI
  `solana` 2.3.9 pode imprimir uma seed phrase se o deploy falhar no meio.
- Os demais achados são baixos ou informativos e viram condições do D4b.
- Toda a checagem do D4a reproduziu sem divergência.
- Nenhum arquivo do clone foi alterado e não houve escrita em devnet.

## 1. Preflight e comandos executados

Raiz própria: `~/.local/share/vericode-spikes/rd4a/`. O `bin/env.sh` é cópia
do helper do D4a com `D` trocado (`524171eb…`). Execução de 12:31 a 13:01
(-03:00), sempre um build ou teste pesado por vez.

| Checagem | Resultado real |
| --- | --- |
| Git (início e fim) | /home/lucas/src/vericode, main, HEAD 24364ae sobre fb4bfba, cfdd9d7 e 2d76441; status --short e --ignored vazios; diff --check 0 |
| Perfil padrão (início e fim) | ~/.rustup, ~/.cache/solana e ~/.config/solana ausentes; ~/.cargo d9e12578…, ~/.avm 7d29f7f8…, ~/.docker 6046f67f…, iguais |
| Locks | raiz 191802b2…, zkvm f5236689…, guest 1116acef…, anchor/ 19a1db26…, tests-local be94760a… |
| Core lane-a 1.85.0 e lane-b 1.89.0 (test --locked --offline) | 42 passed em cada |
| sbf_build rd4a (target novo) | exit 0, 97 s, 15 warnings de macro; vericode_escrow.so com 395.064 bytes, cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133 |
| prog_tests com o verificador dab6746d… | 57/57: escrow 26, settlement 15, regressions 7, layout 7, fixtures 2. CU: release 126.385, refund_on_fail 121.714. Logs: 21 × THq1q… invoke [2], sendo 17 × 99.541 CU, 2 × 6000 e 2 × 6003 |
| prog_tests com o dump de devnet 34ae6e5c… | 57/57; release 121.885, refund_on_fail 123.214; mesma distribuição de CPIs e erros |
| anchor-0.31.1 idl build -p vericode_escrow | e8ce2c20…, 21.334 bytes, 6 instruções, 37 erros; create_job.mint com endereço 9TE2V…; verifier_program sem endereço (R-04d) |
| sha256sum -c receipts-out.sha256 | 28/28 OK |
| f1_recon.py (RPC, PYTHONDONTWRITEBYTECODE=1) | Router 6JvFf…: executável, authority None, PDA e entrada inexistentes, verify → 3012 (3 casos). Verificador THq1q…: authority None, ELF on-chain 34ae6e5c…; FIB aceito com 99.541 CU; seal adulterado → 6003; ImageID errado → 6000 |
| sim_vectors.py S A A-fail B | 4 × sucesso (99.541 CU); journal A com seal S → 6000 |
| devnet_state.py (03050c3c…) | deployer com 5.000.000.000 lamports (uma transferência de dev2JBjy…, faucet web, slot 507.776.307); buyer e executor com 0; 9TE2V… e GZqbL2Tb… inexistentes e sem assinaturas |
| Análise de bytes (item 4) | vk_bytes.py bf631881…, sbf_diff.py 7ffdba7f…, sbf_align.py 3f9fe476…, sbf_ctx.py 2070cfcf…, sbf_imm32.py 99d8279f…, sbf_stores.py 896572c0… |
| Receipts por caminho independente | journals.py 5ce0c50b…: os 12 campos conferem nas 4 receipts |

**PoCs.** Ficam numa cópia de `anchor/tests-local` em `rd4a/poc/tests-local`,
com caminhos absolutos e o mesmo lock `be94760a…`.
- `tests/rd4a_poc.rs` (`4fa40958…`): 8/8 com o rebuild e 8/8 com os bytes de
  devnet. Os resultados rotulados são iguais nas duas variantes; só a CU
  varia ±1.500 (bump da ATA).
- `tests/rd4a_mutant.rs` (`8cf5c96b…`): 2/2 contra um mutante.
  - Mutante: `rd4a/mutant`, `lib.rs` `d221ac2e…`; `.so` `cfdc26ff…`,
    renomeado `…MUTANT-DO-NOT-DEPLOY.so`.
  - Mutações: sem a constraint `address = GROTH16_VERIFIER_ID` (M1) e sem a
    checagem de selector (M2).
  - A suíte do repositório contra o mutante: escrow 26/26 e regressions 7/7;
    settlement 13/15, com falha exatamente em `verifier_program_is_fixed` e
    `seals_of_another_selector_are_rejected_by_the_escrow`. As duas mutações
    são detectadas.
- Checagem de vermelho (suíte nova contra o D2e `6457aecf…`): escrow 25/26,
  regressions 1/7, settlement 0/15, layout 7/7, fixtures 2/2.
- Programas auxiliares nos PoCs:
  - Router `1b26b017…`, carregado no endereço `6JvFf…`;
  - `noop_aligned.so` do Agave 2.3.9 (`2dab999b…`), que aceita qualquer
    entrada. Ele aceitou diretamente o payload de `verify`.

## 2. Achados

| ID | Sev. | Componente | Local | Resumo | Evidência | Recomendação | Bloqueia D4b |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RD4A-01 | Médio | operação do deploy | CLI solana 2.3.9 (fora do repo); AGENTS.md:17 | program deploy sem --buffer cria buffer efêmero; se a escrita falhar no meio, a CLI imprime a seed phrase de 12 palavras, o que violaria o princípio 9 na saída do agente | string "Recover the intermediate account's ephemeral keypair file with solana-keygen recover and the following -word seed phrase" no binário (rd4a/analysis/solana-cli.strings). O caminho não foi executado aqui | CD4: --buffer com keypair em d4/keys (--silent, stdout descartado, 0600); saída da CLI em log 0600, filtrada | Não, com CD4 |
| RD4A-02 | Baixo | mint admitido | lib.rs:396-400; common/mod.rs:651-663 | a criação do mint é irreversível. Token-2022 no endereço admitido → 3007 para sempre. Freeze authority → 6024 (reversível pela própria freeze authority). 9 decimais (padrão do spl-token create-token) passam calados | PoC poc_i5_admitted_address_variants: 6024, 3007, aceito, 3012 (ausente) | CD3: Tokenkeg e --decimals 6 explícitos, sem freeze; conferir por RPC antes do smoke | Não, com CD3 |
| RD4A-03 | Baixo | endereços públicos | lib.rs:30,63-64; docs/d4a-direct-verifier-results.md:304-310 | program ID, mint e job_ids S/A/B são públicos. Pré-financiar o mint faz create_account falhar (contornável). Pré-financiar o program ID faz a CLI recusar o deploy ("is not an upgradeable program or already in use", string do binário). Squatting de job_id depois do deploy inutiliza a receipt (F-09). Hoje nada foi ocupado | RPC: GZqb… e 9TE2V… inexistentes; PoC poc_i5_prefunded_mint_address: Custom(0), depois transfer+allocate+assign+InitializeMint2 funciona | CD2: checar imediatamente antes; mint primeiro; program ID ocupado → BLOQUEADO | Não, com CD2 |
| RD4A-04 | Info | selector | lib.rs:286-289; docs/decisions.md:1212-1213 | 73c457ba é validação de formato, não vínculo criptográfico; o verificador nunca vê o selector. Não bloqueia seal legítimo | mutante M2: prova válida com deadbeef liquida; prova zerada com o selector certo → 6000 no verificador | manter; trocar "vínculo explícito" por "checagem de formato do seal" | Não |
| RD4A-05 | Info (positivo) | verificador | dump 34ae6e5c… × rebuild dab6746d… | equivalência estrutural, não só funcional. VK, control root, identity control ID, tags, discriminador e program ID iguais. As diferenças vêm do build da platform-tools em macOS: caminhos, 3 TypeId e ordem de funções | seção 4, item 4 | errata: "equivalência estrutural e funcional" em decisions, relatório D4a, escrow-program e agent-control | Não |
| RD4A-06 | Info | confiança | upstream lib.rs:83-90 | sem e-stop com o escrow finalizado. Um bug de soundness permite forjar o veredito de um Job entregue, mas o destino continua fixo (ATA canônica do executor ou do buyer); nada vai a terceiros. Test USDC em devnet | leitura; PoC-1, F-06 | aceito (decisão D4a); operação: parar de criar Jobs se surgir aviso upstream | Não |
| RD4A-07 | Baixo | docs e testes | settlement.rs:214-215; groth16_fixtures.rs:5; fixtures/groth16/README.md:22,35; agent-control.md:86; d4a-direct-verifier-results.md:389-390; .env.example:8 | (a) o comentário cita "program that accepts anything", mas o teste não usa um; (b) fixtures ainda citam o Router como verificador; (c) estimativa de 2,75 SOL, quando o medido é ≈2,01 (rent do RPC, --max-len padrão = tamanho do programa); (d) "deployer sem saldo" desatualizado; (e) RISC0_VERIFIER_ROUTER_PROGRAM_ID obsoleto; (f) a suíte não roda as receipts do D4b, as variantes do mint admitido nem o verificador ausente (só os PoCs da R-D4a) | leitura; RPC; PoCs | errata no D4b; testes no D7 | Não |
| RD4A-08 | Baixo | operação | d4/out/{baseline,redcheck}/vericode_escrow.so (6457aecf…); rd4a/out/mutant | há vários vericode_escrow.so fora do clone; finalizar o binário errado não tem volta | sha256sum | CD1: implantar só cdf6967f…, conferido antes do deploy e pelo dump antes da finalização | Não, com CD1 |

## 3. R-01 a R-07 e C1 a C7

| Item | Situação | Evidência |
| --- | --- | --- |
| R-01 | Resolvido por (b′): não há Router; o verificador fixo está implantado e é imutável em devnet | RPC; suíte com o dump |
| R-02 | Aberto (baixo), limitação documentada | PoC-4 na suíte; create_job+fund atômicos revertem juntos (PoC i12: 6012, Job inexistente) |
| R-03 | Aberto (baixo), mitigado | job_ids aleatórios, distintos e ≠ 0x11; resíduo de squatting em RD4A-03 |
| R-04 | Resolvido nos itens listados | erratas novas em RD4A-04, 05 e 07 |
| R-05 | Resolvido | PoCs 1 a 7 na suíte, iguais ao R-D2e; mutações M1 e M2 detectadas; lacunas novas em RD4A-07(f) |
| R-06 | Não se aplica (não há Router) | substituído por RD4A-06 |
| R-07 | Atualizado | 1.019 B, ou 1.031 B com SetComputeUnitPrice, de 1.232; CU 121–128 k (com deliver); CPI em [2] de 5 |
| C1 | Cumprido: decisão humana (b′) registrada, com divergência do guia | decisions.md D4a |
| C2 (F-04) | Pendente no D4b | CD5 |
| C3 (F-05) | Cumprido: allowlist no programa (6036) | suíte e PoC i5 |
| C4 | Cumprido para as receipts | journals.py, sim_vectors.py; o D4b precisa usá-las |
| C5 | A aplicar no D4b | atomicidade provada (PoC i12) |
| C6 | Pendente | CD7 |
| C7 | Pendente | CD6 |

## 4. Checklist

1. **CPI direta: verificado, sem achado.**
   - Os dados têm 328 B: discriminador, `pi_a`, `pi_b`, `pi_c`,
     `job.image_id` e `sha256(journal)` (`lib.rs:337-348`). É o mesmo formato
     de `verify(Proof, image_id, journal_digest)` (upstream `lib.rs:91-100`) e
     do que `Router.verify` repassa (`router/mod.rs:234-240`). Conta única:
     system program, readonly.
   - `invoke` recebe `[system_program, verifier_program]`.
   - PoC `poc_i1_cpi_payload…`: uma montagem independente chamada direto
     custa 99.541 CU, igual à CPI do escrow em `invoke [2]`.
   - Substituir o verificador pelo Router `6JvFf…` ou pelo noop → 2012, sem
     invocar nenhum dos dois.
   - Verificador ausente, ou conta de dados não executável em `THq1q…` →
     `UnsupportedProgramId`; nada move.
   - Mutante M1 (sem a constraint) → `MissingAccount`. O que fixa o alvo é o
     `program_id` constante na `Instruction`; a constraint é defesa em
     profundidade.
   - Profundidade: 2 de 5 (3 se o escrow for chamado por outro programa).
     Não há reentrância: o verificador não faz CPI.
2. **Claim: verificado, sem achado.**
   - `hash_claim` fixa input 0, `pre = job.image_id`,
     `post = SYSTEM_STATE_ZERO_DIGEST`, assumptions 0 e exit (0,0). Só aceita
     `Halted(0)` incondicional.
   - O `image_id` vem do Job (`lib.rs:292`), nunca do chamador nem do
     journal.
   - PoC `poc_i2…` com as receipts novas:
     - journal e seal de S no Job A → 6014, antes da CPI;
     - A′ no Job A → 6017, antes da CPI;
     - journal A com seal S → 6000 no verificador;
     - seal adulterado → 6003;
     - A′ no Job B → 6014.
   - F-12 continua sem teste discriminante, por construção.
3. **Selector: achado RD4A-04.**
   - A checagem não tem efeito de soundness.
   - Não pode bloquear seal legítimo: o selector é o prefixo do digest de
     parâmetros (`73c457ba541936f0…`), e mudar esses parâmetros já falharia
     contra o VK e o control root fixos do verificador.
4. **Confiança no verificador: verificado.**
   - Por RPC, a authority é `None`:
     - deploy no slot 416.492.112 (23/10/2025 06:14:33 UTC);
     - último deploy (Upgrade) no slot 416.495.876 (06:38:37 UTC);
     - `SetAuthority` final no slot 416.495.880 (06:38:38 UTC);
     - só 3 transações no ProgramData.
   - Ocorrências de bytes, rebuild e dump, na ordem de bytes do fonte:
     - `vk_alpha_g1`, `vk_beta_g2`, `vk_gamma_g2`, `vk_delta_g2` e
       `vk_ic[0..5]`: 1 e 1 cada, com o mesmo layout relativo;
     - `ALLOWED_CONTROL_ROOT`: 1 e 1, em `0x272d8`;
     - `OUTPUT_TAG`, `RECEIPT_CLAIM_TAG` e o program ID: 1 e 1;
     - `SYSTEM_STATE_ZERO_DIGEST` e o discriminador: imediatos `lddw`, 4/4 e
       1/1 nos dois;
     - `BN254_IDENTITY_CONTROL_ID`, invertido: montado por stores imediatos
       idênticos em `.text` (`0x8e60–0x8eb0`) nos dois;
     - `SYSTEM_STATE_TAG`: 0 e 0 (não é usado no fonte);
     - Q: 0 e 0 (`verify_scalar_in_field` é dobrada em compilação;
       `negate_g1` não é chamado).
   - Nada falta no dump que exista no rebuild.
   - `.text`: 18.691 instruções nos dois. Mascarando endereços, 99,49% são
     iguais. Os únicos literais diferentes são 3 `TypeId` (funções `type_id`
     de 5 instruções).
   - Sem e-stop: RD4A-06.
5. **Mint: achados RD4A-02 e RD4A-03.**
   - 6024 vem antes de 6036 (suíte e PoC). Os dois só aparecem depois dos
     CPIs de `init`, e a transação inteira reverte.
   - Token-2022 → 3007.
   - A injeção no genesis é coerente com o plano (Tokenkeg, 6 decimais, sem
     freeze).
   - A constante é igual à pubkey de `pubkeys.txt`.
   - Dependência permanente:
     - perder o keypair do mint antes da criação inviabiliza o programa, mas
       a finalização só vem depois do smoke, que exige o mint;
     - depois da criação, o keypair do mint não tem poder;
     - perder a mint authority encerra a emissão, mas o estoque existente
       continua circulando;
     - inflação pelo deployer não afeta a custódia.
6. **Testes: verificado; lacunas em RD4A-07.**
   - Nenhum enfraquecimento sem motivo:
     - o squatter passou a usar o mint admitido, por causa do 6036;
     - a remoção do teste de e-stop é coerente, porque não há e-stop.
   - `verifier_program_is_fixed` cobre SPL Token, system program e conta
     inexistente.
   - As regressões reproduzem o R-D2e:
     - PoC-1: 6010, e paga quando o owner volta;
     - PoC-2: 3012, e a ATA criada por terceiro destrava;
     - PoC-3: 6021 no slot do prazo, release aceito;
     - PoC-4: documentado como limitação R-02;
     - PoC-5: 838/878/979/1.019 B (−99 B: 3 chaves e 3 índices);
     - PoC-6: 18/18;
     - PoC-7: 3007/2006/3007/3008/2000.
   - A checagem de vermelho reproduziu: 25/26, 1/7 e 0/15.
7. **Interface: verificado, sem achado.**
   - Erros 6000–6036 por literal; tags 0–5; `JobAccount` com 276 bytes; IDL
     `e8ce2c20…`; `declare_id` sem diff desde `c0aba7d`.
   - Renomeações: `Groth16Seal` tem o mesmo Borsh; `VERIFY_DISCRIMINATOR` é
     constante Rust com os mesmos bytes.
   - `SettleWithProof` passou de 10 para 7 contas. Isso quebra clientes do
     D2e, mas não existe nenhum; a CLI do D7 deve sair da IDL do D4a e da
     tabela de constantes.
8. **Receipts: verificado, sem achado.**
   - Frame `job_id ‖ artefato 12 B ‖ image_id`. Exige o ImageID admitido e
     `Composite` → `Groth16`; `disable-dev-mode` impede `Fake`. O journal é
     igual ao do core, e os parâmetros são o default.
   - Shim: 4 execuções com `--pull=never --network=none` e o digest
     `7f173963…`.
   - Lock `ec0dd8d6…`: só acréscimos do core, sem fonte git;
     `ahash-0.7.8.crate` com checksum `891477e0…` conferido.
   - `job_id`s distintos e ≠ `0x11`.
   - A′ = FAIL (7,15) do Job A.
   - Hashes 28/28.
9. **`JournalV1`: verificado, sem achado.**
   - Python recalculou spec `af642b56…` e harness `01124025…`, e conferiu os
     offsets 0/4/36/68/100/132/164 nas 4 receipts.
   - `git diff c0aba7d..HEAD -- crates zkvm` vazio.
   - A regra de mudança (novo program ID) é coerente com o escrow
     finalizado.
10. **Claims:** sem claim proibido. Erratas: RD4A-04, 05 e 07.
11. **Riscos herdados.**
    - R-02 e R-03: seção 3.
    - F-09 (baixo): agravado pelos `job_id`s públicos (RD4A-03).
    - F-13: rent preso de ≈0,0036 SOL por Job em devnet.
    - F-14 (info): sem mudança.
    - ImageID não recertificado e spec trivial: sem mudança.
    - ATA inexistente → 3012; qualquer um pode criar.
    - Margens: 1.031/1.232 B; CU de até 128.484 com `deliver`.
    - Os ELFs são SBPF v0 (`e_flags` 0), como o verificador que o devnet
      executa hoje.
12. **Prontidão para o D4b: sim, com condições.**
    - SOL: 5 SOL. Programa ≈2,0086 (ProgramData 2,0078 + conta 0,0008),
      mint 0,0011, 2 ATAs 0,0030, 4 Jobs 0,0143, taxas < 0,01. Total
      ≈2,03 SOL, com folga de ≈2,9.
    - Buyer e executor estão com 0. O buyer paga o rent do Job, então
      precisa de uma transferência (≈0,05 SOL). Negativos com
      skip-preflight pagam taxa.

## 5. Veredito para o D4b: APROVADO COM RESSALVAS

- **CD1:**
  - implantar só `d4/out/d4a-final/vericode_escrow.so`, com `sha256sum` =
    `cdf6967f…` antes;
  - depois do deploy, `program show` e `program dump` com o mesmo hash, antes
    de qualquer outro passo;
  - nunca usar `d4/out/{baseline,redcheck}` nem `rd4a/out/*`.
- **CD2:** checar por RPC, logo antes das escritas, que `GZqb…`, `9TE2V…` e
  as PDAs de S, A e B não existem.
  - Program ID ou `job_id` ocupado → BLOQUEADO.
  - Mint pré-financiado → transfer + allocate + assign + `InitializeMint2`.
- **CD3:** criar o mint:
  - de preferência antes do deploy;
  - Tokenkeg e decimals 6 explícitos, sem freeze, authority = deployer;
  - conferir owner, 82 bytes, decimals e freeze por `getAccountInfo`;
  - cunhar o estoque do D4b e do D7 de uma vez;
  - keypairs só em `d4/keys` (`0600`).
- **CD4:** deploy com `--buffer` usando um keypair em `d4/keys`; saída da CLI
  só em log `0600`, filtrada; nunca exibir mnemônico.
- **CD5:** ordem: mint → deploy → show e dump → SOL, ATAs e `mint_to` → smoke
  S (`create_job`+`fund` atômicos; `deliver`+`release`) →
  `set-upgrade-authority --final` → show (Authority none) → evidências. Se o
  smoke falhar, não finalizar.
- **CD6:** depois da finalização, negativos primeiro.
  - Job A: A′ → 6017; S → 6014; journal A + seal S → 6000; seal adulterado →
    6003; selector 0 → 6033; depois o release.
  - Job B: release → 6019; A′ → 6014; timeout cedo → 6021; depois
    `refund_on_fail`.
  - Job C: timeout depois do prazo.
  - Negativos com skip-preflight, ou rotulados como simulação.
  - C7: `MintCannotFreeze` primeiro por simulação, depois tamanhos e CU.
- **CD7:** claim só depois de liquidação em devnet: "verificada em devnet por
  CPI ao verificador Groth16 imutável de risc0-solana v3.0.0", com assinatura
  e link. Nunca "Verifier Router". O smoke não é evidência.
- **CD8:** erratas RD4A-04, 05 e 07 nos documentos vivos.
- **CD9:** R-D4a registrado antes de qualquer escrita em devnet.

## Artefatos fora do clone (não versionados, persistentes)

- `~/.local/share/vericode-spikes/rd4a/`:
  - `bin/env.sh` (`524171eb…`);
  - `logs/`, com timeline, testes, PoCs, RPC e análise de bytes;
  - `analysis/*.py`, com os hashes da seção 1;
  - `out/`: `rd4a`, `rd4a-local`, `rd4a-devnet`, `idl`, `mutant` e
    `redcheck`. Os dois últimos contêm só `.so` marcados `DO-NOT-DEPLOY`.
- PoCs em `rd4a/poc/tests-local` (lock `be94760a…`):
  - `tests/rd4a_poc.rs` `4fa40958…`;
  - `tests/rd4a_mutant.rs` `8cf5c96b…`.
- Mutante: `rd4a/mutant` (`lib.rs` `d221ac2e…`; `.so` `cfdc26ff…`), só para
  testes.

## Fronteiras

- Rede: só `api.devnet.solana.com`, somente leitura. Nenhuma assinatura,
  airdrop, deploy ou transação.
- Sem Docker, prova, instalação, `/tmp`, commit ou push.
- `d4/keys`: só `pubkeys.txt` foi lido. O keypair do escrow, que o helper
  copia para `rd4a/out/rd4a`, foi apagado depois do build.
- `git status --short` e `--ignored` vazios no fim.
- `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…` e `~/.docker` `6046f67f…`
  inalterados.
