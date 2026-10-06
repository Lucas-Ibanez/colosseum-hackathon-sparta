# R-D10a — revisão delta curta do endurecimento da CLI e do prover

## Identificação
- Gate: `R-D10a`  ·  Dia da sequência: D10 (preparação de D10–D12)  ·
  Marcos do guia: M5/M6 (robustez do caminho por CLI), M7 (claims iguais ao
  código); revisão do guia §11
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D10a`. Commits:
  - `50dede0` (`cli: harden negative matching, rpc reads and tests
    (D10a)`);
  - `7fe9c3b` (`prover: force local prover and exact docker shim argv
    (D10a)`);
  - topo `docs: record D10a hardening`.

  Relatório: `docs/d10a-hardening-results.md`.
- Base da revisão: `39a87f7` (D9). O delta é `39a87f7..HEAD`.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço max**, em **sessão
  separada e somente leitura**. É a última revisão do código de envio de
  transações, do rótulo dos negativos e da escolha do prover antes de o
  worker e as telas passarem a usá-los. Pela política de `CLAUDE.md`,
  revisões de segurança não editam arquivos.
- Plan Mode: não se aplica, porque a revisão não altera o repositório. Se a
  revisão concluir que algo precisa mudar, isso vira achado e condição, não
  edição.
- Prazo do MVP: 11/10. Registrar timestamps por fase.

## Leitura obrigatória (integral, antes de revisar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`.
- `docs/r-d7-review-results.md`: RD7-01 a 10, CR1 a CR8 e as PoCs.
- `docs/d10a-hardening-results.md`, inteiro, inclusive os incidentes.
- `docs/decisions.md`: entradas "Decisões humanas para o D10a" e "D10a".
- **Código do delta**, `git diff 39a87f7..HEAD -- cli prover`:
  - `cli/src/tx.rs`, `rpc.rs`, `main.rs`, `keys.rs`, `escrow.rs`;
  - `cli/tests/negative_runs.rs`, `guards.rs`, `fake_rpc/mod.rs`,
    `instructions.rs`;
  - `prover/src/lib.rs`, `prover/docker-shim/docker`,
    `prover/tests/docker_shim.rs`.
- **Docs do delta:**
  - `README.md` ("Limitações"), `cli/README.md`, `prover/README.md`,
    `docs/demo-script.md`;
  - `docs/manifest-schema.md:86` (errata).
- **Fora do clone, somente leitura:**
  - `~/.local/share/vericode-spikes/d10a/`:
    - `poc/rpc/fake10.py`, `run10.sh`;
    - `poc/prover/listener.py`, `bonsai10.sh`;
    - `logs/` (`timeline.log`, `fake-{d9,new}.log`,
      `bonsai-{d9,new}.log`, `c2`, `c3`, `p2`, `p4`, `x-*`, `wrongbin-*`,
      `before-src.diff`);
  - as PoCs do R-D7 em `rd7/poc/`.
- **Fonte upstream pinado** (nas homes isoladas):
  - `risc0-groth16 3.0.2` `src/prove/docker.rs`;
  - `risc0-zkvm 3.0.3` `src/lib.rs` (exports) e
    `src/host/client/prove/{mod.rs,local.rs}`;
  - `solana-transaction-error 2.2.1` (texto de `AlreadyProcessed`).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -4`. Esperado:
  `docs: record D10a hardening` → `7fe9c3b` → `50dede0` → `39a87f7`. Se não
  for, parar.
- `git status --short` e `--ignored` vazios; `git diff --check` 0. Repetir
  no fim.
- Perfil padrão, no início e no fim:
  - `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes;
  - `cd <dir> && find . -printf '%p %s %T@\n' | sort | sha256sum`:
    `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…`.
- Raiz própria `~/.local/share/vericode-spikes/rd10a/` (`0700`, logs `0600`).
  **Não alterar `d9/`, `d10a/` nem `rec-*`.**
