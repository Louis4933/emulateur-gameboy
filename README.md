# Gameboy Emulator

Ce projet est un émulateur Gameboy simple développé en Rust. Il utilise Cargo comme gestionnaire de paquets et de build.

## Prérequis

Assurez-vous d'avoir Rust installé sur votre système avant de procéder.

### Installation
Clonez ce dépôt sur votre machine locale :
```bash
git clone https://github.com/Louis4933/emulateur-gameboy.git

cd emulateur-gameboy
```


### Lancement de l'émulateur

Pour lancer l'émulateur avec une ROM Tetris, utilisez la commande suivante :
```bash
cargo run "./roms/tetris.gb"
```

Cette commande utilise Cargo pour construire et exécuter le projet avec la ROM Tetris spécifiée en tant qu'argument. Assurez-vous que la ROM est dans le répertoire roms du projet.