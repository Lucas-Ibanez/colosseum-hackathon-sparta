# D4 — escrow em devnet com Test USDC e Verifier Router verificado

## Identificação
- Gate: `D4`  ·  Dias da sequência: D4 (devnet) e parte de D7/D8 (fluxo E2E e
  negativos em devnet)  ·  Marcos do guia: M4 (devnet); M5 (devnet) somente
  com CPI ao Router em devnet bem-sucedida
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `R-D2e` (revisão adversarial somente leitura, **APROVADO COM
  RESSALVAS**), registrado no commit `docs: record R-D2e adversarial review`
  sobre `c0aba7d`; relatório `docs/r-d2e-adversarial-review-results.md`

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh**. O gate envolve
  devnet, deploy, autoridade, contas, CPI e claims públicos.
- **Plan Mode obrigatório** (`CLAUDE.md`: programa e integração do Router).
  As decisões de produto já foram tomadas (abaixo). O Plan Mode aprova o
  desenho técnico, o roteiro de transações e o caminho (a) ou (b).
- **Orçamento: 1 dia.** Registrar timestamps por fase. Se estourar, parar com
  evidência parcial e recomendação. O prazo é por volta de 8/10.

## Leitura obrigatória (integral, antes de qualquer ação)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§2, §3, §7, §8, §9, §10, §12)
- `docs/r-d2e-adversarial-review-results.md` (R-01 a R-07, C1 a C7) e
  `docs/r-d2-adversarial-review-results.md` (F-04, F-05, F-09, F-15)
- `docs/decisions.md`: D2a.2, D2c, D2d, D2b.1, D2e, R-D2e e **"Decisões
  humanas para o D4"** (fonte de todas as decisões abaixo)
- `docs/escrow-program.md`, `docs/escrow-state-machine.md`,
  `docs/router-notes.md`, `docs/architecture.md`, `docs/manifest-schema.md`,
  `docs/mvp-agent-operating-guide.md`
- `docs/d2e-router-settlement-results.md` e
  `docs/d2d-groth16-router-spike-results.md` (prover Docker, ambiente zkVM,
  dono de teste do Router)
- `anchor/programs/vericode-escrow/src/lib.rs`, `anchor/tests-local/tests/*.rs`,
  `anchor/tests-local/fixtures/groth16/README.md`
- `zkvm/host/src/main.rs`: frame de entrada do guest, `guest_input` =
  `job_id ‖ artefato 12 B ‖ image_id`
- Fora do clone, somente leitura:
  - `~/.local/share/vericode-spikes/d2d/receipts/src/main.rs` (subcomandos
    `fib-vector` e `compress`);
  - `~/.local/share/vericode-spikes/rd2e/bin/env.sh`;
  - `~/.local/share/vericode-spikes/rd2e/poc/tests-local/tests/rd2e_poc.rs`;
  - `~/.local/share/vericode-spikes/d2c/staging/lane-b/risc0-solana` (commit
    `ee415935`).

## Preflight
- `pwd`; raiz Git; branch; HEAD (esperado: commit `docs: record R-D2e
  adversarial review` sobre `c0aba7d`); `git status --short` (vazio);
  `git diff --check`.
- `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes. Snapshots no
  início e no fim: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker`
  `6046f67f…`. Docker local pode alterar `~/.docker`; registrar a diferença.
- Raiz de trabalho `~/.local/share/vericode-spikes/d4/`, com o helper copiado
  de `rd2e/bin/env.sh` e `D` ajustado. Nunca usar `/tmp`: o WSL reinicia e o
  limpa.
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou
  stash.

## Checagem da tarefa anterior
- Core lane-a `1.85.0` e lane-b `1.89.0` `test --locked --offline` → 42
  passed em cada.
- `cargo-build-sbf -- --locked` → `vericode_escrow.so` com 398.504 bytes,
  `6457aecf471e6d2cb38796fd7dc4442572001b3025fdf8330cde29b93ca9ca96`.
  Router `1b26b017…` e verificador `dab6746d…` no mesmo out-dir.
- `anchor/tests-local` `--no-fail-fast` → escrow 25, settlement 16, layout 6,
  fixtures 2.
