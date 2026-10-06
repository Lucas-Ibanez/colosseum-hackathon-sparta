# D10a — endurecimento da CLI e do prover (RD7-01, 02, 03, 04 e 07)

## Identificação
- Gate: `D10a`  ·  Dia da sequência: preparação de D10–D12  ·  Marcos do
  guia: M5/M6 (robustez do caminho por CLI) e M7 (claims iguais ao código)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D9`, commit `docs: record D9 clean-environment demo and
  freeze claims` sobre `9d3efa7`; relatório `docs/d9-demo-results.md`.
- Depois deste gate: revisão delta curta `R-D10a`, em sessão separada e
  somente leitura.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço high**. É código de
  cliente e de prover, sem efeito econômico on-chain e sem escrita em
  devnet. Mesmo assim, mexe no envio de transações, no rótulo dos negativos
  e na escolha do prover, que são superfícies de segurança. Se o humano
  incluir uma escrita em devnet (ver "Decisões a confirmar"), usar
  **xhigh**.
- **Plan Mode obrigatório** antes de alterar `cli/` ou `prover/`, com a
  lista de mudanças por achado, os testes novos e a confirmação de que
  nenhum lock muda.
- **Plan Mode também** antes de qualquer mudança em
  `docs/manifest-schema.md`, se a errata da linha 86 entrar.
- Prazo do MVP: 11/10. Registrar timestamps por fase.

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`.
- `docs/r-d7-review-results.md`: RD7-01 a 10, CR1 a CR8 e as PoCs da seção
  1.
- `docs/d9-demo-results.md` e `docs/decisions.md` (entradas D7, R-D7,
  "Decisões humanas para o D9" e D9).
- Código:
  - `cli/src/tx.rs`, `cli/src/rpc.rs`, `cli/src/main.rs`, `cli/src/keys.rs`;
  - `cli/tests/instructions.rs`;
  - `prover/src/lib.rs`, `prover/src/main.rs`, `prover/docker-shim/docker`,
    `prover/Cargo.toml`.
