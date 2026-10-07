# brand/MANIFEST.md

Catálogo dos assets de marca da Hive. **Fonte de verdade para o agente de código:** em caso de dúvida sobre qual arquivo usar, valem as colunas "Quando usar" e "Canônico". Não deduza pelo nome da pasta.

> Estado atual (identidade de 05.10.2026): a direção do logotipo (opção 5C, abelha em voo com corpo em células) está escolhida, mas **o desenho mestre vetorial ainda não foi construído**. As pranchas do documento de identidade são estudos gerados por imagem e **não são arquivos de produção**.
>
> Existem SVGs **provisórios**, gerados por rastreio da prancha 1 (veja *Notas dos arquivos provisórios*). Eles servem para a demonstração do hackathon e devem ser **substituídos pelo mestre**. Marque um arquivo como `canônico` somente quando ele vier do mestre vetorial.

## Regras para o agente

1. Use os arquivos **como estão**. Não redesenhe, não trace a partir de imagens, não converta a prancha do PDF em SVG.
2. Não redigite "hive" com a fonte da interface para simular o logotipo.
3. Não gire, espelhe, distorça, recolora fora das variantes abaixo, nem aplique sombra, brilho, relevo, textura ou degradê.
4. Nunca separe ou reposicione as partes da abelha. O símbolo é uma unidade.
5. Respeite a área de proteção e os tamanhos mínimos (seção abaixo).
6. A abelha não é selo de aprovação. Resultado de verificação usa os rótulos e ícones de status do `DESIGN.md`.
7. O mascote não aparece nas telas do produto.
8. Se um arquivo necessário não existir, use o `BrandMark` provisório (`TODO(brand)`) e **avise**; não improvise um desenho.

## Estrutura

```
brand/
├── MANIFEST.md          este arquivo
├── logo/                assinaturas (símbolo + nome) e wordmark
├── symbol/              símbolo isolado e versão reduzida
├── app-icons/           favicon e app icon
├── mascot/              mascote (uso institucional; fora do produto)
└── reference/           pranchas e estudos NÃO canônicos (só para consulta humana)
```

Convenção de nomes: `hive-<asset>-<variante>.<ext>`, minúsculas, hífens. **SVG é o formato-matriz.** PNG só para destinos que não aceitam SVG (favicon legado, app icon, redes sociais).

Arquivos usados em tempo de execução são copiados de `brand/` para a pasta pública da aplicação (ex.: `public/brand/`) por script ou etapa de build. `brand/` é a origem e **não** é servida diretamente.

## Catálogo

Legenda de status: `provisório` (SVG rastreado da prancha; uso temporário), `pendente` (a produzir a partir do mestre), `estudo` (existe só na prancha), `pronto` (derivado do mestre e validado).

### logo/

| Arquivo | O que é | Quando usar | Fundo | Status | Canônico |
|---|---|---|---|---|---|
| `hive-horizontal-berinjela.svg` | Abelha à esquerda, wordmark à direita; asas em âmbar, corpo e nome em berinjela | Cabeçalhos, documentos, materiais sobre fundo claro | Claro | provisório | não |
| `hive-horizontal-branco.svg` | Assinatura horizontal inteira em branco quente | **Navegação lateral da aplicação** (fundo berinjela), apresentações escuras | Escuro | provisório | não |
| `hive-horizontal-ambar.svg` | Assinatura horizontal inteira em âmbar | Destaque e aplicações especiais | Escuro | provisório | não |
| `hive-vertical-berinjela.svg` | Abelha acima do wordmark | Capas, composições centrais, espaços estreitos com altura disponível | Claro | pendente | não |
| `hive-vertical-branco.svg` | Versão vertical em branco | Idem, sobre fundo escuro | Escuro | pendente | não |
| `hive-wordmark-berinjela.svg` | Somente a palavra "hive" | Só quando o contexto já identifica a marca | Claro | provisório | não |
| `hive-wordmark-branco.svg` | Somente a palavra "hive" | Idem, sobre fundo escuro | Escuro | provisório | não |
| `hive-wordmark-manrope600-berinjela.svg` | **Alternativa**: contornos reais da Manrope 600, conforme a regra escrita da identidade. Visivelmente mais leve que a prancha | Somente se a identidade decidir seguir a regra escrita em vez da prancha | Claro | provisório | não |

### symbol/

| Arquivo | O que é | Quando usar | Fundo | Status | Canônico |
|---|---|---|---|---|---|
| `hive-symbol.svg` | Abelha 5C: duas asas, cabeça facetada, abdômen facetado com recorte | Aplicação principal sobre fundo claro (asas âmbar, corpo berinjela) | Claro | provisório | não |
| `hive-symbol-ambar.svg` | Símbolo inteiro em âmbar | Sobre fundo berinjela | Escuro | provisório | não |
| `hive-symbol-branco.svg` | Símbolo inteiro em branco quente | Sobre fundo berinjela | Escuro | provisório | não |
| `hive-symbol-reduced.svg` | Versão reduzida: silhueta e quatro componentes mantidos, espaços internos ampliados | Tamanhos pequenos onde a assinatura completa perde legibilidade | Ambos | estudo | não |

### app-icons/

