# Piłkarzyki
![](https://i.imgur.com/ZDKIK3I.png)
## Opis
Dwuwymiarowa gra rozgrywającą się w czasie rzeczywistym. Gracze podzieleni na dwie drużyny rozgrywają mecz z zasadami zbliżonymi do piłki nożnej na boisku z dwoma bramkami. Celem gry jest zdobycie jak największej liczby goli dla swojego zespołu w przeciągu ustalonego czasu.

## Obecna funkcjonalność
Gra obecnie obsługuję grę dla dwóch graczy czerwonego i niebieskiego na jednym komputerze. Na boisku oboje z graczy mająswoje bramki. Gracze wchodzą w kolizje między sobą oraz piłką i dodatkowo mogą nią strzelać. Jeśli piłka przejdzie przez linię bramki wyświetlane jest powiadomienie o zdobyciu gola a następnie pozycje graczy i piłki są resetowane.

## Jak uruchomić grę

### 1. Pobierz repozytorium na maszynę lokalną
### 2. Otwórz terminal w głównym katalogu repozytorium
### 3. Uruchom w terminalu komendę "wasm-pack build" aby skompilować kod Rust do WASM
Instalacja wasm-pack na systemach Unixowych: komenda "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
Instalacja na innych systemach: https://rustwasm.github.io/wasm-pack/installer/#
### 4. Przejdź do podfolderu "\webpage"
### 5. Uruchom w terminalu komendę "npm install" aby zainstalować potrzebne paczki
Instalacja npm na systemach Unixowych: komenda "npm install -g npm"
Instalacja na innych systemach: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
### 6. Uruchom w terminalu komendę "npm run start" aby uruchomić lokalny serwer
### 7. Włącz przeglądarkę i przedź na stronę "localhost:8080/"
