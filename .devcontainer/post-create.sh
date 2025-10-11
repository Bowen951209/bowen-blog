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

# Install Node.js (Node 18) with NodeSource if not present
if ! command -v node >/dev/null 2>&1; then
  echo "Node.js not found — installing Node.js (Node 18 LTS)..."
  export DEBIAN_FRONTEND=noninteractive
  curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
  sudo apt-get install -y nodejs
  sudo apt-get clean
  sudo rm -rf /var/lib/apt/lists/*
else
  echo "node already installed: $(node -v)"
fi

# Install Ruby gems from Gemfile if present
if [ -f Gemfile ]; then
  echo "Installing Ruby gems from Gemfile..."
  # Install gems into the project's vendor/bundle directory to avoid global installation permission issues
  bundle config set --local path 'vendor/bundle' || true
  bundle install --jobs=4 --retry=3
fi
