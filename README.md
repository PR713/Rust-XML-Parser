# Rust XML Parser Benchmark

Projekt porównywarczy różnych implementacji parserów XML w języku Rust, z dokładnym mierzeniem wydajności czasowej i zużycia pamięci.

## 📋 Opis

Ten projekt implementuje i porównuje 5 różnych metod parsowania plików XML:

1. **whole_file** - Parser ładujący cały plik do pamięci
2. **line_by_line** - Parser przetwarzający plik linia po linii  
3. **my_parser** - Własna implementacja parsera XML
4. **xml-rs** - Parser wykorzystujący bibliotekę `xml-rs`
5. **quick-xml** - Parser wykorzystujący bibliotekę `quick-xml`

Każdy parser konwertuje XML na format NDJSON z odpowiednimi tagami `start_element`, `end_element` i `text`.

Przykładowo:
```json
{"type":"start_element","name":"root","attributes":{"id":"1","class":"main"}}
{"type":"text","content":"Some text content"}
{"type":"end_element","name":"root"}
```

## 🚀 Funkcjonalności

- **Benchmarking** - Precyzyjny pomiar czasu wykonania i zużycia pamięci
- **Monitoring pamięci** - Ciągłe monitorowanie pamięci w osobnym wątku
- **Wizualizacja wyników** - Automatyczne generowanie wykresów PNG
- **Uniwersalny emitter** - Wspólny interfejs wyjściowy dla wszystkich parserów

## 📁 Struktura projektu

```
src/
├── main.rs              # Główna logika benchmarkingu
├── benchmark.rs         # Moduł benchmarkingu i monitorowania pamięci
├── emitter.rs          # Uniwersalny emitter JSON
├── generate_plot.rs    # Generowanie wykresów wyników
├── tools.rs            # Narzędzia pomocnicze dla parserów
├── parsers/            # Implementacje różnych parserów
│   ├── mod.rs
│   ├── parse_xml_whole_file.rs
│   ├── parse_xml_line_by_line.rs
│   ├── my_parser.rs
│   ├── xml_rs.rs
│   └── quick_xml.rs
├── inputs/             # Pliki wejściowe XML
│   └── 85mb.xml
└── outputs/            # Wygenerowane pliki NDJSON
    ├── whole_file.txt
    ├── line_by_line.txt
    ├── my_parser.txt
    ├── xml-rs.txt
    └── quick-xml.txt
```



## 🚧 Możliwe rozszerzenia

- Obsługa większej liczby formatów wejściowych
- Dodanie walidacji XML Schema
- Implementacja parsowania strumieniowego
- Wsparcie dla namespace'ów XML
- Optymalizacje dla bardzo dużych plików (>1GB)


**Autor**: Radosław Szepielak, Dominik Jurkowski
