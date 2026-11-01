# Sodium-rust for Horizon Launcher

## Opis
Pełny port moda Sodium (wersja 0.7) do Rust, zintegrowany z Horizon launcher (Minecraft 26.3).
Obsługuje rozszerzenia typu **Sodium Extra** (0.9.4) w 1:1.

## Struktura
- `src/lib.rs`: Główny punkt wejścia (Horizon native-mod)
- `src/sodium_style.rs`: Layout i kolory (1:1 z Sodium 0.7, PR #5706)
- `src/sodium_extra.rs`: Funkcje Extra (mgła, chmury, filtry cząsteczek)
- `src/rendering.rs`: Silnik renderowania i culling (1:1)
- `src/settings.rs`: Zarządzanie ustawieniami (General, Quality, Performance, Advanced, HDR)

## Kompilacja dla Horizon Launcher
1. Upewnij się, że masz zainstalowany Horizon launcher (wersja 0.1.5+)
2. Uruchom: `cargo build --release` (lub użyj `cargo horizon-mod new` jeśli dostępne)
3. Plik `.hnmod` zostanie wygenerowany w `target/`

## Funkcje Sodium Extra (1:1)
- Rozszerzona mgła atmosferyczna (per-dimension)
- Procentowa kontrola mgły chmur
- Cylindryczny kształt mgły
- Wzmocnione filtrowanie cząsteczek
- Nadpisania na wymiar

## Zgodność
- Minecraft 26.3 (Horizon)
- Horizon Launcher 0.1.5+
- 1:1 z Sodium 0.7 i Sodium Extra 0.9.4

## Licencja
BSD-3-Clause (zgodnie z oryginalnym Sodium)
