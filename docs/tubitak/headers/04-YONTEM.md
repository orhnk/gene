# YÖNTEM

> Araştırma yönteminin, veri toplama araçlarının, deney ve gözlem düzeneklerinin ve verilerin analiz yönteminin
> verildiği bölümdür.


GENE Ekosistemini geliştirmek için Rust programlama dilini kullandık.

### Cargo Paket Yöneticisi

[//]: # (- Robust build system for rust)

[//]: # (	- Standerdized Package Management)

[//]: # (	- Automatic linkage)

[//]: # (	- Easy to maintain thanks to systems like yanking and semantical versioning)

[//]: # (	- Automatic feature management by local compilation)

[//]: # (	- multi-paradigm package manager allowing you to manage your package with smart actions &#40;e.g expand, watch, add&#41;)

Günümüz programlama dil paket yöneticilerinin modern standartlarına uygun bir paket yöneticisi olan rust programlama
dilinin `cargo` paket yöneticisi, GENE ekosisteminin geliştirilmesinde büyük rol oynamıştır.

`cargo`'yu seçmemizin temel nedeni modern program geliştirme araçları ile kolay kullanılabilir bir yapılandırma sistemi
ile gelmesiydi.

[//]: # (- Güçlü bir yapılandırma sistemi ile birlikte kullanılması)

[//]: # (- Rust programlama dilinin endüstriyel standardı olması)

[//]: # (- Yanking veya anlamsal versiyonlama &#40;semver&#41; gibi sistemlerin sağladığı sürdürülebilirlik &#40;maintenance&#41; kolaylığı.)

[//]: # (- Yerel derleme ile otomatik özellik yönetimi)

[//]: # (- Çoklu paradigma ile paket yönetimi &#40;örneğin `cargo expand`, `cargo watch`, `cargo add` gibi&#41;)

Rust'ın standart paket yöneticisi olan Cargo, GENE'nin geliştirme, bakım, sürdürülebilirlik ve dağıtım süreçlerde büyük
kolaylık
sağlamıştır.
Başlıca sistem programlama dilleri arasında sık karşılaşılan standart olmayan paket yöneticileri [C/C++ vcpkg canon]
yerine Rust'ın standart
olarak belirlediği Cargo, diğer sistem programlama dilleri ile karşılaştırıldığında daha kolay kullanılabilir bir
yapılandırma sistemine sahiptir.

Proje geliştirme sürecinde cargo'nun kullandığı semantik versiyonlama (semver), yanking gibi sistemlerin sağladığı
sürdürülebilirlik (maintenance) kolaylığı ile birlikte yerel derleme (local compilation) ile otomatik özellik yönetimi
( feature management), plugin yönetimi (plugin management) gibi özellikler, GENE projesini geliştirmeyi
kolaylaştırmıştır.

### Yüksek Seviye Sözdizimi

[//]: # (- High level syntax)

Rust, C ve C++ gibi düşük seviye programlama dillerinin aksine yüksek seviye bir sözdizimine sahiptir. Bu sayede
programcılar, düşük seviye programlama dillerinde karşılaştıkları okunabilirik, yeniden düzenleme (refactoring)
gibi konularda sıkıntılar yaşamazlar.

### Ödünç Alma Denetleyicisi (Borrow Checker)

Rust sahiplik (ownership) ve ödünç alma (borrowing) kavramları sayesinde bütün bellek yönetimini derleme
zamanında (compile-time) yapar. Bu sayede programcılar, bellek yönetimi ile uğraşmak zorunda kalmazken
aynı zamanda işleyiş anındaki bellek hatalarının çoğunun önüne geçilmiş
olur. [bkz. reference cycles (referans döngüleri)](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

Rust programlama dilinin en önemli özelliklerinden biri de manuel bellek yönetimi olmamasıdır. Rust,
yazdığınız programı derlerken, programınızın bellek yönetimini otomatik olarak yapar. Bu sayede programcılar,
bellek yönetimi ile uğraşmak zorunda kalmazlar. Bunun için yenilikçi bir yöntem olan `borrow checker` sistemini
kullanır. Bu sistem, programcıların yazdığı programın risk oluşturmayacak biçimde olmasını zorunlu kılar.
Bu kurallara uymayan programlar derlenmez. Böylece tamamen soyutlanmış (abstracted) hızlı ve güvenli programlar
yazılabilir.

### Sistem Seviyesinde Performans

[//]: # (- LLVM Based programming language)

Rust, LLVM Derleyici altyapısını temel alan bir programlama dilidir. Bu sayede sistem seviyesinde performans sağlar.

### Yeni Nesil Programlama

Rust yarı fonksiyonel bir programlama dili olduğundan [bkz. data/figures/programming-languages-classification]
fonksiyonel programlama dillerinin sahip olduğu kısa ve okunabilir kod yapısına sahiptir.

Desteklediği güçlü makro sistemi projemizi geliştirirken kod tasarrufu yapmamızı sağlamıştır.

Güçlü desen eşleme (pattern matching) idiyomları, programcıların kodlarını daha okunabilir ve daha az hata ile
yazmalarını sağlamıştır.

Güçlü (Strongly-typed) ve Cebirsel Veri Tipleri (Algebraic Data Types), fonksiyonel programlama dillerinde öne çıkan ve
programın geliştirilmesinde çarpım türleri (product types) yerine toplam türleri (sum types) kullanarak daha temiz ve
deyimsel (idiomatic) kod yazılmasını sağlayan bir programlama paradigmasıdır.

Rust derleyicisi (rustc) Çapraz derleme (cross-compilation) sayesinde yazdığımız programı farklı işletim sistemleri için
derleyebilmesi, kullanım kolaylığı sağlamıştır.

### Eskiye Uyumlu (Backward Compatiblity)

Rust programlama dili büyük ve genişleyen kominitesi ile genel programlama dillerinin karşılaştığı bir
problemle karşı karşıya kalmıştır. Örneğin Python2 ve Python3 arasındaki temel değişiklikler bile
programlama forumları, kütüphaneler gibi pek çok açıdan büyük ses getirmiştir. Rust bu sorunun güvencesini
resmi anlamda sunarak rust programlarının eski derleyiciler ile kullanılabileceğini kesinleştirmiştir.

## Git Versiyon Kontrol Sistemi (VCS - Version Control System)

GENE projesinin mekandan bağımsız ve eş zamanlı geliştirilebilmesi için bir organizasyon sistemine ihtiyaç duyduk.
Projeyi geliştirirken, sürdürülebilirlik (maintenance), geliştirme, test, dağıtım gibi pek çok aşamada işimizi
kolaylaştırması adına endüstriyel standartlardan birisi olan Git Versiyon Kontrol sistemini kullandık.

## Github

> Git hoster

Sıkıntı takipçisi (issue tracker), kod incelemesi (code review), özellik istekleri (feature request), wiki gibi
geniş çaplı projeler için gerekli olan yapıları oluşturacak ve Git ile entegre çalışacak bir barındırma servisi
(host) olarak GitHub platformunu kullandık.

## JetBrains IDE, VSCode ve Vimacs

GENE, IDE ve editör sektöründe profesyonel yazılımcıların önerdiği JetBrains temelli RustRover, CLion IDE'leri,
Neovim kod editörü ve Microsoft tarafından geliştirilen VSCode uygulaması kullanılarak geliştirilmiştir.

Büyük bir kod tabanı (code base) ile çalışırken, kodun okunabilirliği ve yeniden düzenlenebilirliği (refactoring) gibi
konulara dikkat etmek gerekir. Bu sebeple JetBrains IDE'lerini her yerde kullanmamızı sağlayan bulut temelli auto-sync,
ileri düzeyde etkili araç entegrasyonları (Git, GitHub, DB, JetBrains AI, Github Copilot vb.) güçlü grafiksel arayüz
tasarımı (GUI), kod üretimi (codegen) ve düzenleme (refactoring) araçları ile konfigüre edip GENE projesini geliştirmek
için kullandık.

3 farklı editörü aynı proje için kullanmamızın sebebi her bir editörün kendine özgü güçlü yanları olmasıdır.

Ana geliştirici makinesi Linux NixOS dağıtımı çalıştırdığından JetBrains IDE'leri kusurlu
çalışmaktadır. [Read-Only File System & absolute paths for dependencies etc.]
Bu nedenle bazı zamanlar VSCode kod editörü kullandık.

Linux kullanıcılarının sık kullandığı sistem kabuğu (system shell) terminalleri içerisinde geçirdiğim zamanlar
kendi neovim dağıtımım olan `vimacs`'i kullandım [https://github.com/utfeight/vimacs]

# GENE

GENE, geliştirilmeye açık olarak tasarlanmak istenildiğinden temel programlama prensiplerine uygun olarak
temiz bir kod tabanı (code base) ile geliştirilmesi planlanmıştır. Bunun için Endişelerin ayrılması ile (Separation of
Concerns) doğru miktarda uyum ve bağlantı (cohesion & coupling) gibi pek çok programlama prensibine uygun olarak
tasarlanmıştır.

Örneğin Uyum ve Bağlantı (Cohesion & Coupling) prensibine uygun olan ve olmayan sistem modellemeleri figür 5'te
gösterilmiştir. Karışık ve uyumsuz sistem modellemeleri, programcıların kodlarına müdahale etmesini zorlaştırdığından
projemizin geliştirilme ivmesini düşüreceğinden GENE, geliştirilirken programlama prensiplerine bağlı kalınmıştır

![figür 5](..%2Fdata%2Ffigures%2Fimg.png)

> TODO: read-em-all
> > Kaynaklar: https://en.wikipedia.org/wiki/Category:Programming_principles
> > https://en.wikibooks.org/wiki/Computer_Programming_Principles
> > https://www.artima.com/weblogs/viewpost.jsp?thread=331531
> > https://github.com/webpro/programming-principles
>
> > https://halilsafakkilic.com/soc

## GPacR

> Pronounced as "Gee-Packer"

> GENE Paket Kayıt Sistemi

GENE'nin pek çok kayıt sistemini anlayabilmesi için geliştirilen GPacR, gerekli sistemler arası bilgileri arşivlemek
için kullanılır.
Etkin ve tasarruflu olarak tasarladığımız algoritmalar yardımı ile pek çok paket kayıt sistemini GENE'nin anlayacağı
biçime sokar.

![img_1.png](..%2Fdata%2Ffigures%2Fimg_1.png)


> NOTE: Add more notes upon the topic here +algorithm_schema +performance_schema

## GTWIN

> Pronounced as "Gee-Twin"

> GENE çift yönlü tercüme (GENE Two-Way InterpretatioN)

GENE programının yeni bir standart oluşturmadan diğer standartları anlaması için geliştirilmiş olan çift yönlü tercüman
modülüdür. Bu modül, Paket Yöneticilerinin eymleri için belirlediği komutların hepsini anlayarak GENE'nin paket kayıt
sistemine dahil etmesini sağlar.

## GPmGet

> GENE Paket Yükleme Aracı

Her paket her kayıt sisteminde yer almadığından dolayı kimi paketleri sisteminize indiremeyebilirsiniz. Böyle bir
durumda GPmGet, GENE'nin paket kayıt sistemine dahil olmayan paketleri altyapı olarak başka bir yönetici kullanmanıza
olanak sağlar.

## GonfiG

> GENE Konfigürasyon Menajeri

GENE programının konfigürasyonlarını yönetmek için geliştirilmiş olan modüldür. GonfiG, GENE'nin konfigürasyon
verilerini sistemler arası (cross platform) kusursuz çalışacak şekilde yönetir.

## GPluG

> GENE Eklenti Yöneticisi

GENE programının eklentilerini yönetmek için geliştirilmiş olan modüldür. GPluG, GENE'nin eklentilerini indirme,
kaldırma, güncelleme gibi işlemlerin yanı sıra eklentilerin GENE'ye entegre olmasını ve belirli altyapıların GENE ile
senkronizasyonunu sağlar.

# Ekler

GitHub: https://en.wikipedia.org/wiki/GitHub
Rust as a functional lang: https://kerkour.com/rust-functional-programming
