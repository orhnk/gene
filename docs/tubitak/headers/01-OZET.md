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
[JetBrains](https://www.jetbrains.com/) IDE'leri gibi ileri seviye araçlar kullanılmaktadır.

Bu gelişmelerden etkilenen araçlardan birisi de paket yöneticileridir.
Amatör ya da profesyonel her yazılım geliştiricisinin kullandığı paket yöneticileri,
herhangi bir programın kurulumu, güncellenmesi, kaldırılması gibi işlemleri basitleştirir.

Örneğin python programlama dilinin standart paket yöneticisi pip,
python ile kod yazan olan herkesin sıkça kullandığı bir programdır.

Paket yöneticilerinin sağladıkları faydalar sebebiyle günümüzde pek çok işletim sistemine uyarlanmış
tipleri bulunmaktadır.

Paket yöneticilerinin kendilerine özgü komutları, dökümanları, paket kayıt sistemleri
ve paket adlandırma standartları olması, kullanıcılar arasında uyuşmazlıklara sebep olmaktadır.

Bu duruma örnek olarak [`make`](https://en.wikipedia.org/wiki/Make_(software)) yapılandırma
sistemi paketinin genel adı linux paket arşivlerinde `make` olmasının yanında
Nix paket arşivinde `gnumake` adında olması. Bu durum karşısında kullanıcıların
paket indirirken istedikleri paketleri bulamamalarına sebep olmaktadır.

Farklılık gösteren durumlardan bir diğeri de komut satırı argümanlarıdır (commandline arguments)
örneğin Archlinux işletim sisteminde `pacman -Syu` komutu Ubunut'da `apt upgrade` ile aynı işlevi görmektedir.

Kullanıcıların bu tarz sorunlarla karşılaşmaması için geliştirdiğimiz GENE paket yöneticisi, paket yöneticileri
arasındaki farklılıkları soyutlayarak ([abstraction principle](https://en.wikipedia.org/wiki/Abstraction_principle_(computer_programming))) ortadan kaldırmayı hedeflemektedir.

GENE, kullanıcı paketleri hakkında gerekli bilgileri kayıt etme amacıyla GPacR (Generic PACkage Registry)
adını verdiğimiz kendi tasarruflu paket arşivini kullanmaktadır.

Bu standartlaştırmayı yaparken yeni bir komut tipi oluşturmak yerine var olan komutları anlamaya yönelik
geliştirildiğinden kullanıcıların bildiği komutları diğer sistemlerde kullanılmasına olanak sağlamaktadır.

> ANAHTAR KELIMELER: paket yöneticisi, soyutlama, cross platform, standartlaştırma

## Kaynaklar

- curl: https://en.wikipedia.org/wiki/CURL
- abstraction principle: https://en.wikipedia.org/wiki/Abstraction_principle_(computer_programming)

<!--
GPacR kayıt sisteminin yeni geliştirilen paketlere adapte olmasını kolaylaştırmak ve paket kayıt sistemini
güncellemek için bir otomasyon sistemi geliştirdik. Bu sistem, paketleri agresif bir elemeden geçiren yüksek
performanslı
bir algoritma kullanır. Bu algoritma sayesinde belirli aralıklarla güncellenen GPacR etkin ve tasarruflu bir biçimde
güncel tutulmuş olur.

Paket kayıt sistemi otomasyonu, bulut temelli CI/CD/CT (Continious Integration, Continious Delivery, Continious Testing)
entegrasyonlarıyla
son teknoloji program geliştirme standartları ve bakım protokollerini destekler nitelikte tasarlanmıştır.
-->