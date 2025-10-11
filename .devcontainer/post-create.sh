#!/usr/bin/env bash

if [ -f package.json ]; then
  bash -i -c "nvm install --lts && nvm install-latest-npm"
  npm i
  npm run build
fi

# Install dependencies for shfmt extension
curl -sS https://webi.sh/shfmt | sh &>/dev/null

# Add OMZ plugins
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ~/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting
git clone https://github.com/zsh-users/zsh-autosuggestions ~/.oh-my-zsh/custom/plugins/zsh-autosuggestions
sed -i -E "s/^(plugins=\()(git)(\))/\1\2 zsh-syntax-highlighting zsh-autosuggestions\3/" ~/.zshrc

# Avoid git log use less
echo -e "\nunset LESS" >>~/.zshrc

# Install TeX + dvisvgm if not present
if ! command -v latex >/dev/null 2>&1 || ! command -v dvisvgm >/dev/null 2>&1; then
  echo "Installing TeX and dvisvgm..."
  export DEBIAN_FRONTEND=noninteractive
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends \
    texlive-latex-base \
    texlive-latex-extra \
    texlive-fonts-recommended \
    dvisvgm
  sudo apt-get clean
  sudo rm -rf /var/lib/apt/lists/*
else
  echo "latex and dvisvgm already installed"
fi