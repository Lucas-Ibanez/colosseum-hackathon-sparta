# D4b — deploy do escrow em devnet, smoke, finalização e liquidações PASS/FAIL/timeout

## Identificação
- Gate: `D4b`  ·  Dia da sequência: D4 (e parte de D7/D8 em devnet)  ·
  Marcos do guia: M4 e M5 (em devnet, via verificador imutável)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `R-D4a` (revisão delta somente leitura, **APROVADO COM
  RESSALVAS**), registrado no commit `docs: record R-D4a delta review` sobre
  `24364ae`; relatório `docs/r-d4a-review-results.md`. Código do D4a em
  `fb4bfba`.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh**. O gate envolve
  devnet, autoridade e contas, com três ações irreversíveis: criação do
  mint, deploy e finalização.
- **Plan Mode obrigatório antes da primeira escrita em devnet.** O plano
  mostra:
  - os comandos exatos de E2 a E6;
  - o hash do `.so`;
  - o uso de `--buffer`;
  - a ordem de CD5.
- Prazo do MVP: 11/10. Registrar timestamps por fase.

## Leitura obrigatória (integral, antes de agir)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/r-d4a-review-results.md` (RD4A-01 a 08 e condições CD1 a CD9)
- `docs/d4a-direct-verifier-results.md`
- `docs/decisions.md`:
  - "Decisões humanas para o D4" (D4-0 a D4-6);
  - "Prazo de 11/10, congelamento do JournalV1 v1 e nomes de gate";
  - "D4a…";
  - "R-D4a…", incluindo as escolhas do agente para o D4b.
- `docs/escrow-program.md`, `docs/router-notes.md`, `docs/manifest-schema.md`,
  `docs/context/guia-mvp-agentes-de-codigo.md` (§2, §7, §8, §9, §12)
- `anchor/programs/vericode-escrow/src/lib.rs`;
  `anchor/tests-local/tests/common/mod.rs` (builders de instrução)
- Fora do clone, somente leitura:
  - `~/.local/share/vericode-spikes/rd4a/poc/tests-local/tests/rd4a_poc.rs`.
    O teste `poc_i2_d4b_receipts_bind_only_to_their_jobs` é o ensaio local
    exato do D4b.
  - `~/.local/share/vericode-spikes/rd4a/analysis/devnet_state.py`;
  - `~/.local/share/vericode-spikes/d4/bin/env.sh`.

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3`. O topo esperado é
  `docs: record R-D4a delta review` sobre `24364ae`. **Se não for, parar:**
  a condição CD9 exige o R-D4a registrado antes de qualquer escrita em
  devnet.
- `git status --short` e `--ignored` vazios; `git diff --check` 0.
- Perfil padrão:
  - `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes;
  - snapshots com `cd <dir> && find . -printf '%p %s %T@\n' | sort |
    sha256sum`: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker`
    `6046f67f…`, no início e no fim.
- Raiz nova `~/.local/share/vericode-spikes/d4b/`, com o helper copiado de
  `d4/bin/env.sh` e `D` trocado.
- Nunca usar `/tmp`. RAM de 7,6 GiB: um build pesado por vez.
- Sem reset, checkout destrutivo, clean ou stash.

## Checagem da tarefa anterior
- `sha256sum ~/.local/share/vericode-spikes/d4/out/d4a-final/vericode_escrow.so`
  → `cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133`
  (395.064 bytes).
- `cd d4/receipts-out && sha256sum -c ../logs/receipts-out.sha256` → 28/28
  OK.
- Locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/`
  `19a1db26…`, `tests-local` `be94760a…`.
- `solana --version` e `spl-token --version` das homes do Perfil A
  (Agave 2.3.9).
- `python3 rd4a/analysis/devnet_state.py` (somente leitura):
  - deployer `617ogw9T…` com cerca de 5 SOL;
  - `GZqbL2Tb…` e `9TE2VPFm…` inexistentes.
- Se divergir: parar e reportar.

## Objetivo
Implantar o escrow D4a em devnet, validá-lo com o smoke, finalizar a upgrade
authority e produzir as liquidações PASS, FAIL e timeout e os negativos, com
links do Explorer, sob as condições CD1 a CD9 da R-D4a.

## Decisões já tomadas
- Caminho (b′): CPI direta ao verificador imutável `THq1q…`, sem Router —
  D4a.
- Mint admitido `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F` — D4a,
  ratificado pelo humano.
- `JournalV1` v1 congelado.
- D4-2: finalizar depois do smoke. Program ID `GZqbL2Tb…`, com o keypair
  `~/.local/share/vericode-spikes/d2c/keys/vericode_escrow-keypair.json`.
- D4-4: receipts prontas por Job em `d4/receipts-out/{S,A,A-fail,B}`.
  Fixtures `0x11` só como fallback rotulado.
