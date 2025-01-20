# Wprowadzenie

Projekt **Rusty Ripper CLI** został opracowany w ramach kursu z zakresu cyberbezpieczeństwa jako implementacja aplikacji umożliwiającej przeprowadzanie ataków słownikowych przy użyciu języka Rust. Aplikacja, inspirowana funkcjonalnościami popularnego narzędzia Jack the Ripper, oferuje obsługę słowników, funkcji skrótu oraz plików haseł w formacie CSV. Celem projektu było stworzenie efektywnego narzędzia umożliwiającego analizę i walidację par login-hasło z wykorzystaniem nowoczesnych i wydajnych mechanizmów dostępnych w Rust.

Aplikacja została zaprojektowana jako interfejs wiersza poleceń (CLI), pozwalający użytkownikom na wykonywanie operacji takich jak:
- ładowanie i wyświetlanie zawartości plików słownikowych,
- stosowanie funkcji skrótu (np. MD5, SHA-256, SHA-512) do słownikowych tokenów,
- weryfikacja par login-hasło z plików CSV względem zaszyfrowanych tokenów.

Projekt wyróżnia się modularną architekturą, wykorzystaniem sprawdzonych bibliotek, takich jak `clap` do przetwarzania argumentów wiersza poleceń czy `sha2` do implementacji funkcji skrótu, oraz dbałością o poprawne obsługiwanie błędów i przypadków użycia. 

W niniejszym raporcie omówiono główne założenia projektowe, strukturę aplikacji, sposób jej implementacji oraz potencjalne obszary zastosowań w kontekście cyberbezpieczeństwa.

---

## Struktura modułu CLI

1. **`src/main.rs`**:
   - Główny punkt wejścia aplikacji, który inicjuje parsowanie argumentów wiersza poleceń poprzez wywołanie `CLI::run_parser()`.

2. **`src/cli/parser.rs`**:
   - Definicja struktury `CLI` oraz implementacja metody `run_parser`, która obsługuje logikę przetwarzania argumentów.

### Parametry CLI

Struktura `CLI` definiuje następujące parametry i flagi:
- `--dictionary (-d)`: Opcjonalny parametr, wskazujący ścieżkę do pliku słownika.
- `--hash`: Opcjonalny parametr, określający funkcję skrótu (`md5`, `sha256`, `sha512`).
- `--passwords (-p)`: Opcjonalny parametr, wskazujący ścieżkę do pliku CSV z loginami i hasłami.
- `--generate (-g)`: Flaga generująca kombinacje tokenów i ich hashów.(tablice tęczowe)
- `--salt`: Opcjonalny parametr określający sól używaną podczas hashowania.

### Logika przetwarzania argumentów

Metoda `run_parser` przetwarza podane argumenty wiersza poleceń i wykonuje odpowiednie operacje na podstawie ich kombinacji:

1. **Tylko `--dictionary`**:
   - Ładowanie i wyświetlanie zawartości pliku słownika za pomocą metody `Dictionary::display`.

2. **Tylko `--hash`**:
   - Wybór funkcji skrótu zdefiniowanej w module `Hasher` i zastosowanie jej do słownikowych tokenów. W przypadku braku wsparcia dla danej funkcji, wyświetlany jest komunikat o błędzie.

3. **Tylko `--passwords`**:
   - Ładowanie i wyświetlanie par login-hasło z pliku CSV za pomocą metody `Passwords::display`.

4. **Kombinacja `--dictionary`, `--hash` i `--passwords`**:
   - Pełny tryb kombinacyjny, w którym dane ze słownika są hashowane, a wyniki weryfikowane względem loginów i haseł z pliku CSV. Proces realizowany jest przez obiekt `Retriver`.

5. **Kombinacja `--dictionary`, `--hash` i `--generate`**:
   - Generowanie i wyświetlanie kombinacji tokenów oraz ich hashów.

6. **Błędne kombinacje**:
   - Jeśli podano nieprawidłowe kombinacje flag, aplikacja wyświetla komunikat o błędzie wraz z pomocą (`CLI::command().print_help()`).

