# D13 — README, texto de submissão, pitch e cortes da demo técnica, em inglês

## Identificação
- Gate: `D13`  ·  Dia da sequência: submissão (10–11/10)  ·  Marcos do guia: M7 (claims
  iguais ao código e à evidência)
- Repositório: `/home/lucas/src/vericode` (WSL), branch `main`
- Gate anterior: `D12a`, commits `ba79080` (`worker: English interface and R-UI fixes
  (D12a)`) e `docs: record D12a`, sobre `9fa4b3b`; relatório `docs/d12a-results.md`.
- Decisões: `docs/decisions.md`, entradas "R-UI", "D12a" e "Decisões humanas depois do
  D12a".
- Prazo: submissão em 11/10.
  - O humano grava a tomada contínua em inglês (10/10).
  - Este gate pode começar antes da gravação, com os links dos vídeos como
    `TODO(video)`.

## Modelo e modo
- Modelo/esforço recomendados: **Opus 5.5, high**. O gate reescreve todo o texto público
  do projeto (README, submissão, pitch e narração da demo) e mexe numa frase congelada.
  Um erro de claim custa mais do que o tempo extra.
- **Plan Mode obrigatório**, porque o gate muda claims públicos. O plano traz:
  - a lista das seções do novo `README.md`;
  - o esboço do texto de submissão, do roteiro do pitch e da lista de cortes;
  - os campos que dependem do humano.

  Espere aprovação antes de editar.

## Leitura obrigatória (integral, antes de editar)
- `AGENTS.md`, `CLAUDE.md`, `docs/project-context.md`, `docs/handoff-protocol.md`,
  `docs/agent-control.md`.
- `README.md` inteiro: as frases em português, "Frozen phrases (English, ratified R-UI)",
  "Não dizer", limitações, versões e reprodução.
- `docs/demo-script.md` inteiro: frases, "Gravação pela interface" (a tomada contínua),
  "Plano B" e "Perguntas prováveis".
- `docs/decisions.md`: entradas "Vídeo de reserva", "Fundação da interface Hive (D11a)",
  "D11–D12", "R-UI", "D12a" e "Decisões humanas depois do D12a".
- Relatórios: `docs/r-ui-review-results.md` (seções 8 a 10), `docs/d12a-results.md`,
  `docs/d11-ui-results.md`, `docs/d9-demo-results.md` (frase 7),
  `docs/context/guia-mvp-agentes-de-codigo.md` (missão, ICP e escopo).
- `HIVE_MVP_UI_GUIDE.md` §2 (leitores e ICP) e §8 (vocabulário);
  `HIVE_MVP_UI_ADAPTATION.md` §6 (marca e frase 1); `brand/MANIFEST.md`, se algum texto
  ou slide usar logotipo.
- `worker/README.md`, `cli/README.md`, `prover/README.md`: só para conferir os
  identificadores e comandos citados.
- `worker/tests/test_static.py`: `README_SECTIONS` e `readme_section()` leem as frases
  do `README.md` por seção.
- Referência externa (para conferir, não para copiar):
  <https://blog.colosseum.com/perfecting-your-hackathon-submission/>.

## Preflight
- `pwd`; raiz Git; branch.
- `git log --oneline -3`: topo = `docs: record D12a`, depois `ba79080`, depois
  `9fa4b3b`.
- `git status --short` vazio; `--ignored` só `worker/ui-tools/node_modules/` e
  `brand/reference/*.pdf`; `git diff --check` 0.
- Perfil padrão no início e no fim:
  - `~/.rustup`, `~/.cache/solana`, `~/.config/solana` e `~/.npm` ausentes;
  - `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…`;
  - método: `cd <dir> && find . -printf '%p %s %T@\n' | sort | sha256sum`.
- Raiz própria `~/.local/share/vericode-spikes/d13/` (`0700`), com TMPDIR próprio.
  - O TMPDIR **não** pode ficar sob `/tmp`: o worker recusa `data_dir` em `/tmp`, e um
    teste falha.
  - Não altere as raízes anteriores.
  - Nunca use as variáveis `R`, `B`, `D` ou `VC`, nem `pkill -f`.
- Worker real em `127.0.0.1:8710`: não pare, não reinicie e não chame rota de escrita.
  Ele pode estar em uso pela gravação.
- Não abra o PDF de identidade (`brand/reference/*.pdf`).
- Preserve alterações existentes; sem reset, checkout destrutivo, clean ou stash.

## Checagem da tarefa anterior
- `TMPDIR=<d13/tmp> python3 -B -m unittest discover -s worker/tests -v`: **58/58**, sem
  `__pycache__`.
