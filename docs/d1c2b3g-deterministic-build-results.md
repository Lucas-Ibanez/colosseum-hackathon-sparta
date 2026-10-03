# D1c2b.3g — dois builds determinísticos do guest

Data: 2026-10-02

## Decisão

**GO** para executar receipts VeriCode locais reais somente após commit e
auditoria deste gate.

Dois contextos exportados independentemente do mesmo commit auditado geraram
vendors próprios da união exata dos locks, usaram targets distintos e
executaram builds efetivos do pacote isolado `vericode-methods`. O ELF guest
e o método combinado com o kernel são idênticos byte a byte entre A e B. Os
dois ImageIDs emitidos pelo build script e os dois ImageIDs recalculados com
`r0vm` coincidem.

## Baseline e isolamento

- clone: `/home/lucas/src/vericode`, branch `main`;
- commit exportado:
  `be23e0194ae416c56bda4fac3aaf45db705cf81d`;
- archives A e B: SHA-256
  `749e694d212869530aa444f5b0aec03d8101dc40d81132a63babdad7a5831f15`;
- contextos:
  - A: `/tmp/vericode-d1c2b3g-A.CzTHjq/src`;
  - B: `/tmp/vericode-d1c2b3g-B.XbDCgq/src`;
- targets host/guest:
  - A: `/tmp/vericode-d1c2b3g-target-A.33rjvK`;
  - B: `/tmp/vericode-d1c2b3g-target-B.4CbD5k`;
- nenhum contexto contém `.git`;
- nonces exclusivos, não consumidos pelas fontes:
  - A: `2f5cc7c95b2332ed34c2db0532a814fb8c337af1d22eb88b4749a08332c61bc7`;
  - B: `275cc0635d3bf562e87d896f59c00c5049be69a4e7104acaa19bbfc75d774f5f`.

Os nonces diferenciam o `COPY` integral dos dois contextos Docker e impedem
reaproveitamento do layer de compilação entre A e B. O build B recompilou o
grafo host em target vazio e levou 1 min 40 s; A levou 36,13 s depois da
compilação host inicial. Nunca houve duas escritas simultâneas.

## Vendor independente A/B

Cada contexto executou, separadamente e com todas as homes prescritas:

```text
cargo +1.89.0 vendor --locked --offline \
  --manifest-path zkvm/Cargo.toml \
  --sync zkvm/methods/guest/Cargo.toml vendor
```

Resultado em cada vendor:

| Medida | A | B |
| --- | ---: | ---: |
| Pacotes esperados na união | 461 | 461 |
| Crates/checksums | 461 | 461 |
| Arquivos regulares | 23.021 | 23.021 |
| Arquivos declarados relidos | 22.560 | 22.560 |
| Bytes | 451.175.743 | 451.175.743 |
| Ausentes/extras/divergentes | 0/0/0 | 0/0/0 |
| SHA-256 dos paths canônicos | `a0c64b2e…34a9f` | `a0c64b2e…34a9f` |
| SHA-256 do inventário de conteúdo | `61355ebb…f686` | `61355ebb…f686` |

Hashes completos:

- paths:
  `a0c64b2ead0c885c43d0ce5a9687a6817d57c1b258463edf9ac8a27b98034a9f`;
- conteúdo:
  `61355ebb50d4babba46c95b45a04ac586716737083e636808d19708fed7af686`;
- configuração raiz A/B:
  `77e9219c27274120197571fd165cbe4121963b5ad3bc0b20b383c86ef0ce6c2b`.

Metadata locked/offline resolveu, em ambos os contextos, 458/458 pacotes
registry do workspace host e 154/154 do guest por paths dentro do vendor.

Arquivos `.pem`, `.key` e nomes `keypair` encontrados pertencem a fixtures
checksummed de crates públicas (`rsa`, `pkcs*`, `pem-rfc7468`, `spki`,
`ring`, `rzup`, `untrusted` e fontes como `web-sys`); nenhum veio do clone ou
da home do usuário. As dez configurações Cargo internas foram pesquisadas e
não contêm substituição de fonte, registry, token, credencial, proxy ou rede.

## Builder local fixado

- imagem:
  `risczero/risc0-guest-builder:r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3`;
- Image ID Docker e RepoDigest:
  `sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3`;