Moduł CLI w aplikacji **Rusty Ripper** umożliwia łatwe i intuicyjne zarządzanie funkcjami aplikacji. Dzięki modularnej implementacji oraz wykorzystaniu biblioteki `clap`, program zapewnia wszechstronność w przetwarzaniu danych i przejrzystą obsługę błędów, co czyni go efektywnym narzędziem w kontekście ataków słownikowych i analizy haseł.

## Moduły aplikacji

Aplikacja **Rusty Ripper** składa się z kilku modułów odpowiedzialnych za różne aspekty jej funkcjonalności. Każdy moduł został zaprojektowany w sposób modularny, umożliwiający przejrzyste zarządzanie kodem i łatwą rozbudowę. W poniższej sekcji opisano najważniejsze moduły i ich role.

### `dictionary.rs`

Moduł `dictionary.rs` obsługuje funkcjonalność ładowania i przetwarzania plików słownikowych.

- **Struktura `Dictionary`**:
  - `path`: Ścieżka do pliku słownika.
  - `content`: Zawartość pliku w formie tekstu.
  - `tokens`: Lista tokenów (słów) wczytanych z pliku.

- **Najważniejsze metody**:
  - `new`: Inicjalizuje obiekt, ładuje zawartość pliku i waliduje jego format.
  - `load_content`: Wczytuje zawartość pliku do pamięci.
  - `validate`: Sprawdza, czy każda linia pliku zawiera dokładnie jedno słowo.
  - `parse_tokens`: Konwertuje zawartość na listę tokenów.

- **Interfejs**:
  - Implementuje `ContentManager`, co umożliwia wyświetlanie informacji o załadowanym słowniku.

---

### `hasher.rs`

Moduł `hasher.rs` odpowiada za hashowanie tokenów przy użyciu różnych funkcji skrótu.

- **Struktura `Hasher`**:
  - `dictionary`: Obiekt `Dictionary` zawierający słowa do hashowania.
  - `hashes`: Lista wygenerowanych skrótów.
  - `tokens`: Lista oryginalnych tokenów.
  - `hash_function`: Wybrana funkcja skrótu (`Sha256`, `Sha512`, `Md5`).
  - `salt`: Opcjonalna sól używana podczas generowania skrótów.

- **Najważniejsze metody**:
  - `new`: Inicjalizuje obiekt i generuje skróty dla tokenów.
  - `load_hashes`: Przetwarza tokeny i tworzy dla nich skróty w sposób równoległy.
  - `hash_tokens_in_parallel`: Obsługuje równoległe przetwarzanie tokenów.

- **Interfejs**:
  - Implementuje `ContentManager` w celu wyświetlania informacji o zastosowanej funkcji skrótu i wynikach hashowania.

---

### `passwords.rs`

Moduł `passwords.rs` obsługuje ładowanie i walidację plików CSV zawierających pary login-hasło.

- **Struktura `Passwords`**:
  - `path`: Ścieżka do pliku CSV.
  - `content`: Zawartość pliku w formie tekstu.
  - `logins`: Lista loginów z pliku.
  - `passwords`: Lista haseł z pliku.

- **Najważniejsze metody**:
  - `new`: Inicjalizuje obiekt i wczytuje zawartość pliku.
  - `load_content`: Parsuje plik CSV i rozdziela dane na loginy i hasła.
  - `validate`: Sprawdza, czy liczba loginów i haseł jest zgodna.

- **Interfejs**:
  - Implementuje `ContentManager`, co umożliwia wyświetlanie informacji o załadowanych danych.

---

### `retriver.rs`

Moduł `retriver.rs` realizuje funkcję weryfikacji par login-hasło z wykorzystaniem wyników hashowania.

- **Struktura `Retriver`**:
  - `tokens`: Lista tokenów użytych do generowania skrótów.
  - `hashes`: Lista wygenerowanych skrótów.
  - `logins`: Lista loginów z pliku CSV.
  - `passwords`: Lista haseł w formie skrótów.

