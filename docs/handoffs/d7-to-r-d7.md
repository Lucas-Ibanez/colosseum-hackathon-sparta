# R-D7 — revisão adversarial final do MVP (CLI, prover, chaves, claims e reprodutibilidade)

## Identificação
- Gate: `R-D7`  ·  Dia da sequência: revisão final antes do D9 (guia §11)  ·
  Marcos do guia: M6 e M7
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D7`:
  - `a7c6e8a` (`prover: add reproducible prover and admitted guest binary
    (D7)`);
  - `deececa` (`cli: add devnet end-to-end client (D7)`);
  - `0ec421b` (`anchor: add D4b fixtures and RD4A-07 tests (D7)`);
  - commit de docs `docs: record D7 CLI, README and demo script`, no topo;
  - relatório `docs/d7-cli-results.md`.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço max**, em sessão nova e
  separada. É a auditoria adversarial final do marco: cliente de devnet,
  chaves, prover e claims públicos.
- **Somente leitura** (`CLAUDE.md`: "Revisões de segurança são somente
  leitura e não devem editar arquivos").
  - Plan Mode opcional: a revisão não edita o repositório.
  - O registro do resultado é feito depois, por outra sessão com permissão
    de escrita, como no R-D2, R-D2e e R-D4a.

## Leitura obrigatória (integral, antes de qualquer ação)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/d7-cli-results.md` (integral, inclusive "Desvios e incidentes" e
  "Riscos abertos") e `docs/d4b-devnet-results.md`
- `docs/decisions.md`: "D4a…", "R-D4a…", "D4b…" e **"D7: CLI e prover no
  repositório; fluxo de ponta a ponta em devnet"**
- `docs/r-d4a-review-results.md` (achados RD4A-01 a RD4A-08, condições CD1 a
  CD9)
- `docs/context/guia-mvp-agentes-de-codigo.md` (§2, §3, §7, §9, §10, §11, §12,
  §14)
- `README.md`, `docs/demo-script.md`, `cli/README.md`, `prover/README.md`,
  `prover/artifacts/README.md`, `docs/architecture.md`,
  `docs/escrow-program.md`, `docs/manifest-schema.md`, `docs/router-notes.md`,
  `docs/mvp-agent-operating-guide.md` (níveis de evidência e claims)
- Código:
  - `cli/` (`Cargo.toml`, `Cargo.lock`, `src/{main,lib,escrow,rpc,tx,keys,receipt}.rs`,
    `tests/instructions.rs`);
  - `prover/` (`Cargo.toml`, `Cargo.lock`, `src/{lib,main}.rs`,
    `docker-shim/docker`);
  - `anchor/tests-local/tests/{common/mod.rs,d4b_receipts.rs,escrow.rs,settlement.rs,groth16_fixtures.rs}`
    e `fixtures/groth16/d4b/`;
  - `anchor/programs/vericode-escrow/src/lib.rs` (não mudou desde o D4a);
  - `git show a7c6e8a deececa 0ec421b` e o commit de docs.
- Fora do clone, somente leitura:
  - `~/.local/share/vericode-spikes/d7/`:
    - `bin/` (`env.sh`, `cli.sh`, `suite.sh`, `prove_p.sh`,
      `d4b_fixtures.py`);
    - `logs/` (`timeline.log`, `cli-tx.jsonl`, `b*-*.log`, `w*-*.log`,
      `prove-P.log`, `compress-P.log`, `verify-P.log`, `mem-P.log`,
      `receipts-P.sha256`);
    - `jobs/{P,T}.json`; `receipts/P`;
  - `~/.local/share/vericode-spikes/d4/receipts-out` e
    `d4/logs/receipts-out.sha256`;
  - `~/.local/share/vericode-spikes/d4/keys/pubkeys.txt` (só pubkeys;
    **nunca abrir** os `.json`).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -5`.
  - Esperado no topo: o commit de docs do D7 sobre `0ec421b`, `deececa`,
    `a7c6e8a` e `599837d`.
- `git status --short` e `--ignored` vazios; `git diff --check` 0.
- Perfil padrão, no início e no fim:
  - `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes;
  - snapshots por `cd <dir> && find . -printf '%p %s %T@\n' | sort |
    sha256sum`: `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker`
    `6046f67f…` (o Docker do D7 pode tê-lo alterado; ver o relatório).
