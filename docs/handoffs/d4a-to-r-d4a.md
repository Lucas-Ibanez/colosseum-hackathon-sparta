# R-D4a — revisão delta adversarial do D4a (CPI direta ao verificador imutável e mint admitido)

## Identificação
- Gate: `R-D4a`  ·  Dia da sequência: revisão do D4 (guia §11)  ·  Marcos do
  guia: pré-condição de M4/M5 em devnet
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D4a`:
  - commit do código `fb4bfba` (`anchor: verify proofs through the immutable
    Groth16 verifier and admit Test USDC (D4a)`);
  - commit de docs `docs: record direct verifier, admitted mint and frozen
    JournalV1 (D4a)`;
  - relatório `docs/d4a-direct-verifier-results.md`.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço max**, em sessão nova e
  separada. É a auditoria adversarial de uma mudança crítica: CPI de
  verificação, modelo de confiança e mint.
- **Somente leitura** (`CLAUDE.md`: "Revisões de segurança são somente
  leitura e não devem editar arquivos").
  - Plan Mode opcional: a revisão não edita o repositório.
  - O registro do resultado é feito depois, por outra sessão com permissão
    de escrita, como no R-D2 e no R-D2e.

## Leitura obrigatória (integral, antes de qualquer ação)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/d4a-direct-verifier-results.md` (integral) e
  `docs/r-d2e-adversarial-review-results.md` (R-01 a R-07, C1 a C7)
- `docs/decisions.md`: "Decisões humanas para o D4", "Prazo de 11/10,
  congelamento do JournalV1 v1 e nomes de gate" e **"D4a: Router upstream
  reprovado em devnet; CPI direta ao verificador Groth16 imutável e mint
  admitido"**. Esta registra a divergência do guia §1/§8.
- `docs/context/guia-mvp-agentes-de-codigo.md` (§1, §2, §7, §8, §9, §11)
- `docs/escrow-program.md`, `docs/manifest-schema.md` (v1 congelado),
  `docs/router-notes.md`, `docs/escrow-state-machine.md`,
  `docs/architecture.md`, `README.md`
- Código:
  - `anchor/programs/vericode-escrow/src/lib.rs`;
  - `anchor/tests-local/tests/{common/mod.rs,escrow.rs,settlement.rs,layout.rs,regressions.rs,groth16_fixtures.rs}`;
  - `git show fb4bfba`.
- Fonte pinado, fora do clone e somente leitura:
  `~/.local/share/vericode-spikes/d2c/staging/lane-b/risc0-solana`
  (`ee415935`):
  - `solana-verifier/programs/groth_16_verifier/src/lib.rs` (`verify`,
    `VerifyProof`, `hash_claim`);
  - `solana-verifier/programs/verifier_router/src/router/mod.rs` (o que o
    Router repassaria).
- Fora do clone, somente leitura, em `~/.local/share/vericode-spikes/d4/`:
  - `bin/env.sh`, `bin/docker-shim/docker`, `bin/f1_recon.py`,
    `bin/sim_vectors.py`, `bin/prove_all.sh`;
  - `receipts/{Cargo.toml,Cargo.lock,src/main.rs}`;
  - `logs/` (`f1-recon.log`, `tests-d4a-*.log`, `tests-redcheck.log`,
    `prove-*.log`, `compress-*.log`, `sim-new-seals.log`, `docker-shim.log`,
    `timeline.txt`, `receipts-out.sha256`);
  - `keys/pubkeys.txt` (só pubkeys; **não abrir** os `.json`).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3`.
  - Esperado no topo: o commit `docs: record direct verifier…` sobre
    `fb4bfba`, `cfdd9d7` e `2d76441`.
- `git status --short` vazio e `--ignored` vazio; `git diff --check` 0.
- Perfil padrão:
  - `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes;
  - snapshots por `cd <dir> && find . -printf '%p %s %T@\n' | sort |
    sha256sum`: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker`
    `6046f67f…`, no início e no fim.
- Raiz própria da revisão: `~/.local/share/vericode-spikes/rd4a/`, com o
  helper copiado de `d4/bin/env.sh` e `D` trocado. Nunca usar `/tmp`.
- **RAM de 7,6 GiB: nunca rodar dois builds ou testes pesados em
  paralelo.** No D4a, dois em paralelo terminaram com exit 137.

## Checagem da tarefa anterior (reexecutar, offline)
- Core lane-a `1.85.0` e lane-b `1.89.0` `test --locked --offline` → 42
  passed em cada.
- `sbf_build rd4a` (target novo) → `vericode_escrow.so` com 395.064 bytes,
  `cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133`.
- `prog_tests` `--no-fail-fast` → escrow 26, settlement 15, regressions 7,
  layout 7, fixtures 2, duas vezes:
  - com `groth_16_verifier.so` `dab6746d…` (copiar de `d4/out/d4a`);
  - com o dump de devnet `34ae6e5c…` (`d4/out/devnet-dump`).
- `anchor-0.31.1 idl build -p vericode_escrow` (`avm/bin` no `PATH`) →
  `e8ce2c20…`, 6 instruções, 37 erros.
- Locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/`
  `19a1db26…`, `tests-local` `be94760a…`.