- IDL `37a3028a…`: 6 instruções, 36 erros.
- Locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/`
  `19a1db26…`, `tests-local` `be94760a…`.
- `sha256sum -c` dos artefatos D2d (`d2d/artifacts`: ELF `63fac491…`,
  `vericode-guest.bin` `e09ba8cf…`, `recursion_zkr.zip` `744b999f…`) e dos
  vetores (`d2d/vectors`).
- Imagem `risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331`
  presente localmente (`docker image inspect`). Se ausente: BLOQUEADO; não
  fazer pull.
- Se algo divergir: parar e reportar.

## Objetivo
Implantar e exercitar o escrow em Solana devnet com Test USDC:
- um Job PASS pago ao executor e um Job FAIL devolvido ao buyer, ambos com
  receipt Groth16 verificada por CPI ao Verifier Router em devnet;
- um Job devolvido por timeout;
- negativos registrados no Explorer.

Tudo sem admin bypass não declarado e sem claim além do executado.

## Decisões já tomadas (fonte: `docs/decisions.md`, "Decisões humanas para o D4")
1. **D4-0 Rede.**
   - Somente `https://api.devnet.solana.com` (JSON-RPC e `requestAirdrop`),
     começando por uma fase somente leitura.
   - crates.io só para o cliente devnet fora do clone, com lock semeado de
     `anchor/tests-local/Cargo.lock`.
   - Links do Explorer são gerados, não acessados. Nenhum outro destino;
     nenhum pull Docker.
2. **D4-1 Router: (a) upstream, se verificável.** Critérios obrigatórios:
   1. `6JvFfBrvCcWgANKh1Eae9xDq4RC6cfJuBcf71rp2k9Y7` e
      `THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge` existem e são
      executáveis; ProgramData lidos, com as upgrade authorities registradas.
   2. PDA `["router"]` (`4Sh5ofCzLmmCg1zQraXRoPL2oB88bDZJEEbsfjXaauZT`) com o
      dono registrado; entrada `["verifier", 73c457ba]`
      (`4Z7ok78xEh7vYmtHSnzF1Tobtm4sozZfHoQfUBgfG8pi`) com
      `verifier = THq1q…` e `estopped = false`.
   3. `simulateTransaction` (`sigVerify=false`, sem custo) de `Router.verify`:
      vetor FIB oficial e fixtures PASS/FAIL aceitos; seal adulterado e
      ImageID errado rejeitados.

   O `solana program dump` do verificador comparado a `dab6746d…` é
   registrado, mas uma divergência de bytes não reprova sozinha (toolchain).
   - Se 1 a 3 passarem: caminho **(a)**, sem nenhuma mudança de código.
   - **Se falharem: caminho (b), automaticamente, só local:**
     - fork do commit `ee415935` fora do clone (`d4/router`), com novos
       `declare_id` do Router e do verificador e `INITIAL_OWNER` = pubkey do
       deployer;
     - verificador com upgrade authority = PDA do Router; Router com upgrade
       authority final no deploy futuro;
     - troca das quatro constantes do escrow (`VERIFIER_ROUTER_ID`,
       `GROTH16_VERIFIER_ID`, `ROUTER_PDA`, `GROTH16_VERIFIER_ENTRY`) e de
       `tests/layout.rs`;
     - mint admitido (D4-3 (i));
     - teste de ciclo completo em `solana-program-test`: verificador
       upgradeable sob a PDA do Router, `initialize`,
       `add_verifier(73c457ba)`, `release` e `refund_on_fail` por CPI;
     - PoCs 1 a 7 do R-D2e incorporados à suíte (R-05);
     - depois, **PARAR** para a revisão delta separada R-D4a, **sem nenhuma
       escrita em devnet**.
   - (c), sem Router, não é escolhida. Só entra por decisão humana nova, se
     (b) também bloquear.
3. **D4-2 Upgrade authority (F-04/C2).**
   - O escrow é implantado com o keypair existente
     `~/.local/share/vericode-spikes/d2c/keys/vericode_escrow-keypair.json`
     (`GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`), sem mudar código em
     (a).
   - Ordem obrigatória:
     1. deploy e `solana program show`;
     2. **smoke run** real (Job S: create+fund, deliver+release com receipt
        própria), rotulado "smoke pré-finalização, não evidência";
     3. `solana program set-upgrade-authority --final` e `program show` de
        novo;
     4. só então as transações de evidência.
   - Se o smoke falhar: não finalizar; diagnosticar. Mudança de código exige
     parar para a R-D4a.
   - Um bug depois da finalização exige novo program ID, mudança de código e
     R-D4a.
   - Em (a), as autoridades do Router e do verificador upstream são
     registradas como confiança explícita, não controlada pelo projeto
     (inclui o e-stop, R-06).
   - Em (b), o deployer, como dono do Router, mantém e-stop e
     `add_verifier`, documentados como confiança explícita.
4. **D4-3 Mint (F-05/C3).**
   - Em (a): opção (ii). O cliente confere e exibe, antes de cada operação,
     mint, `decimals = 6`, `freeze_authority = None` e os termos do Job
     (spec, harness, ImageID, executor, prazo). A limitação fica declarada; o
     programa não muda.
   - Em (b): opção (i). Constante do mint admitido = pubkey do Test USDC
     (keypair em `d4/keys`), erro novo **no fim** (6036), coberta pela R-D4a.
