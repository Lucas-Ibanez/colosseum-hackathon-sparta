# `worker/ui-tools/` — ferramentas de desenvolvimento da interface Hive

Ferramentas usadas **só para desenvolver** a interface servida pelo worker
(`worker/static/`). Nada daqui é servido pelo worker nem roda no caminho do MVP:
a interface em execução é HTML, CSS e JavaScript sem framework nem dependência
(decisão "Fundação da interface Hive", `docs/decisions.md`;
`HIVE_MVP_UI_ADAPTATION.md`, seção 7).

| Pacote (versão exata) | Licença | Para quê |
| --- | --- | --- |
| `@google/design.md` 0.4.0 | (repositório google-labs-code/design.md) | lint do `DESIGN.md` e exportação dos tokens (`css-vars`, `dtcg`) |
| `lucide-static` 1.52.0 | ISC | origem dos ícones de linha (Lucide), copiados um a um para `worker/static/icons/` |
| `playwright` 1.63.0 | Apache-2.0 | capturas para a verificação visual que o `DESIGN.md` pede |

`package-lock.json`: 96 pacotes, todos de `https://registry.npmjs.org/`, nenhum de fonte
git. A instalação usa `--ignore-scripts`.

## Ambiente (fora do clone, isolado do perfil padrão)

Node.js 24 LTS (v24.21.0, SHA-256 conferido com o `SHASUMS256.txt` oficial),
cache do npm, navegador do Playwright e as bibliotecas de sistema que faltavam ao
Chromium ficam em `~/.local/share/vericode-spikes/ui/`:

```bash
source ~/.local/share/vericode-spikes/ui/env-ui.sh   # só variáveis VU_*, PATH, npm_config_*, PLAYWRIGHT_*, LD_LIBRARY_PATH
```

| Item | Onde |
| --- | --- |
| Node e npm | `ui/node-v24.21.0-linux-x64/` |
| Cache do npm | `ui/npm-cache/` |
| Chromium headless do Playwright (153.0.8010.12) | `ui/ms-playwright/` |
| `libnss3`, `libnspr4`, `libasound2t64` (sem `sudo`: `apt-get download` + `dpkg -x`) | `ui/syslibs/` |

Nada foi instalado em `/usr`, `~/.npm`, `~/.cargo`, `~/.avm` ou `~/.docker`.

## Comandos

```bash
source ~/.local/share/vericode-spikes/ui/env-ui.sh
npm ci --ignore-scripts --prefix worker/ui-tools        # recria node_modules a partir do lock
npm --prefix worker/ui-tools run lint:design            # design.md lint DESIGN.md
```

Desde o D11–D12:

```bash
npm --prefix worker/ui-tools run gen:tokens     # DESIGN.md -> worker/static/css/tokens.css
npm --prefix worker/ui-tools run check:tokens   # regenera em memória e compara byte a byte (deriva = exit 1)
npm --prefix worker/ui-tools run sync:assets    # copia a tabela static-assets.json e confere cada SHA-256
npm --prefix worker/ui-tools run check:assets   # só confere destino e origem contra a tabela
VU_WORKER_TOKEN=<token> node worker/ui-tools/capture.mjs <dir-fora-do-clone> jobs='#/jobs' [--blocks] [--dark] [--reduced-motion] [--width=1280]
```

- **`gen-tokens.mjs`**: quatro camadas, todas do `DESIGN.md`: (1) a saída de
  `design.md export --format css-vars --prefix hive` (cores, espaços, raios), sem
  mudança; (2) tipografia e dimensões dos componentes, do modelo que a API de lint do
  pacote resolve; (3) os papéis do "Mapa de papéis" em `:root` (claro, canônico) e
  `[data-theme="dark"]` (preparado); (4) os valores que o `DESIGN.md` dá em prosa
  (sombra de nível 2, scrim, misturas de hover/pressionado, 120/200 ms, `68ch`,
  tamanhos de ícone e de `PartyMark`, altura de linha de tabela, afastamento do foco),
  cada um com a seção de origem, e a largura mínima da assinatura (`brand/MANIFEST.md`).
  O cabeçalho grava o SHA-256 do `DESIGN.md`; `worker/tests/test_static.py` confere esse
  hash e as cores sem precisar de Node.
- **`sync-assets.mjs` e `static-assets.json`**: tabela fixa (destino, origem, SHA-256)
  das 43 cópias: 6 fontes, `fonts.css` e 2 licenças OFL de `brand/fonts/`, o logotipo
  `brand/logo/hive-horizontal-branco.svg`, 32 ícones de `lucide-static` e a licença ISC.
  Sem listagem nem glob. `--pin` só na primeira cópia.
- **`capture.mjs`**: capturas para a verificação visual; só lê a tela (não clica em
  ações que escrevem). O token vem do ambiente, é digitado no campo de senha e nunca é
  impresso; erros de console, de página e de CSP saem no fim (exit 1).

Para recriar o navegador do Playwright: `npx --no-install playwright install --only-shell chromium`.
Se um dia o `sudo` estiver disponível, `sudo apt-get install -y libnss3 libnspr4 libasound2t64`
substitui a pasta `ui/syslibs/`.
