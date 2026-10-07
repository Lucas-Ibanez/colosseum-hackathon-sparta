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

Exportação dos tokens (o gerador e o teste de deriva entram no D11):

```bash
cd worker/ui-tools
npx --no-install design.md export --format css-vars --prefix hive ../../DESIGN.md   # cores, espaços, raios
npx --no-install design.md export --format dtcg ../../DESIGN.md                      # inclui a tipografia
```

Para recriar o navegador do Playwright: `npx --no-install playwright install --only-shell chromium`.
Se um dia o `sudo` estiver disponível, `sudo apt-get install -y libnss3 libnspr4 libasound2t64`
substitui a pasta `ui/syslibs/`.
