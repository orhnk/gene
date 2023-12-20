# YÖNTEM

> Araştırma yönteminin, veri toplama araçlarının, deney ve gözlem düzeneklerinin ve verilerin analiz yönteminin
> verildiği bölümdür.

GENE Ekosistemini geliştirmek için Rust programlama dilini kullandık.

### Cargo Paket Yöneticisi

Günümüz programlama dil paket yöneticilerinin modern standartlarına uygun bir paket yöneticisi olan rust programlama
dilinin `cargo` paket yöneticisi, GENE ekosisteminin geliştirilmesinde büyük rol oynamıştır.

`cargo`'yu seçmemizin temel nedeni modern program geliştirme araçları ile kolay kullanılabilir bir yapılandırma sistemi
ile gelmesiydi.

Rust'ın standart paket yöneticisi olan Cargo, GENE'nin geliştirme, bakım, sürdürülebilirlik ve dağıtım süreçlerde büyük
kolaylık sağlamıştır. Başlıca sistem programlama dilleri arasında sık karşılaşılan standart olmayan paket
yöneticilerine [C/C++ vcpkg canon] karşın Rust'ın standart olarak belirlediği Cargo, diğer sistem programlama dilleri
ile
karşılaştırıldığında daha kolay kullanılabilir bir yapılandırma sistemine sahiptir.

Proje geliştirme sürecinde cargo'nun kullandığı semantik versiyonlama (semver), yanking gibi sistemlerin sağladığı
sürdürülebilirlik (maintenance) kolaylığı ile birlikte yerel derleme (local compilation) ile otomatik özellik yönetimi
(feature management), plugin yönetimi (plugin management) gibi özellikler, GENE projesini geliştirmeyi
kolaylaştırmıştır.

### Yüksek Seviye Sözdizimi

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

Rust, LLVM Derleyici altyapısını temel alan bir programlama dilidir. Bu sayede sistem seviyesinde performans sağlar.

### Yeni Nesil Programlama

Rust yarı fonksiyonel bir programlama dili olduğundan [bkz. data/figures/programming-languages-classification]
fonksiyonel programlama dillerinin sahip olduğu kısa ve okunabilir kod yapısına sahiptir.

Desteklediği güçlü makro sistemi projemizi geliştirirken kod tasarrufu yapmamızı sağlamıştır.

Desen eşleme (pattern matching), Trait sistemi, Güçlü (Strongly-typed) ve Cebirsel Veri Tipleri (Algebraic Data Types),
fonksiyonel programlama dillerinde öne çıkan ve
programın geliştirilmesinde çarpım türleri (product types) yerine toplam türleri (sum types) kullanarak daha temiz ve
deyimsel (idiomatic) kod yazılmasını sağlayan bir programlama paradigmasıdır.

Rust derleyicisi (rustc) Çapraz derleme (cross-compilation) sayesinde yazdığımız programı farklı işletim sistemleri için
derleyebilmesi, kullanım kolaylığı sağlamıştır.

### Eskiye Uyumlu (Backward Compatiblity)

Rust programlama dili büyük ve genişleyen kominitesi ile genel programlama dillerinin karşılaştığı bir
problemle karşı karşıya kalmıştır. Örneğin Python2 ve Python3 arasındaki temel değişiklikler bile
programlama forumları, kütüphaneler gibi pek çok açıdan büyük ses getirmiştir. Rust bu sorunun güvencesini
resmi anlamda sunarak rust programlarının eski derleyiciler ile kullanılabileceğini kesinleştirmiştir.

Bu sayede GENE, eski veya yeni versiyon rust derleyicilerini deskteklemektedir.

## Git Versiyon Kontrol Sistemi (VCS - Version Control System)

GENE projesinin mekandan bağımsız ve eş zamanlı geliştirilebilmesi için bir organizasyon sistemine ihtiyaç duyduk.
Projeyi geliştirirken, sürdürülebilirlik (maintenance), geliştirme, test, dağıtım gibi pek çok aşamada işimizi
kolaylaştırması adına endüstriyel standartlardan birisi olan Git Versiyon Kontrol sistemini kullandık.

