# R-D7 — revisão adversarial final do MVP (CLI, prover, chaves, claims e reprodutibilidade)

Data: 2026-10-05 · Revisor: Claude Code (Opus 5.5, esforço max), sessão
separada e somente leitura · HEAD revisado: `c550a98` (docs D7) sobre
`0ec421b`, `deececa` e `a7c6e8a`.

> Registro feito por outra sessão, com permissão de escrita, a partir da
> resposta da revisão. As seções 1 a 5 reproduzem a resposta sem mudança de
> conteúdo; só a formatação (tabelas e listas) foi refeita em markdown. A
> seção 1 é a versão já formatada pela própria revisão (item 6c).

## Resultado

**APROVADO COM RESSALVAS para o D9 e o D10–D12**, com as condições CR1 a CR8
(seção 5).
- Nenhum achado crítico ou alto; 5 baixos (RD7-01 a 05) e 5 informativos
  (RD7-06 a 10).
- Nenhum achado permite mover fundos para destino escolhido pelo chamador nem
  faz a CLI reportar sucesso falso numa transação que moveu fundos.
- Toda a checagem do D7 reproduziu sem divergência, a partir de um clone.
- Nenhum arquivo do repositório foi alterado e não houve escrita em devnet.

## 1. Preflight e comandos executados

Raiz própria `~/.local/share/vericode-spikes/rd7/` (`bin/env.sh`
`1f9dcd29…`); clone local `rd7/clone`, árvore `ad14a995…`, igual à do HEAD.
Execução de 19:46 a 21:00 (-03:00), sempre um build ou teste pesado por vez,
destacado (`setsid nohup`, `nice`, `CARGO_BUILD_JOBS=2`; prover com 4).

| Checagem | Resultado real |
| --- | --- |
| Git (início e fim) | `main`, HEAD `c550a98`; `status --short` e `--ignored` vazios; `diff --check` 0 |
| Perfil padrão (início e fim) | `~/.rustup`, `~/.cache/solana`, `~/.config/solana` ausentes; `~/.cargo` `d9e12578…`, `~/.avm` `7d29f7f8…`, `~/.docker` `6046f67f…` |
| Locks | raiz `191802b2…`, zkvm `f5236689…`, guest `1116acef…`, `anchor/` `19a1db26…`, `tests-local` `be94760a…`, `cli` `4d979577…`, `prover` `8b76f1e1…` |
| Sementes | `cli` × `tests-local`: 708 = 708, só a raiz renomeada; `prover` × `d4/receipts` `ec0dd8d6…`: 460 = 460, só a raiz; 0 pacotes novos, 0 fontes git |
| Guest | 180.300 B `e09ba8cf…` (disco e blob); igual ao D1c2b.3h/3j e a `d2d/artifacts` |
| Core lane-a `1.85.0` / lane-b `1.89.0` | 42/42 e 42/42 |
| CLI | build release 1 min 08 s, 0 warnings; `test --locked --offline` 14/14 |
| `tests-local` | 61/61 com o verificador `dab6746d…` e 61/61 com o dump `34ae6e5c…` (d4b_receipts 2, escrow 27, fixtures 2, layout 7, regressions 7, settlement 16); cada execução carregou o `.so` do próprio `SBF_OUT_DIR`; 28 CPIs ao verificador em cada |
| Prover | build release 56 min 41 s (`JOBS=4`, RSS 2,1 GB); `test --release` 4/4; `check` (`e09ba8cf…`, `4da06f90…`, `73c457ba`); `verify` do P: tudo `matches`, PASS |
| Receipts | `d4/receipts-out` 28/28; P 7/7; fixtures `d4b/` iguais byte a byte |
| IDL `e8ce2c20…` | discriminadores, contas, flags e argumentos iguais aos da CLI |
| `vericode check`, `job show` (sem keypair) | `check=ok`; P `Released`, T `RefundedOnTimeout`, A `Released`, B `RefundedOnFail`; vaults 0, authority = PDA |
| `getTransaction` | 14 assinaturas do D7, do README e do roteiro com o resultado declarado; W7 com o verificador (99.541 CU) e +1.000.000 na ATA do executor |
| Bytes aterrissados | W5, W7 e W10 = codificação da CLI e receipt P |
| SOL | deployer −120.025.000; buyer +92.822.200; executor +19.990.000; nenhuma tx depois do W10 |
| Segredos | HEAD, 337 blobs do histórico, `d7/{logs,bin,jobs}`, `d4b/logs`: 0 arrays de 64 bytes; 0 base58 de 64 bytes com metade final = pubkey do projeto |

