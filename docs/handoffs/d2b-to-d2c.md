# D2c — Perfil A e programa Anchor local com Job e custódia SPL

## Identificação
- Gate: `D2c`  ·  Dias da sequência: D1 (Anchor skeleton), D2 (custódia SPL),
  D3 (create/fund/refund em teste local)  ·  Marcos do guia: preparação do M4
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D2b`, commit `core: align escrow policy with product guide (D2b)`,
  relatório `docs/d2b-escrow-guide-alignment-results.md`

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, esforço xhigh** — toolchain
  Anchor/Agave, contas, PDA, custódia SPL e autoridade.
- **Plan Mode obrigatório** (`CLAUDE.md`: `programs/` e integração on-chain).

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`,
  `docs/handoff-protocol.md`, `docs/agent-control.md`
- `docs/context/guia-mvp-agentes-de-codigo.md` (§4, §5, §7, §8, §11 e §12)
- `docs/escrow-state-machine.md`, `docs/d2b-escrow-guide-alignment-results.md`
- `docs/decisions.md` (D1a.1, D1a.2, D1a.3, D2a.2 e D2b),
  `docs/toolchain-matrix.md`, `docs/bootstrap-plan.md`,
  `docs/d1a2-spike-plan.md`, `docs/d1a3-spike-results.md`,
  `docs/router-notes.md`, `docs/zkvm-notes.md`
- `crates/vericode-core/src/escrow.rs`, `.gitignore`, `.env.example`

## Preflight
- `pwd`; raiz Git; branch; HEAD (esperado: commit D2b sobre `402426f`);
  `git status --short` (esperado: vazio); `git diff --check`.
- Confirmar que `/home/lucas/.rustup`, `/home/lucas/.cargo` e
  `/home/lucas/.cache/solana` continuam ausentes antes de qualquer comando.
- Preservar alterações existentes; sem reset, checkout destrutivo, clean ou
  stash.

## Checagem da tarefa anterior
- `git log --oneline -5`: D2b, `402426f`, `0622709`, `4d7e18f` e `a3eb9e0`.
- Reexecutar o core nas duas raias (comandos do D2b): esperado
  `test result: ok. 36 passed; 0 failed` em `+1.85.0` e `+1.89.0`.
- Hashes esperados:
  - `Cargo.lock` `191802b234a6aa0f6bb9ce58a61963c377aed435a2baa6dec9576d13d8283b87`;
  - `zkvm/Cargo.lock` `f52366893cfb3024c5643041e70bf773063781b319fdbe2f5c0e5fa37340e226`;
  - `zkvm/methods/guest/Cargo.lock` `1116acef90aa4a1cddb74cae0ba9c03c92b825de478b9d0d2ac7d3d31656dbfa`;
  - `crates/vericode-core/src/escrow.rs` `8ea483d207552cded42869c243327516f466ac1bc4f3b8c764d70ba1ad4a1711`;
  - `crates/vericode-core/src/lib.rs` `c51a810f058d3d9e8042f6261708e1bea574f54c71088454a28ead253dbcc372`.
- Se algo divergir: parar, registrar e reportar.

## Objetivo
1. Escolher, por evidência, o Perfil A Anchor/Agave/Rust e desbloquear o D1b.
2. Criar um programa Anchor local que persiste o Job, custodia Test USDC mock
   num vault controlado por PDA e implementa `create_job`, `fund` e
   `refund_on_timeout` aplicando a política de `vericode-core`, com testes
   locais.

## Decisões já tomadas
- A escolha do Perfil A é do agente (D2a.2). Critério:
  - combinação que compila SBF o exemplo `counter` de
    `risc0-solana v3.0.0` (commit `ee415935…`) com locks preservados e
    também o programa VeriCode;
  - se ambas as raias passarem, preferir a fidelidade à CI do Router
    (Agave `2.3.9`);
  - registrar o motivo.
- O agente pode criar keypairs efêmeros de localnet/devnet, fora do clone,
  `0600`, sem exibir segredos (`AGENTS.md` §9). Neste gate: somente
  localnet/in-process.