- entrypoint: `[/bin/sh]`;
- Docker client/server: `29.8.1`/`29.8.1`;
- `risc0-build 3.0.3` não passa `--pull` ao `docker build`; o digest já
  estava presente localmente;
- o Dockerfile gerado executa apenas fetch/build Cargo e recebeu
  `CARGO_NET_OFFLINE=true` do `build.rs` versionado;
- essa API não expõe `--network none`; nenhuma consulta/download de rede foi
  solicitada ou observada, mas a ausência de isolamento de socket de rede
  permanece uma limitação explícita do builder upstream.

## Comandos de build

Em cada contexto, sequencialmente, foi executado o mesmo comando com apenas
`CARGO_TARGET_DIR` diferente:

```text
env \
  CARGO_HOME=/home/lucas/.local/share/vericode-spikes/d1c2b/cargo \
  RUSTUP_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/rustup \
  RISC0_HOME=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/risc0 \
  PATH=/home/lucas/.local/share/vericode-spikes/d1a3/homes/zkvm/cargo/bin:/usr/bin:/bin \
  RUSTUP_AUTO_UPDATE=0 \
  CARGO_NET_OFFLINE=true \
  RISC0_DOCKER_CONTAINER_TAG=r0.1.88.0@sha256:3e12f71bacd27527a61dea96fa0e53e468c99aa261d3a1019b593f6dbd943eb3 \
  CARGO_TARGET_DIR=<target-A-ou-B> \
  cargo +1.89.0 build --release --locked --offline \
    --manifest-path zkvm/Cargo.toml -p vericode-methods
```

A primeira invocação A dentro do sandbox compilou as dependências host, mas
o build script foi impedido de acessar `/var/run/docker.sock`; terminou com
exit `101` antes de qualquer build Docker. O mesmo comando, autorizado para
o socket local, terminou com exit `0`. B foi iniciado somente depois do
término e auditoria do contexto A e terminou com exit `0`.

## Comparação dos artefatos

O builder produz o ELF do programa e, depois, `risc0-build` o combina com o
kernel V1 compatível no arquivo `.bin` incorporado por `methods.rs`.

| Propriedade | Build A | Build B | Comparação |
| --- | --- | --- | --- |
| ELF guest, bytes | 147.880 | 147.880 | `cmp`: idêntico |
| ELF guest, SHA-256 | `3fc668dfd97db52343df267c2148aa49181c813a426a9549782743e09fc6c42d` | igual | igual |
| Método combinado, bytes | 180.304 | 180.304 | `cmp`: idêntico |
| Método combinado, SHA-256 | `5c3c82c43e804d119ed4bc0cc57734f27786911d566762e282c5c029124988bc` | igual | igual |
| Array emitido | `[3831410741, 799025126, 3005513567, 4204460368, 3637343564, 242046379, 188276875, 1103064922]` | igual | igual |
| ImageID `r0vm` | `35b05ee4e627a02f5f7f24b350f99afa4c75cdd8ab556d0e8be0380b5a6fbf41` | igual | igual |

O ELF é ELF32 little-endian, executável RISC-V estático, entry point
`0x2058e8`. `r0vm --id --elf <método-combinado>` foi executado separadamente
para A e B com as homes prescritas. Uma primeira chamada diagnóstica usou o
path como argumento posicional e foi rejeitada com exit `2`; `--help`
confirmou a sintaxe correta, e as duas chamadas válidas terminaram com exit
`0`. O ImageID em bytes é exatamente a codificação little-endian das oito
palavras emitidas pelo build script.

## Limites e próxima transição

- Os contextos, vendors, targets e artefatos existem somente em `/tmp`.
- Ainda não foi produzido ou verificado receipt VeriCode neste gate.
- A incompatibilidade transitiva já observada no lock host pode bloquear o
  próximo gate; nenhum lock foi alterado por inferência.
- Não houve dev mode, Bonsai, Solana, Anchor, wallet, validator, Router/CPI,
  rede blockchain, deploy, front-end ou push.
- Router/CPI/devnet permanecem `STATUS: NÃO VALIDADO`.

A próxima transição permitida é registrar/auditar este gate e criar commit
local limitado. Depois, executar o host real com o método A ou B, produzir e
verificar receipts locais de PASS e FAIL e executar os negativos exigidos,
sem promover `Verdict::Fail` a erro operacional.
