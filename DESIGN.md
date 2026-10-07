---
version: alpha
name: Hive
description: >
  Sistema visual da interface do MVP da Hive (escrow condicionado a prova verificável).
  Tokens normativos no front matter; justificativa e regras de uso no corpo.
  O modo claro é o canônico; o modo escuro é provido por tokens com prefixo dark-.
colors:
  # --- Marca (documento de identidade de 05.10.2026; valores inalterados) ---
  primary: "#392D40"          # berinjela profundo: identidade, títulos, navegação, texto principal (claro)
  secondary: "#796680"        # malva: texto secundário e apoio (claro)
  tertiary: "#E8B44C"         # âmbar: ação principal, foco/seleção, detalhes da marca
  neutral: "#FAF7F5"          # branco quente: fundo geral (claro)
  surface: "#FFFFFF"          # branco puro: área de trabalho, tabelas, painéis, modais
  surface-variant: "#E7DFE9"  # lilás suave: superfícies secundárias, seleção, divisórias
  on-tertiary: "#392D40"      # texto sobre botão âmbar (claro)
  outline: "#E7DFE9"          # bordas e divisórias suaves (claro)
  # --- Derivados para acessibilidade (NÃO vêm do documento de identidade) ---
  outline-strong: "#9B8AA2"   # borda funcional de campos e controles (3,2:1 sobre branco)
  focus-ring: "#392D40"       # anel de foco no claro: o âmbar tem apenas 1,9:1 sobre branco
  # --- Status (propostos: o documento de identidade deixou estes códigos indefinidos) ---
  success: "#1B6B43"
  success-container: "#E3F1E8"
  danger: "#A8281F"
  danger-container: "#FBE7E4"
  caution: "#7A4B00"
  caution-container: "#F9EBD0"
  pending: "#5E4B66"
  pending-container: "#E7DFE9"
  # --- Modo escuro (identidade: versão clara aprovada, fundo berinjela acinzentado) ---
  dark-background: "#45404C"
  dark-nav: "#35303D"
  dark-topbar: "#3C3643"
  dark-panel: "#4D4655"
  dark-control: "#57505F"
  dark-selected: "#65556F"
  dark-outline: "#726878"
  dark-on-surface: "#FCF8F2"
  dark-on-surface-muted: "#DED5E3"
  dark-on-tertiary: "#302936"
  # --- Derivados do modo escuro (NÃO vêm do documento de identidade) ---
  dark-outline-strong: "#BDB1C4"  # borda funcional (3,8:1 sobre controles, 4,9:1 sobre o fundo)
  dark-focus-ring: "#E8B44C"      # no escuro o âmbar tem 5,3:1 sobre o fundo e serve como foco
  dark-success: "#8EDDB0"
  dark-success-container: "#2F4B41"
  dark-danger: "#FFB4AB"
  dark-danger-container: "#5B3340"
  dark-caution: "#F5CB82"
  dark-caution-container: "#5A4A35"
  dark-pending: "#DED5E3"
  dark-pending-container: "#57505F"
typography:
  # Família principal: Manrope. Família técnica: IBM Plex Mono (somente conteúdo técnico).
  title-lg:
    fontFamily: Manrope
    fontSize: 28px
    fontWeight: 600
    lineHeight: 36px
    letterSpacing: -0.01em
  title-md:
    fontFamily: Manrope
    fontSize: 20px
    fontWeight: 600
    lineHeight: 28px
  title-sm:
    fontFamily: Manrope
    fontSize: 16px
    fontWeight: 600
    lineHeight: 24px
  label-lg:
    fontFamily: Manrope
    fontSize: 14px
    fontWeight: 600
    lineHeight: 20px
  label-md:
    fontFamily: Manrope
    fontSize: 13px
    fontWeight: 500
    lineHeight: 18px
  button:
    fontFamily: Manrope
    fontSize: 14px
    fontWeight: 600
    lineHeight: 20px
  table-head:
    fontFamily: Manrope
    fontSize: 13px
    fontWeight: 600
    lineHeight: 18px
  body-lg:
    fontFamily: Manrope
    fontSize: 16px
    fontWeight: 400
    lineHeight: 24px
  body-md:
    fontFamily: Manrope
    fontSize: 14px
    fontWeight: 400
    lineHeight: 20px
  body-sm:
    fontFamily: Manrope
    fontSize: 13px
    fontWeight: 400
    lineHeight: 18px
  caption:
    fontFamily: Manrope
    fontSize: 12px
    fontWeight: 400
    lineHeight: 16px
  numeric:
    fontFamily: Manrope
    fontSize: 14px
    fontWeight: 400
    lineHeight: 20px
    fontFeature: "tnum"
  metric-lg:
    fontFamily: Manrope
    fontSize: 32px
    fontWeight: 600
    lineHeight: 40px
    fontFeature: "tnum"
  metric-md:
    fontFamily: Manrope
    fontSize: 20px
    fontWeight: 600
    lineHeight: 28px
    fontFeature: "tnum"
  mono-md:
    fontFamily: IBM Plex Mono
    fontSize: 13px
    fontWeight: 400
    lineHeight: 20px
  mono-lg:
    fontFamily: IBM Plex Mono
    fontSize: 14px
    fontWeight: 400
    lineHeight: 22px
  mono-sm:
    fontFamily: IBM Plex Mono
    fontSize: 12px
    fontWeight: 400
    lineHeight: 16px
  doc-body:
    fontFamily: Manrope
    fontSize: 16px
    fontWeight: 400
    lineHeight: 26px
  doc-code:
    fontFamily: IBM Plex Mono
    fontSize: 14px
    fontWeight: 400
    lineHeight: 21px
rounded:
  xs: 4px
  sm: 8px
  md: 12px
  full: 9999px
spacing:
  xs: 4px
  sm: 8px
  md: 12px
  lg: 16px
  xl: 24px
  2xl: 32px
  3xl: 48px
  4xl: 64px
  nav-width: 224px
  topbar-height: 56px
  statusbar-height: 32px
  container-max: 1120px
  control-height: 40px
