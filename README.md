# TimeTracker

## Description
C'est une application qui regarde combien de temps vous passez sur le pc et sur chaque application en utilisant un timer systemd.

## Prérequis
Installer les outils de rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation
```
wget https://github.com/fadri14/TimeTracker/archive/refs/heads/main.zip
unzip main.zip
rm -fr main.zip
cd TimeTracker-main
cargo build --release
sudo ./install.sh
rm -fr TimeTracker-main
```