- Política econômica: `docs/escrow-state-machine.md` (D2b). O programa aplica
  `JobV1::new`, `fund` e `refund_on_timeout` do core e não duplica a regra.
- O slot vem do sysvar `Clock` on-chain; o core continua sem relógio.

## Decisões a confirmar no Plan Mode
- Localização do workspace Anchor. Recomendação: workspace separado em
  `anchor/`, com `anchor/Anchor.toml`,
  `anchor/programs/vericode-escrow` e lock próprio, consumindo o core por
  path. O lock raiz e os locks de `zkvm/` não mudam.
- Seeds e layout do Job e do vault (proposta documentada antes de codar).
- Framework de teste local:
  - preferir testes Rust em processo (LiteSVM ou `solana-program-test`),
    com versão compatível com o Agave escolhido;
  - alternativa: `solana-test-validator` só em `127.0.0.1`;
  - não instalar Node/yarn.

## Rede e instalação autorizadas (somente estas)
- Download das platform-tools pelo `cargo-build-sbf` da versão pinada,
  apenas da release oficial `anza-xyz/platform-tools`, registrando URL,
  versão e SHA-256.
- `index.crates.io` e `static.crates.io` para resolver e baixar dependências
  do novo workspace Anchor (versões exatas: `anchor-lang`/`anchor-spl`
  `0.31.1` e crates de teste escolhidas) e para o `counter` com `--locked`.
- Tudo dentro de homes isoladas em `~/.local/share/vericode-spikes/d2c/`.
  Usar `HOME`/caches isolados para que nada seja escrito em `~/.cache/solana`,
  `~/.cargo` ou `~/.rustup`; confirmar o local de cache na fonte/`--help` da
  versão pinada.
- Ferramentas já instaladas (não reinstalar):
  - raia A: `d1a3/homes/lane-a`, com Rust `1.85.0`, AVM/Anchor `0.31.1` e
    Agave `2.1.0` em `d1a3/tools/lane-a/solana-release`;
  - raia B: `d1a3/homes/lane-b`, com Rust `1.89.0`, AVM/Anchor e Agave
    `2.3.9` em `d1a3/tools/lane-b/solana-release`;
  - checkouts em `d1a3/checkouts`.

## Escopo autorizado
- Criar `anchor/` (ou o local aprovado no Plan Mode) com o programa
  `vericode-escrow`, seus testes e lock próprio.
- Declarar `declare_id!` com a chave pública de um keypair de programa criado
  fora do clone. A chave pública de localnet pode ser versionada; não é
  deploy.
- Criar:
  - `docs/escrow-program.md` (contas, seeds, instruções, erros, mapeamento
    para o core);
  - `docs/d2c-anchor-local-escrow-results.md`;
  - `docs/handoffs/d2c-to-<próximo>.md`.
- Atualizar `docs/toolchain-matrix.md`, `docs/bootstrap-plan.md`,
  `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md`,
  `docs/project-context.md`, `docs/architecture.md` e, se a capacidade for
  comprovada, o README.

## Fora de escopo / proibido
- Devnet, RPC remoto, airdrop, deploy, transação em rede pública.
- Router, CPI, verificação de receipt/Groth16 e instruções `release` e
  `refund_on_fail`: ficam para o gate seguinte, que decidirá Router ou
  fallback atestado.
- Alterar `crates/vericode-core`, `JournalV1`, wire format, lock raiz, `zkvm/`
  ou `docs/context/*`. Se o core não compilar para SBF: parar.
- Instruções administrativas, upgrade authority usada como bypass, destino de
  token livre.
- Keypair dentro do clone (inclusive `target/deploy`), seed ou chave privada
  exibida, `.env` real.
- Docker, `curl | sh`, Node/yarn, push.

## Implementação esperada
- Perfil A escolhido com tabela de evidência:
  - build SBF do `counter` e do programa VeriCode;
  - versões, comandos, exit codes;
  - SHA-256 dos `.so` e das platform-tools.
