# Arquitetura do MVP

## Estado D1c2a

A primeira implementação existe em `crates/vericode-core`. É uma crate Rust
pura que modela `Hash32`, `JobId`, `ImageId`, `Verdict`, os compromissos
esperados e `JournalV1`. D1c2a acrescenta um wire format Borsh `0.10.4`
candidato, compromissos SHA-256 por `sha2 0.10.9` e um harness determinístico
para um único registro de desenvolvimento restrito.

O registro contém somente versão, entrada `u32` e saída alegada `u32`. O
harness aplica a especificação fixa `saída = entrada * 2`, sem I/O, e trata
`PASS` e `FAIL` como resultados normais. Entrada malformada, versão
incompatível e valor fora do limite retornam erro explícito.

A crate não contém lógica Solana/Anchor/RISC Zero e não autoriza pagamentos.
Borsh e SHA-2 são bibliotecas Rust puras fixadas pelo lock; não introduzem SDK
de blockchain. Desde 2026-10-05 o formato é o **`JournalV1` v1 congelado**
(165 bytes, `docs/manifest-schema.md`), sem nenhum byte alterado desde o
D1c2a.

## Estado D2a/D2b

O módulo `crates/vericode-core/src/escrow.rs` contém a política pura de
escrow (D2a, alinhada ao guia de produto no D2b): identidades de 32 bytes
para buyer, executor e mint, `Amount` não nulo, `JobV1` imutável com
`deadline_slot`, estados `Created`/`Funded`/`Released`/`Refunded`, erros
explícitos e três liquidações. `Pass` vinculado ao Job libera ao executor até
o prazo; `Fail` vinculado devolve ao buyer em qualquer slot; timeout devolve
ao buyer somente após o prazo. O `artifact_hash` era registrado na liquidação
(substituído no D2b.1 pelo compromisso de entrega, abaixo).
A política é um predicado puro: não custodia fundos, não transfere tokens,
não verifica receipt/prova e recebe o slot como entrada. Detalhes em
[`docs/escrow-state-machine.md`](escrow-state-machine.md).

## Estado D2c

O workspace `anchor/` contém o programa local `vericode_escrow`
(Anchor `0.31.1`, Agave `2.3.9`):
- `create_job` persiste os termos imutáveis do Job no PDA `["job", job_id]` e
  cria o vault PDA `["vault", job]`, controlado pelo Job;
- `fund` deposita exatamente o valor do Job;
- `refund_on_timeout`, permissionless, devolve ao buyer após o prazo.

Toda regra econômica é delegada ao `vericode-core`. O programa foi testado em
processo (`solana-program-test 2.3.9`) e nunca implantado. Ainda não existem
`release`, `refund_on_fail` nem verificação de prova. Especificação em
[`docs/escrow-program.md`](escrow-program.md).

## Estado D2b.1

A revisão adversarial R-D2 mostrou que, sem compromisso de entrega, não
existe "o" artefato do Job: qualquer pessoa provava um FAIL de artefato
arbitrário para qualquer `job_id`. O D2b.1 vincula a liquidação à entrega:

- `deliver(artifact_hash)`, assinado só pelo executor, uma única vez, até o
  prazo, grava `Delivered { artifact_hash }`;
- `release` e `refund_on_fail` (core) exigem o journal desse artefato;
- o timeout devolve ao buyer a partir de `Funded` ou `Delivered`;
- `create_job` só admite a spec, o harness e o ImageID da v1, e um prazo na
  janela de 1.500 a 1.512.000 slots.

"Artefato vinculado ao Job" passa a significar: o artefato cujo hash o
executor comprometeu com `deliver`. Isso está testado no core e no programa
local; a verificação da receipt on-chain continua pendente (D2e).

## Estado D2e

`release` e `refund_on_fail` existem no programa local. Cada uma, na mesma
instrução:
1. decodifica o journal de 165 bytes;
2. aplica o core (Job, artefato entregue, veredito, prazo e destinatário);
3. exige a ATA canônica da parte paga;
4. exige o selector `73c457ba`;
5. chama o Verifier Router por CPI com `SHA-256(journal)` e o `image_id` do
   Job;
6. só então transfere.

