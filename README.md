# TimeTracker

## Description
C'est une application qui regarde combien de temps vous passez sur le pc et sur chaque application en utilisant un timer systemd.

## Prérequis
- cron (vérifier que le service est activé)
- rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Installation
```
cargo install
crontab -e
```

Il fau y écrire ceci
```
* * * * * timetracker update
```