- Docs: `cli/README.md`, `prover/README.md`, `README.md` ("Frases
  permitidas", "Limitações"), `docs/demo-script.md`.
- Fora do clone, somente leitura: as PoCs do R-D7 em
  `~/.local/share/vericode-spikes/rd7/poc/` (`rpc/fake_rpc.py`,
  `shim/shim_poc.sh`, `shim/fake-docker`, `prover/prover_poc.sh`).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3`. O topo esperado é
  `docs: record D9 clean-environment demo and freeze claims` sobre `9d3efa7`.
  Se não for, parar.
- `git status --short` e `--ignored` vazios; `git diff --check` 0.
- Perfil padrão, no início e no fim:
  - `~/.rustup`, `~/.cache/solana` e `~/.config/solana` ausentes;
  - `find . -printf '%p %s %T@\n' | sort | sha256sum` dá `~/.cargo`
    `d9e12578…`, `~/.avm` `7d29f7f8…` e `~/.docker` `6046f67f…`.
- Raiz própria `~/.local/share/vericode-spikes/d10a/` (`0700`, logs `0600`).
- **Não tocar em `d9/`:** os binários e o `bin/env9.sh` de lá são os da
  seção "Gravação", que o humano usa para o vídeo de reserva.
- Scripts próprios com nomes prefixados. O `env.sh` herdado define `R`, `B`,
  `D` e `VC`; não carregá-lo nem reutilizar esses nomes. O `d9/bin/env9.sh`
  pode servir de modelo; copie-o, não o altere.
- Homes isoladas: copiar de `d9/homes`. A home `zkvm` de lá já tem todos os
  proxies do rustup, inclusive `rustdoc` (incidente do D9).
- **Memória (7,6 GiB):**
  - `free -m` antes de cada processo pesado;
  - um por vez, destacado (`setsid nohup`, `nice`, `CARGO_BUILD_JOBS=2`;
    prover até 4);
  - o build do prover levou 32 min no D9 e 57 min no R-D7;
  - a compressão Groth16 deixou 117 MB livres no D9.
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou
  stash.

## Checagem da tarefa anterior
- Locks:
  - raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`;
  - `anchor/` `19a1db26…`, `tests-local` `be94760a…`;
  - `cli` `4d979577…`, `prover` `8b76f1e1…`.
- Guest `prover/artifacts/vericode-guest.bin`: 180.300 B, `e09ba8cf…`.
- Devnet, somente leitura (`vericode check` e `job show`):
  - `check=ok`;
  - P′ `91ea6fcd…` `Released { 225384a7… }`;
  - T′ `ec9afb74…` `RefundedOnTimeout`;
  - vaults 0.
- Saldos esperados, se o humano ainda não gravou o vídeo:
  - SOL: deployer 2.805.214.240; buyer 121.288.800; executor 29.955.000
    lamports;
  - Test USDC: ATA do buyer 999.996.000.000; ATA do executor 4.000.000.
- Se o vídeo já tiver sido gravado, os saldos e Jobs a mais são escritas do
  humano: listar as assinaturas novas de buyer, executor e deployer
  (`getSignaturesForAddress`) e registrá-las como dele. Qualquer outra
  divergência: parar, registrar e reportar.

## Objetivo
Fechar RD7-01, 02, 03, 04 e 07 na CLI e no prover, com testes, sem mudar o
programa, o core, `zkvm/`, os locks, o guest nem o `JournalV1`. O worker e as
telas do D10–D12 devem herdar um cliente que rotula falhas corretamente e
prova só localmente.

## Decisões já tomadas
- Claim único (CD7) e **frases permitidas congeladas no D9**: `README.md`
  ("Frases permitidas (congeladas no D9)") e `docs/demo-script.md`. Mudar a
  lista exige decisão registrada. Fonte: `docs/decisions.md`, entrada D9.
- Escrow `GZqb…` imutável; verificador `THq1q…`; mint `9TE2V…`; `JournalV1`
  v1 congelado.
- Jobs consumidos, nunca reutilizar: S, A, B, C, P, T, P′ e T′, mais os da
  gravação do humano.
- Sem mudança de código no D9; as correções da RD7 vêm para este gate,
  seguidas de revisão delta curta. Fonte: "Decisões humanas para o D9", item
  3.
- CR8 continua valendo para o D10–D12.

## Decisões a confirmar ou pendentes
- **Escrita em devnet no D10a:** a recomendação é **nenhuma**; os testes são
  offline e a checagem em devnet é só leitura. Se o humano quiser um smoke
  em devnet da CLI endurecida, ele precisa de uma lista fechada (ex.: um Job
  novo com `escrow:6021` e refund por timeout) aprovada em Plan Mode. Sem
  isso: `AGUARDANDO_AUTORIZAÇÃO` para qualquer escrita.
- **Docker local:** um `prove` + `compress` local, com `job_id` aleatório e
  sem devnet, é necessário para validar o shim novo contra o argv real do
  `risc0-groth16 3.0.2`. Confirmar no Plan Mode.
- **Errata `docs/manifest-schema.md:86`** ("como o escrow terá a upgrade
  authority finalizada"): opcional neste gate, só em Plan Mode.

## Escopo autorizado
- `cli/src/*.rs` e `cli/tests/*`, sem dependência nova; `cli/Cargo.lock`
  inalterado.
- `prover/src/*.rs`, `prover/docker-shim/docker` e testes em `prover/`, sem
  dependência nova; `prover/Cargo.lock` inalterado.
- Documentação:
  - `cli/README.md`, `prover/README.md`, `README.md` ("Limitações"),
    `docs/demo-script.md` (notas da RD7);
  - `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`,
    `docs/project-context.md`;
  - `docs/d10a-hardening-results.md`, `docs/handoffs/d10a-to-r-d10a.md`;
  - opcional: errata de `docs/d7-cli-results.md:128` e
    `docs/manifest-schema.md:86` (Plan Mode).
- Rede: só `api.devnet.solana.com`, somente leitura, e `127.0.0.1` para RPC
  falso.
- Docker local: só `vericode-prover compress` pelo shim, com a imagem já
  presente.

## Fora de escopo / proibido
- Programa (`anchor/programs`), core, `zkvm/`, guest, `JournalV1`, schema e
  qualquer lock, inclusive `cli/Cargo.lock` e `prover/Cargo.lock`. Se uma
  correção exigir crate ou feature nova: parar e pedir decisão.
- Escrita em devnet sem lista aprovada; deploy; airdrop; mainnet.
- Docker pull; instalação no perfil padrão; `/tmp`.
- Rede além de devnet (só leitura) e `127.0.0.1`.
- Keypair novo; exibir ou copiar conteúdo de chave; keypair dentro do clone.
- Alterar `d9/`.
- Claims além da lista congelada.
- Push.

## Implementação esperada
- **RD7-01** (`cli/src/tx.rs`): `escrow:N` só passa quando a falha mais
  interna é do escrow, isto é, nenhum outro programa tem linha `Program <id>
  failed` na transação. `verifier:N` continua exigindo a linha do
  verificador.
- **RD7-02** (`prover/docker-shim/docker`):
  - allowlist exata do argv, remontado do zero;
  - `run --rm -v <caminho absoluto>:/mnt <tag>` vira `run --pull=never
    --network=none --rm -v <caminho>:/mnt <digest>`;
  - `image inspect --format {{.Id}} <digest>` (chamado pelo próprio prover);
  - qualquer outro argv é recusado;
  - antes, levantar o argv exato de todos os chamadores;
  - corrigir o comentário do cabeçalho e o `prover/README.md`.
- **RD7-03** (`prover/src/lib.rs:196,285`):
  - instanciar o prover local direto (`LocalProver`), sem `default_prover()`;
  - recusar antes de provar `RISC0_PROVER` diferente de `local` e qualquer
    `BONSAI_*`;
  - sem feature ou crate nova.
- **RD7-04** (`cli/`):
  - testes de `expected_failure`: código certo com linha certa; código
    certo com outro programa; sobreposição 6000–6003; sucesso inesperado;
  - teste do `--tamper-seal`: o seal muda só em `pi_c[10]` e a instrução
    continua igual aos builders, fora isso.
- **RD7-07** (`cli/src/rpc.rs`, `main.rs`):
  - leituras depois da transação com `minContextSlot` = slot da transação;
  - um "already processed" no envio ou reenvio é resolvido por
    `getSignatureStatuses`, nunca vira erro falso nem sucesso falso.
- **Opcionais:**
  - RD7-06: `nlink == 1` nas chaves; `set_permissions(0600)` no `--log`
    existente;
  - RD7-09: mensagem "past the deadline"; não imprimir `PASS` antes do erro
    de verificador não invocado;
  - RD7-10: exigir `https://`; conferir no `check` o hash do ProgramData do
    verificador (`34ae6e5c…`).
- Atualizar os textos das limitações (README, `cli/README.md`,
  `prover/README.md`, roteiro) para o que o código passa a garantir. A lista
  de frases permitidas não muda.

## Testes obrigatórios
- Um teste por achado corrigido, que **falhe no código do D9** e passe no
  novo. Registrar as duas saídas.
- Regressão, `--locked --offline`, homes isoladas copiadas, um processo
  pesado por vez:
  - core 42/42 ×2;
  - CLI 14 + novos;
  - suíte `anchor/tests-local` 61/61, com os `.so` de devnet
    (`cdf6967f…`/`34ae6e5c…`);
  - prover 4 + novos;
  - `vericode-prover check`.
- Prover:
  - `prove` local com `job_id` aleatório; `RISC0_PROVER=bonsai` e
    `BONSAI_API_URL` definidos → recusa antes de qualquer rede;
  - `compress` pelo shim novo com o argv real → `Groth16` e
    `docker-shim.log`;
  - shim com docker falso → todo argv fora da allowlist é recusado (os
    casos S1–S7 do R-D7).
- CLI contra RPC falso em `127.0.0.1` (modelo:
  `rd7/poc/rpc/fake_rpc.py`):
  - `overlap6003` e `overlap6000` com `escrow:N` → recusado;
  - com `verifier:6003` → aceito;
  - "already processed" → resolvido pelo status;
  - `flip`, `changed`, `wrongsig`, `notx` e `mainnet` continuam como no
    R-D7.
- Devnet, só leitura: `vericode check` e `job show` de P′ e T′ com o binário
  novo.

## Evidências exigidas
- Relatório `docs/d10a-hardening-results.md`:
  - comandos e saídas reais;
  - antes/depois de cada teste novo;
  - locks inalterados;
  - diff de código revisado.
- Atualizar `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`
  e `docs/project-context.md`.

## Critério de pronto
- RD7-01, 02, 03, 04 e 07 fechados com testes que falham antes e passam
  depois; nenhum lock alterado; regressão completa verde.
- Docs de limitação iguais ao comportamento do código; frases permitidas
  intactas.
- `git diff --check` 0; diff integral revisado; busca de segredos limpa;
  perfil padrão inalterado; `d9/` intocado.

## Condições de parada
- Divergência de baseline, hash ou saldo não explicada pela gravação do
  humano → parar e reportar.
- Correção que exija crate, feature ou lock novo → `AGUARDANDO_AUTORIZAÇÃO`.
- Qualquer escrita em devnet sem lista aprovada → `AGUARDANDO_AUTORIZAÇÃO`.
- Mnemônico ou conteúdo de chave numa saída → parar e registrar o incidente.
- Prova com exit 137 → repetir uma vez, sozinha; se persistir, `BLOQUEADO`.

## Commit
- Commit local só com autorização explícita do humano, com identidade via
  `git -c`. Sugestões:
  - `cli: harden negative matching, rpc reads and tests (D10a)`;
  - `prover: force local prover and exact docker shim argv (D10a)`;
  - `docs: record D10a hardening`.
- Push proibido.

## Relatório final
1. arquivos modificados;
2. testes e saídas reais (antes/depois);
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de fronteiras;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`, salvo em
   `docs/handoffs/d10a-to-r-d10a.md`:
   - **R-D10a:** revisão delta curta, somente leitura, Opus 5.5 high ou max,
     sessão separada, do diff do D10a contra RD7-01/02/03/04/07;
   - depois: D10 (worker com prova local forçada), D11–D12 (telas finas,
     CR8), revisão curta da interface, vídeo definitivo e submissão até
     11/10.
