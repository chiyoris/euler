# Project Euler w języku Rust 🦀

Moje rozwiązania zadań z portalu [Project Euler](https://projecteuler.net/archives) napisane w języku Rust.

## 🛠 Struktura projektu

Projekt korzysta z mechanizmu **Cargo Workspace**, co pozwala na wygodne zarządzanie wieloma niezależnymi programami w jednym repozytorium:

```text
euler/
├── Cargo.toml              # Główny plik konfiguracyjny przestrzeni roboczej (Workspace)
├── euler_utils/            # Wspólna biblioteka z narzędziami i funkcjami
│   ├── Cargo.toml
│   └── src/lib.rs
└── solutions/              # Katalog z rozwiązaniami
    ├── 001/                # Zadanie nr 1 (pakiet: euler_001)
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── ...

```

---

## 🚀 Jak uruchamiać i testować zadania

Wszystkie komendy należy uruchamiać z **głównego katalogu** repozytorium (tam, gdzie znajduje się główny plik `Cargo.toml`).

### 1. Uruchomienie rozwiązania

Aby uruchomić konkretne zadanie (np. `001` o nazwie pakietu `euler_001`):

```bash
cargo run -p euler_001

```

Dla zadań wymagających większej mocy obliczeniowej, dodaj flagę `--release` (kod wykona się znacznie szybciej):

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

## 📝 Jak dodać nowe zadanie

Ponieważ Cargo nie pozwala, aby nazwy pakietów zaczynały się od cyfr, foldery nazywamy numerami (np. `002`), ale w pliku `Cargo.toml` definiujemy nazwę jako `euler_002`.

Aby wygenerować idealną strukturę dla nowego zadania, skopiuj poniższy kod, **zmień numer w pierwszej linijce** i wklej całość do terminala w głównym katalogu projektu:

```bash
# 1. Zdefiniuj numer zadania (zmień tę wartość na odpowiednią, np. 002, 003, 015)
export NUM="002"

# 2. Stwórz strukturę katalogów dla nowego zadania
mkdir -p "solutions/$NUM/src"

# 3. Utwórz plik konfiguracyjny Cargo.toml
cat <<EOF > "solutions/$NUM/Cargo.toml"
[package]
name = "euler_$NUM"
version = "0.1.0"
edition = "2021"

[dependencies]
euler_utils = { path = "../../euler_utils" }
EOF

# 4. Utwórz startowy plik src/main.rs z gotowym szablonem
cat <<INNER_EOF > "solutions/$NUM/src/main.rs"
// Użycie wspólnej biblioteki (odkomentuj jeśli potrzebne):
// use euler_utils::*;

fn solve() -> u64 {
    // Miejsce na Twoje rozwiązanie
    0
}

fn main() {
    let answer = solve();
    println!("Rozwiązanie dla zadania $NUM: {}", answer);
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

echo "✅ Pomyślnie utworzono szablon dla zadania $NUM!"

```

Po wykonaniu tych komend możesz od razu otworzyć plik `solutions/$NUM/src/main.rs`, pisać kod i uruchomić go poleceniem:
`cargo run -p euler_<twój_numer>`

---

## 🧰 Jak korzystać ze wspólnej biblioteki `euler_utils`

Jeśli napiszesz funkcję, która może przydać się w wielu zadaniach (np. testowanie liczb pierwszych, NWD, sito Eratostenesa):

1. Zdefiniuj ją w pliku `euler_utils/src/lib.rs` i oznacz jako publiczną (`pub`):

```rust
pub fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 { return false; }
    }
    true
}

```

1. Użyj jej w dowolnym zadaniu, importując ją na początku pliku `main.rs`:

```rust
use euler_utils::is_prime;
```