components:
  # ---------- Modo claro (canônico) ----------
  page:
    backgroundColor: "{colors.neutral}"
    textColor: "{colors.primary}"
    typography: "{typography.body-md}"
  app-nav:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.surface}"
    typography: "{typography.label-lg}"
    width: "{spacing.nav-width}"
  top-bar:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.primary}"
    typography: "{typography.body-md}"
    height: "{spacing.topbar-height}"
  panel:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.primary}"
    typography: "{typography.body-md}"
    rounded: "{rounded.md}"
    padding: 24px
  button-primary:
    backgroundColor: "{colors.tertiary}"
    textColor: "{colors.on-tertiary}"
    typography: "{typography.button}"
    rounded: "{rounded.sm}"
    height: "{spacing.control-height}"
    padding: 0 16px
  button-secondary:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.primary}"
    typography: "{typography.button}"
    rounded: "{rounded.sm}"
    height: "{spacing.control-height}"
    padding: 0 16px
  field:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.primary}"
    typography: "{typography.body-md}"
    rounded: "{rounded.sm}"
    height: "{spacing.control-height}"
    padding: 0 12px
  field-mono:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.primary}"
    typography: "{typography.mono-md}"
    rounded: "{rounded.sm}"
    height: "{spacing.control-height}"
    padding: 0 12px
  helper-text:
    backgroundColor: "{colors.neutral}"
    textColor: "{colors.secondary}"
    typography: "{typography.body-md}"
  table-header:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.secondary}"
    typography: "{typography.table-head}"
  table-row-selected:
    backgroundColor: "{colors.surface-variant}"
    textColor: "{colors.primary}"
    typography: "{typography.body-md}"
  divider:
    backgroundColor: "{colors.outline}"
    height: 1px
  divider-strong:
    backgroundColor: "{colors.outline-strong}"
    height: 1px
  focus-indicator:
    backgroundColor: "{colors.focus-ring}"
    height: 2px
  selection-indicator:
    backgroundColor: "{colors.tertiary}"
    width: 3px
  status-success:
    backgroundColor: "{colors.success-container}"
    textColor: "{colors.success}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  status-danger:
    backgroundColor: "{colors.danger-container}"
    textColor: "{colors.danger}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  status-caution:
    backgroundColor: "{colors.caution-container}"
    textColor: "{colors.caution}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  status-pending:
    backgroundColor: "{colors.pending-container}"
    textColor: "{colors.pending}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  code-block:
    backgroundColor: "{colors.neutral}"
    textColor: "{colors.primary}"
    typography: "{typography.doc-code}"
    rounded: "{rounded.sm}"
    padding: 12px 16px
  hash-field:
    backgroundColor: "{colors.neutral}"
    textColor: "{colors.primary}"
    typography: "{typography.mono-md}"
    rounded: "{rounded.xs}"
    padding: 2px 6px
  provenance-badge:
    backgroundColor: "{colors.surface-variant}"
    textColor: "{colors.primary}"
    typography: "{typography.caption}"
    rounded: "{rounded.full}"
    padding: 2px 8px
  environment-chip:
    backgroundColor: "{colors.surface-variant}"
    textColor: "{colors.primary}"
    typography: "{typography.label-md}"
    rounded: "{rounded.full}"
    padding: 4px 10px
  limits-of-proof:
    backgroundColor: "{colors.neutral}"
    textColor: "{colors.primary}"
    typography: "{typography.body-lg}"
    rounded: "{rounded.md}"
    padding: 24px
  # ---------- Modo escuro (por função, não por inversão) ----------
  page-dark:
    backgroundColor: "{colors.dark-background}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.body-md}"
  app-nav-dark:
    backgroundColor: "{colors.dark-nav}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.label-lg}"
    width: "{spacing.nav-width}"
  top-bar-dark:
    backgroundColor: "{colors.dark-topbar}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.body-md}"
    height: "{spacing.topbar-height}"
  panel-dark:
    backgroundColor: "{colors.dark-panel}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.body-md}"
    rounded: "{rounded.md}"
    padding: 24px
  button-primary-dark:
    backgroundColor: "{colors.tertiary}"
    textColor: "{colors.dark-on-tertiary}"
    typography: "{typography.button}"
    rounded: "{rounded.sm}"
    height: "{spacing.control-height}"
    padding: 0 16px
  field-dark:
    backgroundColor: "{colors.dark-control}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.body-md}"
    rounded: "{rounded.sm}"
    height: "{spacing.control-height}"
    padding: 0 12px
  helper-text-dark:
    backgroundColor: "{colors.dark-background}"
    textColor: "{colors.dark-on-surface-muted}"
    typography: "{typography.body-md}"
  table-row-selected-dark:
    backgroundColor: "{colors.dark-selected}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.body-md}"
  divider-dark:
    backgroundColor: "{colors.dark-outline}"
    height: 1px
  divider-strong-dark:
    backgroundColor: "{colors.dark-outline-strong}"
    height: 1px
  focus-indicator-dark:
    backgroundColor: "{colors.dark-focus-ring}"
    height: 2px
  status-success-dark:
    backgroundColor: "{colors.dark-success-container}"
    textColor: "{colors.dark-success}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  status-danger-dark:
    backgroundColor: "{colors.dark-danger-container}"
    textColor: "{colors.dark-danger}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  status-caution-dark:
    backgroundColor: "{colors.dark-caution-container}"
    textColor: "{colors.dark-caution}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  status-pending-dark:
    backgroundColor: "{colors.dark-pending-container}"
    textColor: "{colors.dark-pending}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    padding: 12px 16px
  code-block-dark:
    backgroundColor: "{colors.dark-background}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.doc-code}"
    rounded: "{rounded.sm}"
    padding: 12px 16px
  hash-field-dark:
    backgroundColor: "{colors.dark-control}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.mono-md}"
    rounded: "{rounded.xs}"
    padding: 2px 6px
  provenance-badge-dark:
    backgroundColor: "{colors.dark-control}"
    textColor: "{colors.dark-on-surface}"
    typography: "{typography.caption}"
    rounded: "{rounded.full}"
    padding: 2px 8px
---

## Overview

Este arquivo é o **sistema visual** da interface do MVP da Hive. Ele responde "como as coisas se parecem e como se organizam". O comportamento, o conteúdo de cada tela e as regras de semântica ("o que cada elemento promete") estão em `HIVE_MVP_UI_GUIDE.md`.

**Precedência.** Se houver conflito sobre o que um elemento *significa ou promete*, vale o `HIVE_MVP_UI_GUIDE.md`. Se houver conflito sobre como ele *se parece*, vale este arquivo.

### Personalidade

Serena, precisa, criteriosa e cooperativa. A interface da Hive é de *justificação*, não de decisão: ela mostra evidência e explica o "porquê". Isso se traduz visualmente em:

