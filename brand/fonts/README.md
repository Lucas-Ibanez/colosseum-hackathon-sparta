# brand/fonts

Fontes da identidade Hive, **auto-hospedadas**: o `DESIGN.md` (seção *Typography*)
exige que a interface funcione na demo sem depender de CDN de terceiros.

- **Manrope** (variável, eixo de peso 200 a 800): família principal da interface.
- **IBM Plex Mono** (400 e 500): só conteúdo técnico (hashes, endereços, IDs,
  comandos, nomes de erro).

## Origem

Baixadas em 2026-10-07 do Google Fonts (decisão humana: "as fontes citadas estão
disponíveis no Google Fonts, você pode importar elas de lá"):

- CSS: `https://fonts.googleapis.com/css2?family=Manrope:wght@200..800&display=swap` e
  `https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&display=swap`,
  pedidas com um User-Agent de navegador moderno (para receber WOFF2).
- Mantidos só os subconjuntos `latin` e `latin-ext` (português incluído). Cirílico,
  grego e vietnamita ficaram de fora.
- `fonts.css` reproduz os blocos `@font-face` do Google Fonts (família, estilo, peso,
  `unicode-range`), trocando só a URL remota pelo arquivo local.

Arquivos e origem exata:

- `manrope-wght-latin-ext.woff2`: <https://fonts.gstatic.com/s/manrope/v20/xn7gYHE41ni1AdIRggmxSuXd.woff2>
- `manrope-wght-latin.woff2`: <https://fonts.gstatic.com/s/manrope/v20/xn7gYHE41ni1AdIRggexSg.woff2>
- `ibm-plex-mono-400-latin-ext.woff2`: <https://fonts.gstatic.com/s/ibmplexmono/v20/-F63fjptAgt5VM-kVkqdyU8n1iEq129k.woff2>
- `ibm-plex-mono-400-latin.woff2`: <https://fonts.gstatic.com/s/ibmplexmono/v20/-F63fjptAgt5VM-kVkqdyU8n1i8q1w.woff2>
- `ibm-plex-mono-500-latin-ext.woff2`: <https://fonts.gstatic.com/s/ibmplexmono/v20/-F6qfjptAgt5VM-kVkqdyU8n3twJwl5FgtIU.woff2>
- `ibm-plex-mono-500-latin.woff2`: <https://fonts.gstatic.com/s/ibmplexmono/v20/-F6qfjptAgt5VM-kVkqdyU8n3twJwlBFgg.woff2>
- `OFL-Manrope.txt`: <https://raw.githubusercontent.com/google/fonts/main/ofl/manrope/OFL.txt>
- `OFL-IBMPlexMono.txt`: <https://raw.githubusercontent.com/google/fonts/main/ofl/ibmplexmono/OFL.txt>

## Integridade

| Arquivo | Família | Peso | Subconjunto | Bytes | SHA-256 |
| --- | --- | --- | --- | ---: | --- |
| `manrope-wght-latin-ext.woff2` | Manrope | 200 800 | latin-ext | 15120 | `3911b66d9f2e005a4b989223405d0e5032619c668597ba467cc76a23c8fffcfb` |
| `manrope-wght-latin.woff2` | Manrope | 200 800 | latin | 24836 | `a30ddcd349703aff7464c34bef3fffdff405ee50c113440d7c8693c02d210972` |
| `ibm-plex-mono-400-latin-ext.woff2` | IBM Plex Mono | 400 | latin-ext | 13348 | `6bc0f226a5b7884a8170e3f62c63d7675609d4631bdc5931b5cdab81821f00eb` |
| `ibm-plex-mono-400-latin.woff2` | IBM Plex Mono | 400 | latin | 14708 | `08949f728dc52d528e69b1667d15c89a5686a4ee9a296ff90983985f99c380f7` |
| `ibm-plex-mono-500-latin-ext.woff2` | IBM Plex Mono | 500 | latin-ext | 13432 | `6bb06407c97584b0867a959e05e8874693bfeb8c317de190811c51598f2d99ea` |
| `ibm-plex-mono-500-latin.woff2` | IBM Plex Mono | 500 | latin | 14888 | `01d285447409c8a588692162439a038b8cbd7871309ee20267b0d2d91c6e8e22` |
| `OFL-Manrope.txt` | licença | — | — | 4386 | `d6a309fcfb963d329a93baf9df66fa5f3c23a4be2b5f67edf24d6cafecde12ac` |
| `OFL-IBMPlexMono.txt` | licença | — | — | 4362 | `d741e57d5f865e294df801f96b7b5161a88b211df65887e4358d271c9fc5fb4f` |

## Licença

As duas famílias são distribuídas sob a **SIL Open Font License 1.1**
(`OFL-Manrope.txt`, `OFL-IBMPlexMono.txt`), que permite uso, incorporação e
redistribuição com a licença junto. As fontes não foram modificadas. O
`OFL-IBMPlexMono.txt` veio com fim de linha CRLF e foi normalizado para LF (o
repositório normaliza texto; `.gitattributes`); o conteúdo é idêntico. Nas duas licenças, os espaços no fim de linha foram removidos (o repositório
recusa espaço no fim de linha). O SHA-256 da tabela é o do arquivo normalizado; o original era `7e6b2818edbd8f6a01ae80641cc8f16a51080d08fb4e532be3a0b6f74adb07da`.

## Uso

`brand/` é a origem e não é servida diretamente (`brand/MANIFEST.md`). Os arquivos
usados em tempo de execução são copiados para `worker/static/fonts/` por uma cópia
verificada byte a byte (decisão "Fundação da interface Hive" em `docs/decisions.md`).