- D4-5: `create_job`+`fund` na mesma transação; `deliver`+`release` juntos
  quando couber.
- D4-6: keypairs de devnet em `d4/keys`, só pubkeys publicadas:
  - deployer `617ogw9Tbem67ZwWDokEAkLV6JMq5avreijFMPrG75nd`;
  - buyer `EZgGUg4JhEAhzMPd4jBuKWFXdDNpFATvCjkj7mLAdxU6`;
  - executor `EdB25bVhdj6rm5b7FbVHe5A3zx2hAhPpK4YAzrLaDs6U`;
  - mint `9TE2VPFm…`.

  O keypair do buffer do deploy é parte do papel de deployer (CD4).
- R-D4a: condições CD1 a CD9.
- **Valores escolhidos pelo agente (entrada R-D4a em `decisions.md`):**
  - `amount` de cada Job = 1 Test USDC (1.000.000 unidades, 6 decimais);
  - estoque cunhado de uma vez para o buyer = 1.000.000 Test USDC, para o
    D4b e o D7;
  - prazo do Job S ≈ slot + 3.000;
  - prazo dos Jobs A e B ≈ slot + 9.000 (cerca de 1 h);
  - prazo do Job C = slot + 1.500 (mínimo da janela);
  - SOL para o buyer ≈ 0,05 e, se o executor pagar taxa, ≈ 0,01 para ele.

## Escopo autorizado
- **Rede:** só `https://api.devnet.solana.com` (JSON-RPC, `simulateTransaction`,
  `sendTransaction`).
  - Links do Explorer são gerados, não acessados.
  - crates.io só para o cliente devnet fora do clone, com lock semeado de
    `anchor/tests-local/Cargo.lock`.
  - Sem airdrop. SOL extra só pelo faucet web, feito pelo humano.
- **Escritas em devnet:**
  - criação do mint e das ATAs; `mint_to`;
  - transferências de SOL a partir do deployer;
  - deploy com `--buffer`; `set-upgrade-authority --final`;
  - instruções do escrow; o C7.
- **Keypair novo:** só o do buffer do deploy, em `d4/keys`, `0600`, criado
  com `solana-keygen new --silent --no-bip39-passphrase`.
- **Repositório, só documentação:**
  - criar `docs/d4b-devnet-results.md`;
  - atualizar `docs/decisions.md`, `docs/evidence.md`,
    `docs/agent-control.md`, `docs/project-context.md`;
  - `docs/escrow-program.md` (program ID em devnet e authority `none`);
  - `docs/router-notes.md` (status de devnet);
  - `README.md` (claim CD7, só com o comprovado);
  - criar `docs/handoffs/d4b-to-d7.md`.

## Fora de escopo / proibido
- Mudar programa, core, `zkvm/`, testes, `.env.example`, locks ou
  `JournalV1`. As pendências RD4A-07 (a), (e) e (f) ficam para o D7.
- Docker ou prova nova. Se um `job_id` estiver ocupado:
  `AGUARDANDO_AUTORIZAÇÃO`.
- Implantar qualquer `.so` que não seja o `cdf6967f…`. Nunca usar
  `d4/out/{baseline,redcheck}` nem `rd4a/out/*`.
- Mainnet; exibir seed phrase, keypair ou conteúdo de `d4/keys/*.json`.
- **Saída bruta da CLI no terminal.**
  - Toda saída de `solana`/`spl-token` vai para um log em `d4b/logs/` com
    `umask 077`.
  - Só se mostram linhas filtradas: program ID, signature, authority,
    balance, erros.
  - Se o log contiver "seed phrase" ou "solana-keygen recover": parar, não
    transcrever, registrar o incidente.
- "Verifier Router", "ZK on-chain" ou claim fora do texto de CD7. Push.

## Implementação esperada (ordem CD5)
1. **E0:** `d4b/` com helper e logs `0600`. Cliente devnet fora do clone:
   - `spl-token` da release do Agave para mint e ATAs;
   - cliente Rust para o escrow, reutilizando os builders de
     `common/mod.rs`;
   - para as positivas, sempre `simulateTransaction` antes de
     `sendTransaction`.
2. **E1 (CD2):** pré-checagem por RPC, imediatamente antes das escritas, de
   `GZqb…`, `9TE2V…` e das PDAs `["job", id]` de S, A e B.
   - Program ID ou `job_id` ocupado → `AGUARDANDO_AUTORIZAÇÃO`.
   - Mint pré-financiado → transfer + allocate + assign + `InitializeMint2`.
3. **E2 (CD3):** criar o mint com Tokenkeg explícito, `--decimals 6`, sem
   freeze e authority = deployer. Conferir por `getAccountInfo`: owner
   Tokenkeg, 82 bytes, decimals 6, freeze `None`.