**PoCs (fora do clone):**
- `poc/rpc/fake_rpc.py` `5b064c3f…`, `run.sh` `91afc5a6…` e `accounts.json` `35df1a29…`: RPC falso em 127.0.0.1 contra o binário real da CLI.
- `poc/keys_poc.sh` `c800c62e…`.
- `poc/shim/shim_poc.sh` `277477b7…` e `fake-docker` `40c17914…`: só registra argv.
- `poc/prover/prover_poc.sh` `923daee4…`.

**Resultados:**
- `flip` e `changed` → `UNEXPECTED`; `wrongsig`, `notx`, `simsuccess` e `mainnet` → erro, nada enviado ou nada aceito.
- `overlap6003` e `overlap6000` → PASS com `escrow:N` (RD7-01).
- Shim: S2/S4/S5/S7 (RD7-02).
- Prover: P1 dev mode → panic; P2 → tentativa de Bonsai em 127.0.0.1 (RD7-03); P4 → prova nova com journal == core em 8,7 s; P5 → journal adulterado recusado.

## 2. Achados

| ID | Sev. | Componente | Local | Resumo | Evidência | Recomendação | Bloqueia D9 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RD7-01 | Baixo | CLI, modo negativo | cli/src/tx.rs:106-113 | --expect-error escrow:N aceita uma falha do verificador com o mesmo N. Quando a CPI falha, o runtime registra também Program GZqb… failed: … 0xN. Os códigos 6000–6003 do verificador coincidem com AmountZero/ZeroBuyer/ZeroExecutor/ZeroMint do escrow. Só o rótulo fica errado: a tx falhou e nada se move | log real 5UwKqSJs… (as duas linhas 0x1773); PoC overlap6003 e overlap6000 → PASS com escrow:6003/escrow:6000; verifier:6003 também PASS, corretamente | para escrow:N, exigir que nenhum outro programa tenha linha failed (a falha mais interna é do escrow); teste unitário | Não, com a CR2 |
| RD7-02 | Baixo | shim do Docker | prover/docker-shim/docker:13-34; prover/README.md:78-79 | o shim não é uma allowlist de argv. Argumentos posteriores (--network=host, --pull=always, --privileged, -v /:/host) passam; docker container run … e docker --context x run … passam sem digest e sem --network=none; a tag é aceita em qualquer posição. O único chamador (risc0-groth16 3.0.2, src/prove/docker.rs:52-58) tem argv fixo run --rm -v <work>:/mnt TAG, então hoje não há impacto. O README promete "qualquer outro docker run é recusado" | PoC S1 a S7; log do shim do D7: run --pull=never --network=none --rm -v …/groth16-work:/mnt …@sha256:7f173963… | aceitar exatamente run --rm -v <abs>:/mnt TAG e remontar o argv do zero; corrigir o texto do README | Não |
| RD7-03 | Baixo | prover | prover/src/lib.rs:196,285; prover/Cargo.toml:16; upstream host/client/prove/mod.rs:183-213 | default_prover(), com a feature default bonsai, respeita RISC0_PROVER=bonsai\|ipc\|actor e BONSAI_API_URL+KEY. A prova local depende do ambiente. A solidez não é afetada: o prover exige Composite/Groth16, verifica localmente com os parâmetros default e recusa dev mode | PoC P2: o binário tentou POST http://127.0.0.1:9/images/upload/4da06f90… (porta local fechada); PoC P1: RISC0_DEV_MODE=1 → panic, exit 101 | instanciar LocalProver diretamente ou recusar RISC0_PROVER ≠ local e BONSAI_* | Não, com a CR3 |
| RD7-04 | Baixo | testes e claims | docs/decisions.md:1410; docs/demo-script.md:50-51; cli/tests/instructions.rs:361-374 | --tamper-seal não tem teste em cli/ ("testado offline" sem respaldo). A lógica de casamento do --expect-error (código, linha de log e snapshot) também não tem teste: só o parse é testado. A suíte testa a mesma mutação pelos próprios builders (d4b_receipts.rs:148-151) | grep em cli/; PoCs p1 cobrem o binário | testes de expected_failure (inclusive sobreposição) e do --tamper-seal, ou corrigir o texto; ensaiar a cena 4b em devnet antes de gravar | Não |
| RD7-05 | Baixo | docs e reprodutibilidade | README.md:52,54,66-67; docs/demo-script.md:43; docs/manifest-schema.md:70; docs/escrow-program.md:137 | (a) "sem rede, exceto o crates.io": o build do prover baixa recursion_zkr.zip do S3, salvo com RECURSION_SRC_PATH, além do toolchain do rustup. (b) Os requisitos omitem a CLI do Agave 2.3.9 (usada no dump) e o pull prévio da imagem Groth16 por digest (o prover nunca faz pull). (c) A cena 4a usa uma "receipt de outro Job" que um clone limpo não tem: as fixtures são .txt em hex, não o diretório binário. (d) Erratas: "a implantar em devnet depois da R-D4a" (conhecida, exige Plan Mode) e "Como o escrow também será finalizado" (já finalizado) | leitura; build do clone | corrigir antes do congelamento dos claims (CR1) | Não, com a CR1 |
| RD7-06 | Info | chaves e log | cli/src/keys.rs:91-113; cli/src/rpc.rs:75-83 | um hardlink fora do work tree para um inode que também está dentro é aceito; --log só aplica 0600 a arquivo novo (um já existente com 0644 continua 0644). Os logs não contêm segredo | PoC K6 e L2 | opcional: nlink == 1; set_permissions(0600) ao abrir | Não |
| RD7-07 | Info | CLI, robustez | cli/src/main.rs:386,594-607,654-662; rpc.rs:179-185,238-247 | as leituras depois da tx usam confirmed sem minContextSlot; um sendTransaction positivo reenviado após timeout pode receber "already processed". Resultado possível: falso erro depois de uma tx aterrissada, nunca falso sucesso. É risco para a demo | leitura | minContextSlot = slot da tx; tratar "already processed" por getSignatureStatuses | Não |
| RD7-08 | Info | CLI, modo negativo | cli/src/main.rs:631-650; tx.rs:151-174 | um negativo pode virar liquidação real se o estado mudar entre a simulação e a aterrissagem (ex.: escrow:6021 perto do prazo). A CLI reporta UNEXPECTED (exit 1), e o programa só paga a parte fixa | PoC flip: tx com sucesso e vault esvaziado → UNEXPECTED | exigir prazo − slot ≥ 60 no negativo 6021 | Não, com a CR2 |
| RD7-09 | Info | CLI, mensagens | cli/src/main.rs:561-566; tx.rs:249-255 + main.rs:591-593 | "too close to the deadline" num Job vencido (confirmado no log do W8); a linha […] PASS é impressa antes de "settlement landed without invoking the Groth16 verifier" (exit 1) | log do W8; PoC noverifier | cosmético | Não |
| RD7-10 | Info | CLI, rede | cli/src/main.rs:161-165,290-301; rpc.rs:70-74 | --rpc-url aceita http:// e qualquer servidor que devolva o genesis de devnet; o reqwest respeita HTTP(S)_PROXY/ALL_PROXY; check não confere o hash do ProgramData do verificador (34ae6e5c…), só a authority. A guarda é contra configuração errada; o programa é a autoridade e as chaves são só de devnet | PoC: o RPC falso passa pela guarda; mainnet → recusado | opcional: exigir https; acrescentar o hash do verificador ao check | Não |

