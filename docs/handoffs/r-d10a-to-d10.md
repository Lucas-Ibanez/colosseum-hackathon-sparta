# D10 — worker local com prova local forçada (base das telas)

## Identificação
- Gate: `D10`  ·  Dia da sequência: D10  ·  Marcos do guia: M5/M6 (fluxo pela
  interface fina), M7 (claims iguais ao código); guia §4 (`worker-api`), §7,
  §10, §12
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `R-D10a` (APROVADO COM RESSALVAS para o D10; condições
  C10-1 a C10-10), relatório `docs/r-d10a-review-results.md`, registrado no
  commit `docs: record R-D10a review` sobre `2a2e3c5`.
- Este prompt foi reconstruído pela sessão de registro a partir das
  condições e das propostas avaliadas pelo R-D10a, porque o item 7 da
  resposta da revisão chegou truncado. Em conflito com o relatório, vale o
  relatório; avise.
- Prazo do MVP: 11/10 (hoje é 06/10). Timebox do D10: até 07/10. Registrar
  timestamps por fase.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh**. É código novo
  que assina transações de devnet e expõe uma API HTTP local.
- **Plan Mode obrigatório** antes de criar `worker/` e antes da primeira
  escrita em devnet. O plano traz:
  - as rotas exatas e o argv fixo de cada ação;
  - o modelo de estados;
  - os testes;
  - W1 a W7 com os comandos HTTP exatos.

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`.
- `docs/r-d10a-review-results.md` (RD10A-01 a 07; **C10-1 a C10-10**;
  avaliação de P1 a P6; lista de escritas).
- `docs/decisions.md`: D10a, R-D10a e "Decisões humanas para o D10".
- `docs/d10a-hardening-results.md`, `docs/d9-demo-results.md`,
  `docs/demo-script.md` (estados, falas e frases congeladas), `README.md`
  ("Frases permitidas (congeladas no D9)", "Não dizer", "Limitações").
- `cli/README.md` e `prover/README.md` (subcomandos, saídas `chave=valor` e
  o `--log` JSONL); `cli/src/main.rs` (formato das saídas); `prover/src/lib.rs`
  (`check`, `prove`, `compress`).
- `docs/context/guia-mvp-agentes-de-codigo.md` §4, §7, §10 e §12;
  `docs/context/sequencia-mvp.md` (D10–D12).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3`. O topo esperado é
  `docs: record R-D10a review` sobre `2a2e3c5`. Se não for, parar.
- `git status --short` e `--ignored` vazios; `git diff --check` 0.
- Perfil padrão, no início e no fim:
  - `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes;
  - `cd <dir> && find . -printf '%p %s %T@\n' | sort | sha256sum`:
    `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…`.
- Raiz própria `~/.local/share/vericode-spikes/d10/` (`0700`, logs `0600`),
  com `data/` do worker e `tmp/` (TMPDIR).
- **Não alterar `d9/`, `d10a/`, `rd10a/` nem `rec-*`.** Os binários de
  `d10a/targets` são só lidos e executados.
- Scripts com nomes prefixados; não carregar o `env.sh` herdado nem usar
  `R`, `B`, `D` ou `VC`.
- **Memória:** o WSL tem 7,6 GiB de RAM e 8 GiB de swap. Um processo pesado
  por vez; `compress` só com MemAvailable ≥ 2,5 GB (C10-4).

## Checagem da tarefa anterior
- Locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/`
  `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover`
  `8b76f1e1…`. Guest 180.300 B `e09ba8cf…`.
- **Binários (C10-1):**
  - `~/.local/share/vericode-spikes/d10a/targets/cli/release/vericode` →
    SHA-256 começa com `e6cd4e29…`;
  - `~/.local/share/vericode-spikes/d10a/targets/prover/release/vericode-prover`
    → `3f66e1c0…`;
  - nunca os do D9 (`7e7a9260…`/`79b83528…`) nem `targets/cli/debug`.
- **Shim (C10-2):** `prover/docker-shim/docker` → `2a8f75b8…`, modo `0755`.
- **Preflight duplo:**
  - `vericode check` → `check=ok`, com `bytes=199256 sha256=34ae6e5c…` no
    verificador;
  - `vericode-prover check` → `prover=LocalProver env=ok`.