## Github

Sıkıntı takipçisi (issue tracker), kod incelemesi (code review), özellik istekleri (feature request), wiki gibi
geniş çaplı projeler için gerekli olan yapıları oluşturacak ve Git ile entegre çalışacak bir barındırma servisi
(host) olarak GitHub platformunu kullandık.

Bu sayede GENE'yi geliştirmek ve kullanıcıların karşılaştıkları problemleri çözmek kolaylaşmıştır.

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

Konfigurasyon dosyaları için bkz. Ek 2:

- NixOS işletim sistemi: https://github.com/utfeight/dotnix
- Vimacs:
    - Geliştirdiğim vimacs yazılımının kaynak kodu: https://github.com/utfeight/vimacs
    - Vimacs konfigurasyon dosyaları: https://github.com/utfeight/vimax
- JetBrains:
    - ideavimrc: https://github.com/utfeight/dotideavimrc
    - TODO: link dotnix ft
- VSCode:
    - Nix ile yazılmış dekleratif konfigürasyon: TODO: link dotnix ft
- bütün configurasyon dosyaları için: TODO: link em all

# GENE

GENE, geliştirilmeye açık olarak tasarlanmak istenildiğinden temel programlama prensiplerine uygun olarak temiz bir kod
tabanı (code base) ile geliştirilmesi planlanmıştır. Bunun için Endişelerin ayrılması ile (Separation of Concerns) doğru
miktarda uyum ve bağlantı (cohesion & coupling) gibi pek çok programlama prensibi göz önünde bulundurularak
tasarlanmıştır.

Geliştirilmesi için bir çok rust kütüphanesinden yararlanılmıştır. (bkz. Cargo.toml's : TODO)

GENE üzerinde yapılabilecek güncellemeler göz önünde bulundurlarak kolay yenilenebilir ve modüler biçimde
tasarlanmıştır. Aşağıda GENE'nin kullandığı modüller ve açıklamaları listelenmiştir.

## GPacR

> Pronounced as "Gee-Packer"

> GENE Paket Kayıt Sistemi

GENE'nin pek çok kayıt sistemini anlayabilmesi için geliştirilen GPacR, gerekli sistemler arası bilgileri arşivlemek
için kullanılır. Etkin ve tasarruflu olarak tasarladığımız algoritmalar yardımı ile pek çok paket kayıt sistemini
GENE'nin anlayacağı biçime çevirir. Böylece GENE, kullanıcı tarafından indirilmek istenen paketi çok daha tasarruflu
şekilde indirebilir.

Figür N de GPacR'ın algoritma şeması verilmiştir.

![img_1.png](..%2Fdata%2Ffigures%2Fimg_1.png)

> NOTE: Add more notes upon the topic here +algorithm_schema +performance_schema

## GTWIN

> Pronounced as "Gee-Twin"

> GENE çift yönlü tercüme (GENE Two-Way InterpretatioN)

GENE programının yeni bir standart oluşturmadan diğer standartları anlaması için geliştirilmiş olan çift yönlü tercüman
modülüdür. Bu modül, Paket Yöneticilerinin eymleri için belirlediği komutları anlamlandırarak diğer paket
yöneticilerinin
komutlarına çevirmek için geliştirilmiştir.

Örneğin DWM linux pencere yöneticisini indirmek isteyen bir kullanıcı aşağıdaki komutlardan herhangi birini
kullanabilir.

```shell
gene apt install dwm
gene pacman -S dwm
gene emerge -i dwm
gene xbps-install dwm
gene pkg install dwm
gene install dwm
```

yukarıda verilmiş olan bütün komutlar bütün sistemlerde çalışacaktır çünkü GENE, bu komut sistemlerinin hepsini
anlamlandırıp kullandığınız işletim sisteminin komutlarına çevirebilecektir.

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