O Router e o verificador são os de `risc0-solana v3.0.0`. Isso foi testado
em `solana-program-test` local com as receipts Groth16 reais. É
"verificado por CPI ao Router em `solana-program-test` local", não "ZK
on-chain": Router em devnet e deploy continuam `STATUS: NÃO VALIDADO`.
Detalhes em [`docs/escrow-program.md`](escrow-program.md).
O Router saiu do caminho no D4a (abaixo).

## Estado D4a

O reconhecimento somente leitura do devnet mostrou que o Verifier Router
upstream está implantado e imutável, mas não inicializado, e que o
verificador Groth16 upstream (`THq1q…`) é imutável e nunca poderá ser
registrado nele. Por decisão humana, o escrow passou a:
- chamar por CPI o verificador Groth16 direto, com `SHA-256(journal)` e o
  `image_id` do Job, sem Router;
- aceitar só o mint Test USDC admitido (`ADMITTED_MINT`, erro 6036).

O `JournalV1` v1 foi congelado como estava. Tudo segue testado só em
`solana-program-test`, agora também com os bytes do verificador implantado
em devnet. O deploy e as transações em devnet ficam para o D4b, depois da
revisão R-D4a. Detalhes em
[`docs/d4a-direct-verifier-results.md`](d4a-direct-verifier-results.md).

## Estado D4b/D7

- **D4b:** o escrow do D4a está implantado em devnet
  (`GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`, 395.064 bytes
  `cdf6967f…`) com upgrade authority `none`. O mint Test USDC admitido
  (`9TE2V…`) existe com 6 decimais e sem freeze. A receipt é **verificada em
  devnet por CPI ao verificador Groth16 imutável de risc0-solana v3.0.0**
  (`docs/d4b-devnet-results.md`).
- **D7:** o caminho de ponta a ponta passou a ser reproduzível a partir do
  repositório, com duas peças fora dos workspaces existentes:
  - `prover/` (`vericode-prover`): prova o guest admitido, versionado em
    `prover/artifacts/vericode-guest.bin` (`e09ba8cf…`, ImageID
    `4da06f90…`), com o prover local do RISC Zero 3.0.3; comprime para
    Groth16 pelo Docker local por digest, sem rede e sem pull. Não recompila
    o guest.
  - `cli/` (`vericode`): monta as instruções a partir da IDL do D4a e dos
    compromissos do `vericode-core`, confere cluster, programa, mint, termos,
    receipt e estado antes de enviar, simula, envia de forma cadenciada e
    confirma. Não contém regra econômica: quem decide é o programa.
- Workspaces e locks: `prover/Cargo.lock` foi semeado do lock do harness do
  D4a; `cli/Cargo.lock`, de `anchor/tests-local/Cargo.lock`. Nenhum lock
  existente mudou. Os detalhes estão em `docs/d7-cli-results.md`.

| Componente | Alvo | Workspace / lock | Papel |
| --- | --- | --- | --- |
| `crates/vericode-core` | host, guest (`no_std`) e SBF | raiz | regra, journal, hashes e política de escrow |
| `zkvm/` | guest RISC-V e host | `zkvm/` | build determinístico do guest (D1c2b); não muda no D7 |
| `anchor/programs/vericode-escrow` | SBF | `anchor/` | custódia e liquidação; imutável em devnet |
| `anchor/tests-local` | host | próprio | suíte em processo, com o verificador real |
| `prover/` | host | próprio | receipts `Composite` → `Groth16` do guest admitido |
| `cli/` | host | próprio | cliente de devnet |
| `worker/` (D10) | host | nenhum (Python 3.12, só biblioteca padrão) | worker local HTTP sobre a CLI e o prover |

## Estado D10 (`worker-api`)

O módulo `worker-api/` do guia é `worker/` (`vericode_worker.py`), decisões P1–P3 e
condições C10-1 a C10-10 do R-D10a:
- servidor HTTP só em `127.0.0.1`, com `Host` conferido, token por execução em header
  próprio em toda rota `/api/*` e nenhum cabeçalho CORS;
- cada ação é uma lista de argv fixa de `vericode` ou `vericode-prover` (só os binários
  do D10a, com hash conferido), sem shell, com ambiente construído do zero, uma por vez;
- as chaves do buyer e do executor ficam em arquivos que só a CLI abre, por caminho; a
  do deployer nunca entra; nenhuma resposta ou log tem caminho ou conteúdo de chave;
- o estado vem do `job show` e do `--log` da CLI; toda escrita termina com `job show`;
- a receipt só é usada com o shim conferido antes do `compress` e com exatamente uma
  linha `docker_run` desta execução depois (C10-2);
