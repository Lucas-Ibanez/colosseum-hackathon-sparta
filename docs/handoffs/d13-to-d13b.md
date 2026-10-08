# D13b — fechamento da submissão: dados do humano, push autorizado e conferência final

## Identificação
- Gate: `D13b`  ·  Dia da sequência: submissão (10–11/10)  ·  Marcos do guia: M7 (claims
  iguais ao código e à evidência)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D13`, commit `docs: English README, submission text and video scripts
  (D13)` sobre `7ada476` (`docs: record D12a`), se o humano o autorizou; relatório
  `docs/d13-results.md`.
- Decisões: `docs/decisions.md`, entradas "R-UI", "Decisões humanas depois do D12a" e "D13".
- Prazo: submissão em 11/10. A tomada contínua é gravada pelo humano em 10/10; a demo
  técnica e o pitch são montados por ele a partir dos roteiros.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, high**. O gate preenche textos públicos com
  dados do humano e pede push; um claim a mais ou um link quebrado custa mais do que o
  tempo extra.
- **Plan Mode obrigatório**: mostra, antes de editar, cada `TODO(…)` com o texto exato que
  entra no lugar e a fonte (o humano), e a lista de links a conferir. Espere aprovação.

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`, `docs/handoff-protocol.md`,
  `docs/agent-control.md`.
- `docs/d13-results.md` inteiro (seção "TODO restantes").
- `README.md`, `docs/README.pt-BR.md`, `docs/submission.md`, `docs/pitch-script.md` e a
  seção "Demo técnica (corte de 2–3 min)" de `docs/demo-script.md`.
- `docs/decisions.md`: entradas "R-UI", "Decisões humanas depois do D12a" e "D13".
- `worker/tests/test_static.py` (`README_SECTIONS`, `readme_section()`).
- `brand/MANIFEST.md`, se o humano trouxer um asset novo (slogan, logotipo).

## Preflight
- `pwd`; raiz Git; branch; `git log --oneline -3` (topo = commit do D13, depois
  `7ada476`, se o D13 foi commitado; senão, a árvore do D13 sem commit: parar e perguntar).
- `git status --short` vazio; `--ignored` só `worker/ui-tools/node_modules/` e
  `brand/reference/*.pdf`; `git diff --check` 0; `git status -sb` (commits à frente de
  `origin/main`).
- Perfil padrão no início e no fim: `~/.rustup`, `~/.cache/solana`, `~/.config/solana` e
  `~/.npm` ausentes; `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…`
  (`cd <dir> && find . -printf '%p %s %T@\n' | sort | sha256sum`).
- Raiz própria `~/.local/share/vericode-spikes/d13b/` (`0700`) com TMPDIR próprio (não sob
  `/tmp`). Nunca use `R`, `B`, `D` ou `VC`, nem `pkill -f`.
- Worker real em `127.0.0.1:8710`: o agente não o inicia, não o para e não chama rota de
  escrita. Ele parou com o reinício do WSL em 2026-10-08 07:43; quem o inicia é o humano,
  para a gravação (C-UI-5).
- Não abra o PDF de identidade. Sem reset, checkout destrutivo, clean ou stash.

## Checagem da tarefa anterior
- `TMPDIR=<d13b/tmp> python3 -B -m unittest discover -s worker/tests -v`: **58/58**, sem
  `__pycache__`.
- `source ~/.local/share/vericode-spikes/ui/env-ui.sh` e `npm --prefix worker/ui-tools run
  lint:design` (0/0), `check:tokens` (`tokens ok`) e `check:assets` (`43 assets match the
  table`).
- Hashes: worker `bbdf6dbe…`; locks raiz `191802b2…`, `zkvm` `f5236689…`, guest
  `1116acef…`, `anchor` `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`,
  `prover` `8b76f1e1…`, `ui-tools` `0c40c2ab…`; guest `e09ba8cf…`.
- `python3 -B -I ~/.local/share/vericode-spikes/d13/bin/scan13.py /home/lucas/src/vericode`:
  `problems: 0`.
- Se algo divergir: parar, registrar e reportar.

## Objetivo
Fechar a submissão de 11/10: substituir cada `TODO(…)` dos textos públicos pelo dado real
que o humano fornecer (ou pela frase de ausência já prevista), conferir todos os links e,
com autorização separada, fazer o push.

## Decisões já tomadas
- Textos públicos em inglês, marca Hive, identificadores legados exatos; só as 8 frases
  ratificadas como claims (D-EN-1; `README.md`, "Frozen phrases (English, ratified R-UI)").
