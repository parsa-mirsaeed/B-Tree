# Foundations of Information Retrieval: Academic B-Tree Engine

[![Language](https://img.shields.io/badge/Language-Rust_1.83-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Course](https://img.shields.io/badge/Course-4594144453-blue.svg?style=flat-square)](http://www.hadisaboohi.com/IR/)
[![Reference](https://img.shields.io/badge/Ref-Manning_2008-lightgrey.svg?style=flat-square)](https://nlp.stanford.edu/IR-book/)
[![Standards](https://img.shields.io/badge/Standards-ISO%2FIEC_10646-00599C.svg?style=flat-square)](https://www.unicode.org/)

> **Course:** Foundations of Information Retrieval and Web Search  
> **Code:** 4594144453 (Semester 1403/1404-2)  
> **Instructor:** Dr. Hadi Saboohi, Assistant Professor  
> **Institution:** Islamic Azad University, Karaj Branch

## 📖 Technical Report

The complete academic technical report is available here:

- **📄 [Foundations of Information Retrieval: Artifact Report](docs/technical_report/Foundations_of_IR_B-Tree_Report.pdf)** — Full technical documentation (PDF)
- **📝 [Report Source](docs/technical_report/)** — Typst source code and assets

The report covers:
- Chapter 1: Inverted Index Concepts
- Chapter 2: Persian Token Normalization (Natural Sort)
- Chapter 3: B-Tree Dictionary Structure
- Chapter 4: Index Construction Implementation
- Standards Compliance (ISO/IEC 10646, 25010)

---

## 📚 Syllabus Alignment

This engineered artifact implements core concepts from the course **Lecture Notes** and the main reference text (**Manning, Raghavan, Schütze 2008**):

| Course Module                     | Implementation Feature                                                                                                                      |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| **Chapter 1: Introduction**       | **Inverted Index Concepts:** Foundation for term-document matrices and index structures.                                                    |
| **Chapter 2: Term Vocabulary**    | **Persian Natural Sort:** Solving token normalization for extended characters ([translate:پ], [translate:چ], [translate:ژ], [translate:گ]). |
| **Chapter 3: Tolerant Retrieval** | **Dictionary Structure:** B-Tree implementation for efficient term lookup and range queries.                                                |
| **Chapter 4: Index Construction** | **B-Tree Engine:** A strictly typed, Order-4 B-Tree demonstrating dynamic index scaling and node splitting.                                 |

## 🏗 Engineering Standards & Architecture

This project is engineered to meet **ISO/IEC 25010** (Systems and software quality models) and **ISO/IEC 10646** (Unicode character encoding), prioritizing:

1. **Functional Suitability:** Strict adherence to Manning (2008) Chapters 2, 3, and 4 for term vocabulary, tolerant retrieval, and index construction.
2. **Reliability:** Memory safety guaranteed by Rust's ownership model and compile-time verification.
3. **Internationalization:** Proper handling of Persian character encoding and linguistic collation weights.
4. **Portability:** Zero-JS WebAssembly architecture (Dioxus) for cross-platform browser execution.

### 📐 Algorithmic Standards

- **Data Structure:** B-Tree (Knuth Vol. 3 Definition).
- **Order:** m = 4 (maximum 3 keys, 4 children per node).
- **Split Logic:** Deterministic Upper Median Promotion (\(\text{Pivot} = \text{Keys}[\lfloor N/2 \rfloor]\)).
- **Localization:** Custom Weighted Collation for Persian (ISO/IEC 10646 compliant character handling).

### 🔐 Standards Compliance

| Standard          | Application                            | Evidence                                                           |
| ----------------- | -------------------------------------- | ------------------------------------------------------------------ |
| **ISO/IEC 10646** | Unicode character encoding for Persian | `char` type and Persian weight mapping (1–33 for Persian alphabet) |
| **ISO/IEC 25010** | Software quality metrics               | Memory safety via Rust ownership model; strict type system         |
| **IETF BCP 47**   | Language tagging for accessibility     | `lang="fa"` declarations for Persian content in Dioxus components  |
| **RFC 5198**      | Unicode Normalization (recommended)    | Future enhancement for production robustness                       |
| **Knuth Vol. 3**  | B-Tree structure definition            | Order-4 tree with deterministic splitting                          |

## ⚡ Engineered in Rust

While the Persian sorting logic ensures **correctness**, the choice of **Rust** ensures **robustness**. This is not a simple visualization; it is a system-level artifact.

### Why Rust?

1. **Memory Safety in Recursive Structures:** B-Trees rely on complex pointer manipulation (`Box<Node>`). Rust's **Borrow Checker** proves memory safety at compile time, preventing segmentation faults and dangling pointers.
2. **Type-Driven Logic:** Using `Option<Box<T>>` makes invalid tree states mathematically impossible.
3. **WebAssembly Native:** Compiles directly to `.wasm` binary for near-native performance in the browser (Zero-Server Architecture).

## 🚀 Usage

Install Dioxus CLI
```bash
cargo install dioxus-cli
```

Run the Visualizer
```bash
dx serve
```

Build for production
```bash
dx build --release
```

The visualizer runs entirely in your browser with no server required.

## � References

- Manning, C. D., Raghavan, P., & Schütze, H. (2008). *Introduction to Information Retrieval*. Cambridge University Press. https://nlp.stanford.edu/IR-book/
- Saboohi, H. (2024). *Lecture Notes: Foundations of Information Retrieval*. Islamic Azad University, Karaj Branch. http://www.hadisaboohi.com/IR/
- Knuth, D. E. (1973). *The Art of Computer Programming, Vol. 3: Sorting and Searching*. Addison-Wesley.