- Programa:
  - `create_job`:
    - buyer assina;
    - valida os termos pelo `JobV1::new`;
    - cria a conta do Job em PDA e o vault (token account do mint do Job,
      autoridade = PDA).
  - `fund`:
    - buyer assina;
    - transfere exatamente `amount` da token account do buyer para o vault;
    - aplica `JobV1::fund`.
  - `refund_on_timeout`:
    - lê o `Clock`;
    - aplica `JobV1::refund_on_timeout`;
    - transfere todo o vault para a token account do buyer do Job, assinando
      com a PDA;
    - persiste `Refunded { Timeout }` na mesma instrução.
- Validação de contas: mint, owner/autoridade das token accounts, PDA e
  seeds, signer.

## Testes obrigatórios (locais)
- `create_job` válido; termos inválidos rejeitados (identidade zero,
  `buyer == executor`, amount zero).
- `fund` exato muda saldo e estado; mint divergente, signer diferente do
  buyer, amount divergente e duplo funding rejeitados sem movimento de tokens.
- Vault controlado por PDA, sem chave privada (invariante 1).
- Timeout:
  - antes do prazo e no slot do prazo falha sem mover tokens;
  - após o prazo devolve exatamente `amount` ao buyer e marca `Refunded`.
- Refund para token account de outro dono ou de outro mint: rejeitado.
- Refund duplicado rejeitado (invariante 9).
- Estado e saldo inalterados em toda instrução rejeitada (invariante 6 na
  parte local).
- A IDL contém somente as instruções previstas; nenhuma é administrativa.
- Core continua com 36/36 nas duas raias; locks existentes inalterados.

## Evidências exigidas
- `docs/d2c-anchor-local-escrow-results.md` com:
  - inventário de ferramentas e decisão do Perfil A com evidência;
  - downloads (URL, versão, SHA-256);
  - comandos e saídas reais;
  - SHA-256 do `.so`, do lock novo e dos locks existentes;
  - matriz de testes × invariantes;
  - riscos e fronteiras.
- Entradas D2c em `docs/decisions.md` e `docs/evidence.md`.
- Toolchain matrix e bootstrap plan atualizados (D1b desbloqueado ou motivo
  do bloqueio).

## Critério de pronto
- Perfil A registrado com evidência executada.
- Programa compila SBF no Perfil A e todos os testes obrigatórios passam.
- Nenhum segredo no Git:
  - `git status --short --ignored` revisado;
  - busca de keypair, seed e `.env`;
  - keypairs somente fora do clone, com `0600`.
- `git diff --check` exit `0`; diff integral revisado.
- Nada escrito em `~/.cargo`, `~/.rustup` ou `~/.cache/solana`.

## Condições de parada
- Nenhuma raia compila o `counter` com locks preservados → `BLOQUEADO`, com
  diagnóstico reproduzível.
- `vericode-core` não compila para SBF ou exige alteração → parar e pedir
  decisão.
- Necessidade de rede, ferramenta ou versão fora do autorizado → parar.
- API, contas ou formato não confirmáveis na versão pinada → parar; não usar
  pseudocódigo como integração.

## Commit
- Commits locais autorizados ao final, somente se o critério de pronto for
  atendido. Separar, se fizer sentido:
  - `docs: choose Anchor/Agave profile (D2c)`;
  - `anchor: add local escrow program with SPL custody (D2c)`.
- Usar a identidade dos commits anteriores via `git -c`. Push proibido.
- Revisão adversarial separada (guia §11): recomendar uma sessão somente
  leitura (Opus 5.5, esforço max) sobre autoridade, contas, PDA, destino de
  token e timeout. Se não for executada, registrá-la como pendente, junto com
  a do D2b.

## Relatório final
1. arquivos modificados;
2. testes e saídas reais;
3. invariantes;
4. decisões pendentes;
5. riscos;
6. confirmação de que devnet, Router, CPI, deploy e push não foram usados;
7. prompt da próxima fase, segundo `docs/handoff-protocol.md`.

O próximo gate provável:
- decide o caminho de verificação (Router/CPI ou fallback atestado
  rotulado);
- implementa `release` e `refund_on_fail` no programa.