- Hierarquia por tamanho, peso, espaço e agrupamento, nunca por ornamento.
- Muito espaço em branco, superfícies planas, bordas finas.
- Dados técnicos (hashes, endereços, assinaturas) tratados como protagonistas legíveis, não como metadado cinza.
- Um único foco de cor de ação por tela (o âmbar), usado com economia.

### Origem dos valores (leia antes de alterar qualquer token)

| Origem | O que é | Pode mudar? |
|---|---|---|
| **Identidade** | Paleta de marca (claro e escuro), famílias tipográficas (Manrope + IBM Plex Mono), raios iniciais (8 e 12 px), regras de uso do âmbar, caixa de frase | **Não.** Vem do documento de identidade de 05.10.2026. |
| **Adaptado** | Escala tipográfica (reduzida para densidade de aplicação), larguras e espaçamentos | Sim, com justificativa registrada. |
| **Derivado** | `outline-strong`, `focus-ring`, `dark-outline-strong`, `dark-focus-ring` | Somente para manter contraste. Não são cores novas de marca. |
| **Proposto** | Cores de status (`success`, `danger`, `caution`, `pending` e variantes escuras) | Sim, pela equipe de identidade. O documento de identidade deixou esses códigos indefinidos. |

### Como o agente de código deve usar este arquivo

1. Gere os tokens do front matter como **variáveis de tema por papel semântico** (veja *Implementation notes*). Componentes nunca contêm cor, fonte ou medida literal.
2. Use o **modo claro** como padrão e o acabamento principal. O escuro é suportado por tokens; só o polia depois que os itens "nunca cortar" do guia de UI estiverem prontos.
3. Valide este arquivo com `npx @google/design.md lint DESIGN.md` depois de qualquer edição.
4. Os estudos de dashboard do documento de identidade são **estudos cromáticos e tipográficos**, não especificação de funcionalidades. Não copie suas telas, navegação, dados ou textos.

## Colors

A direção cromática é **berinjela + âmbar**. O berinjela organiza identidade e superfícies; o âmbar concentra atenção em ações e foco. A cor expressa estrutura e foco, nunca veredito.

### Paleta de marca (inalterada)

| Token | Hex | Papel |
|---|---|---|
| `primary` | `#392D40` | Identidade, títulos, texto principal no claro, navegação lateral |
| `tertiary` | `#E8B44C` | Ação principal, seleção, foco no escuro, detalhes da marca |
| `neutral` | `#FAF7F5` | Fundo geral no claro |
| `surface-variant` | `#E7DFE9` | Superfícies secundárias, seleção, divisórias suaves |
| `secondary` | `#796680` | Texto secundário no claro |
| `surface` | `#FFFFFF` | Área de trabalho, tabelas, painéis, menus, modais |

### Mapa de papéis (tema)

Esta tabela é a ponte entre os tokens e o CSS. Crie uma variável por **papel** e alterne entre as colunas por modo.

| Papel | Claro | Escuro |
|---|---|---|
| `bg-page` | `neutral` | `dark-background` |
| `bg-nav` | `primary` | `dark-nav` |
| `bg-topbar` | `surface` | `dark-topbar` |
| `bg-panel` | `surface` | `dark-panel` |
| `bg-control` | `surface` | `dark-control` |
| `bg-sunken` (código, hash) | `neutral` | `dark-background` ou `dark-control` (o que contrastar com o painel) |
| `bg-selected` | `surface-variant` | `dark-selected` |
| `bg-chip` | `surface-variant` | `dark-control` |
| `text-primary` | `primary` | `dark-on-surface` |
| `text-secondary` | `secondary` | `dark-on-surface-muted` |
| `text-on-nav` | `surface` | `dark-on-surface` |
| `text-on-accent` | `on-tertiary` | `dark-on-tertiary` |
| `accent` | `tertiary` | `tertiary` |
| `border-subtle` | `outline` | `dark-outline` |
| `border-strong` | `outline-strong` | `dark-outline-strong` |
| `focus` | `focus-ring` | `dark-focus-ring` |
| `link` | `primary` (sempre sublinhado) | `tertiary` |
| `status-*` / `status-*-bg` | `success`, `danger`, `caution`, `pending` e `*-container` | variantes `dark-*` |

### Regras de uso do âmbar

- Reservado para: **ação primária**, **indicador de foco ou seleção** (barra de 3 px) e detalhes da marca.
- **Nunca** como cor de resultado, aprovação, sucesso ou "ativo". Ação e veredito não compartilham significante.
- **Nunca** como texto ou ícone informativo sobre fundo claro (contraste 1,9:1 sobre branco).
- Evite grandes áreas preenchidas. No máximo um botão primário por região da tela.
- Hover do botão primário: `color-mix(in srgb, var(--accent) 88%, var(--text-primary))`. Pressionado: 78%. Não introduza novos hex.

### Status: a cor expressa valência, o ícone e o rótulo expressam o quê

Existem quatro famílias. A cor diz apenas se algo é positivo, negativo, de atenção ou neutro. **O cenário é identificado por ícone + rótulo + posição**, nunca só pela cor (veja os cenários no `HIVE_MVP_UI_GUIDE.md`).

| Família | Valência | Usos |
|---|---|---|
| `success` | Resultado verificado positivo | `PASS` verificado, `Released`, vínculo confere, proof verificada |
| `danger` | Resultado negativo verificado ou falha de integridade | `FAIL` verificado, vínculo diverge, proof incompatível |
| `caution` | Atenção, bloqueio, estado operacional ou prazo | `Failed` operacional, replay/dupla liquidação bloqueada, timeout, **fallback** |
| `pending` | Neutro ou em andamento | `Draft`, `Funded`, `Proving`, `Submitted`, `Refunded`, não aplicável, indisponível |

Regras:

- Sempre `ícone + rótulo de texto`. Cor jamais sozinha.
- `caution` nunca usa o âmbar de marca. Ele usa o ocre escuro `caution` sobre `caution-container`, sempre com ícone de alerta triangular. Isso evita confundir "atenção" com "ação".
- No modo escuro, os containers de status se distinguem pouco do painel (a variante `dark-success-container` tem só 1,05:1 contra o painel). Adicione **borda de 1 px na cor do próprio status** em banners escuros.
- Um estado não carrega duas famílias ao mesmo tempo.

### Contraste (validado)

Valores calculados (WCAG 2.x). Texto normal exige 4,5:1; elementos não textuais, 3:1.

