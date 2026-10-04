<!--
Fonte: documento fornecido pelo responsável humano em 2026-10-04
(`sequencia_mvp.md`). Conteúdo preservado sem alteração abaixo deste
comentário. É plano, não evidência: o estado real de cada dia está em
`docs/project-context.md` e nos relatórios de gate.
-->

# Cronograma

| Dia | Construir / decidir | Evidência de saída |
|-----|--------------------|--------------------|
| **D0 - 26 set** | Leitura operacional, sem código de produto: exemplos oficiais RISC Zero, README e exemplos do risc0-solana. Fixar versões, mapear contas/instrução e criar um repositório com comandos de build. Conferir inscrição, elegibilidade da Trilha Brasil e requisitos de ambas submissões. Disparar 5-10 mensagens curtas de outreach e abrir o roteiro de pitch. | Nota de compatibilidade; checklist administrativo com dono; lista de design partners; pitch de 5 frases. |
| **D1 - 27 set** | Fechar o contrato de escopo: um job Rust, inputs, saída e testes puros. A inicia Crate compartilhado e guest mínimo. B cria Anchor skeleton e modela estados. C define as quatro telas e agenda conversas. Criar manifesto do job e registrar a promessa honesta no README. | Manifesto exemplo; diagrama de estados; repositório compila; outreach enviado; roteiro v0. |
| **D2 - 28 set** | A produz uma receipt real de uma função trivial, com ImageID e journal. B implementa/ensaiar custódia SPL e testes unitários de estados. C grava um take de 30-60 segundos explicando problema, prova e limite, sem esperar a UI. | GATE 48H: receipt local verificada e comando reproduzível. Sem isso, não se inicia UI. |
| **D3 - 29 set** | A faz o journal carregar job_id, hash do artefato, hash do harness, versão e PASS/FAIL. B termina create/fund/refund em teste local e estuda a integração do Router contra o exemplo exato escolhido. C faz follow-up de outreach e recolhe objeções. | Provas PASS e FAIL; mapa de contas e API do Router; escrow testável; registro de respostas de mercado. |
| **D4 - 30 set** | B coloca escrow em devnet com Test USDC e confirma que só as transições autorizadas movem saldo. A congela o formato do journal e documenta campos públicos. C prepara conteúdo da UI e uma página curta de arquitetura. | Transação devnet de depósito + refund em Explorer; especificação de journal assinada pelo time. |
| **D5 - 1 out** | DIA DE GORDURA. Se D4 estiver verde, iniciar adaptador Router e o caminho de verificação. Se estiver amarelo/vermelho, corrigir prova/manifesto/escrow e reduzir o job. Não adicionar feature nova. | Status explícito: verde, amarelo ou vermelho; plano de corte assinado pelo time. |
| **D6 - 2 out** | Integrar a instrução Anchor ao Router escolhido. Confirmar contas, dados de receipt e versão exatamente contra os exemplos oficiais. Testar rejeição de prova/journal incompatível antes de testar happy path. | Teste de integração ou diagnóstico reproduzível com causa registrada; prova errada rejeitada. |
| **D7 - 3 out** | Montar o caminho crítico por CLI: criar job, depositar Test USDC, gerar receipt, submeter, verificar, liberar. Se a peça Router não fechar, preservar CLI do fallback atestado, marcar claramente como não-ZK e continuar a investigar. | GATE E2E: ao menos um caminho de liquidação end-to-end reproduzível. Caminho ZK só recebe esse nome se verificou on-chain. |
| **D8 - 4 out** | Completar estados ruins: teste falha, prova para job diferente, job expirado, tentativa de dupla liquidação e refund. Manter o contrato pequeno; nenhuma regra econômica implícita. | Matriz de estados executada em teste; pelo menos uma tentativa adversarial rejeitada. |
| **D9 - 5 out** | Rodar a demo em máquina/ambiente limpo. Repetir o fluxo sem intervenção manual escondida. Decidir com evidência qual faixa de demo será submetida: ZK on-chain ou fallback atestado. | README com comandos exatos, versões e hashes; decisão de claims congelada. |
| **D10 - 6 out** | Criar o menor worker/API de prova necessário para a demo e tratar erro/tempo estimado sem fingir sincronismo. Se a prova for lenta, tornar o estado "proving" explícito. | Fluxo CLI + worker testado; mensagem de falha inteligível; sem chave privada exposta no front. |
| **D11 - 7 out** | Construir a tela Buyer: criar job, ver commits/hashes, conectar carteira devnet e depositar. B revisa se os dados exibidos são os mesmos do contrato. | UI chama o fluxo real ou usa uma camada fina que o chama; depósito aparece no Explorer. |
| **D12 - 8 out** | Construir Submit e Result: apresentar receipt/status, transação de verificação e settlement. Congelar a lógica central; só bug crítico altera A/B depois deste dia. | Demo visual completa, com cenário PASS e um cenário de falha/refund preparado. |