- Raiz própria da revisão: `~/.local/share/vericode-spikes/rd7/`, com o
  helper copiado de `d7/bin/env.sh` e `D` trocado. Nunca usar `/tmp`.
- **Memória (7,6 GiB, editor aberto):**
  - nunca dois builds ou testes pesados em paralelo;
  - rode builds de `solana-program-test` destacados (`setsid nohup`),
    com `CARGO_BUILD_JOBS=2` e `nice`, e acompanhe o log;
  - no D7, builds com mais paralelismo derrubaram a sessão do editor (exit
    137).

## Checagem da tarefa anterior (reexecutar, offline salvo indicação)
- Locks:
  - raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/`
    `19a1db26…`, `tests-local` `be94760a…`;
  - `cli` `4d979577a6c7e6f60fce119c7dc5ca57b73c8da17a836475e085467fd5304db4`;
  - `prover` `8b76f1e1a8595208c0d6ce901b28e255f1ae0107a5549ffeca0a87f484446814`.
  - Confirmar que os dois novos só renomeiam a raiz das sementes
    (`anchor/tests-local/Cargo.lock` e `d4/receipts/Cargo.lock`) e não têm
    fonte git.
- `sha256sum prover/artifacts/vericode-guest.bin` → `e09ba8cf…`, 180.300
  bytes (também `git cat-file blob`).
- Core lane-a `1.85.0` e lane-b `1.89.0` `test --locked --offline` → 42
  passed em cada.
- `prover`: `cargo +1.89.0 test --locked --offline --release` → 4/4;
  `vericode-prover check` e `verify d7/receipts/P`. Com homes zkVM isoladas,
  como `zk()` do `d7/bin/env.sh`.
- `cli`: `cargo +1.89.0 build --release` e `test --locked --offline` → 14/14
  (Perfil A, `d2c`).
- `anchor/tests-local`: `cargo +1.89.0 test --locked --no-fail-fast` → 61/61,
  duas vezes:
  - escrow `cdf6967f…` + verificador `dab6746d…` (`d7/out/rebuild`);
  - escrow + dump de devnet `34ae6e5c…` (`d7/out/devnet`).
- `cd d4/receipts-out && sha256sum -c ../logs/receipts-out.sha256` e
  `sha256sum -c d7/logs/receipts-P.sha256`, ajustando o caminho.
- Rede somente leitura em `https://api.devnet.solana.com`:
  - `vericode check`;
  - `vericode job show` de P (`3e4ca0269e4af134738120703ccbfd751ef0d51fa6dcd2d99525d0d3f24c9c57`),
    T (`05f7493483cba42146cada07822160d2fbec4e67d854fafa32f70ef0bb01d758`),
    A (`3f0dd1c8…aec8a`) e B (`5a25ae48…fc309`);
  - `getTransaction` das assinaturas da tabela "Transações" do relatório.
- Se divergir: parar e reportar.

## Objetivo
Decidir, com evidência, se o MVP pode seguir para a demo final (D9). O
veredito é **APROVADO**, **APROVADO COM RESSALVAS** ou **REPROVADO**, com
condições.

## Decisões já tomadas (não reabrir; avaliar a implementação e os riscos)
- Escrow `GZqb…` imutável; CPI direta ao verificador `THq1q…`; mint `9TE2V…`;
  `JournalV1` v1 congelado (D4a, R-D4a, D4b).
- Claim único (CD7): "verificada em devnet por CPI ao verificador Groth16
  imutável de risc0-solana v3.0.0", com links. Nunca "Verifier Router" nem
  mainnet.
- Decisões 1 a 4 do D7 (CLI em `cli/`, locks semeados, prover em `prover/`
  com o guest versionado, lista fechada de escritas), mais as escolhas do
  plano: T antes de P, negativo 6014 como `deliver`+`release`, artefato
  `(21,42)`.

## Checklist adversarial (no mínimo)
1. **Instruções da CLI.**
   - Os bytes batem com o programa (não só com a suíte): conferir contra o
     fonte e a IDL `e8ce2c20…`.
   - Flags signer/writable; destino = ATA canônica; `CreateIdempotent` só
     quando falta a ATA.
   - Nenhum caminho de destino escolhido pelo chamador.