4. **E3 (CD1, CD4):**
   - conferir o `sha256sum` do `.so` imediatamente antes;
   - deploy com `--program-id` (keypair do D2c), `--buffer` (keypair novo) e
     `max-len` padrão;
   - `solana program show`;
   - `solana program dump` com SHA-256 = `cdf6967f…`. Se o dump vier maior
     (zeros finais), comparar os primeiros 395.064 bytes, confirmar que o
     resto é zero e registrar.
5. **E4:**
   - SOL ao buyer (≈0,05) e, se necessário, ao executor;
   - ATAs do buyer e do executor;
   - `mint_to` de 1.000.000 Test USDC para o buyer.
6. **E5 (smoke, não é evidência):** Job S.
   - `create_job`+`fund` numa transação; `deliver`+`release` (receipt S)
     noutra.
   - Conferir `Released` e saldos.
7. **E6:** `solana program set-upgrade-authority GZqb… --final`;
   `program show` com authority `none`. **Se o E5 falhou, não finalizar:
   parar.**
8. **E7 (CD6):** Jobs A, B e C com os negativos primeiro; depois o C7.
9. **E8:** relatório, claims, handoff `d4b-to-d7.md`.

## Testes obrigatórios (em devnet, com assinatura e link do Explorer)
- **Job A** (receipt A):
  - A′ → 6017;
  - journal+seal de S → 6014;
  - journal A + seal S → 6000 do verificador;
  - seal adulterado (`pi_c[10] ^= 1`) → 6003, ou o código observado no
    ensaio local `poc_i2`;
  - selector `00000000` → 6033;
  - depois, `release` com A → `Released`.
- **Job B** (receipt B):
  - `release` → 6019;
  - `refund_on_fail` com A′ → 6014;
  - `refund_on_timeout` antes do prazo → 6021;
  - depois, `refund_on_fail` com B → `RefundedOnFail`.
- **Job C:** `create_job`+`fund`; espera; `refund_on_timeout` depois do
  prazo → `RefundedOnTimeout`.
- Em toda rejeição, estado do Job, vault e saldos iguais antes e depois.
- Negativos enviados com skip-preflight. Se algum não puder ser enviado,
  registrar a simulação, rotulada como tal.
- **C7:** `SetAuthority(FreezeAccount)` no mint admitido → `MintCannotFreeze`,
  primeiro por simulação e depois enviado com skip-preflight.
- Tamanho e CU de cada transação, tirados dos logs.

## Evidências exigidas
- `docs/d4b-devnet-results.md` com:
  - timeline;
  - comandos e saídas reais filtradas;
  - `program show` antes e depois da finalização e o hash do dump;
  - dados do mint e pubkeys;
  - assinaturas e links `https://explorer.solana.com/tx/<sig>?cluster=devnet`;
  - saldos, CU e tamanhos;
  - o smoke separado e rotulado "não evidência".
- Atualizar `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`,
  `docs/project-context.md` (D4 concluído na tabela gate → dia) e o README.
  O claim do README é exatamente o de CD7: "verificada em devnet por CPI ao
  verificador Groth16 imutável de risc0-solana v3.0.0", com links.

## Critério de pronto
- Escrow `GZqb…` implantado com o `cdf6967f…` e authority `none`.
- Jobs S, A, B e C nos estados esperados, com todos os negativos
  registrados.
- Nenhum segredo exibido ou versionado.
- `git diff --check` 0; diff revisado; busca de segredos limpa; perfil
  padrão igual.

## Condições de parada
- Hash do `.so` ou do dump diferente de `cdf6967f…` → `BLOQUEADO`.
- Program ID ou `job_id` ocupado → `AGUARDANDO_AUTORIZAÇÃO`.
- Smoke com falha → não finalizar; diagnosticar. Mudança de código exige
  revisão nova.
- Mnemônico na saída da CLI → parar, não transcrever, registrar o incidente.
- SOL insuficiente → informar ao humano a pubkey do deployer e o valor.
- A CLI recusa o deploy por feature de SBPF → `BLOQUEADO`; não recompilar.

## Commit
- Commits locais autorizados pelo humano ao enviar este prompt, com
  identidade via `git -c`. Push proibido.
  - Ao atingir o critério: `docs: record devnet deploy, finalization and
    settlements (D4b)`.
  - Parada com evidência parcial: `docs: record partial devnet attempt
    (D4b)`, só com evidência real.

## Relatório final
1. arquivos modificados;
2. transações e saídas reais, com links;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`: **D7**, com
   - CLI de ponta a ponta no repositório para o fluxo devnet, saindo da IDL
     do D4a;
   - README com versões, hashes, links e limitações (M6/M7);
   - roteiro da demo (conteúdo do D9);
   - RD4A-07 (a), (e) e (f).

   Worker e telas (D10–D12) só se houver tempo.
