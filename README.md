# Piłkarzyki
![](https://i.imgur.com/111ChMK.png)
## Opis
Dwuwymiarowa gra rozgrywającą się w czasie rzeczywistym. Gracze podzieleni na dwie drużyny rozgrywają mecz z zasadami zbliżonymi do piłki nożnej na boisku z dwoma bramkami. Celem gry jest zdobycie trzech bramek zanim zrobi to drużyna przeciwna.

## Obecna funkcjonalność
Gra obecnie obsługuję grę dla dowolnej liczby graczy większej niż 2. Gracze łączą się między sobą poprzez przesłanie między sobą `session_id` hosta rozgrywki. Na boisku oboje z drużyn mają swoje bramki. Gracze wchodzą w kolizje między sobą oraz piłką i dodatkowo mogą nią strzelać. Jeśli piłka przejdzie przez linię bramki wyświetlane jest powiadomienie o zdobyciu gola a następnie pozycje graczy i piłki są resetowane.

## Jak zagrać online
Gra jest dostępna pod linkiem (stan na 1.02.2022):
http://rusty-games-footballers.s3-website.eu-central-1.amazonaws.com/

## Jak uruchomić grę lokalnie
Aby użytkownik mógł się łączyć lokalnie należy uruchomić lokalny serwer sygnałowy za pomocą komendy `cargo run` dostępny tutaj:
https://github.com/rusty-games/rusty-games/tree/main/signaling-server

### 1. Pobierz repozytorium na maszynę lokalną
### 2. Otwórz terminal w głównym katalogu repozytorium
### 3. Uruchom w terminalu komendę `wasm-pack build` aby skompilować kod Rust do WASM
Aby kompilacja kodu Rust się powiodła należy zdefiniować zmienne środowiskowe
- `STUN_SERVER_URLS`
- `TURN_SERVER_URLS`
- `TURN_SERVER_USERNAME`
- `TURN_SERVER_CREDENTIAL`

Instalacja wasm-pack na systemach Unixowych: komenda `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
Instalacja na innych systemach: https://rustwasm.github.io/wasm-pack/installer/#
### 4. Przejdź do podfolderu `\webpage`
### 5. Uruchom w terminalu komendę `npm install` aby zainstalować potrzebne paczki
Instalacja Node.js oraz npm na systemach Unixowych: komenda `curl -sL https://deb.nodesource.com/setup_16.x | sudo -E bash -` a następnie `sudo apt install nodejs`
Instalacja na innych systemach: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
### 6. Uruchom w terminalu komendę `npm run start` aby uruchomić lokalny serwer
### 7. Włącz przeglądarkę i przedź na stronę `localhost:8080/`
