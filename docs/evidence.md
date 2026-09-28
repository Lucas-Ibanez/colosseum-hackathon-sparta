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