- `source ~/.local/share/vericode-spikes/ui/env-ui.sh` e depois
  `npm --prefix worker/ui-tools run lint:design` (0/0), `check:tokens` (`tokens ok`) e
  `check:assets` (`43 assets match the table`).
- Hashes:
  - worker `bbdf6dbe…`;
  - locks: raiz `191802b2…`, `zkvm` `f5236689…`, guest `1116acef…`, `anchor`
    `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover` `8b76f1e1…`,
    `ui-tools` `0c40c2ab…`;
  - guest `e09ba8cf…`.
- No código:
  - `LANG = "en"` em `worker/static/js/i18n.js`;
  - o `claim.f1` em inglês tem "Hive's" e é igual ao da seção inglesa do README.
- Se algo divergir: parar, registrar e reportar.

## Objetivo
Deixar prontos, em inglês, com a marca Hive, os identificadores legados exatos e só os
claims ratificados, para a submissão de 11/10:
- o `README.md` inteiro;
- o texto de submissão;
- o roteiro do pitch (até 3 min);
- a lista de cortes da demo técnica (2–3 min) a partir da tomada contínua.

O português fica preservado em `docs/README.pt-BR.md`.

## Decisões já tomadas
Em `docs/decisions.md`, salvo indicação:
- **Idioma:** vídeos e submissão em inglês. Frases e "Do not say" em inglês ratificados
  (D-EN-1, entrada "R-UI").
- **README:** o `README.md` passa a ser inteiro em inglês. O texto português atual vai,
  completo, para `docs/README.pt-BR.md`, que vira a fonte das frases portuguesas
  ("Decisões humanas depois do D12a").
- **Frase 1 portuguesa:** "A receipt Groth16 **da Hive**…", em vez de "do VeriCode". É
  só a palavra de marca: mesmo claim, mesma regra do link. Vale em
  `docs/README.pt-BR.md` e em `docs/demo-script.md` ("Decisões humanas depois do D12a").
- **Vídeos** ("Decisões humanas depois do D12a"):
  - **pitch de no máximo 3 min**: time, problema, para quem é, validação e visão. É um
    pitch de startup, não uma demo;
  - **demo técnica de 2–3 min**: um corte da tomada contínua, com cada corte e cada
    aceleração rotulados na tela;
  - a **tomada contínua de ~13 min** fica como evidência completa, sem cortes, linkada
    no README.
- **Marca** (`AGENTS.md`, bloco `hive-ui-core`):
  - o produto é **Hive**;
  - código, comandos, programas, erros, caminhos, crates, binários, variáveis de
    ambiente, o header e o nome do repositório **não** mudam e aparecem exatamente como
    existem (`vericode`, `vericode-prover`, `vericode_escrow`, `VericodeEscrowError`,
    `X-VeriCode-Token`, `VERICODE_*`, `vericode-core` etc.).
- **Slogan:** o oficial não é reescrito nem traduzido por conta própria (`DESIGN.md`,
  "Labels and vocabulary"; guia §8.5). A versão inglesa oficial não está no
  repositório: use `TODO(brand)` até o humano fornecer o texto.

## Decisões a confirmar ou pendentes (dados do humano; nada é inventado)
São campos a preencher. Não bloqueiam o gate: o texto sai com marcadores explícitos, e
o relatório final lista cada um.
- `TODO(video)`: links da tomada contínua, da demo técnica e do pitch.
- `TODO(form)`: campos, limites de caracteres e regras da edição atual da Colosseum.
  - Se o humano fornecer, o texto de submissão segue exatamente esses campos.
  - Se não, use seções genéricas: one-liner, problem, solution, how it works, what is
    proven, limitations, links.
- `TODO(team)`: nomes, papéis e histórico do time.
- `TODO(validation)`: feedback ou validação de usuários. **Só o que for real e vier do
  humano**; se não houver, a seção diz que ainda não há, sem enfeitar.
- `TODO(business)`: modelo de monetização. O guia de produto pode orientar o esboço,
  mas o texto final é do humano.
- `TODO(brand)`: o slogan oficial em inglês.
- Se o plano exigir mudar alguma frase ratificada ou algum arquivo fora do escopo:
  AGUARDANDO_AUTORIZAÇÃO.

## Escopo autorizado
- `README.md`: reescrito inteiro em inglês. Mantenha a seção "Frozen phrases (English,
  ratified R-UI)" com esse título, porque os testes e a adaptação a citam.