| Arquivo | O que é | Quando usar | Status | Canônico |
|---|---|---|---|---|
| `favicon.svg` | Abelha reduzida em uma cor, área quadrada; preferencial âmbar sobre berinjela | Favicon da aplicação (navegador) | estudo | não |
| `favicon-32.png` | Versão de 32 × 32 px | Favicon complementar | estudo | não |
| `favicon-16.png` | Versão de 16 × 16 px (exceção para microaplicações, exige simplificação própria) | Só se o desenho permanecer legível | estudo | não |
| `app-icon-principal.png` | Fundo berinjela, símbolo âmbar, cantos arredondados, **sem a palavra "hive"** | Ícone de aplicativo | estudo | não |
| `app-icon-alternativo.png` | Fundo âmbar, símbolo berinjela | Variante | estudo | não |
| `app-icon-mono-escuro.png` | Fundo berinjela, símbolo branco quente | Variante monocromática | estudo | não |
| `app-icon-mono-claro.png` | Fundo branco quente, símbolo berinjela | Variante monocromática | estudo | não |

### mascot/

| Arquivo | O que é | Quando usar | Status | Canônico |
|---|---|---|---|---|
| `hive-mascot.svg` | Mascote em células, vista frontal | **Fora do produto.** Vídeo, materiais institucionais, apresentações | estudo | não |

### reference/ (não canônico)

| Arquivo | O que é |
|---|---|
| `Hive-posicionamento-identidade-2026-10-05.pdf` | Documento de identidade completo; as pranchas são estudos gerados por imagem |

| `preview-logo-provisorio.png` | Folha de comparação entre os SVGs provisórios e a prancha de origem |

O conteúdo de `reference/` serve para consulta. **Nunca extraia desenhos dele para uso na aplicação.**

## Notas dos arquivos provisórios

**Como foram gerados.** Os SVGs de `symbol/` e `logo/` vêm de um rastreio da prancha 1 do PDF de identidade (extraída em resolução nativa, 1536 × 1024 px). O símbolo foi ajustado como polígonos com cantos arredondados (três das quatro peças) e curvas (a asa inferior); a sobreposição com a fonte ficou acima de 0,99 por peça. O wordmark foi rastreado letra a letra, e o pingo do "i" foi substituído por um círculo exato. A assinatura horizontal foi montada com a escala, a lacuna e o alinhamento vertical **medidos na própria versão horizontal da prancha**; sua sobreposição com a prancha é de 0,94.

**Cores.** Os preenchimentos usam os hex oficiais (`#E8B44C`, `#392D40`, `#FAF7F5`), não as cores amostradas da imagem.

**Limites.** A fonte tem baixa resolução e pequenas imperfeições de geração, que o rastreio herda: há leves ondulações nos cantos das letras, e os raios dos cantos do símbolo foram estimados. Nenhum arquivo passou por teste de redução óptica nem de tamanho mínimo.

**Conflito da identidade (decisão pendente).** O documento escreve que a base do wordmark é Manrope Semibold 600, mas a prancha mostra letras bem mais pesadas: a proporção de tinta do desenho (0,50) é maior que a da Manrope 800 (0,44), o peso máximo da família. O melhor ajuste com a Manrope fica em IoU 0,77. Por isso existem dois wordmarks: o rastreado (fiel à prancha, usado nas assinaturas) e o `manrope600` (fiel à regra escrita). A identidade precisa escolher um.

**Ainda não existem:** versão vertical, símbolo reduzido, favicon e app icons.

## Área de proteção e tamanhos mínimos

Valores propostos no documento de identidade. **São candidatos a teste, não limites comprovados.** Em interface web referem-se ao tamanho de exibição, não à resolução do arquivo.

- `H` = altura total do símbolo (do ponto mais alto da asa ao ponto mais baixo do corpo). Margem mínima `x = H ÷ 4` em todos os lados da caixa que contém a assinatura inteira. Na assinatura vertical, `H` continua sendo a altura da abelha.
- Wordmark isolado: 72 px de largura. Assinatura horizontal: 128 px de largura. Assinatura vertical: 80 px de largura. Símbolo isolado: 24 px de altura. Favicon: 16 × 16 px (simplificado) e 32 × 32 px.
- Valide cada tamanho em fundo claro e escuro (ponto do "i", aberturas das letras, espaços entre as partes da abelha, recorte do corpo). Se um detalhe se perder, amplie a aplicação ou use a versão reduzida. Não reduza a assinatura completa ao tamanho de um favicon.

## Uso na aplicação

| Local | Arquivo |
|---|---|
| Navegação lateral (fundo berinjela, também no modo claro) | `hive-horizontal-branco.svg` |
| Navegação lateral no modo escuro | `hive-horizontal-branco.svg` |
| Aba do navegador | `favicon.svg` (com `favicon-32.png` de reserva) |
| Estado vazio, erro, carregamento | **Nenhum.** Sem logotipo, sem mascote. |
| Rodapé / barra de status | Nenhum |

## Pendências (da identidade)

- Construir **um único mestre vetorial** do 5C; derivar todas as variantes dele.
- Ajuste real da Manrope no wordmark, convertido em contornos.
- Redução óptica e teste dos tamanhos mínimos.
- Exportar os arquivos finais (SVG e PDF vetoriais como matrizes).
- Verificar exclusividade e disponibilidade para registro (não confirmadas).