- Vídeos: pitch ≤ 3 min; demo técnica de 2–3 min cortada da tomada contínua, com cortes e
  acelerações rotulados; tomada contínua completa linkada ("Decisões humanas depois do
  D12a").
- Validação: só a real, fornecida pelo humano; sem ela, "No user validation yet".
- Slogan: só o texto oficial fornecido pelo humano; nunca traduzido pelo agente.

## Decisões a confirmar ou pendentes (dados do humano)
- `TODO(video)`: links do pitch, da demo técnica e da tomada contínua, e a data da tomada
  (`README.md` "Videos", `docs/submission.md` "Links", cartões da demo).
- `TODO(form)`: campos, limites e regras da edição atual; o `docs/submission.md` é
  reorganizado por campo, cortado ao limite sem claim novo.
- `TODO(team)`, `TODO(validation)`, `TODO(business)`: textos do humano (pitch e submissão).
- `TODO(brand)`: slogan oficial em inglês (pitch).
- `TODO(repo)`: **resolvido na sessão de registro (2026-10-08).** O repositório
  `https://github.com/Lucas-Ibanez/colosseum-hackathon-sparta` é público: a página
  respondeu HTTP 200 sem login (`curl`). No D13b, troque o `TODO(repo)` de
  `docs/submission.md` por essa constatação e repita a checagem antes da submissão.
- **Push do D13:** autorizado pelo humano em 2026-10-08, mas o `git push` da sessão de
  registro falhou porque o shell do agente não tem credenciais do GitHub (`could not read
  Username`). O humano faz o push do próprio terminal. No preflight, confira `git status
  -sb` e `git ls-remote origin main`, e não configure credenciais.
- **Push:** só com autorização separada e explícita do humano. Sem ela:
  AGUARDANDO_AUTORIZAÇÃO.
- Dado que mude o sentido de uma frase ratificada ou peça claim novo:
  AGUARDANDO_AUTORIZAÇÃO.

## Escopo autorizado
- `README.md`, `docs/README.pt-BR.md` (só se um dado também valer para o português),
  `docs/submission.md`, `docs/pitch-script.md`, a seção "Demo técnica (corte de 2–3 min)"
  de `docs/demo-script.md` (tempos e rótulos reais depois da edição).
- Documentos do gate: `docs/d13b-results.md`, `docs/decisions.md`, `docs/evidence.md`,
  `docs/agent-control.md`, `docs/project-context.md`, `docs/handoffs/d13b-to-<próximo>.md`.
- Fora do clone: `~/.local/share/vericode-spikes/d13b/`.

## Fora de escopo / proibido
- Código, interface, worker (`worker/static/`, `worker/vericode_worker.py`), `cli/`,
  `prover/`, `anchor/`, `crates/`, `zkvm/`, locks, `DESIGN.md`, `HIVE_MVP_UI_GUIDE.md`,
  `HIVE_MVP_UI_ADAPTATION.md`, `brand/`. Renomear qualquer identificador.
- Inventar time, validação, métricas, usuários, parceiros, valores, campos de formulário ou
  slogan. Visão como capacidade atual.
- Termos da lista "Do not say" e do guia §8.3; "mainnet" fora da F5 e das limitações que a
  negam; "verified on-chain" fora da F1 com o link.
- Escrita em devnet, airdrop, rota de escrita do worker real, abrir arquivo de chave.
- Push sem autorização separada; reescrita de histórico; force push; stash; reset;
  `pkill -f`; builds pesados.

## Implementação esperada
1. Substituir cada `TODO(…)` pelo texto do humano, citando a fonte no relatório. Os que
   ficarem sem dado continuam com a frase de ausência prevista e entram na lista de riscos.
2. Se o humano trouxer os campos do formulário, reorganizar `docs/submission.md` por campo,
   com a contagem de caracteres de cada um.
3. Atualizar a lista de cortes com os tempos reais da edição, se o humano os fornecer.
4. Conferir que cada link dos vídeos abre (HTTP 200 ou página do serviço) sem login, se
   público.
5. Com autorização de push: `git push origin main` (sem `--force`), depois `git status -sb`
   e `git ls-remote origin main` iguais ao HEAD local.

## Testes obrigatórios
- `unittest` 58/58; `lint:design`, `check:tokens`, `check:assets` verdes.
- `scan13.py` (copiado para `d13b/bin/` se precisar mudar): 0 problemas, com
  `--rpc` opcional (só leitura).
- Nenhum `TODO(…)` restante nos textos públicos sem estar listado no relatório.
- `git diff --check` 0; diff integral revisado; busca de segredos limpa.

## Evidências exigidas
- Relatório `docs/d13b-results.md`: comandos e saídas reais, cada `TODO` preenchido com a
  fonte, os que ficaram, links conferidos, saída do push (se autorizado).
- Atualizar `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md` e
  `docs/project-context.md`.

## Critério de pronto
- Textos públicos sem `TODO(…)` ou com os restantes explicitamente aceitos pelo humano.
- Só claims ratificados; links de devnet e dos vídeos funcionais.
- Testes e varreduras verdes; nenhum arquivo fora do escopo alterado; perfil e locks
  inalterados.
- Push feito com autorização, ou AGUARDANDO_AUTORIZAÇÃO registrado.

## Condições de parada
- **AGUARDANDO_AUTORIZAÇÃO:** push; dado que mude uma frase ratificada; arquivo fora do
  escopo.
- **BLOQUEADO:** testes do D10–D13 quebrados por motivo fora do escopo; link de vídeo que
  não abre.

## Commit
- Não autorizado por este prompt. Ao fim, peça autorização.
- Mensagem sugerida: `docs: fill submission data and final links (D13b)`.
- **Push:** exige autorização separada do humano; o relatório lista os commits ainda não
  enviados (`git status -sb`).

## Relatório final
1. Arquivos modificados.
2. Testes e saídas reais.
3. Invariantes: só claims ratificados; "verified on-chain" só com a F1 e o link; nenhum
   identificador renomeado; nada sem fonte apresentado como real; visão rotulada como
   futuro.
4. Decisões e dados pendentes.
5. Riscos.
6. Fronteiras: nenhuma escrita em devnet, worker real intocado, perfil e locks iguais.
7. Prompt da próxima fase, salvo em `docs/handoffs/` e reproduzido na resposta, segundo
   `docs/handoff-protocol.md`.
