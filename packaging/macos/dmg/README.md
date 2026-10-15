# Janela do DMG (macOS)

Dois arquivos prontos, copiados por `package.sh` para dentro da imagem (nada é gerado na hora do build):

| Arquivo | O que é |
|---|---|
| `background.tiff` | Imagem de fundo da janela (HiDPI, 1x e 2x). Vai para `.background/background.tiff`. |
| `dmg-layout.DS_Store` | Ajustes do Finder para o volume: tamanho da janela, tamanho e posição dos ícones e o fundo. Vai para `.DS_Store`. |

Ambos são gerados por `suite-fisilab/scripts/gerar_mac.py`, no repositório
[mrcandrade/larfic](https://github.com/mrcandrade/larfic). Para alterar o visual, mude lá e copie os
arquivos novos para cá.

O layout espera estes nomes (exatamente):

- volume: `Foto` (sem versão; o Finder acha o fundo pelo nome do volume);
- aplicativo: `Foto.app`;
- atalho para a pasta de programas: `Aplicativos`.
