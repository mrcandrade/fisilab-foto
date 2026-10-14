# Modificações do FisiFoto em relação ao PhotoCraft

O **FisiFoto** é uma versão modificada do [PhotoCraft](https://github.com/storytold/photocraft)
(© 2026 ArtCraft Team e contribuidores do PhotoCraft, MIT OR Apache-2.0), disponibilizada pelo
**FisiLab — LARFIC · UFRGS** em parceria com o **CTA — Centro de Tecnologia Acadêmica**, como parte
da Suíte FisiLab, para uso de estudantes, professores e técnicos da UFRGS.

Este arquivo lista o que foi alterado e por quê. As modificações estão sob os mesmos termos do
original (MIT OR Apache-2.0); veja [NOTICE](NOTICE).

## O que mudou

### Nome e marca
- O nome visível do programa passou de **PhotoCraft** para **FisiFoto**: título da janela, nome do
  aplicativo, menus ("Sobre o FisiFoto", "Sair do FisiFoto"), diálogos, mensagens de status e de
  erro, tela inicial, dicas, textos de `--help`/`--version` da linha de comando e as mensagens do
  motor exibidas ao usuário. Os catálogos de tradução (`crates/ui-egui/src/i18n/*.tsv`) foram
  atualizados nas chaves e nos valores, para que as traduções continuem valendo.
- **Por quê:** o NOTICE original diz que o nome, a marca nominativa e as logos da ArtCraft não estão
  cobertos pela licença livre, e que versões modificadas devem removê-los; além disso, a Suíte
  FisiLab tem nomes próprios.
- A pasta `docs/brand/` (logos e marca da ArtCraft) foi apagada, e as referências a ela no README
  original (agora `README.upstream.md`) foram retiradas.
- O ícone do aplicativo (executável, janela, barra de tarefas e a marca no canto da barra de título)
  passou a ser o ícone do FisiFoto (`assets/fisilab/`). O ícone original do PhotoCraft continua em
  `assets/app-icon/` (é MIT OR Apache-2.0 e é usado por testes e pelos pacotes de Linux/macOS).

### Tema de cores
- Dois temas novos, **UFRGS Noite** (escuro, padrão) e **UFRGS Dia** (claro), feitos com a paleta da
  Suíte FisiLab (`crates/fisilab-brand`: o marinho e o vermelho da chama do LARFIC, o creme da logo,
  o azul do CTA e da UFRGS). Eles ficam em primeiro lugar na lista de temas e mantêm o mesmo leiaute
  do tema padrão original (Pro Cinza médio): só as cores mudam.
- UFRGS Noite é o tema padrão para o modo escuro e UFRGS Dia para o modo claro (preferências
  `interface.theme`, `interface.darkTheme` e `interface.lightTheme`). Os demais temas do PhotoCraft
  continuam disponíveis em *Janela › Tema* e em *Preferências › Interface*.
- Testes de contraste foram acrescentados para os dois temas novos.

### Tela de abertura e janela "Sobre"
- Uma tela de abertura com a logo do CTA aparece nos primeiros segundos (um clique ou uma tecla a
  pula; a variável de ambiente `FISILAB_NO_SPLASH=1` a desliga).
- Novo item **Ajuda › Sobre a Suíte FisiLab…**, que abre a janela com os créditos ao projeto
  original, o manifesto e as logos da UFRGS e do CTA. A janela "Sobre o FisiFoto" original foi
  mantida (ela credita os contribuidores do PhotoCraft) e ganhou uma linha de crédito.

### Links e comunidade
- O menu Ajuda, a tela inicial e a janela "Sobre o FisiFoto" não apontam mais para o Discord nem para
  os sites da ArtCraft. Os links agora são: **Site da Suíte FisiLab**, **Manifesto pelas tecnologias
  livres**, **Projeto original (PhotoCraft)** e **Reportar um problema…**
  (`https://github.com/mrcandrade/fisifoto/issues`).
- O botão "Join us on Discord" da tela inicial e da janela Sobre virou o botão **Sobre a Suíte
  FisiLab…**; o atalho "Discord" da barra de título virou o botão **Suíte FisiLab**.
- Contato com servidores do projeto original: o PhotoCraft para desktop não verifica atualizações,
  não envia telemetria nem relatórios de falha e não baixa nada de `getartcraft.com`. Os únicos
  endereços da ArtCraft eram os links acima e os links de "Sobre/Ajuda" do instalador MSI
  (`packaging/windows/photocraft.wxs`), que também foram trocados.

### Idioma padrão
- Numa instalação nova (sem arquivo de preferências), o FisiFoto abre em **português do Brasil**.
  Quem escolher outro idioma (ou "seguir o sistema") em *Preferências › Interface › Idioma* mantém a
  escolha. O padrão do motor continua sendo "auto", então a linha de comando, os testes e as
  preferências antigas não mudam.
- Os itens de menu novos têm texto em português e foram acrescentados a todos os catálogos.

### Executáveis, identificador e pastas
- Os executáveis passaram a se chamar `fisifoto.exe` (aplicativo) e `fisifoto-cli.exe` (linha de
  comando). Os pacotes Cargo continuam `photocraft` e `photocraft-cli`, e os crates internos
  continuam `photocraft-*`.
- O identificador do aplicativo passou de `ai.storyteller.photocraft` para
  `br.ufrgs.fisilab.fisifoto` (app id da janela e classe da janela em Linux).
- As configurações ficam em `%APPDATA%\FisiLab\FisiFoto` no Windows (macOS:
  `~/Library/Application Support/br.ufrgs.fisilab.fisifoto`; Linux: `~/.config/fisifoto`), para não
  se misturarem com um PhotoCraft instalado na mesma máquina. No modo portátil o marcador é
  `portable.txt` ou `FisiFoto.portable` e a pasta de dados é `FisiFotoData`.
- O atalho gerado por *Arquivo › Automatizar › Criar Droplet* e as mensagens do servidor MCP agora
  chamam `fisifoto-cli`/`fisifoto`.
- Recursos do executável no Windows (VERSIONINFO): ProductName "FisiFoto", empresa
  "FisiLab — LARFIC/UFRGS", direitos autorais com crédito ao PhotoCraft, ícone do FisiFoto.
- O script e o WiX do pacote Windows (`packaging/windows/`) foram renomeados para FisiFoto (nomes dos
  executáveis, ProgIds `FisiLab.FisiFoto.*`, novo UpgradeCode, links). O instalador oficial da suíte
  é feito em Inno Setup, fora deste repositório.

### Outros
- A pasta `.github/` (CI, publicação de versões e financiamento do projeto original) foi apagada,
  para que os fluxos do PhotoCraft não rodem neste fork.
- O `README.md` foi reescrito em português; o original virou `README.upstream.md`.
- O `NOTICE` ganhou um cabeçalho do FisiFoto; o texto original segue completo abaixo dele.

## O que **não** mudou
- O motor de edição, as ferramentas, os filtros, os formatos de arquivo e o comportamento do
  programa. O formato nativo continua sendo `.pcraft` (e o filtro "PhotoCraft" nos diálogos de
  arquivo continua com esse nome, por compatibilidade), e os metadados gravados nos arquivos
  (por exemplo, o criador em DPX/EPS/XMP) não foram alterados.
- Nomes internos: crates, módulos, identificadores Rust, comandos (`help.about`, `window.theme.*` …),
  variáveis de ambiente (`PHOTOCRAFT_*`), o protocolo de controle e o nome do servidor MCP.
- `LICENSE-MIT`, `LICENSE-APACHE` e `ATTRIBUTION.md`, a documentação de desenvolvimento (`docs/`,
  `book/`), o histórico de contribuidores e os créditos aos contribuidores do PhotoCraft.
- Os pacotes para Linux, macOS, FreeBSD e web não foram adaptados (apenas o arquivo `.desktop`
  ganhou o novo nome, executável e classe de janela). O aplicativo web continua compilando com a
  mesma interface, mas não é distribuído pela suíte.
