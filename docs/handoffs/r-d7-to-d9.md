# D9 — demo em ambiente limpo, congelamento dos claims e roteiro de gravação

## Identificação
- Gate: `D9`  ·  Dia da sequência: D9  ·  Marcos do guia: M6 (CLI em ambiente
  limpo) e M7
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `R-D7` (revisão adversarial final, **APROVADO COM
  RESSALVAS**, condições CR1 a CR8), revisado sobre `c550a98` e registrado no
  commit `docs: record R-D7 final review`; relatório
  `docs/r-d7-review-results.md`.
- Este prompt é a versão revisada do item 7 da resposta do R-D7, com as
  decisões humanas já tomadas.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh**. O gate envolve
  escritas em devnet, claims públicos e um arquivo que exige Plan Mode
  (`docs/manifest-schema.md`).
- **Plan Mode obrigatório** antes:
  - da primeira escrita em devnet; o plano lista W1 a W8 com comandos
    exatos;
  - da errata em `docs/manifest-schema.md`.
- Prazo do MVP: 11/10. Registrar timestamps por fase.

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/r-d7-review-results.md` (RD7-01 a 10; CR1 a CR8)
- `docs/d7-cli-results.md`, `docs/demo-script.md`, `README.md`,
  `cli/README.md`, `prover/README.md`, `prover/artifacts/README.md`
- `docs/decisions.md`: D7, R-D7 e "Decisões humanas para o D9"
- `docs/context/guia-mvp-agentes-de-codigo.md` §2, §3, §9, §10, §12, §14;
  `docs/context/sequencia-mvp.md` (D9)
- Fora do clone, somente leitura: `~/.local/share/vericode-spikes/d7/bin/`
  (`env.sh`, `cli.sh`, `prove_p.sh`) e `rd7/bin/env.sh`.

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3`. O topo esperado é
  `docs: record R-D7 final review` sobre `c550a98`. Se não for, parar.
- `git status --short` e `--ignored` vazios; `git diff --check` 0.
- Perfil padrão, no início e no fim:
  - `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes;
  - `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…`.
- Raiz própria `~/.local/share/vericode-spikes/d9/` (`0700`).
- **Atenção:** o `env.sh` herdado define `R`, `B`, `D` e `VC`. Não
  reutilizar esses nomes em scripts próprios. No R-D7, essa colisão gravou
  um arquivo no diretório errado.
- **Memória (7,6 GiB):**
  - `free -m` antes de cada processo pesado;
  - um por vez, destacado (`setsid nohup`, `nice`, `CARGO_BUILD_JOBS=2`;
    prover até 4);
  - o build do prover leva cerca de 57 minutos.
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou
  stash.

## Checagem da tarefa anterior
- Locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor/`
  `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover`
  `8b76f1e1…`.
- Guest `prover/artifacts/vericode-guest.bin`: 180.300 B, `e09ba8cf…`.
- Devnet, somente leitura:
  - `vericode check` → `check=ok`;
  - `job show` de P, T, A e B como em `docs/r-d7-review-results.md`.
- Saldos esperados:
  - SOL: deployer 2.805.224.240; buyer 128.466.600; executor 29.970.000
    lamports;
  - Test USDC: ATA do buyer 999.997.000.000; ATA do executor 3.000.000.
- Se divergir: parar, registrar e reportar.

## Objetivo
Rodar o fluxo do MVP pela CLI e pelo prover num ambiente limpo, em devnet,
com Jobs novos e os negativos seguros. Depois:
- congelar os claims;
- deixar o roteiro pronto para o humano gravar o vídeo de reserva.

## Decisões já tomadas
- Claim único (CD7): "verificada em devnet por CPI ao verificador Groth16
  imutável de risc0-solana v3.0.0", com links. Nunca "Verifier Router" nem
  mainnet.
- Escrow `GZqb…` imutável; verificador `THq1q…`; mint `9TE2V…`; `JournalV1`
  v1 congelado.