## 3. Situação de RD4A-01 a RD4A-08

| Item | Situação | Evidência |
| --- | --- | --- |
| RD4A-01 | Fechado na prática | D4b usou --buffer; nenhum deploy desde então; Agave só via run_cli, saída em logs 0600; varreduras sem mnemônico. A regra vale para qualquer deploy futuro |
| RD4A-02 | Mitigado; resíduo aceito | mint Tokenkeg, 6 decimais, sem freeze (check e mint decodificado); a CLI confere decimais e freeze; escrow.rs documenta "9 decimais aceito". O programa imutável não confere decimais |
| RD4A-03 | Mitigado | job_id vem de getrandom e a PDA livre é conferida; S, A, B, C, P e T consumidos. Resíduo: front-run do job_id só causa DoS, sem perda (create+fund atômicos) |
| RD4A-04 | Fechado | docs/escrow-program.md:114-116 ("checagem de formato") |
| RD4A-05 | Fechado | README.md:41; escrow-program.md:34,131 |
| RD4A-06 | Aceito e aberto | sem e-stop; README.md:117 |
| RD4A-07 | Fechado | (a)/(b) no 0ec421b (settlement.rs:219-222, groth16_fixtures.rs:1-7, fixtures README); (c)/(d) no registro do R-D4a; (e) .env.example; (f) d4b_receipts.rs, variantes do mint e verificador ausente |
| RD4A-08 | Fechado | só cdf6967f… implantado; vericode check confirma hoje 395.064 B cdf6967f… com authority none |

