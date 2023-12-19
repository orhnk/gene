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
sürdürülebilirlik
(maintenance) kolaylığı ile birlikte yerel derleme (local compilation) ile otomatik özellik yönetimi (feature
management)
ve ya plugin yönetimi (plugin management) gibi özellikler sayesinde GENE projesini geliştirirken kolaylık sağlamıştır.

### Yüksek Seviye Sözdizimi

[//]: # (- High level syntax)

Rust, C ve C++ gibi düşük seviye programlama dillerinin aksine yüksek seviye bir sözdizimine sahiptir. Bu sayede
programcılar, düşük seviye programlama dillerinde karşılaştıkları okunabilirik, yeniden düzenleme (refactoring)
gibi konularda sıkıntılar yaşamazlar.

### Borrow Checker Sistemi

- No Manual memory management with a smart compiler
- New and refreshing paradigm to programming
- Less ways of causing Undefined Behaviour
- Fearless concurrency!

GENE'nin yüksek performanslı çalışabilmesi için ileriye dönük planlarımızdan birisi de güçlü bir bellek sistemiydi.
Her yazılım sanal ortamda bir hacim kaplar. Bu hacmin tutulduğu pek çok yer olmasına rağmen yazılımların işleyiş süresi
boyunca
kullandığı iki temel tip hafıza bulunur:

- **CPU (Central Processing Unit - Merkezi İşlem Birimi)**: Kayıtları (registries) genellikle düşük seviye programlama
  yapan kişilerin sık kullandığı, rax, eax gibi kimi x86 assembly sembollerinin (mnemonics) temsil ettiği düşük hacimli
  yüksek performanslı hafıza birimleridir.
- **RAM (Random Access Memory - Rastgele Erişim Hafızası)**: Bir bilgisayar işlemcisinin kısa süreli hafızasıdır.
  İşlemci Kayıtları (CPU Registries)'den sonra en hızlı işlemci birimidir.

Yazılımlar önce dahili diskinize (HDD/SSD) ardından RAM'inizde ve en son parçalanarak CPU kayıtlarına aktarılır.
Bu süreçte bilgisayarınız, yazılımın ihtiyacı olmadığı kimi değişkenleri belleğinden temizlemeli ve böylece sistem
performansını maksimumda tutmalıdır.
Bu temizleme işlemini yapan kimi algoritmalar aşağıda listelenmiştir:

#### Çöp Toplama (Garbage Collection)

Özellikle sözdizimini (syntax) basit tutmak için Go, Python, C#, Haskell, Java, Swift gibi yazılım sektöründe çok
kullanılan programlama dilleri arasında kullanılmaktadır.

Temel çalışma mantığı yazılımınız çalışırken başka bir programın kimi algoritmaları kullanarak [
[reference counting, tracing](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)),
[kaçış analizleri (escape analysis)](https://en.wikipedia.org/wiki/Escape_analysis) vs. ]
programınızın kullanmadığı değişkenleri bellekten temizlemektir.

##### Sıkıntıları

Çöp Toplama; kimi endüstriler tarafından problem oluşturan bazı sıkıntılara karşılaşılmasına sebep olmuşturş.

Örneğin dünya çapında kulanılan anlık iletişim ve dijital dağıtım platformudur. Discord,
Kimi sunucu (server side) ve alıcı (client side) yazılımlarını Go programlama dilinden Rust programlama diline
geçiş yapmaktadır. [https://discord.com/blog/why-discord-is-switching-from-go-to-rust]
Bunun temel sebebi Go ile yazılmış olan okunmuş mesajlar servisinin (Read States service)
Go gibi çöp toplayıcı (garbage collected) bir programlama dili ile yazıldığından dolayı
Çöp toplama sivrileri (Garbage Collection Spikes) denilen bir performans sorunu ile karşılaşmışlar.
Böyle bir durumda Discord'un işlemeye çalıştığı inanılmaz büyüklükteki veriler, sunucularında birikerek
servisin gerekli performansa oluşamamasına ve ortalama iki dakikada bir sunucu gecikmelerine ve işlemci
zorlanmalarına sebep olmaktadır ( bkz. Figür 1 )

![figür 1](https://github.com/denizbaba0/gene/assets/107251435/27e4526b-4133-4541-bd16-4e5983150a3a)

Bu problemin sebebinin çöp toplama sistemleridir çünkü bu sistemler veriyi kullanıldıktan hemen sonra değil,
belirli aralıklarla temizler. Bu da Discord'un sunucuları gibi inanılmaz büyüklükte veri işleyen bilgisayarların
hafızalarında milyonlarca insanın verilerinin belirli aralıklarla birikmesine ve bu veriler temizlenirken de işlemcinin
zorlanmasına neden olmaktadır.

Go programlama dilinin kaynak koduna bakacak olursak bütün programların minimum 2 dakika içerisinde çöp toplama işlemi
geçirmesinin
zorunlu tutulduğunu
görmekteyiz [bkz. src/runtime/proc.go S4481-S4486](https://github.com/golang/go/blob/895b7c85addfffe19b66d8ca71c31799d6e55990/src/runtime/proc.go#L4481-L4486)

Discord bu problemi çözmek adına Go programlama dilinin çöp toplayıcı sistemini kendilerine göre ayarlama
çalışmıştır [bkz. Go SetGCPercent fonksiyonu](https://pkg.go.dev/runtime/debug#SetGCPercent).

Performans düşüklüğünü giderilemeyince problemi daha derin bir araştırmadan geçiren ekip, Go çöp toplama sisteminin
devasa boyutlarda olan en son kullanılan kullanıcı okumaları önbelleğinin tamamını
referans edilmeyen ve temizlenmesi gereken veriler için taradığını ve bu sebeple sunucularının bekledikleri performansta
olmadıklarını anladı.

Yapılan denemeler sonucu daha küçük önbellek yığınları kullanarak sistem performansını artırmayı deneyen yazılımcı ekibi
gecikmeyi azalatmayı başarsalar da
düşük önbellek hacimleri
sunucunun [99'uncu gecikmelerini](https://cloud.google.com/spanner/docs/latency-metrics#:~:text=99th%20percentile%20latency%3A%20The%20maximum,in%20less%20than%202%20seconds.)
artırmıştır.

Discord'un bu problemi çözme amacıyla aynı yazılımı Rust ile yeniden yazmış ve Rust'ın yenilikçi ve derleme-zamanı (
compile-time) bellek yönetimi (memory management) sayesinde çöp toplama sistemlerine ihyitaç duymadan
yüksek performanslı bir yazılım geliştirmiştir.

> Go: Mor | Rust: Mavi
>
> Discord ekibinin rust programlama dilini kullanmaya karar vermesinin ardından sistem işlemci
> performans sorunları ve gecikmeleri sona ermiştir. Bunun üzerine son derece optimize edilmiş
> eski Go altyapılarını yarı-optimize edilmiş yeni Rust yapılandırmaları bütün metrikler açısından geçmiştir.
>
> ![figür 2](https://github.com/denizbaba0/gene/assets/107251435/97982e37-54e0-455c-b9c6-86b2499bface)

> Rust ekosisteminin hızla gelişmesi ile beraber tokio (rust asenkron işlem kütüphanesi) adapte edilen yazılım,
> ayın figür 3'te 16'sınından itibaren ciddi performans artışları göstermiştir.
>
> ![image](https://github.com/denizbaba0/gene/assets/107251435/cdead23a-bf43-4c77-810b-4f7ffafa19b7)

> Rust'ın yüksek performans gösteren yeni bellek yönetim sistemi sonucunda önbellek yığın hacmini artırmaya karar veren
> ekip, figür 4'te gösterildiği gibi performansı ciddi seviyede artırmayı başarmıştır.
>
> NOT: ortalama süre (avarage time) mikrosaniye (microseconds) ve maksimumum @mention milisaniye cinsinden ölçülmüştür.
>
> ![image](https://github.com/denizbaba0/gene/assets/107251435/35429ff3-1589-46ac-9c39-ad13ac68501e)

Daha ayrıntılı bilgi için [Ek I](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)

#### Ödünç Alma Denetleyicisi (Borrow Checker)

Rust sahiplik (ownership) ve ödünç alma (borrowing) kavramları sayesinde bütün bellek yönetimini derleme
zamanında (compile-time) yapar. Bu sayede programcılar, bellek yönetimi ile uğraşmak zorunda kalmazken
aynı zamanda işleyiş süresi bellek yönetimi ile ilgili hataların neredeyse tamamının önüne geçilmiş
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

Ana geliştirici makinesi Linux NixOS dağıtımı çalıştırdığından JetBrains IDE'leri kusurlu çalışmaktadır. [Read-Only File System & absolute paths for dependencies etc.]
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

GENE'nin pek çok kayıt sistemini anlayabilmesi için geliştirilen GPacR, gerekli sistemler arası bilgileri arşivlemek için kullanılır.
Etkin ve tasarruflu olarak tasarladığımız algoritmalar yardımı ile pek çok paket kayıt sistemini GENE'nin anlayacağı biçime sokar.

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