- **Najważniejsze metody**:
  - `new`: Inicjalizuje obiekt, mapując tokeny i skróty na loginy i hasła.
  - `run`: Weryfikuje, czy skróty haseł pasują do skrótów wygenerowanych dla tokenów, i wyświetla zgodne pary login-hasło.

---

## Testowanie aplikacji

W celu zapewnienia wysokiej jakości oraz poprawności działania aplikacji **Rusty Ripper**, opracowano zestaw testów automatycznych, napisanych w języku Bash. Testy weryfikują funkcjonalność aplikacji w różnych scenariuszach, takich jak poprawne przetwarzanie danych, obsługa błędów, czy generowanie wyników w trybie kombinacyjnym. Testy są uruchamiane za pomocą skryptu `test.sh` i obejmują różnorodne przypadki użycia aplikacji.

### Struktura testów

- **Katalog testowy**: Wszystkie pliki testowe znajdują się w katalogu `./tests`.
  - Pliki wejściowe (`*.txt` lub `*.csv`) zawierają dane słowników i par login-hasło.
  - Pliki wyjściowe (`[test_number]-output.txt`) przechowują wyniki działania aplikacji dla danego testu.
  - Pliki oczekiwane (`[test_number]-expected.txt`) definiują poprawne wyniki dla testów.
  - Pliki różnic (`[test_number]-diff.txt`) są generowane w przypadku niezgodności wyników.

### Skrypt testowy

Skrypt testowy obsługuje kilka poleceń:
- **`build`**: Kompiluje aplikację w trybie `release` za pomocą `cargo build --release`.
- **`test`**: Uruchamia wszystkie testy zdefiniowane w skrypcie.
- **`clean`**: Usuwa pliki wyjściowe i różnic po testach.
- **`help`**: Wyświetla pomoc dotyczącą dostępnych poleceń.

### Scenariusze testowe

1. **Testy podstawowej funkcjonalności**:
   - **Test 001**: Weryfikacja działania funkcji `md5` na słowniku i pliku haseł.
   - **Test 002**: Uruchomienie aplikacji bez żadnych parametrów.
   - **Test 003-004**: Weryfikacja działania funkcji `sha256` i `sha512`.

2. **Testy trybu generowania**:
   - **Test 005-007**: Generowanie hashów dla różnych funkcji skrótu (`md5`, `sha256`, `sha512`).

3. **Testy obsługi błędów**:
   - **Test 008**: Obsługa niepoprawnej funkcji skrótu.
   - **Test 009-010**: Weryfikacja zachowania aplikacji w przypadku braku plików wejściowych.

4. **Testy z wykorzystaniem soli**:
   - **Test 011-013**: Generowanie hashów z użyciem soli dla różnych funkcji skrótu.

5. **Dodatkowe scenariusze**:
   - **Test 014**: Weryfikacja wielkości liter w nazwach funkcji skrótu.
   - **Test 015-016**: Obsługa pustego i niepoprawnego pliku słownika.
   - **Test 017**: Obsługa niepoprawnego pliku z hasłami.
   - **Test 018**: Generowanie hashów z nietypową solą.
   - **Test 019-020**: Walidacja niepoprawnych kombinacji flag.

### Metoda weryfikacji wyników

Każdy test porównuje wynik działania aplikacji (`[test_number]-output.txt`) z plikiem oczekiwanym (`[test_number]-expected.txt`) za pomocą polecenia `diff`. Dla testów, gdzie kolejność wyników nie jest istotna, wyjście jest najpierw sortowane. W przypadku wykrycia różnic generowany jest plik `[test_number]-diff.txt`, który ułatwia diagnozowanie problemów.

### Przykładowe uruchomienie

1. Kompilacja aplikacji:
   ```bash
   ./test.sh build
   ```

2. Uruchomienie wszystkich testów:
   ```bash
   ./test.sh test
   ```

3. Czyszczenie wyników testowych:
   ```bash
   ./test.sh clean
   ```