- `Proving` é etapa local do executor; `Submitted` e `Failed` são estados do worker;
  nenhuma transição econômica fora do programa.

Em devnet, as escritas W1–W7 do D10 foram dirigidas pela API do worker
(`docs/d10-worker-results.md`). Não há carteira no navegador (P3): as telas do D11–D12
chamam o worker.

## Fluxo

`Buyer cria Job (termos da v1) e deposita Test USDC` -> `Executor entrega e compromete o hash do artefato (deliver)` -> `host executa o harness e gera receipt Groth16` -> `contrato valida journal contra o Job e a entrega e verifica a prova pelo verificador Groth16 (CPI)` -> `release ao executor em PASS válido ou refund conforme as regras do Job`

O fluxo completo roda em `solana-program-test` (D2e, D4a) e em devnet (D4b,
com ferramentas fora do clone; D7, com `vericode` e `vericode-prover` do
repositório):

```text
vericode job create      -> create_job + fund (buyer; job_id aleatório)
vericode-prover prove    -> receipt Composite do guest admitido (local)
vericode-prover compress -> receipt Groth16 (Docker local por digest)
vericode job settle      -> deliver + release (PASS) ou refund_on_fail (FAIL),
                            com CPI ao verificador Groth16 na mesma instrução
vericode job refund-timeout -> refund_on_timeout depois do prazo
```

Desde o D10, o worker local executa esses mesmos comandos por argv fixo, atrás de uma
API em `127.0.0.1` (`worker/README.md`). A UI ainda não existe (D11–D12).

## Componentes e responsabilidades

| Componente | Responsabilidade | Não deve fazer |
| --- | --- | --- |
| Core Rust puro | Tipos de domínio, bytes candidatos, hashes, regra determinística, validação do artefato e `Verdict`; política pura de escrow (D2a) | Depender de Solana, Anchor ou RISC Zero; custodiar fundos, transferir tokens, verificar prova ou deter autoridade de release |
| Guest RISC Zero | Ler entrada restrita, chamar o core e publicar `JournalV1` com `PASS` ou `FAIL` | Tratar `FAIL` como panic/assert ou expor dados privados desnecessários |
| Host e `prover/` | Preparar entrada, executar/provar, obter receipt e conferir journal localmente | Ser fonte de verdade para liberar fundos |
| CLI (`cli/`) | Montar, conferir, simular, enviar e confirmar as instruções do escrow em devnet | Decidir veredito, escolher destino ou guardar chaves dentro do clone |
| Programa Anchor (`anchor/programs/vericode-escrow`) | Manter Job/escrow, custodiar no vault PDA e liquidar somente pelo core após validações do Job | Reexecutar regra de negócio, aceitar admin bypass ou confiar apenas no host |
| Verificador Groth16 (`risc0-solana v3.0.0`) | Verificar receipt Groth16 por CPI direta do escrow (D4a em `solana-program-test`; em devnet desde o D4b) | Ser chamado de "Verifier Router" ou ter o claim estendido a mainnet |
| Worker local (`worker/`, D10) | Orquestrar CLI e prover por argv fixo, uma operação por vez, e expor estado reconciliado com a cadeia | Decidir pagamento, repetir escrita sozinho, aceitar caminho/flag do cliente ou devolver caminho ou conteúdo de chave |
| Front-end | Criar e consultar Jobs e apresentar estados/transações | Decidir verdict ou custodiar segredos do usuário |

## Fronteiras

- O core é a única implementação da regra de negócio e deve compilar como Rust puro.
- Host e futuro guest devem chamar a mesma API do core e comparar os vetores
  candidatos byte a byte; nenhuma cópia paralela da regra é autorizada.
- Guest e programa Anchor são adaptadores separados para alvos incompatíveis; nenhum importa APIs do outro.
- O host é não confiável para a decisão final: ele transporta artefato, receipt e journal.
- O contrato valida identidade do Job, destinatário, mint, compromissos críticos e, quando comprovado, a prova.
- O front-end nunca substitui validação do contrato.

## Fora do escopo deliberado

- Repositórios arbitrários ou patches gerais.
- Testes que dependem de rede durante a avaliação do artefato.
- Marketplace de tarefas.
- Sistema de reputação.
- Juiz baseado em LLM.