| Par | Razão | Observação |
|---|---|---|
| `primary` sobre `neutral` / `surface` | 12,1 / 12,9 | Texto principal |
| `secondary` sobre `neutral` / `surface` | 4,9 / 5,2 | OK para texto secundário |
| `secondary` sobre `surface-variant` | **4,0** | **Não passa. Nunca use texto `secondary` sobre `surface-variant`; use `primary`.** |
| `on-tertiary` sobre `tertiary` | 6,8 | Botão primário |
| `tertiary` sobre `surface` | **1,9** | **Não use âmbar como foco ou texto no claro.** |
| `outline` sobre `surface` | 1,3 | Só divisória decorativa |
| `outline-strong` sobre `surface` | 3,2 | Borda funcional de campo |
| `dark-on-surface` sobre `dark-selected` | 6,4 | Pior caso do texto principal no escuro |
| `dark-on-surface-muted` sobre `dark-selected` | 4,8 | Pior caso do texto secundário no escuro |
| `tertiary` sobre `dark-background` | 5,3 | Links e foco no escuro |
| `dark-outline` sobre `dark-background` | 1,9 | Só decorativo; campos usam `dark-outline-strong` |

Revalide contraste na interface implementada. Estas tabelas registram direção, não são auditoria.

## Typography

Uma família principal para a interface, uma família técnica restrita a conteúdo técnico. Não adicione uma terceira.

| Família | Uso | Carregamento recomendado |
|---|---|---|
| **Manrope** (variável, eixo de peso 200 a 800) | Títulos, texto, rótulos, botões, navegação, números comuns | `@fontsource-variable/manrope` (auto-hospedada) |
| **IBM Plex Mono** | **Somente** hashes, endereços, assinaturas, IDs de job, `image_id`, versões, comandos, código, nomes de erro do programa | `@fontsource/ibm-plex-mono`, pesos 400 (e 500 se houver ênfase) |

Auto-hospede as fontes. A interface precisa funcionar na demo mesmo com rede ruim e sem depender de CDN de terceiros.

### Verificações feitas nas fontes reais

- A Manrope tem `tnum` (algarismos tabulares), mas **os dígitos padrão são proporcionais**. Ative `font-variant-numeric: tabular-nums` em todo contexto numérico (tabelas, métricas, contadores, cronômetros, slots, saldos).
- A Manrope tem o sinal de menos tipográfico `−` (U+2212). Use-o em deltas negativos, não o hífen.
- **Nem a Manrope nem a IBM Plex Mono (subconjunto latino) têm `≈` (U+2248) nem `→` (U+2192).** O navegador cairia em fonte do sistema com aparência inconsistente. **Não use esses caracteres no texto.** Para estimativas use `~` ou "aprox."; para setas use ícone SVG.
- Desative ligaduras na fonte técnica: `font-variant-ligatures: none`.

### Escala da aplicação

Adaptada da escala do documento de identidade, que foi dimensionada para site e dashboard de marketing (H1 32 px, corpo 16 px). Em telas densas de evidência isso gera hierarquia pesada. Ajustes, todos dentro da faixa 14 a 16 px que a própria identidade prioriza para interface:

| Token | Fonte e peso | Tamanho / entrelinha | Uso |
|---|---|---|---|
| `title-lg` | Manrope 600 | 28 / 36 (identidade: 32) | Título de página (ex.: identificador do job) |
| `title-md` | Manrope 600 | 20 / 28 | Título de bloco (ex.: "Compromissos e observado") |
| `title-sm` | Manrope 600 | 16 / 24 | Subtítulo, título de cartão |
| `label-lg` | Manrope 600 | 14 / 20 | Grupos internos, rótulo de status, navegação selecionada |
| `label-md` | Manrope 500 | 13 / 18 | Rótulo de campo, chip, contexto |
| `button` | Manrope 600 | 14 / 20 | Botões |
| `table-head` | Manrope 600 | 13 / 18 | Cabeçalho de tabela |
| `body-lg` | Manrope 400 | 16 / 24 | Texto explicativo, "Limites da prova" |
| `body-md` | Manrope 400 | 14 / 20 | **Padrão de interface** |
| `body-sm` | Manrope 400 | 13 / 18 | Ajuda e dados secundários |
| `caption` | Manrope 400 | 12 / 16 | Somente conteúdo complementar (badges, notas) |
| `numeric` | Manrope 400 + `tnum` | 14 / 20 | Números em tabelas, datas, slots |
| `metric-lg` | Manrope 600 + `tnum` | 32 / 40 | Valor principal (ex.: valor em escrow) |
| `metric-md` | Manrope 600 + `tnum` | 20 / 28 | Cronômetro de prova, métricas de bloco |
| `mono-md` | IBM Plex Mono 400 | 13 / 20 | Hashes, IDs, endereços, assinaturas |
| `mono-lg` | IBM Plex Mono 400 | 14 / 22 | Hash em destaque (ex.: `image_id` ativo) |
| `mono-sm` | IBM Plex Mono 400 | 12 / 16 | Somente conteúdo complementar técnico |

O "título de destaque" de 56 px da identidade **não é usado na aplicação**; é do site e das apresentações.

### Regras

- **Caixa de frase** em todo rótulo ("Novo job", "Critérios atendidos"). Maiúsculas só para siglas e códigos: `API`, `ID`, `PASS`, `FAIL`, `PDA`.
- Pesos: use 400, 500 e 600. Evite 700 ou mais na interface. Hierarquia vem de tamanho, espaço e agrupamento.
- Entrelinha de títulos de várias linhas: 1,15 a 1,3 vezes o tamanho.
- Largura de leitura de texto corrido: 60 a 75 caracteres. Limite com `max-width: 68ch`.
- **Não reduza a fonte para fazer algo caber.** Quebre, amplie o componente ou reorganize. Piso: 12 px, só para complemento.
- **Números comuns** (valores, durações, contagens, datas) usam Manrope. **Identificadores e sequências técnicas** usam a fonte monoespaçada. Não aplique mono a todos os números.
- Nunca use texto em caixa alta com espaçamento largo como "eyebrow" acima de títulos.
- Teste nomes longos, acentos do português e caracteres ambíguos (`0`/`O`, `1`/`I`/`l`) na fonte técnica.

## Layout

Interface desktop-first. Alvo principal: 1280 px ou mais. Entre 1024 e 1279 px deve funcionar sem ajustes. Abaixo de 1024 px, apenas **não quebrar** (coluna única, sem recorte de conteúdo). Mobile não é alvo do MVP.