5. **D4-4 Jobs e receipts (C4/R-03).**
   - `job_id` aleatório de 32 bytes por Job (CSPRNG do SO), nunca `0x11`.
   - Receipts novas por Job, provadas a partir do **ELF D1c2b preservado**
     (`d2d/artifacts/vericode-guest.bin`, ImageID `4da06f90…fb1a`), sem
     reconstruir o guest.
   - Harness `d4/receipts` (cópia de `d2d/receipts`) com o subcomando
     `prove <job_id> <entrada> <saída> <out>`, que reproduz o frame do host,
     mais `compress`.
   - Prover local (`RISC0_PROVER=local`), sem dev mode; receipt verificada
     localmente contra o ImageID admitido e nunca `Fake`.
   - Groth16 pela imagem Docker local por digest, com `--pull=never` e um
     cenário por vez (pico de cerca de 6,25 GiB).
   - As fixtures `0x11` são só fallback rotulado.
6. **D4-5 Janela (C5/R-02).**
   - `create_job` e `fund` na mesma transação; a prova é gerada antes.
   - `deliver` e `release` na mesma transação, quando couberem em 1.232
     bytes (demonstrado no Job S).
   - O executor confere os slots restantes antes de entregar.
   - Mudar `fund` no core fica para gate próprio, depois do MVP.
7. **D4-6 Chaves e SOL.**
   - Keypairs efêmeros de devnet novos em `d4/keys` (`0600`), criados com
     `solana-keygen new --silent --no-bip39-passphrase`, só com pubkeys
     publicadas, dentro dos papéis do princípio 9:
     - deployer/payer, buyer, executor e mint do Test USDC;
     - em (b), também os keypairs de programa do Router e do verificador
       (papel de deployer).
   - Nenhum keypair "terceiro": negativos assinados pelo buyer ou pelo
     deployer.
   - SOL por `solana airdrop`, dentro dos limites. O deploy exige cerca de
     2,8 SOL de rent, com pico próximo de 2× pelo buffer. Se for
     insuficiente: **BLOQUEADO**, informando ao humano a pubkey do deployer
     e o valor, para obter SOL pelo faucet web. Nunca pedir chave.

## Escopo autorizado
- Fora do clone, em `~/.local/share/vericode-spikes/d4/`:
  - `keys/` (`0600`);
  - harness de receipts;
  - cliente devnet em Rust;
  - em (b), o fork do Router;
  - logs e saídas.
- No repositório, caminho (a), **somente documentação**:
  - criar `docs/d4-devnet-results.md`;
  - atualizar `decisions.md`, `evidence.md`, `agent-control.md`,
    `project-context.md`;
  - `router-notes.md`: status de devnet;
  - `escrow-program.md`: Program ID de devnet e autoridade finalizada;
  - `README.md`: status de devnet com links, somente o comprovado;
  - criar `docs/handoffs/d4-to-d5.md`.
- No repositório, caminho (b), além da documentação:
  - `anchor/programs/vericode-escrow/src/lib.rs`: quatro constantes, mint
    admitido e erro 6036;
  - `anchor/tests-local/tests/*.rs`: layout, ciclo do Router, mint admitido e
    PoCs 1 a 7.
  - Sem dependência nova; locks inalterados.
  - Criar `docs/d4a-own-router-results.md` e
    `docs/handoffs/d4a-to-r-d4a.md`.

## Fora de escopo / proibido
- Mainnet, dinheiro real, seed phrase; keypair dentro do clone, impresso ou
  versionado.
- Alterar `JournalV1`, wire format, guest, core ou locks. Em (a), também o
  programa.
- Deploy de `.so` com `INITIAL_OWNER` de teste. Qualquer escrita em devnet no
  caminho (b) antes da R-D4a.
- Rede além do D4-0; Docker além do D4-4.
- Transações de evidência antes da finalização da upgrade authority.
- Usar a receipt de outro Job como do Job exibido.
- "ZK on-chain" ou "verificado on-chain" sem transação devnet de CPI
  bem-sucedida e link do Explorer.
- Push.

## Implementação esperada (fases, com parada entre elas)
1. **F0:** preflight e checagem.
2. **F1, somente leitura:** reconhecimento D4-1 (critérios 1 a 3 e o dump) e
   registro. Se (a) for aprovado, segue para F2. Senão, segue o caminho (b) e
   **para** ao fim dele.
3. **F2, local:**
   - keypairs;
   - `job_id`s de S, A e B;
   - receipts novas, verificadas localmente:
     - S `(7,14)` PASS;
     - A `(7,14)` PASS;
     - A' `(7,15)` FAIL, para o negativo de artefato não entregue;
     - B `(7,15)` FAIL;
   - cliente devnet compilado;
   - suíte local verde.
