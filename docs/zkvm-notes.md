# Notas de zkVM

## Estado D0

A versão de `risc0-zkvm` e a toolchain RISC Zero ainda não estão pinadas. A documentação oficial consultada está marcada como versão 3.0 e a página de releases mostra `v3.0.6` como release estável mais recente consultada em 2026-09-28, mas isso não constitui decisão de pin.

Não há neste repositório evidência de build do guest, ImageID calculado ou receipt local válida.

## Versão de `risc0-zkvm`

- Versão pinada: pendente.
- Requisito do spike: escolher uma release estável suportada, alinhar `risc0-zkvm`, `risc0-build`, `cargo-risczero`/`rzup` e verificar avisos de segurança antes de gerar evidência.
- A versão não deve ser escolhida apenas por ser a mais recente; precisa ser compatível com o caminho Groth16/Router que vier a ser validado.

## Exemplo oficial escolhido

Referência inicial para o spike local: tutorial oficial **Building zkVM Hello World**, por demonstrar separação host/guest, entrada privada, commit no journal, geração e verificação de receipt. O exemplo será referência estrutural; a lógica do VeriCode continuará no core Rust puro e `Verdict::Fail` não usará panic.

## ImageID

- ImageID do VeriCode: não gerado.
- Gate: build determinístico do guest, registro do comando e do ImageID real.
- O ImageID identifica o guest e deve ser conferido ao verificar a receipt; não é substituto de `harness_hash` ou de Program ID Solana.

## Receipt PASS/FAIL

| Caso | Status | Evidência necessária |
| --- | --- | --- |
| `PASS` | NÃO EXECUTADO | Receipt real verificada localmente com journal `JournalV1` e ImageID esperado. |
| `FAIL` | NÃO EXECUTADO | Receipt real verificada localmente contendo `Verdict::Fail`, sem panic/assert. |

Dev mode não satisfaz esses gates de prova. Falha de execução não pode ser apresentada como receipt `FAIL`.

## Fontes oficiais consultadas

- [zkVM Quick Start](https://dev.risczero.com/api/zkvm/quickstart)
- [Building zkVM Hello World](https://dev.risczero.com/api/zkvm/tutorials/hello-world)
- [Receipts 101](https://dev.risczero.com/api/zkvm/receipts)
- [Terminologia: Image ID, Journal e Receipt](https://dev.risczero.com/terminology)
- [Releases do repositório risc0/risc0](https://github.com/risc0/risc0/releases)
