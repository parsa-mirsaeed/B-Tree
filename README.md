# Academic Storage Engine (B-Tree Visualizer)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Dioxus](https://img.shields.io/badge/Dioxus-0.7.1-green?style=for-the-badge)
![Version](https://img.shields.io/badge/version-0.2.0-blue?style=for-the-badge)

A high-performance, strictly typed B-Tree implementation in Rust, featuring a WebAssembly visualization engine with native Persian language support.

## 🏗 Engineering Architecture

This project is not just a data structure; it is an engineered system designed for correctness, maintainability, and cultural localization.

### 1. Algorithmic Core (ISO/IEC 23001-1 Compliant)

We implement a Standard B-Tree (Order 4), strictly adhering to the definition where data exists in internal nodes (Knuth Vol. 3), distinguishing it from B+Trees.

*   **Order ($m=4$):** Max 3 keys, 4 children per node.
*   **Split Strategy:** Upper Median Promotion (Right-Biased). When a node overflows (e.g., [A, B, C, D]), the element at index len/2 (index 2, "C") is strictly selected as the pivot. This deterministic behavior ensures the tree structure matches academic simulators perfectly.
*   **Complexity:** Guaranteed $O(\log n)$ for search, insert, and delete operations.

### 2. "Natural String" Sorting Engine

Standard lexicographical sorting fails for human needs (e.g., "10" comes before "2", and Persian 'پ' is sorted incorrectly by Unicode). We engineered a custom `NaturalString` type:

*   **Hybrid Comparators:** Automatically detects if input is numeric or text.
    *   "10" vs "2" $\rightarrow$ Parsed as Integers $\rightarrow$ 2 < 10 (Correct).
    *   "Ali" vs "Bob" $\rightarrow$ Text Comparison.
*   **Persian Collation Algorithm:**
    *   Standard Unicode sorts Persian characters like پ (Pe), چ (Che), ژ (Zhe), گ (Gaf) at the end of the alphabet because they are "extended" Arabic characters.
    *   **Our Solution:** A weighted look-up table strictly maps these characters to their correct phonetic position (e.g., پ immediately after ب), ensuring cultural accuracy.

### 3. Zero-JS Recursive Visualization

The visualization layer avoids heavy JavaScript layout libraries (like D3.js) in favor of a pure CSS Recursive Flexbox architecture.

*   **Robust Connectors:** Uses CSS pseudo-elements (`::before`, `::after`) to draw "curved elbows" connecting parents to children.
*   **Layout Stability:** Enforces `direction: ltr` for structural alignment while maintaining `direction: rtl` for text content, solving the classic "RTL Tree Layout" bug.

## 🛠 Tech Stack

*   **Language:** Rust (2021 Edition) - For memory safety and zero-cost abstractions.
*   **Frontend Framework:** Dioxus 0.7.1 - Modern, signal-based UI library compiling to WebAssembly.
*   **Styling:** Custom CSS with Vazirmatn Font integration.
*   **Build Target:** `wasm32-unknown-unknown`.

## 🚀 Usage

### Prerequisites

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install Dioxus CLI:
```bash
cargo install dioxus-cli
```

### Development

Run the local development server:
```bash
dx serve
```

Open http://localhost:8080.

## 👤 Credits & Authorship

**Engineered and Developed by Parsa MirSaeed.**

This system was designed to meet high-quality software engineering standards, prioritizing memory safety, algorithmic correctness, and inclusive localization.

For any questions, please contact: [parsa.mirsaeed@gmail.com](mailto:parsa.mirsaeed@gmail.com)