4. **F3, devnet.** Os negativos vêm antes dos positivos e usam
   `skipPreflight` para ficar no Explorer.
   1. SOL; mint Test USDC (6 decimais, sem freeze authority); ATAs
      idempotentes; saldo do buyer.
   2. Deploy do escrow e `program show`.
   3. **Job S (smoke):** create+fund → deliver+release na mesma transação.
   4. Finalização e `program show`.
   5. **Job A (PASS, prazo de cerca de 9.000 slots):** create+fund →
      deliver `(7,14)` →
      - journal A com seal de S → Router/verificador 6000;
      - `refund_on_fail` com a receipt A' → 6017;
      - selector `deadbeef` → 6033;
      - destino não canônico (conta de token do executor criada com
        `create_account_with_seed`, sem keypair novo) → 6035;
      - depois, release → saldos;
      - repetição → 6007.
   6. **Job B (FAIL):** create+fund → deliver `(7,15)` →
      - release com o journal FAIL → 6019;
      - deliver de novo → 6026;
      - depois, `refund_on_fail` → saldos;
      - repetição → 6008.
   7. **Job C (timeout, prazo de cerca de 1.500 slots):** create+fund →
      - `refund_on_timeout` antes do prazo → 6021;
      - espera;
      - `refund_on_timeout` → saldos.
   8. C7: `SetAuthority(FreezeAccount)` no mint → `MintCannotFreeze` no
      Tokenkeg de devnet.
   9. CU e tamanho de cada transação.
5. **F4:** documentos, claims e handoff.

## Testes obrigatórios
- Suíte local completa (core A/B, escrow, settlement, layout, fixtures) antes
  de qualquer escrita em devnet. Em (b), mais os testes novos.
- Em devnet, todos os casos da F3, com:
  - saldos de buyer, executor e vault antes e depois;
  - status do Job decodificado;
  - CU e tamanho;
  - código de erro observado em cada negativo.

## Evidências exigidas
- `docs/d4-devnet-results.md` com:
  - timestamps por fase;
  - critérios D4-1 com saída real (contas, autoridades, simulações, dump);
  - program ID; `program show` antes e depois da finalização;
  - mint, ATAs e signers (só pubkeys);
  - `job_id`s;
  - hashes das receipts, journals e seals novos;
  - assinaturas e links `https://explorer.solana.com/tx/<sig>?cluster=devnet`;
  - saldos, CU e tamanhos;
  - smoke separado e rotulado.
- Entradas D4 em `decisions.md` e `evidence.md`; `router-notes.md` com o
  status de devnet.

## Critério de pronto
- Caminho (a):
  - escrow finalizado;
  - depósito, release PASS, refund FAIL e refund por timeout no Explorer;
  - negativos registrados.
- Claims coerentes com o executado: "verificado por CPI ao Verifier Router
  implantado em Solana devnet (endereços e autoridades …)", somente com as
  transações listadas.
- Caminho (b): suíte local verde com o ciclo do Router, nenhuma escrita em
  devnet e handoff da R-D4a salvo.
- `git diff --check` exit 0; busca de segredos limpa; keypairs só fora do
  clone; perfil padrão inalterado.

## Condições de parada
- Decisão não coberta → `AGUARDANDO_AUTORIZAÇÃO`.
- Router upstream não verificável → caminho (b) local e parada para R-D4a.
- Qualquer mudança de código → parada para R-D4a antes de escrever em devnet.
- Smoke falhou → não finalizar; parar com diagnóstico.
- SOL insuficiente, faucet ou RPC bloqueando → `BLOQUEADO`, com diagnóstico
  e a pubkey do deployer.
- Imagem Docker ausente, divergência de baseline, hash ou ImageID → parar.
- Orçamento de 1 dia estourado → parar com evidência parcial.

## Commit
- Commits locais autorizados, com identidade via `git -c`. Push proibido.
  - Caminho (a), ao atingir o critério: `docs: record devnet escrow (D4)`.
  - Caminho (b), na parada: `anchor: pin own devnet Router and admitted mint
    (D4a)` e `docs: record own Router and admitted mint (D4a)`.
  - Parada por bloqueio: `docs: record partial devnet attempt (D4)`, só com
    evidência real.

## Relatório final
1. arquivos modificados;
2. comandos e saídas reais, com links do Explorer;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`:
   - em (a): D5, com CLI reproduzível no repositório para o fluxo devnet,
     README com versões, hashes, links e limitações (M6/M7), roteiro da demo
     e R-05; worker e UI só se sobrar tempo;
   - em (b): R-D4a, revisão delta separada e somente leitura (Opus 5.5,
     max).