2. **Conferências e modo negativo.**
   - Fora de `--expect-error`, nenhuma conferência pode ser contornada.
   - `--expect-error` nunca pode reportar sucesso numa transação que moveu
     fundos. Avaliar a comparação de snapshot com `minContextSlot`, a
     checagem do log `Program <id> failed` e do `Custom(code)`, e códigos
     sobrepostos entre programas.
   - `--tamper-seal` só no modo negativo.
3. **Envio e confirmação.**
   - Simulação com `sigVerify`; reenvio da mesma transação; expiração do
     blockhash.
   - Assinatura devolvida == assinatura local.
   - `getTransaction` com retries; risco de reportar `PASS` sem a
     transação.
   - Pacing e backoff (429, `-32016`).
4. **Cluster e rede.**
   - Guarda por genesis de devnet; `--rpc-url` arbitrário.
   - TLS (`rustls` + webpki-roots).
   - Nenhum outro destino de rede no binário.
5. **Chaves.**
   - `keys::load`: work tree Git, symlinks, modo `0600`/`0400`, mensagens
     sem conteúdo.
   - Impressão só de pubkeys; `--log` com `0600`.
   - Os logs JSONL do D7 sem segredo.
   - Busca de segredos no repositório e nos logs (arrays de 64 bytes,
     base58 de 87–88 caracteres, "seed phrase").
   - A CLI do Agave só via `run_cli`.
6. **`job_id` e prazos.**
   - `getrandom`; PDA livre; `0x11` recusado.
   - Margem de 60 slots e janela [1.560, 1.512.000].
   - Saldos de SOL e Test USDC conferidos.
   - Squatting (F-09) residual.
7. **Prover.**
   - Guest embutido (`include_bytes!`) com SHA-256, tamanho e ImageID
     conferidos.
   - Frame igual ao `zkvm/host`; journal == core; `Composite` → `Groth16`;
     `disable-dev-mode`; nunca `Fake`; parâmetros do verificador default.
   - Shim Docker: troca de tag por digest, `--pull=never --network=none`,
     recusa de outro `run`, injeção no `PATH` do processo,
     `VERICODE_REAL_DOCKER`.
   - `RISC0_WORK_DIR` com arquivos de `root`; `TMPDIR`.
   - Proveniência do binário (D1c2b.3h) e ImageID não recertificado.
8. **Testes novos de `tests-local`.**
   - Vínculo das fixtures `d4b/` às receipts originais.
   - O replay cobre os negativos do D4b e a invariante 9.
   - Variantes do endereço do mint (9 decimais aceito = limitação RD4A-02).
   - Verificador ausente (`UnsupportedProgramId`).
   - Algum teste tautológico ou enfraquecido?
9. **Evidência em devnet.**
   - Cada assinatura de `docs/d7-cli-results.md` aterrissou com o resultado
     declarado; o verificador foi invocado na liquidação de P.
   - Estados finais; saldos fecham por lamport.
   - A invariante 9 (6007/6008) realmente exercitada.
10. **Claims.**
    - README, `docs/demo-script.md`, READMEs de `cli/` e `prover/` e docs vivos:
      - nenhum "Verifier Router" como caminho atual;
      - nenhum "ZK on-chain" genérico, mainnet, "trustless" ou "o código
        está correto";
      - claim CD7 com links;
      - limitações presentes.
    - O plano B do roteiro não apresenta evidência antiga como ao vivo.
    - Erratas pendentes, por exemplo `docs/manifest-schema.md`: "a implantar
      em devnet".
11. **Reprodutibilidade a partir de um clone.**
    - `git clone` local para um diretório da raiz `rd7/`.
    - Build e testes `--locked` (offline com as homes isoladas).
    - Os comandos do README estão certos e completos: seção 1 com dump dos
      `.so`, `cargo test` na raiz, caminhos fora do clone.
    - O que um terceiro não consegue reproduzir: Test USDC só da mint
      authority.
12. **Riscos herdados e prontidão para o D9.**
    - Sem e-stop; rent preso; mint authority; RPC; memória.
    - O que precisa ser condição explícita para a demo.

