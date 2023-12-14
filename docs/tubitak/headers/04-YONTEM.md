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
dilinin `cargo` paket yöneticisi, GENE ekosisteminin geliştirilmesinde büyük rol oynamıştır.

`cargo`'yu seçmemizin temel nedeni modern program geliştirme araçları ile kolay kullanılabilir bir yapılandırma sistemi ile gelmesiydi.

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


GENE'nin yüksek performanslı çalışabilmesi için ileriye dönük planlarımızdan birisi de güçlü bir bellek sistemiydi.
Her yazılım sanal ortamda bir hacim kaplar. Bu hacmin tutulduğu pek çok yer olmasına rağmen yazılımların işleyiş süresi boyunca
kullandığı iki temel tip hafıza bulunur:

- **CPU (Central Processing Unit - Merkezi İşlem Birimi)**: Kayıtları (registries) genellikle düşük seviye programlama yapan kişilerin sık kullandığı, rax, eax gibi kimi x86 assembly sembollerinin (mnemonics) temsil ettiği düşük hacimli yüksek performanslı hafıza birimleridir.
- **RAM (Random Access Memory - Rastgele Erişim Hafızası)**: Bir bilgisayar işlemcisinin kısa süreli hafızasıdır. İşlemci Kayıtları (CPU Registries)'den sonra en hızlı işlemci birimidir.

Yazılımlar önce dahili diskinize (HDD/SSD) ardından RAM'inizde ve en son parçalanarak CPU kayıtlarına aktarılır.
Bu süreçte bilgisayarınız, yazılımın ihtiyacı olmadığı kimi değişkenleri belleğinden temizlemeli ve böylece sistem performansını maksimumda tutmalıdır.
Bu temizleme işlemini yapan kimi algoritmalar aşağıda listelenmiştir:

#### Çöp Toplama (Garbage Collection)

Özellikle sözdizimini (syntax) basit tutmak için Go, Python, C#, Haskell, Java, Swift gibi yazılım sektöründe çok kullanılan programlama dilleri arasında kullanılmaktadır.

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

Go programlama dilinin kaynak koduna bakacak olursak bütün programların minimum 2 dakika içerisinde çöp toplama işlemi geçirmesinin
zorunlu tutulduğunu görmekteyiz [bkz. src/runtime/proc.go S4481-S4486](https://github.com/golang/go/blob/895b7c85addfffe19b66d8ca71c31799d6e55990/src/runtime/proc.go#L4481-L4486)

Discord bu problemi çözmek adına Go programlama dilinin çöp toplayıcı sistemini kendilerine göre ayarlama çalışmıştır [bkz. Go SetGCPercent fonksiyonu](https://pkg.go.dev/runtime/debug#SetGCPercent).

Performans düşüklüğünü giderilemeyince problemi daha derin bir araştırmadan geçiren ekip, Go çöp toplama sisteminin devasa boyutlarda olan en son kullanılan kullanıcı okumaları önbelleğinin tamamını 
referans edilmeyen ve temizlenmesi gereken veriler için taradığını ve bu sebeple sunucularının bekledikleri performansta olmadıklarını anladı.

Yapılan denemeler sonucu daha küçük önbellek yığınları kullanarak sistem performansını artırmayı deneyen yazılımcı ekibi gecikmeyi azalatmayı başarsalar da
düşük önbellek hacimleri sunucunun [99'uncu gecikmelerini](https://cloud.google.com/spanner/docs/latency-metrics#:~:text=99th%20percentile%20latency%3A%20The%20maximum,in%20less%20than%202%20seconds.) artırmıştır.

Discord'un bu problemi çözme amacıyla aynı yazılımı Rust ile yeniden yazmış ve Rust'ın yenilikçi ve derleme-zamanı (compile-time) bellek yönetimi (memory management) sayesinde çöp toplama sistemlerine ihyitaç duymadan
yüksek performanslı bir yazılım geliştirmiştir.


> Go: Mor | Rust: Mavi
![figür 2](https://github.com/denizbaba0/gene/assets/107251435/97982e37-54e0-455c-b9c6-86b2499bface)

![image](https://github.com/denizbaba0/gene/assets/107251435/cdead23a-bf43-4c77-810b-4f7ffafa19b7)

![image](https://github.com/denizbaba0/gene/assets/107251435/35429ff3-1589-46ac-9c39-ad13ac68501e)


Daha ayrıntılı bilgi için [Ek I](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)

#### Ödünç Alma Denetleyicisi (Borrow Checker)

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

Rust yarı fonksiyonel bir programlama dili olduğundan [bkz. data/figures/programming-languages-classification]

### Eskiye Uyumlu (Backward Compatiblity)

Rust programlama dili büyük ve genişleyen kominitesi ile genel programlama dillerinin karşılaştığı bir
problemle karşı karşıya kalmıştır. Örneğin Python2 ve Python3 arasındaki temel değişiklikler bile
programlama forumları, kütüphaneler gibi pek çok açıdan büyük ses getirmiştir. Rust bu sorunun güvencesini
resmi anlamda sunarak rust programlarının eski derleyiciler ile kullanılabileceğini kesinleştirmiştir.
