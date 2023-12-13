# YÖNTEM

> Araştırma yönteminin, veri toplama araçlarının, deney ve gözlem düzeneklerinin ve verilerin analiz yönteminin
> verildiği bölümdür.

## Rust Programlama Dili

GENE Ekosistemini geliştirmek için Rust programlama dilini kullandık.

### Cargo Paket Yöneticisi

- Robust build system for rust
	- Standerdized Package Management
	- Automatic linkage
	- Easy to maintain thanks to systems like yanking and semantical versioning
	- Automatic feature management by local compilation
	- multi-paradigm package manager allowing you to manage your package with smart actions (e.g expand, watch, add)

Günümüz programlama dil paket yöneticilerinin modern standartlarına uygun bir paket yöneticisi olan rust programlama
dilinin
`cargo` paket yöneticisi, GENE ekosisteminin geliştirilmesinde büyük rol oynamıştır.

`cargo`'yu kullanmamızın sebepleri:

- Güçlü bir yapılandırma sistemi ile birlikte kullanılması
- Rust programlama dilinin endüstriyel standardı olması
- Yanking veya anlamsal versiyonlama (semver) gibi sistemlerin sağladığı bakım (maintenance) kolaylığı.
- Yerel derleme ile otomatik özellik yönetimi
- Çoklu paradigma ile paket yönetimi (örneğin `cargo expand`, `cargo watch`, `cargo add` gibi)

### Yüksek Seviye Sözdizimi

- High level syntax

Rust, C ve C++ gibi düşük seviye programlama dillerinin aksine yüksek seviye bir sözdizimine sahiptir. Bu sayede
programcılar, düşük seviye programlama dillerinde karşılaştıkları okunabilirik, yeniden düzenleme (refactoring)
gibi konularda sıkıntılar yaşamazlar.

### Borrow Checker Sistemi

- No Manual memory management with a smart compiler
- New and refreshing paradigm to programming
- Less ways of causing Undefined Behaviour
- Fearless concurrency!

Rust programlama dilinin en önemli özelliklerinden biri de manuel bellek yönetimi olmamasıdır. Rust,
yazdığınız programı derlerken, programınızın bellek yönetimini otomatik olarak yapar. Bu sayede programcılar,
bellek yönetimi ile uğraşmak zorunda kalmazlar. Bunun için yenilikçi bir yöntem olan `borrow checker` sistemini
kullanır. Bu sistem, programcıların yazdığı programın risk oluşturmayacak biçimde olmasını zorunlu kılar.
Bu kurallara uymayan programlar derlenmez. Böylece tamamen soyutlanmış (abstracted) hızlı ve güvenli programlar
yazılabilir.

### Sistem Seviyesinde Performans

- LLVM Based programming language

Rust, LLVM Derleyici altyapısını temel alan bir programlama dilidir. Bu sayede sistem seviyesinde performans sağlar.

### Yeni Nesil Programlama

- Algebraic Data Types
- Procuderal Macros
- Strong Pattern Matching idioms
- Semi-functional approach
- Strongly typed
- Easy Cross-compilation

- Rust, yeni nesil programlama paradigmalarını destekler.
- Cebirsel Veri Tipleri (Algebraic Data Types), fonksiyonel programlama dillerinin en önemli özelliklerinden biridir.
- Procedural Macros, makroların derleme zamanında çalışmasını sağlar. Bu sayede programcılar, makrolar ile kod tasarrufu
  yapabilirler.
- Güçlü desen eşleme (pattern matching) idiyomları, programcıların kodlarını daha okunabilir ve daha az hata ile
  yazmalarını sağlar.
- Yarı fonksiyonel yaklaşım, programcıların fonksiyonel programlama dillerindeki gibi fonksiyonları kullanmasını sağlar.
- Güçlü tür sistemi, programcıların daha az hata yapmasını sağlar.
- Kolay çapraz derleme, programcıların yazdıkları programları farklı işletim sistemlerinde çalışmasına olanak sağlar.

### Eskiye Uyumlu (Backward Compatiblity)

Rust programlama dili büyük ve genişleyen kominitesi ile genel programlama dillerinin karşılaştığı bir
problemle karşı karşıya kalmıştır. Örneğin Python2 ve Python3 arasındaki temel değişiklikler bile
programlama forumları, kütüphaneler gibi pek çok açıdan büyük ses getirmiştir. Rust bu sorunun güvencesini
resmi anlamda sunarak rust programlarının eski derleyiciler ile kullanılabileceğini kesinleştirmiştir.
