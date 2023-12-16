# ÖZET

> Özetin tamamı 150-250 kelime arasında olmalıdır. Proje özetinde, çalışmanın ayrıntılarından, yorumlardan ve
> kaynaklardan bahsedilmez. Özette projenin amacı, kullanılan yöntem, yapılan gözlem ve elde edilen temel bulgular ve
> sonuçlardan birkaç cümle ile bahsedilir. Ayrıca proje özetinin altına, proje konusunu genel olarak yansıtan en fazla
> beş
> kelimeden oluşan anahtar kelimeler verilir. İdeal olan başlarken taslak bir özet oluşturup, çalışma bittiğinde proje
> raporunun içeriğine uygun bir şekilde özeti güncellemektir.

Son yıllarda dijital dünya adına geliştirilen teknolojiler sayesinde program geliştirmek
çeşitli yollarla kolaylaştırılmıştır. Örneğin eskiden program geliştirmek
için [vi](https://en.wikipedia.org/wiki/Vi_(text_editor)) gibi metin editörleri kullanılırken günümüzde
[JetBrains](https://www.jetbrains.com/) IDE'leri gibi LSP protokolleri,
ileri seviye düzenleme, kod üretimi gibi araçlar kullanılmaktadır.

Bu gelişmelerden etkilenen en yaygın araçlardan birisi paket yöneticileridir.
Amatör ya da profesyonel her yazılım geliştiricisinin kullandığı paket yöneticileri,
herhangi bir programın kurulumu, güncellenmesi, kaldırılması gibi işlemleri kolaylaştırır.

Örneğin python programlama dilinin standart paket yöneticisi `pip`
python ile kod yazan olan herkesin sıkça kullandığı bir yazılımdır.

Paket yöneticilerinin yazılımcıların günlük işlerini hızlandırdıkları için günümüzde pek çok işletim sistemine
uyarlanmış tipleri bulunmaktadır. Bunlara Windows için `chocolatey`, Linux ve MacOS için `nix` örnek verilebilir.

Farklı paket yöneticilerinin komutları, dökümanları, paket kayıt sistemleri
hatta paket adlandırma standartlarının birbiriyle uyuşmaması, kullanıcıların diğer işletim sistemlerinden aldıkları
bilgilerden kendi işletim sistemlerinde faydalanamamalarına sebep olmaktadır.
Örneğin kurulumu yapılan bir yazılımın bağımlı olduğu kütüphaneler kullanıcının sisteminde farklı standartlara bağlıysa
geliştiricinin önerdiği yöntemler size uymayacağından kütüphaneleri nasıl indireceğini kullanıcı bulmalıdır.

Bu bağlamda Debian paket kayıt sistemindeki bir kütüphane paketinin isimlendirme standardı
`lib<kütüphane adı>` iken RedHat paket kayıt sistemi için aynı standart `<kütüphane>-devel`
şeklinde belirlenmiştir. Örnek olarak günümüz network protokolleri arasında veri aktarımı için kullanılan
[`Client for URL (cURL)`](https://en.wikipedia.org/wiki/CURL) yazılımı, Debian paketlerinde `libcurl`, RedHat
paketlerinde `curl-devel` olarak adlandırılmıştır.

Kimi adlandırma farklılıkları bir standarda bağlı olmadığından dolayı tahmin edilmesi zor durumlarla karşılaşılabilir.
Örneğin make yapılandırma sistemi pek çok paket yöneticisi tarafından `make` adına sahipken `nixpkgs` paket platformu
üzerinde `gnumake` adındadır.

Kullanıcıların bu tarz sorunlarla karşılaşmaması için geliştirdiğimiz GENE Paket Yöneticisi, paket yöneticileri
arasındaki farklılıkları
soyutlayarak ([abstraction principle](https://en.wikipedia.org/wiki/Abstraction_principle_(computer_programming)))
ortadan kaldırmayı hedeflemektedir.

Bu farklılıklar temelde komutlar (commandline arguments) ve kayıt sistemlerinden (package registry) oluşur.

GENE, kayıt sistemlerini birleştirmek için GPacR adını verdiğimiz her bir paketin bütün sistemlerdeki isimlerini otomatik olarak
güncellenebilen bir arşivleme algoritması kullanır. Kullanıcı bir paket indirmek istediği zaman kullanıcının kullandığı
paket yöneticileri ve GPacR'ı kullanarak paket yöneticisine uygun paket ismini gerekli komutlarla sistem kabuğunuza
ileterek paketin kurulumunu yapar.

Paket kayıt sistemi otomasyonu, bulut temelli CI/CD/CT (Continious Integration, Continious Delivery, Continious Testing)
entegrasyonlarıyla son teknoloji program geliştirme standartları ve bakım protokollerini destekler nitelikte
tasarlanmıştır.

> ANAHTAR KELIMELER: paket yöneticisi, soyutlama, cross platform, standartlaştırma

## Kaynaklar

https://en.wikipedia.org/wiki/CURL