- Scripts com nomes prefixados. Não carregar o `env.sh` herdado nem usar
  `R`, `B`, `D` ou `VC`. `d10a/bin/env10.sh` pode servir de modelo; copie-o,
  não o altere.
- Homes: copiar de `d10a/homes` (ou `d9/homes`).
- **Um `CARGO_TARGET_DIR` por árvore.** No D10a, um target compartilhado
  entre a árvore "antes" e o repositório sobrescreveu o binário do prover
  (incidente 1).
- **Memória (7,6 GiB):** `free -m` antes de cada processo pesado; um por
  vez, destacado (`setsid nohup`, `nice`, `CARGO_BUILD_JOBS=2`; prover até
  4). O build do prover leva cerca de 32 min. Uma compressão Groth16 com 80
  MB livres derrubou a sessão no D10a; evite compressão real, salvo
  necessidade.

## Checagem da tarefa anterior
- Locks iguais:
  - raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`;
  - `anchor/` `19a1db26…`, `tests-local` `be94760a…`;
  - `cli` `4d979577…`, `prover` `8b76f1e1…`.
- Guest 180.300 B `e09ba8cf…`.
- Delta de código só em `cli/src`, `cli/tests`, `prover/src`,
  `prover/tests` e `prover/docker-shim/docker`.
  `git diff --stat 39a87f7..HEAD -- '*.lock' '*Cargo.toml' anchor crates zkvm prover/artifacts`
  tem de ser vazio.
- Reproduzir num clone de HEAD, `--locked --offline`, com targets próprios:
  - CLI `cargo +1.89.0 test` **30/30**;
  - prover `cargo +1.89.0 test --release` **5/5** e `docker_shim` **2/2**;
  - `vericode-prover check` → `prover=LocalProver env=ok`;
  - opcional, se houver tempo: core 42/42 ×2 e suíte 61/61 com os `.so` de
    devnet (`cdf6967f…`/`34ae6e5c…`).
- Devnet, somente leitura, com a CLI compilada na revisão:
  - `vericode check` → `check=ok`, inclusive `check.verifier=… bytes=199256
    sha256=34ae6e5c…`;
  - `job show` de P′ `91ea6fcd…` (`Released`) e T′ `ec9afb74…`
    (`RefundedOnTimeout`).
- Saldos esperados (fim do D10a, sem escrita do agente):
  - SOL: deployer 2.805.194.240; buyer 103.351.800; executor 29.925.000
    lamports;
  - Test USDC: ATA do buyer 999.992.000.000; ATA do executor 6.000.000.

  Se divergir, comparar `getSignaturesForAddress` com novas pastas `rec-*`
  do humano (só campos públicos). Escrita não explicada: parar e reportar.

## Objetivo
Confirmar, ou refutar com evidência, que o delta do D10a fecha RD7-01, 02,
03, 04 e 07 (e os opcionais 06, 09 e 10) sem regressão nem risco novo, e
dar o veredito para o D10 (worker).

## Decisões já tomadas
- Claim único (CD7) e frases permitidas congeladas no D9 (`README.md`,
  `docs/demo-script.md`). A revisão confere que o delta não mudou a lista.
  Fonte: `docs/decisions.md`, entrada D9.
- Escrow `GZqb…` imutável; verificador `THq1q…`; mint `9TE2V…`; `JournalV1`
  v1 congelado.
- Jobs consumidos: S, A, B, C, P, T, P′, T′ e os das gravações do humano
  (`8ab4ee8d`, `8bce67f2`, `cd77e7bd`, `104f9a21`, `3e115ca8`). Fonte:
  entrada D10a.
- Decisões do D10a (nenhuma escrita em devnet; Docker local só para o
  `compress`; errata da linha 86; `d7-cli-results.md` não editado) e os
  acréscimos aprovados no Plan Mode:
  - `--context default` e a guarda de `DOCKER_HOST` no shim;
  - a chave `d4/keys/executor.json`, só por caminho, contra o RPC falso;
  - os opcionais.

  Fonte: "Decisões humanas para o D10a".
- CR8 continua valendo para o D10–D12.

## Decisões a confirmar ou pendentes
- Nenhuma para a revisão.
- A revisão **propõe** no prompt do D10, para decisão humana:
  - se a UI pode, a partir do D10a, rotular qual programa rejeitou a partir
    de `--expect-error` (CR8 condicionava isso à RD7-01);
  - se o D10 terá escritas em devnet, e em qual lista fechada;
  - o que fazer com os Jobs `Funded` das gravações.

## Escopo autorizado
- Leitura do repositório e do histórico Git. Clone local em `rd10a/`.
- Builds e testes `--locked --offline` fora do clone, com homes copiadas.
- **Rede:**
  - `api.devnet.solana.com`, só leitura;
  - `127.0.0.1` para o RPC falso, o listener e testes próprios.
- **Docker real:** opcional e só `vericode-prover compress` pelo shim, com a
  imagem já presente por digest, sem pull. Prefira o `docker` falso.
- `d4/keys/executor.json`: só por caminho, só contra 127.0.0.1, se
  reexecutar o roteiro do RPC falso.

## Fora de escopo / proibido
- Editar, criar ou apagar arquivos do repositório; commit; push.
- Escrita em devnet; deploy; airdrop; mainnet.
- Docker pull; instalação no perfil padrão; `/tmp`.
- Keypair novo; exibir ou copiar conteúdo de chave.
- Alterar `d9/`, `d10a/` ou `rec-*`.
- Rede além de devnet (só leitura) e 127.0.0.1.

## Implementação esperada
Não se aplica (revisão). Checklist mínimo:
1. **RD7-01** (`tx.rs`):
   - `failing_program`, `innermost_failure` e `expected_failure` com
     `Program log:`/`Program data:` forjados;
   - logs truncados;
   - ordem das instruções de topo (`CreateIdempotent` antes do escrow) e
     CPI aninhada;
   - erro de Token dentro do escrow;
   - `Expect::Verified` na simulação e na transação aterrissada.

   Há caso em que `escrow:N` ainda casa com falha de outro programa, ou em
   que `verifier:N` deixa de casar com uma rejeição real (W5 do D9,
   `HXgnGUhh…`)?
2. **RD7-02** (shim):
   - a lista de chamadores está completa (`is_docker_installed`,
     `shrink_wrap`, `image inspect`)?
   - argv exatos; caminho (`realpath -e`, `/`, `:`, `,`, quebra de linha,
     `RISC0_WORK_DIR` relativo ou com barra final);
   - `--context default` diante de `DOCKER_CONTEXT`, `DOCKER_HOST` e
     `DOCKER_CONFIG`;
   - `VERICODE_REAL_DOCKER`; injeção pelo `PATH`;
   - o log com `REFUSED`.
3. **RD7-03** (`lib.rs`):
   - resta algum caminho em que o ambiente escolha o prover?
     (`get_prover_server`, `ProverOpts`, `RISC0_SERVER_PATH`,
     `local_executor`);
   - cobertura de `local_prover_env` em `check`, `prove` e `compress` (o
     `verify` não prova);
   - os valores de `BONSAI_*` nunca aparecem na saída.
4. **RD7-04:** os testes comparam contra referências independentes
   (builders da suíte, logs reais do W5) e não são tautológicos? O RPC falso
   em processo é fiel ao formato do Agave?
5. **RD7-07** (`rpc.rs`, `tx.rs`, `main.rs`):
   - detecção de `AlreadyProcessed`;
   - nenhum sucesso falso, inclusive com status `null` e blockhash vencido;
   - `read_after` com `minContextSlot` e o retry de `-32016`, nos quatro
     comandos;
   - as leituras antes da transação continuam corretas?
6. **Opcionais:**
   - ordem da checagem de `nlink`; `0600` no `--log`;
   - `check_url` (maiúsculas, userinfo, IPv6, `localhost.`);
   - `no_proxy`;
   - constantes do verificador (`ENdLkqHp…`, 199.256 B, `34ae6e5c…`).
7. **Docs = código:**
   - "Limitações" do README, `cli/README.md`, `prover/README.md` e roteiro;
   - a seção "Gravação" continua descrevendo os binários do D9;
   - a lista de frases permitidas e o "Não dizer" intactos;
   - a errata só na linha 86 de `manifest-schema.md`.
8. **Incidentes do D10a** (binário sobrescrito, desconexão): o relatório os
   trata com honestidade, e a evidência válida está separada da execução
   errada.
9. **Segredos:** varredura dos blobs novos do histórico (`39a87f7..HEAD`) e
   de `d10a/{logs,bin,poc}`.
10. **CR8 e D10:** o que muda para o worker e as telas com o D10a. Por
    exemplo, o worker deve usar só o binário novo e o `check` do prover
    como preflight.

## Testes obrigatórios
- Reprodução: CLI 30/30 e prover 5/5 + 2/2 a partir de um clone de HEAD,
  com targets próprios.
- **Antes × depois:** pelo menos um caso por achado, contra binários
  compilados na revisão:
  - CLI de HEAD contra um RPC falso em 127.0.0.1 (modelo `d10a/poc/rpc`):
    `overlap6003`/`6000` com `escrow:N` recusado; `verifier:6003` aceito;
    `already` e `lag` → PASS;
  - shim com `docker` falso: casos hostis novos, inventados pela revisão;
  - prover de HEAD com ambiente Bonsai e listener local → recusa, 0
    conexões.
- Devnet só leitura: `check` e `job show` de P′ e T′.
- Comandos: os do relatório do D10a, com `env -i`, homes copiadas,
  `--locked --offline`.

## Evidências exigidas
A revisão **não edita arquivos**. A resposta final traz:
- tabela de achados (ID `RD10A-NN`, severidade, componente, local,
  evidência, recomendação, se bloqueia o D10), veredito e condições;
- **texto de registro pronto** para uma sessão com escrita:
  - entrada em `docs/decisions.md`;
  - linha em `docs/evidence.md`;
  - conteúdo integral de `docs/r-d10a-review-results.md`;
- o **prompt do D10** (worker com prova local forçada), segundo
  `docs/handoff-protocol.md`.

## Critério de pronto
- Veredito para o D10 (APROVADO, APROVADO COM RESSALVAS ou REPROVADO), com
  evidência reproduzida.
- Repositório intocado: `status --short` e `--ignored` vazios, HEAD igual.
- Perfil padrão inalterado; `d9/`, `d10a/` e `rec-*` intocados; busca de
  segredos limpa.

## Condições de parada
- HEAD, lock, hash ou saldo divergente sem explicação → parar e reportar.
- Necessidade de editar → não editar; registrar como achado.
- Mnemônico ou conteúdo de chave numa saída → parar e registrar o
  incidente.
- Exit 137 → repetir uma vez, sozinho; se persistir, `BLOQUEADO`.

## Commit
Não se aplica: a revisão é somente leitura. O registro é feito depois, por
uma sessão com permissão de escrita, a partir da resposta. Push sempre
proibido salvo autorização separada.

## Relatório final
1. arquivos lidos e comandos executados (nenhum arquivo do repositório
   modificado);
2. testes e saídas reais (reprodução e antes × depois);
3. invariantes (guia §7);
4. decisões pendentes (humanas);
5. riscos;
6. confirmação de fronteiras;
7. texto de registro (`decisions.md`, `evidence.md`,
   `docs/r-d10a-review-results.md`) e o **prompt do D10**, segundo
   `docs/handoff-protocol.md`:
   - **D10:** worker de prova com prova local forçada;
   - modelo/esforço recomendados: Opus 5.5, xhigh;
   - Plan Mode obrigatório;
   - CR8;
   - preflight com `vericode check` e `vericode-prover check`.

   Depois: D11–D12 (telas finas, CR8), revisão curta da interface, vídeo
   definitivo e submissão até 11/10.