## 4. Checklist

1. **Instruções: verificado, sem achado.**
   - IDL `e8ce2c20…` = CLI: discriminadores = `sha256("global:…")[..8]`,
     mesma ordem e flags (`fund.buyer` é signer readonly; vault, job e
     destino são writable).
   - Programa (`lib.rs:388-473`) igual.
   - Bytes aterrissados de W5, W7 e W10 iguais aos da CLI.
   - `CreateIdempotent` só quando a ATA não existe (`main.rs:475-479`; PoC
     "missing; prepended").
   - O destino é `ata(job.executor|job.buyer)` escolhido pelo veredito
     (`main.rs:536-580`) e reimposto pelo programa
     (`require_canonical_destination`). Não existe caminho de destino
     escolhido pelo chamador.
2. **Conferências e modo negativo: RD7-01 e RD7-08.**
   - Fora de `--expect-error`, qualquer falha de conferência recusa a
     operação (`Prechecks::finish`).
   - `--expect-error` exige `Custom(code)` na tx e a linha `failed` do
     programa indicado, além do snapshot igual com `minContextSlot` = slot da
     tx.
   - PoCs: `flip` (tx com sucesso) e `changed` (estado alterado) →
     `UNEXPECTED`, exit 1; `simsuccess` → nada enviado.
   - `--tamper-seal` sem `--expect-error` é recusado antes de qualquer RPC.
   - O prefixo `PROGRAMA:CÓDIGO` desfaz a ambiguidade para `verifier:N`, mas
     não para `escrow:N`.
3. **Envio: verificado; RD7-07 como info.**
   - Simulação com `sigVerify=true` e `replaceRecentBlockhash=false`.
   - Reenvio dos mesmos bytes a cada ~6 s; para no `lastValidBlockHeight`
     com consulta ao histórico.
   - A assinatura devolvida pelo RPC é comparada com a local (PoC `wrongsig`
     → erro).
   - `getTransaction` com 30 tentativas; PoC `notx` → erro, nunca PASS.
   - Cadência de 250 ms; backoff de 1 a 16 s em 429/5xx/−32016/−32005/−32004.
4. **Cluster e rede: verificado; RD7-10 como info.**
   - Guarda por genesis (PoC mainnet → recusado em `check` e `job show`).
   - `rustls` + webpki-roots; `ldd` sem libssl.
   - URLs no binário: só `api.devnet.solana.com` e o Explorer.
5. **Chaves: verificado; RD7-06 como info.**
   - PoC K1–K8: recusa work tree (inclusive via symlink e `.git` como
     arquivo), modos 640/604/660/644, diretório e lixo (mensagem sem
     conteúdo); aceita 400 e 700.
   - Só pubkeys são impressas.
   - `cli-tx.jsonl` tem `0600` e só campos públicos.
   - Varreduras limpas; Agave só via `run_cli`.
