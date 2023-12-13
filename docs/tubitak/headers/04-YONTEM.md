# YÖNTEM

> Araştırma yönteminin, veri toplama araçlarının, deney ve gözlem düzeneklerinin ve verilerin analiz yönteminin verildiği bölümdür.

## Rust Programlama Dili

GENE Ekosistemini geliştirmek için Rust programlama dilini kullandık.

### Cargo Paket Yöneticisi

- Robust build system for rust 
  - Standerdized Package Management
  - Automatic linkage
  - Easy to maintain thanks to systems like yanking and semantical versioning
  - Automatic feature management by local compilation
  - multi-paradigm package manager allowing you to manage your package with smart actions (e.g expand, watch, add)

### Yüksek Seviye Sözdizimi

- High level syntax
- Less ways of causing Undefined Behaviour
- Easy Cross-compilation

### Borrow Checker Sistemi

- No Manual memory management with a smart compiler
- New and refreshing paradigm to programming
- Fearless concurrency!

### Sistem Seviyesinde Performans

- LLVM Based programming language

### Yeni Nesil Programlama 

- Algebraic Data Types
- Procuderal Macros
- Strong Pattern Matching idioms
- Semi-functional approach

Rust yarı fonksiyonel bir programlama dili olduğundan [bkz. data/figures/programming-languages-classification]

### Eskiye Uyumlu (Backward Compatiblity)

Rust programlama dili büyük ve genişleyen kominitesi ile genel programlama dillerinin karşılaştığı bir
problemle karşı karşıya kalmıştır. Örneğin Python2 ve Python3 arasındaki temel değişiklikler bile
programlama forumları, kütüphaneler gibi pek çok açıdan büyük ses getirmiştir. Rust bu sorunun güvencesini
resmi anlamda sunarak rust programlarının eski derleyiciler ile kullanılabileceğini kesinleştirmiştir.
