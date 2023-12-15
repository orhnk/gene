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

Örneğin python programlama dilinin standart paket yöneticisi "Preffered Installer Program" (PIP)
python ile kod yazan olan herkesin sıkça kullandığı bir programdır.

Paket yöneticilerinin sağladıkları faydalar sebebiyle günümüzde pek çok işletim sistemine uyarlanmış
tipleri bulunmaktadır.

Farklı paket yöneticilerinin farklı komutları, farklı dökümanları, farklı paket kayıt sistemleri
hatta farklı paket adlandırma standartları olması, kullanıcıların işletim sistemleri arasında geçiş
yaparken adaptasyon sorunları ile karşılaşmalarına ve zaman kaybetmelerine sebep olmaktadır.
Bunun yanında manuel indirilen yazılımlar için bağımlılık yönetimi, güncelleme, kaldırma gibi
işlemler kullanıcılar için zorluklar içermektedir.

Bu bağlamda [dpkg](https://en.wikipedia.org/wiki/Dpkg) temmeli [`APT`](https://en.wikipedia.org/wiki/APT_(software))
paket yöneticisi için kütüphane paketlerinin adlandırma standardı
`lib<kütüphane adı>` iken [RPM](https://en.wikipedia.org/wiki/RPM_Package_Manager)
temelli [YUM](https://en.wikipedia.org/wiki/Yum_(software)) paket yöneticisi için aynı standart `<kütüphane>-devel`
şeklinde belirlenmiştir. Örnek olarak günümüz network protokolleri arasında veri aktarımı için kullanılan standart

[//]: # (dünya üzerinde en çok indirilen program ünvanını taşıyan ve [//] # &#40;Validate this information&#41;)

[//]: # (geliştiricisi [Daniel Stenberg]&#40;https://en.wikipedia.org/wiki/Daniel_Stenberg&#41;'e İsveç Polhem Ödülü kazandıran)

[`Client for URL (cURL)`](https://en.wikipedia.org/wiki/CURL) yazılımı,
`APT` paket yöneticisi için `libcurl`, `YUM` paket yöneticisi için `curl-devel` olarak adlandırılmıştır.

[//]: # (better example for a better demo. ~Rhetoric)

[//]: # (Bu gibi isim farklılıkları curl programını indirmek isteyen birisinin kafasını karıştırabilir. Özellikle ünlü olmayan)

[//]: # (yazılımlarda sık karşılaşılan bir problem olan isim farklılıkları, kullanıcılara paket indirme ve kurma konusunda)

[//]: # (sıkıntılar oluşturmaktadır.)

Aynı Şekilde sistemler arası standartlara bağlı olmayan temel paket adlandırmaları arasındaki farklar da kullanıcıların
indirmek istedikleri
paketleri bulmalarını zorlaştırmaktadır. Örneğin APT paket yöneticisi kaydında çok yaygın olarak
kullanılan [`make`](https://en.wikipedia.org/wiki/Make_(software)) yapılandırma
sistemi paketinin adı `make` iken Nix paket yöneticisi kaydında aynı paketin adı `gnumake`'dir. Bu gibi durumlar program
geliştirirken hız ve üretkenlik kaybına sebep olur.

Kullanıcıların bu tarz sorunlarla karşılaşmaması için geliştirdiğimiz GENE Paket Yöneticisi, paket yöneticileri
arasındaki farklılıkları
soyutlayarak ([abstraction principle](https://en.wikipedia.org/wiki/Abstraction_principle_(computer_programming)))
ortadan kaldırmayı hedeflemektedir.

GENE, kullanıcı paketleri hakkında gerekli biygileri kayıt etme amacıyla GPacR (Generic PACkage Registry) adını
verdiğimiz kendi paket kayıt sistemini kullanmaktadır.

GPacR kayıt sisteminin yeni geliştirilen paketlere adapte olmasını kolaylaştırmak için paket kayıt sistemini
güncellemek için bir otomasyon sistemi geliştirdik. Bu sistem, paketleri agresif bir elemeden geçiren yüksek
performanslı
bir algoritma kullanır. Bu algoritma sayesinde belirli aralıklarla güncellenen GPacR etkin ve tasarruflu bir biçimde
güncel tutulmuş olur.

Paket kayıt sistemi otomasyonu, bulut temelli CI/CD/CT (Continious Integration, Continious Delivery, Continious Testing)
entegrasyonlarıyla
son teknoloji program geliştirme standartları ve bakım protokollerini destekler nitelikte tasarlanmıştır.

Bunun yanında GENE'nin hedef edindiği sistem-bağımsız paket yönetiminin kilit taşlarından birisi olan komut satırı
arayüzü (Commandline Interface) parametrelerinin (cli arguments) tek boyuta indirgenmesi bu problemin çözülmesi
açısından önem taşımaktadır. Bu probleme çözüm olarak GENE, çok dill konuşabilen (polyglot) bir yazılım olarak
geliştirilmiştir. Böylece siz hangi paket yöneticisinin komutlarını kullanırsanız GENE kullandığınız komutları kodların
çalıştığı sistemin anlayacağı hale çevirecek ve böylece özgür bir standardalizasyon sağlanmış olacaktır. GENE'nin bu
yaklaşımının temel sebebi yeni bir standart oluşturmadan var olan standartları birleştirmektir.

> ANAHTAR KELIMELER: paket yöneticisi, soyutlama, cross platform, standartlaştırma

## Kaynaklar

https://en.wikipedia.org/wiki/CURL