- `docs/README.pt-BR.md` (novo): o README português atual, com a frase 1 "da Hive", um
  aviso de que a versão de referência é a inglesa e os links relativos corrigidos para
  a nova pasta.
- `docs/demo-script.md`:
  - a frase 1 portuguesa "da Hive";
  - o link `../README.md#frases-permitidas-congeladas-no-d9`, que passa a apontar para
    `README.pt-BR.md`;
  - uma seção nova, "Demo técnica (corte de 2–3 min)", com a lista de cortes.
- `docs/pitch-script.md` (novo): roteiro do pitch em inglês.
- `docs/submission.md` (novo): texto de submissão em inglês.
- `worker/tests/test_static.py`, só a leitura das frases:
  - PT vem de `docs/README.pt-BR.md`, com igualdade direta da f1 "da Hive" (sai a troca
    "do VeriCode" → "da Hive");
  - EN vem do `README.md`.
- Documentos do gate: `docs/d13-results.md`, `docs/decisions.md`, `docs/evidence.md`,
  `docs/agent-control.md`, `docs/project-context.md`,
  `docs/handoffs/d13-to-<próximo>.md`.
- Fora do clone: `~/.local/share/vericode-spikes/d13/`.

## Fora de escopo / proibido
- Renomear qualquer coisa do código. Não tocar em `worker/static/`,
  `worker/vericode_worker.py`, `cli/`, `prover/`, `anchor/`, `crates/`, `zkvm/`, locks,
  `DESIGN.md`, `HIVE_MVP_UI_GUIDE.md`, `HIVE_MVP_UI_ADAPTATION.md` ou `brand/`.
- Mudar o sentido de qualquer frase ratificada ou acrescentar claim fora da lista.
- Apresentar visão ou roadmap como capacidade atual. Na visão do pitch, tudo o que não
  foi demonstrado aparece explicitamente como futuro, por exemplo "next", "roadmap" ou
  "we plan to".
- Termos proibidos: "Verifier Router" como caminho atual, "fallback", "trustless",
  "audited", "secure", "safe", "guaranteed", "certified", "real money", "any
  repository", "new machine", "verified agent", "reputation", "score", "trust level",
  "approved" como veredito, "mainnet" fora da frase 5 e das limitações que a negam, e
  "ZK on-chain" sem a frase 1.
- Apresentar uma execução anterior como ao vivo, cortar sem rótulo, usar links do
  Explorer fora de devnet ou mostrar uma receipt sem dizer de qual Job ela é.
- Inventar time, validação, métricas, usuários, parceiros, valores ou campos de
  formulário.
- Qualquer escrita em devnet; airdrop; rota de escrita do worker real; abrir arquivo de
  chave.
- Push (exige autorização separada); reescrita de histórico; stash; reset; `pkill -f`;
  builds pesados.

## Implementação esperada
1. **`README.md` em inglês:**
   - título "Hive", com uma nota curta: o repositório e o código usam o nome anterior
     `vericode`;
   - o que está demonstrado: a frase 1 com o link de uma liquidação real, e a tabela de
     casos em devnet com os mesmos links de hoje, todos com `?cluster=devnet`;
   - o que a prova não diz (frase 3);
   - "Frozen phrases (English, ratified R-UI)" e "Do not say";
   - escopo; versões, hashes e endereços;
   - reprodução a partir de um clone, com os comandos exatos e os nomes legados;
   - a interface Hive: `http://127.0.0.1:8710/ui/`, token na memória da aba, sem
     carteira no navegador (P3);
   - limitações: todas as atuais, inclusive P3, sem e-stop, regra v1 trivial, RD7-08,
     rent preso e mint authority;
   - vídeos (`TODO(video)`), estrutura, evidências e licença;
   - um link para `docs/README.pt-BR.md`.
2. **`docs/README.pt-BR.md`:** o português atual, f1 "da Hive", links corrigidos e um
   aviso no topo.
3. **`docs/pitch-script.md`:** até 3 min, cerca de 400 palavras faladas.
   - Partes: time (`TODO(team)`), problema, para quem é (ICP do guia §2), o que a Hive
     faz hoje (só frases ratificadas), validação (`TODO(validation)`), modelo de negócio
     (`TODO(business)`), visão (rotulada como futuro) e fechamento.
   - Um esboço de 5 a 7 slides, com a marca de `brand/` conforme o `brand/MANIFEST.md`.
   - Tempo estimado por parte.
