@AGENTS.md

# Claude Code

<!-- hive-ui-core:start -->
Marca (Hive), nomenclatura legada e UI/UX: as regras estão no bloco `hive-ui-core` de `AGENTS.md`, importado acima. Os arquivos centrais de UI/UX são carregados em toda sessão (a adaptação ao MVP construído vem primeiro e prevalece sobre o guia em fatos do MVP):

@HIVE_MVP_UI_ADAPTATION.md
@HIVE_MVP_UI_GUIDE.md
@DESIGN.md
<!-- hive-ui-core:end -->

- Usar Plan Mode antes de qualquer alteração em `zkvm/`, `programs/`, `docs/manifest-schema.md`, na política de escrow (`crates/vericode-core/src/escrow.rs`, `docs/escrow-state-machine.md`) ou na integração do Router.
- Revisões de segurança são somente leitura e não devem editar arquivos.
- Reportar falhas reais; nunca simular sucesso.
- Ao concluir uma tarefa, seguir `docs/handoff-protocol.md`: salvar o prompt da próxima fase em `docs/handoffs/` e reproduzi-lo na resposta final, com modelo e esforço recomendados. A seleção de modelo/esforço é feita pelo humano.
- Commit somente com autorização explícita; push nunca sem autorização separada.
