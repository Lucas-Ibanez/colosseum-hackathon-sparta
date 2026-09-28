# Evidências

| Gate | Comando | Resultado | Link/log | Responsável |
| --- | --- | --- | --- | --- |
| Estado inicial | `git status --short` | limpo; apenas aviso de permissão no exclude global do Git | saída da execução D0 | Codex |
| Ambiente base | informações de runtime do PowerShell | Microsoft Windows `10.0.26200`, x64; PowerShell `7.6.5` | saída da execução D0 | Codex |
| Ferramentas presentes | comandos `--version` | Git `2.53.0.windows.2`; Node `v24.15.0`; npm `11.12.1`; pnpm `11.5.2` | saída da execução D0 | Codex |
| Ferramentas ausentes | comandos `--version` | `rustc`, `cargo`, `rustup`, `solana`, `anchor`, `docker` e `rzup`: `NOT_FOUND` | saída da execução D0 | Codex |
| WSL2 | `wsl.exe --status`; `wsl.exe --list --verbose` | subsistema não instalado; exits `50` e `1` | saída da execução D0 | Codex |
| Higiene do diff | `git diff --check` | exit `0` | saída da execução D0 | Codex |
| Segredos/artefatos locais | `git check-ignore -v --no-index` | `CLAUDE.local.md`, `.env` e keypair em `target/deploy/` ignorados; exit `0` | `.gitignore` | Codex |
| Arquivos compartilhados | `git check-ignore -q` | AGENTS, CLAUDE, `.env.example` e todos os documentos não ignorados; exit `1` esperado | `.gitignore` | Codex |
| D1a — matriz e bootstrap | pesquisa oficial; `git diff --check`; `git status --short` | matriz/plano registrados; `git diff --check` exit `0`; alterações somente em `docs/`; nenhum toolchain instalado | `docs/toolchain-matrix.md`; `docs/bootstrap-plan.md` | Codex |
| D1a.1 — correção Anchor/Agave | leitura da release Anchor e do workflow/manifests/locks dos tags exatos; `git diff --check`; `git status --short`; presença das ferramentas | Anchor 0.31.x recomenda Agave `2.1.0`; workflow `risc0-solana v3.0.0` usa `2.3.9`; interseção não demonstrada; **SEM PERFIL PRONTO PARA INSTALAÇÃO**; `git diff --check` exit `0`; somente `docs/` alterado; toolchains críticas continuam `NOT_FOUND`; Node permanece `v24.15.0` | `docs/toolchain-matrix.md`; `docs/bootstrap-plan.md`; `docs/zkvm-notes.md`; `docs/router-notes.md`; saída D1a.1 | Codex |
| D1b0.1 — WSL base e clone canônico | `wsl.exe --status`; `wsl.exe --list --verbose`; `/etc/os-release`; `uname -m`; `dpkg-query -W`; `git clone`; validações Git/filesystem | WSL2 com Ubuntu `24.04.5 LTS` x86_64; seis pacotes-base validados; clone `~/src/vericode` em ext4, branch `main`, commit `7aaf40b`, status inicial limpo; `git diff --check` exit `0`; somente três documentos alterados; nenhuma toolchain de produto instalada | saída real D1b0.1; `docs/bootstrap-plan.md`; `docs/decisions.md` | Codex |