### Estrutura do aplicativo

- **Navegação lateral** (`app-nav`, 224 px) em berinjela, também no modo claro. O menu escuro não transforma a interface em modo escuro. Itens: apenas os do MVP (Jobs, Novo job). Sem seções que prometam produtos inexistentes.
- **Barra superior** (56 px): trilha de navegação à esquerda; à direita o chip de ambiente e o estado do signatário.
- **Barra de status inferior** (32 px, fixa): cluster, token de teste, `image_id` ativo, versões. Texto `caption`/`mono-sm`. Detalhes completos em popover.
- **Conteúdo**: contêiner de até 1120 px, centralizado, com 32 px de margem lateral.

### Espaçamento

Base de 4 px: `4, 8, 12, 16, 24, 32, 48, 64`.

- Entre blocos de uma página: 32 px.
- Padding interno de painel: 24 px. De painel compacto (tabela): 16 px.
- Entre rótulo e campo: 8 px. Entre campos: 16 px.
- Respiro generoso é parte da personalidade (serenidade). Não comprima para "caber mais".

### Gabaritos de página (estrutura, não desenho)

**Lista de jobs.** Tabela de largura total. Colunas: identificador (mono), tarefa, estado (`StatusLabel`), valor (à direita, tabular), prazo. Poucos itens reais. Sem busca, filtro nem paginação até haver mais de ~20 itens. Linha inteira clicável.

**Novo job.** Duas colunas num grid de 12: formulário (7 colunas) e **resumo do compromisso** (5 colunas, fixo ao rolar). O resumo mostra, ao vivo, os hashes calculados, o vault PDA derivado, o mint de teste e o prazo traduzido. É onde o usuário *vê o hash nascer*. Após o financiamento, o resumo vira somente leitura e é o mesmo componente exibido no detalhe.

**Detalhe do job.** Ordem vertical fixa dos blocos (A a F do guia de UI):

1. Cabeçalho: identificador, `StatusLabel`, valor, prazo.
2. **A** Linha do tempo de estados (faixa horizontal).
3. **B** Compromissos e observado (tabela de largura total).
4. **C** Verificação: duas colunas iguais, *local* à esquerda e *on-chain* à direita.
5. **D** Liquidação: anatomia da transação (7 colunas) e saldos antes/depois (5 colunas).
6. **E** Limites da prova (largura total, peso visual equivalente aos demais blocos).
7. **F** Reproduzir na CLI.

Abaixo de 1024 px, as colunas empilham, mantendo a ordem.

## Elevation & Depth

Profundidade vem de **tom de superfície e borda de 1 px**, não de sombra.

| Nível | Uso | Tratamento |
|---|---|---|
| 0 | Fundo da página | `bg-page` |
| 1 | Painéis, tabelas, cartões | `bg-panel` + borda `border-subtle` de 1 px. Sem sombra. |
| 2 | Menus, popovers, tooltips, modais | `bg-panel` + borda `border-strong` + **uma** sombra suave, tingida com o berinjela: `0 8px 24px rgba(57, 45, 64, 0.12)` no claro; `0 8px 24px rgba(0, 0, 0, 0.32)` no escuro |
| Scrim de modal | Fundo atrás do modal | `rgba(57, 45, 64, 0.48)` |

Não use brilho, múltiplas sombras empilhadas, degradês decorativos, desfoque de fundo nem relevo.

## Shapes

Geometria precisa com suavidade controlada: retângulos de cantos suaves para informação, linhas para relações, círculos para eventos, hexágono somente para identidade.

| Elemento | Raio |
|---|---|
| Botões, campos, selects | 8 px (`rounded.sm`) |
| Cartões, painéis, banners de status | 12 px (`rounded.md`) |
| Hash, código em linha | 4 px (`rounded.xs`) |
| Chips de ambiente e badges de proveniência | Cápsula (`rounded.full`). **Cápsula só para essas funções específicas** (e filtros, se houver). |

- Borda padrão: 1 px. Seleção: barra de **3 px** em âmbar na borda esquerda da linha ou do item de navegação, mais o tom `bg-selected`.
- Traço de espessura **uniforme**. Não varie espessura ou densidade de linha para sugerir confiança, nível ou pontuação.
- **Ícones**: um único conjunto de linha com traço de 1,5 px (ex.: Lucide). Tamanhos 16 e 20 px. Todo ícone de estado acompanha texto. Evite metáforas de escudo, troféu, selo ou medalha.
- **Hexágono** (uso contido): hexágono regular com vértice para cima, cantos discretamente arredondados, traço uniforme. Aparece apenas como `PartyMark` (identidade do comprador ou do executor) em tamanho 24 ou 32 px. **É idêntico em PASS, FAIL, Released e Refunded.** A célula representa identidade, não certificação. Sem halo, sem brilho, sem cor que varie com o resultado.

## Components

Cada componente usa exclusivamente tokens. Estados interativos comuns: padrão, hover, pressionado, foco, desabilitado, selecionado.

### Botões

| Variante | Visual | Uso |
|---|---|---|
| **Primário** | Fundo `accent`, texto `text-on-accent`, 40 px de altura | A única ação principal da região (ex.: "Financiar job", "Novo job") |
| **Secundário** | Fundo `bg-control`, borda `border-strong`, texto `text-primary` | Ações de apoio (ex.: "Copiar comando", "Solicitar refund") |
| **Texto/link** | Sem fundo; cor `link`, **sublinhado** no claro | Navegação e "Ver no Explorer" |

- Rótulo começa com verbo. O nome de uma ação permanece o mesmo em todo o fluxo ("Financiar job" gera "Job financiado").
- **Desabilitado** sempre com explicação visível (texto de ajuda ou tooltip acessível por teclado). Ex.: "Solicitar refund" antes do prazo informa "Disponível a partir do slot N".
- Não existe botão que aprove, rejeite ou libere pagamento.

### Campos de formulário

Altura de 40 px, borda `border-strong`, raio 8 px. Rótulo acima (`label-md`), texto de ajuda abaixo (`body-sm`, `text-secondary`), erro abaixo em `danger` com ícone. Campos de conteúdo técnico usam `field-mono`. Foco: anel de 2 px na cor `focus`, afastado 2 px.

### Tabelas

Cabeçalho `table-head` em `text-secondary`, sem fundo, borda inferior `border-subtle`. Linhas de 48 px, divisórias `border-subtle`. Valores numéricos alinhados à direita, tabulares, com casas decimais consistentes. Linha selecionada: `bg-selected` + barra âmbar de 3 px. **Seleção indica foco, não aprovação.**

