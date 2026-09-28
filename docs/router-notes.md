# Notas do Router Solana

**STATUS: NÃO VALIDADO**

**Decisão D0: pendente de spike**

## Objetivo

Verificar uma receipt RISC Zero Groth16 por CPI em Solana e, somente após verificar a prova e os campos críticos do journal contra o Job, permitir que o programa Anchor considere o release.

## Evidência oficial consultada

- Repositório oficial consultado: `risc0/risc0-solana`, atualmente redirecionado para `boundless-xyz/risc0-solana`.
- Versão/tag consultada: release `v3.0.0`, marcada como a primeira versão totalmente auditada com suporte a RISC0 zkVM 3.0.
- Exemplo de referência encontrado: `examples/counter` no repositório oficial. Ele é apenas candidato para o spike de CPI; não foi clonado, compilado nem executado neste D0.
- O README oficial descreve Verifier Router, verificação Groth16 on-chain e um exemplo de integração.

## Redes e deployments encontrados

Nenhum Program ID ou deployment do Router em Solana devnet ou mainnet foi comprovado nas fontes oficiais consultadas. O link “list of verifier router deployments” do README oficial resolveu, durante esta pesquisa, para uma página de contratos verificadores EVM e não forneceu uma lista de deployments Solana. Endereços EVM não são evidência de deployment Solana e não foram copiados para a configuração do projeto.

Consequências:

- `RISC0_VERIFIER_ROUTER_PROGRAM_ID` permanece vazio.
- Não alegar verificação ZK on-chain.
- Não implementar CPI antes de confirmar Program ID, cluster, interface/IDL, compatibilidade de versões e uma transação de teste reproduzível.
- Qualquer atestado da plataforma deve ser rotulado como fallback, separado de prova ZK.

## Gate do spike

1. Fixar uma tag/revisão oficial compatível com a versão zkVM escolhida.
2. Inspecionar o exemplo `examples/counter` e a interface do Router na mesma revisão.
3. Confirmar em fonte oficial o Program ID e o cluster Solana.
4. Executar CPI em ambiente controlado e registrar transação/log real.
5. Validar receipt Groth16, ImageID e digest do journal; testar rejeição para Job, ImageID e journal divergentes.

## Fontes oficiais consultadas

- [Repositório risc0-solana](https://github.com/boundless-xyz/risc0-solana)
- [Release risc0-solana v3.0.0](https://github.com/boundless-xyz/risc0-solana/releases/tag/v3.0.0)
- [Exemplo counter](https://github.com/boundless-xyz/risc0-solana/tree/v3.0.0/examples/counter)
- [Documentação RISC Zero de contratos verificadores](https://dev.risczero.com/api/blockchain-integration/contracts/verifier)