## Escopo autorizado
- Leitura do repositório e dos diretórios fora do clone listados acima.
- Clone local e builds/testes em targets novos em
  `~/.local/share/vericode-spikes/rd7/`.
- PoCs próprios só fora do clone, em cópias de `cli/` ou
  `anchor/tests-local`, com os mesmos locks. Por exemplo:
  - um cliente que tenta burlar as conferências;
  - um snapshot alterado;
  - uma chave dentro de um work tree.
- RPC somente leitura em devnet (`getAccountInfo`, `getMultipleAccounts`,
  `getSignaturesForAddress`, `getTransaction`, `simulateTransaction` com
  `sigVerify=false`, `vericode check`/`job show`).
- `vericode-prover check`/`verify` e, se necessário, `prove` (`Composite`,
  local) de um `job_id` qualquer fora do clone. Nada de `compress`.

## Fora de escopo / proibido
- Editar, criar ou apagar qualquer arquivo do clone; commit; push.
- Qualquer escrita em devnet: airdrop, transação assinada, criação de
  Job/ATA ou transferência.
- Abrir, imprimir ou copiar o conteúdo dos keypairs de `d4/keys`. Só
  `pubkeys.txt` pode ser lido.
- Docker: comprimir de novo ou fazer pull.
- Instalar algo no perfil padrão; usar `/tmp`; rede além do devnet somente
  leitura.

## Testes obrigatórios
- Reexecução da checagem acima, com saída real.
- PoCs adversariais próprios para os itens 2, 3, 5 e 7, no mínimo, com o
  resultado observado.
- Clone limpo com build e testes `--locked` (item 11).

## Evidências exigidas
- Resposta com:
  - preflight, comandos e saídas reais;
  - tabela de achados (ID, severidade, componente, local, resumo,
    evidência, bloqueia D9);
  - checklist 1–12;
  - situação de RD4A-01 a RD4A-08;
  - veredito com condições.
- A sessão de registro grava `docs/r-d7-review-results.md`, as entradas em
  `decisions.md` e `evidence.md`, atualiza `agent-control.md` e
  `project-context.md` e salva o handoff seguinte (D9 ou correção).

## Critério de pronto
- Veredito justificado por evidência executada.
- `git status --short` e `--ignored` vazios no início e no fim.
- Perfil padrão igual aos snapshots.

## Condições de parada
- Baseline, hash ou lock divergente → parar e reportar.
- Achado crítico ou alto → REPROVADO, com a correção recomendada. Nada de
  corrigir na revisão.
- Necessidade de escrita em devnet ou no clone → parar; está fora do escopo.

## Commit
- Não autorizado nesta revisão. Push sempre proibido.

## Depois da R-D7 (não executar aqui)
- **D9:**
  - demo em ambiente limpo (clone novo, homes novas), seguindo
    `docs/demo-script.md`;
  - Jobs novos com `job_id` aleatório;
  - gravação do vídeo.

  Precisa de autorização humana para as escritas em devnet e de Test USDC
  para o buyer.
- **D10–D12, se houver tempo:** worker de prova e telas Buyer, Submit e
  Result, como camadas finas sobre `vericode` e `vericode-prover`, sem regra
  econômica nova.
- **Chaves de devnet** em `d4/keys`:
  - deployer `617ogw9Tbem67ZwWDokEAkLV6JMq5avreijFMPrG75nd`, cerca de 2,805
    SOL;
  - buyer `EZgGUg4JhEAhzMPd4jBuKWFXdDNpFATvCjkj7mLAdxU6`, cerca de 0,128
    SOL e 999.997 Test USDC;
  - executor `EdB25bVhdj6rm5b7FbVHe5A3zx2hAhPpK4YAzrLaDs6U`, cerca de 0,030
    SOL.
- **Jobs consumidos** (nunca reutilizar): S, A, B, C, P, T.

## Relatório final
1. arquivos lidos e comandos; 2. saídas reais; 3. achados e severidade;
4. invariantes; 5. decisões pendentes; 6. riscos; 7. confirmação de
fronteiras; 8. veredito; 9. prompt da próxima fase (D9 ou correção),
segundo `docs/handoff-protocol.md`, para a sessão de registro salvar.
