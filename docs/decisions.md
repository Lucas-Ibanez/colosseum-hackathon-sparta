# Decisões

Registre decisões relevantes do projeto neste formato.

## Template

- **Data:**
- **Decisão:**
- **Motivo:**
- **Evidência:**
- **Risco aberto:**

## 2026-09-28 — D0: fronteiras do MVP

- **Data:** 2026-09-28
- **Decisão:** restringir a avaliação a artefato serializado, core Rust puro e `JournalV1` mínimo; manter integração Router como não validada.
- **Motivo:** separar a regra determinística dos alvos incompatíveis do guest e Anchor e impedir alegações sem evidência.
- **Evidência:** `docs/architecture.md`, `docs/manifest-schema.md`, `docs/zkvm-notes.md` e `docs/router-notes.md`.
- **Risco aberto:** versão zkVM, serialização do journal, ImageID, Program ID/cluster do Router e CPI ainda dependem de spikes com saída real.