4. Wyświetlenie pomocy:
   ```bash
   ./test.sh help
   ```

---

## Uruchamianie aplikacji

Aplikacja **Rusty Ripper** została zaprojektowana jako interfejs wiersza poleceń (CLI), który obsługuje różne tryby pracy. Poniżej opisano kroki wymagane do skompilowania aplikacji oraz sposób uruchamiania jej w różnych trybach.

### Kompilacja aplikacji

Aby przygotować aplikację do działania, należy skompilować jej kod źródłowy. Proces ten wymaga zainstalowanego środowiska **Rust**. Jeśli Rust nie jest zainstalowany, można go pobrać i zainstalować za pomocą polecenia:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Po zainstalowaniu Rust można skompilować aplikację w trybie **release**:

```bash
cargo build --release
```

Po zakończeniu procesu kompilacji plik wykonywalny będzie dostępny w katalogu `./target/release` pod nazwą `rusty-ripper`.

---

### Tryby pracy aplikacji

Aplikacja obsługuje różne tryby pracy, które można uruchomić za pomocą odpowiednich flag:

1. **Ładowanie i wyświetlanie słownika**:
   ```bash
   ./target/release/rusty-ripper -d <ścieżka_do_słownika>
   ```
   - Wyświetla liczbę tokenów w załadowanym słowniku.

2. **Hashowanie tokenów ze słownika**:
   ```bash
   ./target/release/rusty-ripper -d <ścieżka_do_słownika> --hash <nazwa_funkcji_skrótu>
   ```
   - Dostępne funkcje skrótu: `md5`, `sha256`, `sha512`.

3. **Ładowanie i wyświetlanie par login-hasło**:
   ```bash
   ./target/release/rusty-ripper -p <ścieżka_do_csv>
   ```
   - Wyświetla liczbę par login-hasło w załadowanym pliku.

4. **Tryb kombinacyjny**:
   ```bash
   ./target/release/rusty-ripper -d <ścieżka_do_słownika> --hash <nazwa_funkcji_skrótu> -p <ścieżka_do_csv>
   ```
   - Weryfikuje, czy skróty wygenerowane z tokenów słownika pasują do skrótów haseł z pliku CSV, i wyświetla pasujące pary login-hasło.

5. **Generowanie hashów ze słownika**:
   ```bash
   ./target/release/rusty-ripper -d <ścieżka_do_słownika> --hash <nazwa_funkcji_skrótu> --generate
   ```
   - Generuje i wyświetla pary token-hash.

6. **Generowanie hashów z użyciem soli**:
   ```bash
   ./target/release/rusty-ripper -d <ścieżka_do_słownika> --hash <nazwa_funkcji_skrótu> --generate --salt <sól>
   ```
   - Generuje skróty tokenów z dodaną solą.

---

### Przykłady uruchomienia

1. Wyświetlenie zawartości słownika:
   ```bash
   ./target/release/rusty-ripper -d tests/dictionaries/dictionary-simple.txt
   ```

2. Hashowanie tokenów ze słownika za pomocą funkcji `sha256`:
   ```bash
   ./target/release/rusty-ripper -d tests/dictionaries/dictionary-simple.txt --hash sha256
   ```

3. Tryb kombinacyjny z plikiem haseł:
   ```bash
   ./target/release/rusty-ripper -d tests/dictionaries/dictionary-simple.txt --hash sha512 -p tests/passwords/passwords-sha512.csv
   ```

4. Generowanie hashów z solą:
   ```bash
   ./target/release/rusty-ripper -d tests/dictionaries/dictionary-simple.txt --hash md5 --generate --salt custom_salt
   ```

---

### Obsługa błędów

W przypadku podania nieprawidłowych flag lub ich kombinacji aplikacja wyświetli komunikat o błędzie wraz z informacją o dostępnych opcjach. Przykład:

```bash
Error: Wrong flags combination.
USAGE:
    rusty-ripper [OPTIONS]
```

---