- `cd d4/receipts-out && sha256sum -c ../logs/receipts-out.sha256`.
- Rede, só se necessário e somente leitura em
  `https://api.devnet.solana.com`:
  - `python3 d4/bin/f1_recon.py` (reconfirma o estado do Router e do
    verificador);
  - `python3 d4/bin/sim_vectors.py d4/receipts-out/{S,A,A-fail,B}`.
- Se divergir: parar e reportar.

## Objetivo
Decidir, com evidência, se o D4a pode seguir para escritas em devnet (D4b).
O veredito é **APROVADO**, **APROVADO COM RESSALVAS** ou **REPROVADO**, com
condições.

## Decisões já tomadas (não reabrir; avaliar só a implementação e os riscos)
- Caminho (b′), por decisão humana registrada no D4a: CPI direta ao
  verificador imutável `THq1q…`, sem Router, com a divergência do guia
  §1/§8 explícita.
- Mint admitido (opção (i)) e `JournalV1` v1 congelado.
- D4-2 a D4-6, que valem para o D4b:
  - finalização depois do smoke;
  - `job_id` aleatório;
  - `create_job`+`fund` atômicos;
  - keypairs de devnet.

## Checklist adversarial (no mínimo)
1. **CPI direta.**
   - Dados de 328 bytes e conta única readonly, iguais a `VerifyProof` e ao
     que o Router repassaria.
   - `invoke` com o programa e o system program.
   - Verificador fixo por `address`.
   - Nenhum caminho para outro programa, entrada ou selector.
   - Profundidade de CPI; reentrância.
2. **Claim verificado.** `hash_claim(job.image_id, SHA-256 dos 165 bytes)`.
   - O `image_id` vem do Job, nunca do journal nem do chamador.
   - `Halted(0)` exigido pelo verificador.
   - Um journal de outro Job, spec, harness, artefato ou ImageID falha
     antes da CPI ou nela.
3. **Selector.** Avaliar se a checagem de `73c457ba` (6033) tem efeito ou é
   inócua sem Router, e se pode bloquear seals legítimos.
4. **Confiança.**
   - Confirmar por RPC a upgrade authority `None` do `THq1q…`.
   - Avaliar o risco "sem e-stop" frente ao escrow finalizado.
   - Avaliar se a equivalência funcional (simulações e suíte com o dump)
     basta, dado que os bytes diferem do rebuild.
5. **Mint admitido.**
   - Ordem das constraints (6024 antes de 6036).
   - Token-2022.
   - Injeção do mint no genesis dos testes.
   - Valor da constante = pubkey de `d4/keys/pubkeys.txt`.
   - Griefing de pré-financiamento do endereço do mint antes do D4b.
   - Autoridade de mint em poder do deployer.
6. **Testes.**
   - Nenhum teste enfraquecido sem motivo.
   - Remoção do teste de e-stop.
   - `verifier_program_is_fixed`.
   - As regressões reproduzem cada PoC do R-D2e (PoC-4 como limitação R-02;
     PoC-5 com 838/878/979/1.019 bytes).
   - Checagem de vermelho (25/26, 1/7, 0/15).
   - Lacunas novas.
7. **Interface estável.** Erros 6000–6036 por literal; tags de
   `EscrowStatus`; `JobAccount` 276; IDL; renomeações (`Groth16Seal`,
   `VERIFY_DISCRIMINATOR`) e efeito em clientes futuros.
8. **Receipts novas.**
   - Código do harness: frame, ImageID admitido, `Composite`/`Groth16`,
     sem `Fake`, journal igual ao core.
   - Shim do Docker (`--pull=never --network=none`, digest).
   - Ampliação do lock (`ahash 0.7.8` copiado com checksum).
   - `job_id`s ≠ `0x11` e distintos.
   - A′ como FAIL do Job A.
   - Hashes.
9. **`JournalV1` congelado.** O documento bate com o código byte a byte
   (offsets, domínios, valores de spec e harness); nenhum byte mudou; a
   regra de mudança é coerente com o escrow finalizado.