6. **`job_id` e prazos: verificado.**
   - `getrandom`; `0x11` recusado na CLI e no prover; PDA do Job e do vault
     livres; janela [1.560, 1.512.000] = core 1.500/1.512.000 + 60.
   - Saldos de SOL (rent + taxa) e de Test USDC conferidos.
   - F-09 residual: só DoS.
   - A mensagem "too close" é cosmética (RD7-09).
7. **Prover: verificado; RD7-02 e RD7-03.**
   - `include_bytes!` com tamanho, SHA-256 e ImageID conferidos em todo
     comando.
   - Frame igual ao `zkvm/host` (76 B); journal == core; `Composite` →
     `Groth16`; parâmetros default; seal de 256 B.
   - `disable-dev-mode` em vigor (P1); `Fake` impossível.
   - PoC P4: prova nova pelo clone limpo, `journal_equal_to_core=true`,
     8,7 s, 0,6 GB. PoC P5: journal adulterado recusado.
   - No D7, o shim do repositório foi o efetivamente usado: há log escrito
     por ele.
   - `proof.json` é de root com `0644`, mas é dado público num diretório do
     usuário.
   - O `tempdir()` do risc0-groth16 usa `TMPDIR` mesmo com `RISC0_WORK_DIR`
     definido.
   - Proveniência D1c2b.3h confirmada. ImageID não recertificado, mas os
     testes de equivalência journal == core cobrem (7,14), (7,15) e (21,42).
8. **Testes novos: verificado; lacuna na RD7-04.**
   - Fixtures `d4b/` vinculadas por hash a `receipts-out`.
   - Replay cobre 6017, 6014, 6000/6003 (verificador), 6033, 6019, 6021 e a
     invariante 9 (6007/6008, inclusive timeout em Job terminal).
   - Variantes do mint: 6024, 3007, 9 decimais aceito e 3012. Verificador
     ausente → `UnsupportedProgramId`, estado igual.
   - Só acréscimos; nada tautológico (as comparações são contra Anchor e
     `spl_token`).
9. **Evidência em devnet: verificado.**
   - As 14 assinaturas aterrissaram com o resultado declarado.
   - W7: verificador invocado com 99.541 CU; vault `MW9jmNCe` −1.000.000 →
     ATA do executor `HpZkHZ59` +1.000.000.
   - W10: `9yrCiJML` → `61tkoEv4`.
   - W6: `InstructionError(1, 6014)`, com o `deliver` revertido.
   - W8/W9: 6007/6008 sem CPI, ou seja, a invariante 9 foi realmente
     exercitada.
   - Estados finais conferidos; saldos fecham por lamport.
10. **Claims: verificado; erratas em RD7-02/04/05.**
    - Nenhum "Verifier Router" como caminho atual (só em seções históricas
      rotuladas); nenhum "ZK on-chain" genérico, mainnet, "trustless" ou "o
      código está correto" como alegação.
    - CD7 com links; limitações presentes.
    - O plano B do roteiro manda dizer que as transações são de execução
      anterior.
11. **Reprodutibilidade: verificado, com lacunas de docs (RD7-05).**
    - Tudo `--locked --offline` a partir do clone, com homes isoladas
      copiadas.
    - Caminho para terceiros sem chaves: `check` e `job show` funcionam sem
      keypair, e os links do Explorer batem.
    - Não reproduzível por terceiros: as escritas, porque o Test USDC só sai
      da mint authority (= deployer `617ogw9T…`).
    - Observação: "homes novas" de verdade exigem rede (crates.io, rustup,
      S3).
12. **Riscos e prontidão: pronto com condições.**
    - Riscos: sem e-stop; rent preso de 3.581.400 lamports por Job, mais a
      taxa; mint authority = deployer; RPC público; memória (prover com
      56 min de build com JOBS=4; compressão a ~142 MB livres no D7).
    - Saldos: buyer 0,128 SOL (~35 Jobs) e 999.997 Test USDC; executor
      0,030 SOL; deployer 2,805 SOL.