- Chaves de devnet em `d4/keys`; só pubkeys podem ser exibidas.
- Jobs consumidos: S, A, B, C, P e T. Nunca reutilizar.
- Condições CR1 a CR8 do R-D7.
- **Decisões humanas para o D9** (ratificadas ao enviar este prompt;
  registradas em `docs/decisions.md`):
  1. **Lista fechada de escritas em devnet**, nesta ordem:
     - **W1:** Job T′ — `job create --deadline-offset 1560`;
     - **W2:** logo depois, `job refund-timeout --expect-error escrow:6021`
       (≥ 60 slots antes do prazo; CR2);
     - **W3:** Job P′ — `job create --deadline-offset 9000`;
     - prova `(21,42)` de P′ (local, não é escrita);
     - **W4:** negativo 6014 em P′ (`settle --deliver --expect-error
       escrow:6014`) com a receipt do Job P do D7 (`d7/receipts/P`),
       declarada como tal (CR6);
     - **W5:** `settle --deliver --tamper-seal --expect-error verifier:6003`
       em P′;
     - **W6:** `settle --deliver` positivo de P′ → `Released`, com o
       verificador invocado;
     - **W7:** dupla liquidação em P′ (`--expect-error escrow:6007`);
     - **W8:** `job refund-timeout --wait` em T′ → `RefundedOnTimeout`.

     Nunca usar `escrow:6000` a `escrow:6003` (RD7-01). Sem airdrop nem
     transferência de SOL.
  2. **Ambiente limpo, opção (a) da CR4:**
     - clone novo do HEAD em `d9/clone`;
     - cópias novas das homes isoladas em `d9/homes`, offline;
     - targets novos, `--locked --offline`;
     - nunca o perfil padrão.

     Declarar no relatório que "limpo" significa clone e targets novos com
     toolchains isoladas copiadas, não uma máquina nova.
  3. **Sem mudança de código no D9.** As correções de RD7-01, 02, 03, 04 e
     07 ficam para o D10a, com testes e revisão curta.
  4. **O vídeo é gravado pelo humano** depois do D9, seguindo a seção
     "Gravação" do `docs/demo-script.md`. O agente não grava.

## Escopo autorizado
- **Documentação:** `README.md`, `docs/demo-script.md`, `prover/README.md`,
  `cli/README.md`, `docs/escrow-program.md`, `docs/manifest-schema.md` (só a
  errata da linha 70, em Plan Mode), `docs/decisions.md`, `docs/evidence.md`,
  `docs/agent-control.md`, `docs/project-context.md`,
  `docs/d9-demo-results.md`, `docs/handoffs/d9-to-d10a.md`.
- **Escritas em devnet:** só W1 a W8.
- **Docker local:** só `vericode-prover compress` via shim, com a imagem já
  presente.

## Fora de escopo / proibido
- Mudar `cli/`, `prover/` (código), o programa, o core, `zkvm/`, os locks, o
  guest ou o `JournalV1`.
- Deploy, airdrop, mainnet, Docker pull, instalação no perfil padrão,
  `/tmp`, rede fora de `api.devnet.solana.com`.
- Exibir ou copiar conteúdo de keypair; keypair dentro do clone.
- Claims além do CD7; apresentar evidência antiga como ao vivo; receipt
  pré-gerada sem dizer de qual Job é.
- Push.

## Implementação esperada (fases)
1. **F0:** preflight e checagem.
2. **F1, ambiente limpo:** clone, homes copiadas, build e testes `--locked
   --offline` (core 42/42 ×2, CLI 14/14, prover 4/4, suíte 61/61), um por
   vez.
3. **F2, CR1:**
   - erratas RD7-05 (a)–(d): `recursion_zkr.zip` e `RECURSION_SRC_PATH`;
     CLI do Agave 2.3.9; imagem Groth16 por digest presente antes; origem
     da receipt da cena 4a; `manifest-schema.md:70` (Plan Mode);
     `escrow-program.md:137`;
   - textos de RD7-04 ("testado offline") e RD7-02 (o que o shim garante de
     fato);
   - **lista de frases permitidas congelada** no README e no roteiro.