- **Devnet, só leitura:**
  - `job show` de `8ab4ee8d…` e `8bce67f2…` → `Funded`, prazo vencido,
    1.000.000 no vault;
  - P′ `91ea6fcd…` `Released`; T′ `ec9afb74…` `RefundedOnTimeout`.
- **Saldos esperados** (fim do R-D10a):
  - SOL: deployer 2.805.194.240; buyer 103.351.800; executor 29.925.000
    lamports;
  - Test USDC: ATA do buyer 999.992.000.000; ATA do executor 6.000.000.
- Se divergir: comparar `getSignaturesForAddress` com novas pastas `rec-*`
  do humano (só campos públicos). Escrita não explicada: parar e reportar.

## Objetivo
Entregar em `worker/` um worker HTTP local, mínimo e testado. Ele:
- executa o fluxo do MVP só chamando os binários do D10a, com prova local
  forçada;
- é a base das telas do D11–D12;
- é validado em devnet com as escritas W1 a W7 dirigidas pela própria API.

## Decisões já tomadas
- Claim único (CD7) e frases permitidas congeladas no D9; nenhuma frase
  nova neste gate.
- Escrow `GZqb…` imutável; verificador `THq1q…`; mint `9TE2V…`; `JournalV1`
  v1 congelado.
- Jobs consumidos, nunca reutilizar: S, A, B, C, P, T, P′, T′ e os das
  gravações (`8ab4ee8d`, `8bce67f2`, `cd77e7bd`, `3e115ca8`, `104f9a21`).
