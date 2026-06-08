# Project Euler w języku Rust 🦀

Moje rozwiązania zadań z portalu [Project Euler](https://projecteuler.net/archives) napisane w języku Rust.

## 🛠 Struktura projektu

Projekt korzysta z mechanizmu **Cargo Workspace**, co pozwala na wygodne zarządzanie wieloma niezależnymi programami w jednym repozytorium:

* **`solutions/`** – folder zawierający poszczególne zadania. Każde zadanie to osobny podfolder o nazwie np. `001`.
* **`euler_utils/`** – wspólna biblioteka pomocnicza. Tutaj trafiają funkcje wielokrotnego użytku (np. testowanie liczb pierwszych, generowanie ciągu Fibonacciego itp.).

---

## 🚀 Jak uruchamiać i testować zadania

Wszystkie komendy należy uruchamiać z **głównego katalogu** repozytorium (tam gdzie znajduje się główny plik `Cargo.toml`).

### 1. Uruchomienie rozwiązania

Aby uruchomić konkretne zadanie (np. `001` o nazwie pakietu `euler_001`):

```bash
cargo run -p euler_001
```

Dla zadań wymagających optymalizacji (gdy kod wykonuje się zbyt długo), dodaj flagę `--release`:

```bash
cargo run -p euler_001 --release
```

### 2. Uruchomienie testów

Każde zadanie posiada wbudowane testy jednostkowe (sprawdzające np. przykładowe dane podane w treści zadania).

* **Testowanie jednego konkretnego zadania:**

    ```bash
    cargo test -p euler_001
    ```

* **Uruchomienie wszystkich testów w repozytorium (wszystkie zadania + biblioteka `euler_utils`):**

    ```bash
    cargo test
    ```

---

## 📝 Jak dodać nowe rozwiązanie (Ściągawka)

Ponieważ Cargo nie pozwala, aby wewnętrzne nazwy pakietów zaczynały się od cyfr, foldery nazywamy numerami (np. `002`), ale w pliku `Cargo.toml` definiujemy nazwę jako `euler_002`.

Aby szybko dodać kolejne zadanie (np. numer **002**), skopiuj i uruchom poniższy blok komend w swoim terminalu:

```bash
# 1. Stwórz strukturę katalogów dla nowego zadania
mkdir -p solutions/002/src

# 2. Utwórz plik konfiguracyjny Cargo.toml z dopuszczalną nazwą pakietu "euler_002"
cat <<EOF > solutions/002/Cargo.toml
[package]
name = "euler_002"
version = "0.1.0"
edition = "2021"

[dependencies]
euler_utils = { path = "../../euler_utils" }
EOF

# 3. Utwórz startowy plik src/main.rs z szablonem kodu i testu
cat <<'INNER_EOF' > solutions/002/src/main.rs
// Użycie wspólnej biblioteki (odkomentuj jeśli potrzebne):
// use euler_utils::twoja_funkcja;

fn solve() -> u64 {
    // Miejsce na Twoje rozwiązanie
    0
}

fn main() {
    let answer = solve();
    println!("Rozwiązanie dla zadania 002: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Tutaj wpisz przykładowy test z treści zadania
        assert_eq!(solve(), 0);
    }
}
INNER_EOF
```

Po wykonaniu tych komend możesz od razu otworzyć plik `solutions/002/src/main.rs`, pisać kod i uruchomić go za pomocą:

```bash
cargo run -p euler_002
```

---

## 🧰 Jak korzystać ze wspólnej biblioteki `euler_utils`

Jeśli napiszesz funkcję, która może przydać się w wielu zadaniach (np. test na liczbę pierwszą):

1. Zdefiniuj ją w pliku `euler_utils/src/lib.rs` jako publiczną (`pub`):

    ```rust
    pub fn is_prime(n: u64) -> bool {
        if n <= 1 { return false; }
        for i in 2..=((n as f64).sqrt() as u64) {
            if n % i == 0 { return false; }
        }
        true
    }
    ```

2. Użyj jej w dowolnym zadaniu (np. w `001`), importując na początku pliku `main.rs`:

    ```rust
    use euler_utils::is_prime;
    ```