### StatusLabel e StatusBanner

- **StatusLabel** (em linha, tabelas, cabeçalhos): ícone + texto `label-lg` na cor da família. Sem container.
- **StatusBanner** (resultados e cenários): container `status-*` com raio 12, padding 12 × 16, ícone, **título** (o que aconteceu), **detalhe** (por quê, uma frase) e **ação** opcional. Borda de 1 px na cor da família no modo escuro.

Mapeamento de estados do job para famílias (confirme os nomes reais no program):

| Estado | Família | Conceito de ícone |
|---|---|---|
| Draft | `pending` | círculo tracejado |
| Funded | `pending` | cadeado |
| Proving | `pending` | indicador de processamento (com tempo real ao lado) |
| Submitted | `pending` | documento com marca de conferência |
| Released | `success` | círculo com marca de verificação |
| Refunded | `pending` | seta de retorno |
| Failed (operacional) | `caution` | triângulo de alerta |

Cenários (apresentação distinta, nunca o mesmo ícone genérico):

| Cenário | Família | Conceito de ícone | Título |
|---|---|---|---|
| FAIL válido | `danger` | círculo com X | Critérios não atendidos (`FAIL`) |
| Vínculo quebrado | `danger` | elo partido | Transação rejeitada: vínculo diverge |
| Replay / dupla liquidação | `caution` | setas de repetição | Transação rejeitada: job já liquidado |
| Proof incompatível | `danger` | documento com X | Transação rejeitada: proof inválida |
| `Failed` operacional | `caution` | triângulo de alerta | Falha operacional: sem veredito |
| Timeout | `caution` | relógio | Prazo expirado sem liquidação |

### HashField

Mono `mono-md`, fundo `bg-sunken`, raio 4 px. Truncamento: endereços e assinaturas Solana **4 + … + 4** (endereços) ou **8 + … + 8** (assinaturas); hashes hex **8 + … + 8**. O valor completo é sempre acessível por clique (expande para valor inteiro com quebra segura) e por teclado. Botão de **copiar** (ícone) com `aria-label`, e confirmação visível por ~1,5 s ("Copiado"). Seleção de texto habilitada. Ligaduras desativadas.

### ProvenanceBadge

Cápsula `caption` com ícone. Variantes: **On-chain** (inclui "slot N"), **Worker**, **Calculado localmente**. Aparece junto a todo dado relevante. Texto `text-primary` sobre `bg-chip`. Nunca `text-secondary` sobre esse fundo.

### CommitmentVsObserved

Tabela de 4 colunas: campo (mono), comprometido, publicado no journal, vínculo. A coluna de vínculo usa `StatusLabel` (`success` "Confere", `danger` "Diverge", `pending` "Não se aplica"). Linha divergente recebe `danger-container` na linha toda e o mesmo rótulo; a divergência deve ser perceptível sem ler.

### StateTimeline

Faixa horizontal com os estados percorridos, o atual e os possíveis à frente. Nós circulares (eventos) ligados por linha de traço uniforme; o atual em destaque por tamanho e peso, **não por âmbar**. Estados futuros em traço tracejado, explicitados na legenda. Cada nó concluído com link para a transação. `Failed` aparece como ramo separado, nunca na linha de `Refunded`.

### VerificationPanel

Dois painéis lado a lado: **Verificação local** e **Verificação on-chain**. Cada um lista verificações como linhas `ícone + rótulo + resultado`. Se o caminho on-chain não passou pelo Router, o painel exibe um `StatusBanner` `caution` "Fallback: não verificado on-chain" no topo, em destaque.

### TransactionAnatomy

Lista ordenada vertical das instruções da transação (nome, programa, resultado), com assinatura (`HashField`), slot, compute units e link de Explorer. A atomicidade fica visível por um contêiner único que envolve as instruções, sugerindo "tudo ou nada". Nomes de instrução e de programa em mono.

### BalanceDelta

Tabela compacta: conta (`PartyMark` + `HashField`), saldo antes, saldo depois, variação com sinal (`+` ou `−`) e unidade. O sinal e o ícone carregam o significado; não colorir o valor como "ganho/perda" de reputação.

### ProvingProgress

Mostra **tempo decorrido real** em `metric-md`, calculado a partir do horário de início reportado pelo worker (não do relógio do cliente), a etapa atual se o worker a informar, e o texto "Gerando prova". **Sem barra de progresso e sem percentual inventado.** Indicador de atividade pequeno e discreto, estático sob `prefers-reduced-motion`. Ao terminar, o tempo final permanece como dado do job.

### LimitsOfProof

Painel `limits-of-proof` (fundo `neutral`, borda `border-strong`, barra de 3 px em `primary` à esquerda), com título `title-md` e texto `body-lg`. Duas listas: **O que esta verificação demonstra** e **O que ela não demonstra**. Mesmo peso visual dos outros blocos. **Nunca em rodapé, nem em cinza claro.**

### CliEquivalent