4. **`docs/demo-script.md`, seção "Demo técnica (corte de 2–3 min)":**
   - **lista de cortes** sobre as cenas da tomada contínua: o que entra, a ordem, a
     duração alvo e o rótulo na tela ("cut", "accelerated");
   - **narração em inglês, que mostra:**
     - a stack: Rust core `no_std`, guest RISC Zero, Groth16 local por Docker, Anchor e
       CLI, worker e interface;
     - as decisões de arquitetura: CPI direta ao verificador imutável em vez do Router,
       `create_job` + `fund` atômico e `deliver` + liquidação na mesma transação;
     - a integração com Solana: vault PDA, CPI ao verificador e liquidação SPL na mesma
       instrução, com 99,541 CU no verificador;
     - pelo menos um negativo;
   - regras: a frase 1 sempre junto do link, a receipt do 6014 identificada, nenhuma
     execução anterior como ao vivo, e a tomada completa linkada.
5. **`docs/submission.md`:**
   - campos do formulário (`TODO(form)`) ou as seções genéricas;
   - cada claim técnico com um link de evidência: Explorer de devnet ou um relatório em
     `docs/`;
   - links dos vídeos (`TODO(video)`) e do repositório;
   - limitações resumidas;
   - nada além das frases ratificadas sobre o que foi provado.

## Testes obrigatórios
- `test_static.py`:
  - PT lido de `docs/README.pt-BR.md`, EN lido do `README.md`;
  - a f1 PT do dicionário igual à de `docs/README.pt-BR.md`, sem troca de marca no
    teste;
  - o teste falha se uma das duas seções sumir.
- Varredura de `README.md`, `docs/README.pt-BR.md`, `docs/pitch-script.md`,
  `docs/submission.md` e da seção nova do roteiro:
  - termos proibidos (lista acima e "Do not say");
  - "mainnet" só na frase 5 e nas limitações que a negam;
  - todo link do Explorer com `?cluster=devnet` e assinatura presente nos relatórios;
  - nenhum identificador legado renomeado (compare com a lista do `AGENTS.md`);
  - links relativos válidos: todo arquivo citado existe, e toda âncora citada existe.
- Conferir os comandos citados contra `cli/README.md`, `prover/README.md` e
  `worker/README.md`.
- `unittest` 58/58 ou mais; `lint:design`, `check:tokens` e `check:assets` verdes.
- Opcional, só leitura: `getSignatureStatuses` das assinaturas citadas em devnet.

## Evidências exigidas
- Relatório `docs/d13-results.md`:
  - comandos e saídas reais;
  - diff resumido;
  - as varreduras;
  - a lista de todos os `TODO(…)` restantes, com quem fornece cada um.
- Atualizar `docs/decisions.md`, `docs/evidence.md`, `docs/agent-control.md` e
  `docs/project-context.md`.

## Critério de pronto
- `README.md` em inglês e `docs/README.pt-BR.md` em português, com a f1 "da Hive".
- Texto de submissão, roteiro do pitch e lista de cortes em inglês.
- Só claims ratificados; links de devnet funcionais; os `TODO(…)` explícitos e
  listados.
- Testes e checks verdes; nenhum arquivo fora do escopo alterado.
- `git diff --check` 0; diff integral revisado; busca de segredos limpa; perfil e locks
  inalterados.

## Condições de parada
- **AGUARDANDO_AUTORIZAÇÃO** se:
  - uma frase ratificada precisar mudar;
  - for preciso tocar código, interface, worker ou os arquivos de UI/UX;
  - algum passo exigir escrita em devnet.
- **BLOQUEADO** se os testes do D10–D12a quebrarem por motivo fora do escopo.

## Commit
- Não autorizado por este prompt. Ao fim, peça autorização.
- Mensagem sugerida: `docs: English README, submission text and video scripts (D13)`.
- **Push:** o repositório precisa estar acessível aos jurados. O push exige autorização
  separada do humano. O relatório final lembra disso e lista os commits ainda não
  enviados (`git status -sb`).

## Relatório final
1. Arquivos modificados.
2. Testes e saídas reais.
3. Invariantes:
   - só claims ratificados;
   - "verified on-chain" só com a frase 1 e o link;
   - nenhum identificador renomeado;
   - nada sem fonte apresentado como real;
   - visão rotulada como futuro.
4. Decisões e dados pendentes: todos os `TODO(…)`.
5. Riscos.
6. Confirmação de fronteiras: nenhuma escrita em devnet, worker real intocado, perfil e
   locks iguais.
7. Prompt da próxima fase (fechamento da submissão: preencher os `TODO`, push autorizado
   e conferência final dos links), salvo em `docs/handoffs/` e reproduzido na resposta,
   segundo `docs/handoff-protocol.md`.