4. **F3, devnet:** W1 a W8 pelo roteiro, com CR2, CR3 (`RISC0_PROVER=local`;
   `BONSAI_*` e `RISC0_DEV_MODE` ausentes, conferidos e registrados), CR6 e
   CR7.
5. **F4, roteiro de gravação** em `docs/demo-script.md`, seção "Gravação",
   para o humano:
   - preparação do terminal: as variáveis e o `source` exatos, sem expor
     chaves;
   - comandos prontos para copiar e colar com um `job_id` novo, a saída
     esperada e o tempo de cada passo (com a prova de cerca de 2 min);
   - falas curtas por cena (problema, prova, limite);
   - quais links do Explorer abrir;
   - plano B com as transações do D9, rotuladas como "execução anterior".
6. **F5:** relatório, registros, handoff.

## Testes obrigatórios
- Antes de qualquer escrita, no ambiente limpo: core 42/42 ×2, CLI 14/14,
  prover 4/4 e suíte 61/61, todos `--locked`.
- Em cada escrita: simulação, assinatura, resultado na transação, snapshot
  (negativos), estado e saldo depois.
- Negativos: 6021 (T′), 6014 (P′), `verifier:6003` (P′), 6007 (P′).
- Positivos: `Released` de P′ com o verificador invocado; `RefundedOnTimeout`
  de T′.

## Evidências exigidas
- `docs/d9-demo-results.md` com:
  - ambiente usado (CR4 (a), declarado);
  - comandos e saídas reais;
  - links do Explorer;
  - `docker-shim.log`;
  - variáveis de CR3;
  - saldos que fecham.
- Atualizar `docs/decisions.md` (inclusive "Decisões humanas para o D9"),
  `docs/evidence.md`, `docs/agent-control.md`, `docs/project-context.md` e
  o README, se houver links novos.

## Critério de pronto
- P′ (PASS) e T′ (timeout) concluídos em devnet pela CLI, no ambiente limpo,
  com os quatro negativos.
- Claims congelados; erratas aplicadas; seção "Gravação" pronta.
- `git diff --check` 0; diff integral revisado; busca de segredos limpa;
  perfil padrão inalterado.

## Condições de parada
- Divergência de baseline, hash ou saldo → parar e reportar.
- Escrita fora de W1 a W8 → `AGUARDANDO_AUTORIZAÇÃO`.
- Qualquer mnemônico ou conteúdo de chave numa saída → parar e registrar o
  incidente.
- Falha de RPC persistente → parar com evidência parcial. Nunca apresentar
  execução anterior como nova.
- Prova com exit 137 → repetir uma vez, sozinha; se persistir, `BLOQUEADO`.

## Commit
- Commits locais autorizados pelo humano ao enviar este prompt, com
  identidade via `git -c`. Push proibido.
  - `docs: record D9 clean-environment demo and freeze claims`.
  - Em parada com evidência parcial: `docs: record partial D9 demo`, só com
    evidência real.

## Relatório final
1. arquivos modificados;
2. testes e saídas reais, com links;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`, salvo em
   `docs/handoffs/d9-to-d10a.md`. **D10a — endurecimento da CLI e do
   prover:**
   - RD7-01: `escrow:N` só quando a falha mais interna é do escrow;
   - RD7-02: shim com allowlist exata do argv;
   - RD7-03: `LocalProver` forçado, recusando `RISC0_PROVER` ≠ `local` e
     `BONSAI_*`;
   - RD7-04: testes de `--expect-error` e `--tamper-seal`;
   - RD7-07: `minContextSlot` e "already processed";
   - opcionais: RD7-06, 09 e 10.

   Tudo com testes, sem tocar no programa nem nos locks, seguido de uma
   revisão delta curta. Depois: D10 (worker), D11–D12 (telas finas, CR8) e
   a revisão final da interface.