10. **Claims.**
    - Nenhum "ZK on-chain", "verificado on-chain em devnet" ou "Verifier
      Router" como caminho atual em docs, README ou código.
    - Erratas pendentes.
11. **Riscos herdados.** R-02, R-03, F-09, F-13, F-14; ImageID não
    recertificado; spec trivial; ATA inexistente; margens de tamanho e CU
    (121–126 k).
12. **Prontidão para o D4b.**
    - SOL necessário (cerca de 2,75 SOL de rent do escrow).
    - Ordem deploy → `program show` → smoke (Job S) → finalização →
      evidência.
    - Negativos planejados para os Jobs A/B/C e C7.
    - O que precisa ser condição explícita.

## Escopo autorizado
- Leitura do repositório e dos diretórios fora do clone listados acima.
- Builds e testes em targets e out-dirs novos em
  `~/.local/share/vericode-spikes/rd4a/`.
- PoCs novos só numa cópia de `anchor/tests-local` fora do clone, com o
  mesmo lock `be94760a…`.
- RPC somente leitura em devnet (`getAccountInfo`,
  `getSignaturesForAddress`, `getTransaction`, `simulateTransaction` com
  `sigVerify=false`, `solana program dump`).

## Fora de escopo / proibido
- Editar, criar ou apagar qualquer arquivo do clone; commit; push.
- Qualquer escrita em devnet: airdrop, deploy, transação assinada ou criação
  de mint/ATA.
- Abrir, imprimir ou copiar o conteúdo dos keypairs em `d4/keys`. Só
  `pubkeys.txt` pode ser lido.
- Docker: provar ou comprimir de novo. As receipts são conferidas por hash,
  pelos logs e por simulação.
- Instalar algo no perfil padrão; usar `/tmp`; rede além do devnet somente
  leitura.

## Testes obrigatórios
- Reexecução da checagem acima, com saída real.
- PoCs adversariais próprios para os itens 1, 3, 5 e 6, no mínimo, com o
  resultado observado (código de erro e snapshot).

## Evidências exigidas
- Resposta com: preflight, comandos e saídas reais, tabela de achados
  (ID, severidade, componente, local, resumo, evidência, bloqueia D4b),
  situação de R-01 a R-07 e C1 a C7, checklist 1–12 e veredito com
  condições.
- A sessão de registro grava `docs/r-d4a-review-results.md`, as entradas em
  `decisions.md` e `evidence.md`, atualiza `agent-control.md` e
  `project-context.md` e salva `docs/handoffs/r-d4a-to-d4b.md`.

## Critério de pronto
- Veredito justificado por evidência executada.
- `git status --short` e `--ignored` vazios no início e no fim.
- Perfil padrão igual aos snapshots.

## Condições de parada
- Baseline, hash ou lock divergente → parar e reportar.
- Achado crítico ou alto → REPROVADO, com a correção recomendada; nada de
  corrigir na revisão.
- Necessidade de escrita em devnet ou no clone → parar; está fora do escopo.

## Commit
- Não autorizado nesta revisão. Push sempre proibido.

## Informação para o D4b (não executar aqui)
- **Keypairs de devnet** em `d4/keys`:
  - deployer `617ogw9Tbem67ZwWDokEAkLV6JMq5avreijFMPrG75nd`;
  - buyer `EZgGUg4JhEAhzMPd4jBuKWFXdDNpFATvCjkj7mLAdxU6`;
  - executor `EdB25bVhdj6rm5b7FbVHe5A3zx2hAhPpK4YAzrLaDs6U`;
  - mint `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F`.
- **SOL:** o deployer precisa de cerca de 3 SOL (rent de 2,75 SOL do escrow
  mais taxas e contas). O humano pode financiá-lo pelo faucet web antes do
  D4b; nunca pedir chave.
- **Receipts prontas** em `d4/receipts-out/{S,A,A-fail,B}`:
  - S `fe6d25fe…` (7,14) PASS;
  - A `3f0dd1c8…` (7,14) PASS e A′ (7,15) FAIL;
  - B `5a25ae48…` (7,15) FAIL.
  - Job C (timeout) não precisa de receipt.
- **Program ID** `GZqbL2Tb…`, com o keypair existente do D2c. O `.so` a
  implantar é `cdf6967f…`.

## Relatório final
1. arquivos lidos e comandos; 2. saídas reais; 3. achados e severidade;
4. invariantes; 5. decisões pendentes; 6. riscos; 7. confirmação de
fronteiras; 8. veredito; 9. prompt da próxima fase (D4b ou correção),
segundo `docs/handoff-protocol.md`, para a sessão de registro salvar.