**Invariantes do guia §7:**
- 1 a 10 continuam válidas: este gate não mudou o programa.
- 8 (journal de outro Job, W6), 9 (W8/W9) e 10 (6003/6000 do D4b;
  verificador ausente na suíte) foram reconferidas.
- A CLI não acrescenta nenhuma regra econômica.

## 5. Veredito para o D9 e o D10–D12: APROVADO COM RESSALVAS

**Condições:**
- **CR1 (docs, antes de gravar):**
  - corrigir RD7-05 (a)–(d), o texto de RD7-04 e o de RD7-02;
  - `manifest-schema.md` exige Plan Mode;
  - congelar a lista de frases permitidas.
- **CR2 (negativos):** usar só `escrow:6014`, `escrow:6007/6008`,
  `escrow:6021` (logo após criar o Job, ≥ 60 slots antes do prazo) e
  `verifier:6003`. Nunca `escrow:6000–6003`. Corrigir a RD7-01 antes é
  opcional e exige teste e revisão curta.
- **CR3 (prover):**
  - `RISC0_PROVER=local`, com `BONSAI_*` e `RISC0_DEV_MODE` ausentes,
    registrados no log;
  - `compress` sozinho e destacado, imagem por digest já presente, sem pull;
  - anexar o `docker-shim.log`.
- **CR4 (ambiente limpo):** clone novo mais homes novas copiadas das homes
  isoladas offline, ou fetch autorizado em lista fechada (crates.io, rustup
  1.89.0, `recursion_zkr.zip` com hash `744b999f…`). Declarar qual foi
  usado; nunca o perfil padrão.
- **CR5 (escritas):** lista fechada com autorização humana; `job_id` novos;
  nunca reutilizar S, A, B, C, P ou T.
- **CR6 (cena 4a):** origem explícita da "receipt de outro Job": a receipt P
  do D7 (`d7/receipts/P`) ou a receipt do novo P′ aplicada ao novo T′.
- **CR7 (memória):** um processo pesado por vez, destacado.
- **CR8 (D10–D12):**
  - camadas finas, sem regra econômica nova;
  - sem chave no front, nos logs ou no repositório;
  - Explorer, frase de limite e estado Proving visíveis;
  - worker com prova local forçada;
  - a UI não deve rotular qual programa rejeitou a partir do PASS do
    `--expect-error` enquanto a RD7-01 estiver aberta;
  - revisão curta final de claims e chaves.

**Decisões pendentes (humanas):**
- aprovar a lista de escritas do D9;
- qual opção da CR4 usar;
- se a RD7-01, 03 e 04 serão corrigidas em código antes da demo. Recomendo
  não mexer em código antes de gravar.

Essas três decisões foram tomadas em "Decisões humanas para o D9"
(`docs/decisions.md`).

**Riscos abertos:** os do item 12, mais as RD7-01 a 10.

## Incidentes da revisão

- O primeiro driver de builds falhou com exit 127 (funções do `env.sh`
  invisíveis em `bash -c`); a fila foi refeita com funções no próprio script.
- O keypair de PoC (localhost, nunca financiado, pubkey `CQoH1DVi…`) foi
  gravado primeiro em `d2c/poc/keys`, porque o `env.sh` redefine `R`. Foi
  movido para `rd7/poc/keys`; o `d2c/poc` vazio foi removido com `rmdir`; o
  segredo foi apagado no fim.

## Fronteiras

- Rede: só `api.devnet.solana.com`, somente leitura, e `127.0.0.1`.
- Sem Docker real, `compress`, assinatura em devnet, instalação, `/tmp`,
  commit ou push.
- `d4/keys`: só `pubkeys.txt` foi lido.
- `git status --short` e `--ignored` vazios no fim; perfil padrão inalterado.

## Artefatos fora do clone

`~/.local/share/vericode-spikes/rd7/`:
- `bin/` (`env.sh`, `builds2.sh`, `rpc_check.py`, `ix_check.py`, `secret_scan.py`);
- `logs/` (`timeline.log`, `c1`–`c10`, `r1`–`r6`, `p1`–`p4`);
- `poc/`;
- `clone/`, `targets/` e `homes/zkvm`;
- `out/{rebuild,devnet}`.