- Condições **C10-1 a C10-10** do R-D10a, integralmente.
- **Decisões humanas para o D10** (ratificadas ao enviar este prompt;
  registradas em `docs/decisions.md`):
  1. **P1 — arquitetura:**
     - worker HTTP em `127.0.0.1` que só chama `vericode` e
       `vericode-prover` por `subprocess`, com argv fixo em lista, sem shell
       e sem duplicar instruções, regras ou hashing;
     - chaves só no worker, por caminho: **buyer e executor; nunca a do
       deployer** (C10-9);
     - estados: `Draft` (só UI), `Funded`, `Proving` (etapa local do
       executor, fora da cadeia), `Submitted` (tx enviada), `Released`,
       `RefundedOnFail`/`RefundedOnTimeout`, `Failed` (erro operacional);
     - nenhuma transição econômica fora do programa.
  2. **P2 — sem dependência nova:**
     - Python 3.12 só com a biblioteca padrão (`http.server` em loopback,
       `subprocess`, `json`, `secrets`, `hmac`);
     - testes com `unittest`;
     - executáveis falsos só nos testes, rotulados; o worker real recusa
       binário com hash diferente;
     - nenhum npm, crate ou lock novo.
  3. **P3 — sem carteira no navegador.**
     - Divergência explícita do D11 da sequência ("conectar carteira
       devnet"), que tem a menor precedência.
     - Limitação declarada: chaves de devnet do projeto num worker local, e
       o mesmo operador local opera buyer e executor.
  4. **P4 — rótulo do programa que rejeitou:** permitido, só a partir da
     saída da CLI do D10a (C10-7). `UNEXPECTED` nunca vira "rejeitado".
  5. **P5 — escritas em devnet, lista fechada, todas pela API do worker:**
     - **W1:** `refund-timeout` de `8ab4ee8d…`, pagador buyer →
       `RefundedOnTimeout`;
     - **W2:** o mesmo para `8bce67f2…`;
     - **W3:** `create`: buyer, executor `EdB25…`, 1.000.000, offset 9.000;
     - **W4:** no Job de W3, logo após criar, `refund-timeout --expect-error
       escrow:6021`, com C10-8 (`deadline_slot − slot ≥ 300` num `job show`
       imediatamente anterior);
     - prova `(21,42)` e `compress` locais (não são escritas), com C10-2 e
       C10-4;
     - **W6:** `settle --deliver --tamper-seal --expect-error verifier:6003`;
     - **W5:** `settle --deliver` → `Released`, com o verificador invocado;
     - **W7:** `settle --expect-error escrow:6007` depois do W5.

     Ordem de execução: W1, W2, W3, W4, prova, W6, W5, W7. Nada além disso.
  6. **P6 — ordem até 11/10:** D10 (até 07/10) → D11–D12 telas (08–09/10) →
     revisão curta da interface (09/10) → vídeo definitivo, numa tomada
     contínua com os mesmos Jobs (10/10) → submissão (11/10).

## Escopo autorizado
- **Criar `worker/`:**
  - código Python (stdlib) e `worker/README.md`;
  - testes `unittest` em `worker/tests/`, com executáveis falsos rotulados
    e um RPC falso só em `127.0.0.1`, se necessário;
  - nenhum arquivo de chave, configuração real ou dado de execução dentro
    do clone.
- **Configuração e dados fora do clone** (`d10/`, `0700`/`0600`):
  - caminhos dos binários e dos keypairs de buyer e executor;
  - hashes esperados;
  - diretório de dados (receipts, JSONL de operações).
- **Rotas fixas** (confirmar no Plan Mode), por exemplo:
  - `GET /api/health`: os dois `check` e o hash do shim;
  - `GET /api/jobs/<job_id>`: `job show` e o estado reconciliado;
  - `POST /api/jobs`: `create`; o servidor gera o `job_id` (a CLI gera) e
    limita o offset;
  - `POST /api/jobs/<id>/prove`: entrada e saída `u32` validadas no
    servidor;
  - `POST /api/jobs/<id>/settle`;
  - `POST /api/jobs/<id>/refund-timeout`;
  - `POST /api/jobs/<id>/negative/<kind>`, com `kind` só em `escrow-6021`,
    `verifier-6003`, `escrow-6007` e, opcionalmente, `escrow-6014`, com uma
    receipt de outro Job deste worker, sempre identificada.
- **Docs:**
  - `worker/README.md`;
  - `README.md` ("Limitações": worker local e P3), sem mudar as frases
    congeladas;
  - `docs/architecture.md` (`worker-api`);
  - `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`,
    `docs/project-context.md`;
  - `docs/d10-worker-results.md`, `docs/handoffs/d10-to-d11.md`.
- **Rede:** `api.devnet.solana.com` (pela CLI) e `127.0.0.1`.
- **Docker:** só via `vericode-prover compress` (shim), imagem já presente.

## Fora de escopo / proibido
- Mudar `cli/`, `prover/` (inclusive o modo ou o conteúdo do shim),
  programa, core, `zkvm/`, guest, `JournalV1`, schema ou qualquer lock.
- Rebuild dos binários sem decisão; usar os do D9.
- Herdar o ambiente do worker nos subprocessos; `BONSAI_*`,
  `RISC0_DEV_MODE`, `RISC0_WORK_DIR`, `VERICODE_*`, `DOCKER_*` ou proxies
  nos subprocessos (C10-3).
- Bind fora de `127.0.0.1`; CORS; rotas dinâmicas para arquivos; caminho ou
  conteúdo de chave em resposta, log ou página (C10-6).
- Chave do deployer no worker; keypair novo; keypair dentro do clone.
- Retry automático de escrita (C10-5); duas operações simultâneas (C10-4).
- Escrita em devnet fora de W1 a W7; airdrop; deploy; mainnet; Docker pull;
  `/tmp`.
- Telas (D11–D12), salvo uma página mínima de diagnóstico, se ajudar os
  testes.
- Push.

## Implementação esperada
- **Inicialização:**
  - lê a configuração fora do clone;
  - confere o hash de cada binário e do shim;
  - roda o preflight duplo;
  - gera o token aleatório da execução e o mostra só no terminal do
    operador;
  - recusa iniciar se qualquer item falhar.
- **Execução:**
  - cada ação = um argv fixo, montado no servidor, com ambiente construído
    do zero (`HOME`, `PATH=/usr/bin:/bin`, `TMPDIR` em `d10/tmp`; para o
    prover, `RISC0_PROVER=local`) e timeout explícito;
  - lock global (409 se ocupado); `compress` só com MemAvailable ≥ 2,5 GB.
- **Estado (C10-5):**
  - vem de `job show` e do `--log` JSONL da CLI (`outcome`, `signature`,
    `slot`), nunca só do exit code;
  - o `job_id` é registrado antes da transação;
  - um `create` com exit ≠ 0 é reconciliado por `job show` antes de
    qualquer outra ação.
- **Shim (C10-2):**
  - antes de cada `compress`, hash `2a8f75b8…` e modo `0755`;
  - depois, exatamente uma linha `compress.docker_run=` desta execução, igual
    ao formato do relatório (`--context default run --pull=never
    --network=none --rm -v <dir>/groth16-work:/mnt …@sha256:7f173963…`);
  - sem ela, estado `Failed`, e a receipt não é usada.
- **Respostas HTTP:**
  - JSON com campos públicos: `job_id`, estado, assinaturas, links do
    Explorer, CU, código e programa da rejeição (C10-7), hashes;
  - saída da CLI como texto, sem HTML.
- **HTTP (C10-6):** `Host` conferido (`127.0.0.1:<porta>`); token em header
  próprio (por exemplo `X-VeriCode-Token`) com `hmac.compare_digest` em toda
  escrita ou prova; sem CORS.

## Testes obrigatórios
- **`unittest`, offline:**
  - binário com hash errado → recusa ao iniciar;
  - shim com hash ou modo errado → `compress` recusado;
  - ausência da linha `docker_run` → `Failed`;
  - ambiente dos subprocessos exatamente o construído: variáveis hostis
    no ambiente do worker não chegam aos filhos;
  - argv fixo; parâmetros inválidos (`job_id`, `u32`, offset, `kind`) → 400
    sem executar nada;
  - `Host` errado → 400/421; sem token ou token errado → 401/403; nenhum
    cabeçalho CORS;
  - duas operações → a segunda recebe 409;
  - `UNEXPECTED` nunca vira "rejeitado";
  - exit ≠ 0 depois de `[label] PASS` → estado reconciliado pelo `job show`
    e pelo log;
  - nenhuma resposta, log ou página contém `/keys/`, `.json` de chave ou
    bytes de chave.
- **Regressão sem rebuild:** os dois `check` com os binários do D10a.
- **Devnet:** W1 a W7 pela API, na ordem decidida, com assinatura, link,
  estado e saldos antes e depois.
- **Fechamento:**
  - ATA do buyer +2.000.000 (W1, W2) −1.000.000 (W3);
  - ATA do executor +1.000.000 (W5);
  - SOL do buyer −(3.581.400 de rent + taxas).

## Evidências exigidas
- `docs/d10-worker-results.md` com:
  - comandos e saídas reais;
  - rotas e argv;
  - resultados dos testes;
  - W1 a W7 com links;
  - linha `docker_run` do `compress`;
  - saldos que fecham;
  - confirmação de que nenhuma chave nem caminho de chave saiu do worker.
- Atualizar `docs/decisions.md` (inclusive "Decisões humanas para o D10"),
  `docs/evidence.md`, `docs/agent-control.md`, `docs/project-context.md` e
  `docs/architecture.md`.

## Critério de pronto
- Worker com testes verdes; preflight e checagens C10-1/C10-2 ativos.
- W1 a W7 concluídos pela API com os resultados esperados; os dois Jobs das
  gravações reembolsados.
- Frases congeladas intactas; limitações atualizadas.
- `git diff --check` 0; diff integral revisado; busca de segredos limpa;
  locks e perfil padrão inalterados; `d9/`, `d10a/`, `rd10a/` e `rec-*`
  intocados.

## Condições de parada
- Hash de binário ou shim divergente; preflight falhou → `BLOQUEADO`.
- Divergência de saldo ou estado não explicada → parar e reportar.
- Escrita fora de W1 a W7 → `AGUARDANDO_AUTORIZAÇÃO`.
- Necessidade de mudar `cli/`, `prover/` ou de dependência nova →
  `AGUARDANDO_AUTORIZAÇÃO`.
- Mnemônico ou conteúdo de chave numa saída → parar e registrar o
  incidente.
- `compress` com exit 137 ou MemAvailable < 2,5 GB → esperar e repetir uma
  vez; se persistir, `BLOQUEADO`.

## Commit
- Commits locais autorizados pelo humano ao enviar este prompt, com
  identidade via `git -c`. Push proibido.
  - `worker: add local proof worker (D10)`;
  - `docs: record D10 worker`.
- Em parada com evidência parcial: commitar só o que estiver testado, com
  evidência real.

## Relatório final
1. arquivos modificados;
2. testes e transações reais, com links;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`, salvo em
   `docs/handoffs/d10-to-d11.md`. **D11–D12:** telas finas Buyer, Submit e
   Result em HTML/JS estático servido pelo worker. Elas mostram:
   - estados e links do Explorer;
   - a frase de limite;
   - `Proving` rotulado como etapa local;
   - um cenário negativo com o programa que rejeitou (C10-7);
   - só frases congeladas.

   Depois: revisão curta da interface (claims e chaves), vídeo definitivo
   numa tomada contínua e submissão até 11/10.
