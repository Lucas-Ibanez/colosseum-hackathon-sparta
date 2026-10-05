# Guest admitido do VeriCode — proveniência

`vericode-guest.bin` é o **método combinado** (ELF do guest com o kernel
RISC Zero) do guest determinístico do D1c2b. É o único guest que um Job v1
admite: o ImageID dele é a constante `ADMITTED_IMAGE_ID_V1` do programa
`vericode_escrow`. O arquivo é público e não contém segredo.

| Item | Valor |
| --- | --- |
| Arquivo | `vericode-guest.bin`, 180.300 bytes |
| SHA-256 | `e09ba8cf16f7e36cb92e10656b7574c598f3e88fe089f02bc747a4bb00c078f5` |
| ImageID (`compute_image_id`, `r0vm --id`) | `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a` |
| ELF do guest contido | 147.876 bytes, SHA-256 `63fac491fdd8935141850b3a3423934c28e988f2db0746bbaa23726740215408` |
| Toolchain | `risc0-zkvm`/`risc0-build 3.0.3`; guest Rust `1.88.0` na imagem `risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3` |
| Fonte | `zkvm/methods/guest` e `crates/vericode-core` da árvore do commit `dc41168` (lock guest `1116acef…`, lock host `f5236689…`) |

## Como foi produzido (D1c2b.3h)

1. Dois `git archive` independentes do mesmo HEAD, cada um com vendor offline
   da união dos locks (467 crates) e target próprios.
2. Dois builds `cargo risczero`/`risc0-build` determinísticos, com a imagem
   fixada por digest e Cargo offline.
3. ELF e método combinado idênticos por `cmp` nos dois builds; dois `r0vm
   --id` deram o mesmo ImageID.

Relatórios: `docs/d1c2b3h-host-lock-and-final-build-results.md` e
`docs/d1c2b3j-final-audit.md`. A cópia versionada aqui veio de
`~/.local/share/vericode-spikes/d2d/artifacts/vericode-guest.bin` (conferida
em D2d, D4a e D7 pelo mesmo SHA-256).

## Limites

- **ImageID não recertificado.** Depois do commit `dc41168`, o core só ganhou
  o módulo `escrow` (política on-chain), que o guest não usa; o caminho de
  `evaluate_restricted_artifact` não mudou. Mesmo assim, o guest não foi
  reconstruído desde então. Recompilar o guest ou recertificar o ImageID fica
  fora do D7.
- O `prover` confere, antes de qualquer operação, o tamanho, o SHA-256 e o
  ImageID deste arquivo, e aborta se algum divergir.
- `.gitattributes` marca `*.bin` como binário: o Git nunca converte fim de
  linha neste arquivo.