`code-block` em mono 13 a 14 px. Prompt `$` não selecionável; "Copiar" copia só o comando. Comandos longos quebram com `\`. Um por operação.

### EnvironmentChip e barra de status

Chip em cápsula com o cluster (ex.: "Devnet") e outro com "Token de teste". Na barra inferior: `image_id` ativo (`HashField`), versão do Verifier Router, versão do program, commit. Texto discreto, mas legível. Nunca oculto.

### Feedback transitório e diálogos

- **Toast**: curto, ação nomeada igual ao botão ("Job financiado"). Some após ~5 s, exceto erros, que permanecem.
- **Confirmação de assinatura** (modal): antes de qualquer transação, mostra o que será executado, o programa, o valor e o comando equivalente. Botão primário com o nome da ação.
- **Tooltip/popover**: nível 2 de elevação. Nunca guarda informação essencial só no hover.

### Estados vazios, carregando e erro

- **Vazio**: uma frase que diz o que falta e o botão que cria. Sem ilustração decorativa nem mascote.
- **Carregando**: nomeie a fonte ("Lendo estado na cadeia"). Esqueleto só para conteúdo que certamente existirá.
- **Erro de fonte**: diga qual falhou (RPC, worker) e mostre o último valor conhecido com o horário e o slot, marcado como desatualizado.
- **Indisponível**: texto "Indisponível" em `pending`, nunca um valor inventado.

## Numbers and indicators

| Dado | Formato | Fonte |
|---|---|---|
| Valor monetário | Duas casas por padrão (`25,00`), alinhado à direita, unidade em `text-secondary` após o número (`D-USDC`). Precisão completa no detalhe expandido. Use `Intl.NumberFormat` com o locale ativo. | Manrope `tnum` |
| Variação | Sinal explícito com `+` ou `−` (U+2212): `+25,00` / `−25,00` | Manrope `tnum` |
| Slot | Inteiro com separador de milhar. Prazo: `slot 345.678.901`, com "faltam 12.345" quando aplicável. | Manrope `tnum` |
| Tempo estimado por slot | Prefixo `~` e rótulo "estimativa" (ex.: `~82 min`). Nunca apresentar como exato. | Manrope |
| Duração | `48 s`, `1 min 12 s`. Cronômetro de prova atualiza a cada segundo. | Manrope `tnum` |
| Compute units | Inteiro + `CU` | Manrope `tnum` |
| Data e hora | Absoluta com fuso: `2026-10-06 14:32:07 UTC`. Relativo ("há 2 min") só como complemento. Formato único em toda a aplicação. | Manrope `tnum` |
| Hash, endereço, assinatura, ID, versão, `image_id` | Veja `HashField` | IBM Plex Mono |
| Nome de erro do program | Texto exato do programa (ex.: o identificador do erro) | IBM Plex Mono |
| `PASS` / `FAIL` | Código em mono, ao lado da frase em linguagem natural ("Critérios atendidos `PASS`") | IBM Plex Mono |

Princípios:

- **Totais e métricas pertencem à leitura comum** (Manrope); só identificadores têm tratamento técnico.
- Um indicador precisa de significado claro. **Não use espessura, densidade, tamanho ou cor para sugerir confiança, nível ou pontuação.** Contagem de jobs não é medida de confiança.
- O tempo de prova é um dado do job, exibido com a mesma importância que o veredito.
- Cada número relevante informa sua **proveniência** (`ProvenanceBadge`).

## Labels and vocabulary

Caixa de frase. Verbos ativos. Nomes de estado do program permanecem em inglês nos dois idiomas (são linguagem técnica da Hive). A tabela traz os dois idiomas porque o idioma padrão ainda é decisão em aberto; todas as strings ficam em um dicionário central.

| Chave | Português | English |
|---|---|---|
| `nav.jobs` | Jobs | Jobs |
| `job.new` | Novo job | New job |
| `job.fund` | Financiar job | Fund job |
| `job.funded` | Job financiado | Job funded |
| `commit.title` | Compromissos e observado | Commitments and observed |
| `commit.committed` | Comprometido no job | Committed in job |
| `commit.observed` | Publicado no journal | Published in journal |
| `link.ok` | Confere | Matches |
| `link.bad` | Diverge | Mismatch |
| `link.na` | Não se aplica | Not applicable |
| `verify.local` | Verificação local | Local verification |
| `verify.onchain` | Verificação on-chain | On-chain verification |
| `verify.fallback` | Fallback: não verificado on-chain | Fallback: not verified on-chain |
| `verdict.pass` | Critérios atendidos | Criteria met |
| `verdict.fail` | Critérios não atendidos | Criteria not met |
| `settle.released` | Liquidado: Released | Settled: Released |
| `settle.refunded` | Liquidado: Refunded | Settled: Refunded |
| `proving.title` | Gerando prova | Generating proof |
| `proving.elapsed` | Tempo decorrido | Elapsed time |
| `limits.title` | Limites desta prova | Limits of this proof |
| `limits.shows` | O que esta verificação demonstra | What this verification shows |
| `limits.not` | O que ela não demonstra | What it does not show |
| `cli.title` | Reproduzir na CLI | Reproduce in the CLI |
| `action.copy` | Copiar | Copy |
| `action.copied` | Copiado | Copied |
| `action.explorer` | Ver no Explorer | View on Explorer |
| `action.refund` | Solicitar refund | Request refund |
| `refund.disabled` | Disponível a partir do slot {n} | Available from slot {n} |
| `env.devnet` | Devnet | Devnet |
| `env.testToken` | Token de teste | Test token |
| `chain.readAt` | Lido da cadeia no slot {n} | Read from chain at slot {n} |
| `data.unavailable` | Indisponível | Unavailable |
| `empty.jobs` | Nenhum job ainda. Crie o primeiro. | No jobs yet. Create the first one. |

Vocabulário a **evitar** (promete mais do que a prova entrega): "aprovado", "rejeitado" (como veredito da UI), "seguro", "confiável", "garantido", "certificado", "trustless", "agente verificado", "reputação", "score", "nível de confiança". Não use ponto médio `·` para juntar metadados nem acrescente `→` a botões e links.

Mensagens de erro e rejeição: neutras, dizem **o que** divergiu e **o que** foi preservado ("O estado do job não foi alterado"). Nunca culpam o usuário nem se desculpam. O nome técnico do erro aparece como dado secundário em mono, copiável.

O slogan oficial e suas versões (português e inglês) não são alterados nem reescritos na interface.

## Motion and interaction states

Movimento serve para mostrar **o que mudou** em resposta a uma ação ou a um dado real. Sem movimento decorativo.

| Situação | Duração | Curva |
|---|---|---|
| Hover, pressionado, foco | 120 ms | ease-out |
| Expandir/recolher (hash, painéis), toast | 200 ms | ease-out |
| Troca de estado do job | 200 ms (cruzamento de opacidade) | ease-out |

Permitido: transições de estado, confirmação de cópia, atualização do cronômetro de prova, indicador de atividade em "Proving".

Proibido: entradas com fade e deslize em cada seção, hover animado em todo cartão, partículas percorrendo conexões, animações de colmeia, contagem animada de números, barras de progresso sem dado real.

`prefers-reduced-motion: reduce` desliga todo movimento não essencial; o indicador de atividade fica estático.

Estados interativos:

| Estado | Tratamento |
|---|---|
| Hover | `color-mix` do fundo com 6% do texto principal |
| Pressionado | 12% |
| Foco (teclado) | Anel de 2 px na cor `focus`, afastado 2 px. **Claro: berinjela. Escuro: âmbar.** Nunca remova o contorno sem substituto. |
| Selecionado | `bg-selected` + barra âmbar de 3 px |
| Desabilitado | `bg-chip`, texto `text-secondary`, com explicação visível do motivo |

## Accessibility

- Cumpra WCAG 2.2 AA: texto normal 4,5:1, texto grande e elementos não textuais 3:1. Bordas de campo e anel de foco usam os tokens `border-strong` e `focus`.
- **Estado nunca só por cor.** Ícone e rótulo sempre.
- Navegação completa por teclado; ordem de tabulação segue a ordem visual. Alvos interativos de pelo menos 40 px de altura.
- `HashField`, botões de copiar e links de Explorer têm rótulo acessível que identifica *qual* valor (ex.: "Copiar hash da especificação").
- Mudanças de estado do job e do cronômetro são anunciadas em região `aria-live="polite"` (sem anunciar a cada segundo; anuncie marcos).
- Tabelas com semântica de tabela (`th` com `scope`).
- Links no modo claro têm a mesma cor do texto, portanto **sempre sublinhados**.
- Funcione com zoom de 200% sem perda de conteúdo e com `prefers-reduced-motion`.
- Defina o idioma do documento (`lang`) conforme o idioma ativo.

## Documentation pages

Aplica-se a qualquer conteúdo longo dentro da aplicação (ex.: "Limites da prova", ajuda) e a páginas de documentação que existam.

| Elemento | Padrão |
|---|---|
| Texto corrido | Manrope 400, 16 px, entrelinha 26 px (`doc-body`) |
| Largura de leitura | 60 a 75 caracteres (`max-width: 68ch`) |
| Títulos | H1 a H4 da escala da aplicação; um H1 por página |
| Ênfase | Manrope 600 em trechos curtos |
| Código em linha | IBM Plex Mono, tamanho visual próximo ao do corpo, fundo `bg-sunken`, raio 4 px |
| Blocos de código | IBM Plex Mono 400, 14 px, entrelinha 21 px (`doc-code`), com botão de copiar |
| Tabelas técnicas | Manrope para explicações; mono para comandos, parâmetros e identificadores |
| Listas | No máximo dois níveis |

Todo comando documentado deve ser **copiável e reprodutível**. Fixe versões e informe o ambiente (devnet) no início.

## Brand assets (ponteiro)

Este arquivo **não define** logotipo, símbolo, mascote nem seus desenhos. Eles ficam em `brand/` (veja `brand/MANIFEST.md`).

- Use os arquivos de `brand/` **como estão**. Não redesenhe, não trace a partir de imagens, não redigite a palavra "hive" com a fonte da interface para simular o logotipo.
- Aplique a área de proteção e os tamanhos mínimos descritos em `brand/MANIFEST.md`.
- Os SVGs atuais em `brand/` são **provisórios**: foram rastreados da prancha de identidade, e o manifesto os marca como `provisório`. Use-os por meio de um único componente `BrandMark`, que centraliza o caminho do arquivo e leva `TODO(brand)` até o mestre vetorial existir. Assim, trocar pelo mestre exige mexer em um ponto só.
- Se o arquivo de que precisa não existir (por exemplo, a versão vertical), mostre um **texto provisório claramente marcado** (`TODO(brand)`). Não tente imitar nem derivar o desenho por conta própria.
- O mascote **não aparece** nas telas do produto.
- Nunca use a marca como selo de aprovação. Resultados de verificação usam os rótulos e ícones de status deste arquivo.

## Implementation notes

**Tokens como variáveis.** Gere uma variável por papel (tabela *Mapa de papéis*), por exemplo `--hive-bg-page`, `--hive-text-primary`, `--hive-accent`. Defina o modo claro em `:root` e o escuro em `[data-theme="dark"]`. Componentes só leem variáveis.

**Exportação.** Se o projeto usa Tailwind v4: `npx @google/design.md export --format css-tailwind DESIGN.md > theme.css`. O exportador converte os tokens planos; **as sobreposições do modo escuro (prefixo `dark-`) precisam ser mapeadas manualmente** pela tabela de papéis. Para um formato neutro: `--format dtcg`.

**Validação.** Rode `npx @google/design.md lint DESIGN.md` após editar. Resolva todo `broken-ref` e todo aviso de contraste.

**Dados.** Todo valor técnico exibido (hash, endereço, slot, saldo, estado) vem do backend ou da cadeia. Se um campo não existir ainda, mostre "Indisponível" e registre `TODO(data)`. Nunca preencha com valor plausível.

**Marcadores de pendência.** `TODO(identity)`: valor ou asset dependente de decisão da identidade. `TODO(brand)`: arquivo de marca ausente. `TODO(data)`: campo sem fonte real.

**Verificação visual.** Ao concluir uma tela, abra-a no navegador, tire capturas nos dois modos e critique contra este arquivo e contra a lista de aceitação do `HIVE_MVP_UI_GUIDE.md`. Teste com foco por teclado e com `prefers-reduced-motion`.

## Do's and Don'ts

**Faça**

- Faça os componentes lerem tokens por papel; trocar a identidade deve exigir mexer só no arquivo de tokens.
- Faça o hash, o endereço e a assinatura serem os elementos mais legíveis da tela.
- Faça cada estado carregar ícone, rótulo e família de status.
- Faça o âmbar aparecer no máximo uma vez como ação primária por região.
- Faça o foco de teclado ser sempre visível, com a cor certa para cada modo.
- Faça o cenário que falha ter a mesma qualidade visual do que passa.
- Faça números tabulares, com `−` tipográfico e unidade consistente.
- Faça o texto dizer o que foi verificado, não que algo é "bom" ou "seguro".

**Não faça**

- Não use âmbar para sucesso, aprovação, "ativo" ou como texto/ícone informativo sobre fundo claro.
- Não use `secondary` (malva) como texto sobre `surface-variant` (lilás): 4,0:1, não passa.
- Não use `≈` nem `→` como caracteres de texto; as fontes não os possuem.
- Não aplique mono a todos os números nem use Manrope em hashes.
- Não varie espessura, densidade, tamanho ou cor para sugerir confiança, nível ou score.
- Não coloque halo, escudo, selo, medalha, padrão de favos ou mascote nas telas do produto.
- Não mude a cor ou a forma da célula/`PartyMark` conforme o resultado.
- Não use sombras empilhadas, brilho, degradês decorativos ou relevo.
- Não construa barra de progresso, contagem animada ou estimativa sem dado real.
- Não copie telas, navegação, dados ou textos dos estudos de dashboard do documento de identidade (catálogo de agentes, políticas, "Produção", "128 execuções").
- Não escreva cor, fonte ou medida literal dentro de um componente.
- Não redesenhe, trace ou redigite o logotipo.
