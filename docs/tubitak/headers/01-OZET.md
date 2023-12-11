# ÖZET

> Özetin tamamı 150-250 kelime arasında olmalıdır. Proje özetinde, çalışmanın ayrıntılarından, yorumlardan ve kaynaklardan bahsedilmez. Özette projenin amacı, kullanılan yöntem, yapılan gözlem ve elde edilen temel bulgular ve sonuçlardan birkaç cümle ile bahsedilir. Ayrıca proje özetinin altına, proje konusunu genel olarak yansıtan en fazla beş kelimeden oluşan anahtar kelimeler verilir. İdeal olan başlarken taslak bir özet oluşturup, çalışma bittiğinde proje raporunun içeriğine uygun bir şekilde özeti güncellemektir.

Son yıllarda dijital dünya adına geliştirilen teknolojiler sayesinde program geliştirmek
çeşitli yollarla kolaylaştırılmıştır. Örneğing eskiden program geliştirmek
için [vi](https://en.wikipedia.org/wiki/Vi_(text_editor)) gibi metin editörleri kullanılırken günümüzde
[JetBrains](https://www.jetbrains.com/) IDE'leri gibi LSP protokolleri,
ileri seviye düzenleme, kod üretimi gibi gelişmiş araçlar kullanılmaktadır.

Bu gelişmelerden etkilenen en yaygın araçlardan birisi paket yöneticileridir.
Amatör ya da profesyonel her yazılım geliştiricisinin kullandığı paket yöneticileri,
herhangi bir programın kurulumu, güncellenmesi, kaldırılması gibi işlemleri otomativize eder.

Örneğin python programlama dilinin standart paket yöneticisi "Preffered Installer Program" (PIP)
python ile kod yazmış olan herkesin sıkça kullandığı bir programdır.

Paket yöneticilerinin sağladıkları faydalar sebebiyle günümüzde pek çok işletim sistemine uyarlanmış
tiplere ayrılmaktadır.

Farklı paket yöneticilerinin farklı komutları, farklı dökümantasyonları, farklı paket kayıt sistemleri
hatta farklı paket adlandırma standartları olması kullanıcıların farklı işletim sistemleri arasında geçiş
yaparken karşılaştıkları sorunlardan birisidir. Bunun yanında manuel indirilen yazılımlar için bağımlılık
yönetimi, güncelleme, kaldırma gibi işlemler kullanıcılar için zorluklar içermektedir.

Özellikle sistemler arası paket adlandırma sistemleri arasındaki farklılıklar kullanıcıların indirmek istedikleri
paketleri bulmalarını zorlaştırmaktadır. Örneğin APT paket Yöneticisini kullanan bir sistemde çok yaygın olarak kullanılan yapılandırma
sistemi [`make`](https://en.wikipedia.org/wiki/Make_(software)) paketinin adı `make` iken Nix paket yöneticisini kullanan bir sistemde
aynı paketin adı `gnumake`'dir. Bu gibi durumlar program geliştirirken hız ve üretkenlik kaybına sebep olur.

Aynı şekilde [dpkg](https://en.wikipedia.org/wiki/Dpkg) temmeli [`APT`](https://en.wikipedia.org/wiki/APT_(software)) paket yöneticisi için kütüphane paketleri için adlandırma standardı
`lib<kütüphane adı>` iken [RPM](https://en.wikipedia.org/wiki/RPM_Package_Manager) temelli [YUM](https://en.wikipedia.org/wiki/Yum_(software))
paket yöneticisi için aynı standart `<kütüphane>-devel` şeklinde belirlenmiştir. Örneğin dünya üzerinde en çok indirilen program ünvanını taşıyan ve [//] # (Validate this information)
geliştiricisi [Daniel Stenberg](https://en.wikipedia.org/wiki/Daniel_Stenberg) İsveç Polhem Ödülü kazandıran [`cURL`](https://en.wikipedia.org/wiki/CURL) yazılımı,
`APT` paket yöneticisi için `libcurl` `YUM` paket yöneticisi için `curl-devel` olarak adlandırılmıştır.

Kullanıcıların bu tarz sorunlarla karşılaşmaması için geliştirdiğimiz GENE Paket Yöneticisi, paket yöneticileri
arasındaki farklılıkları soyutlayarak ([abstraction principle](https://en.wikipedia.org/wiki/Abstraction_principle_(computer_programming))) ortadan kaldırmayı hedeflemektedir.

GENE, GPacR (Generic PACkage Registry) adını verdiğimiz kendi paket kayıt sistemini otomatik [CI](https://en.wikipedia.org/wiki/Continuous_integration) algoritmalarıyla
güncel tutarak paket yöneticilerinin kayıt sistemleri arasındaki adlandırma farklarını ortadan kaldırmayı
hedeflemektedir.

Ayrıca paket yöneticilerinin komutlarını standartlaştırarak kullanıcıların farklı sistemler arasında geçiş yaparken
paketi indirme, güncelleme, kaldırma gibi komutları yeniden öğrenmesinin önüne geçmeyi hedeflemektedir.

> ANAHTAR KELIMELER: paket yöneticisi, soyutlama, cross platform, standartlaştırma
