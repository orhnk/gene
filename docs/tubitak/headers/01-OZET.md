# ÖZET

> Özetin tamamı 150-250 kelime arasında olmalıdır. Proje özetinde, çalışmanın ayrıntılarından, yorumlardan ve
> kaynaklardan bahsedilmez. Özette projenin amacı, kullanılan yöntem, yapılan gözlem ve elde edilen temel bulgular ve
> sonuçlardan birkaç cümle ile bahsedilir. Ayrıca proje özetinin altına, proje konusunu genel olarak yansıtan en fazla
> beş
> kelimeden oluşan anahtar kelimeler verilir. İdeal olan başlarken taslak bir özet oluşturup, çalışma bittiğinde proje
> raporunun içeriğine uygun bir şekilde özeti güncellemektir.

Son yıllarda dijital dünya adına geliştirilen teknolojiler sayesinde program geliştirmek için başvurulan yöntemlerin verimi zamanla artmıştır.

Örneğin 70'lerde bilgisayar yazılımcıları, programlarını delikli kartlar üzerine yazarken bu işlem daha sonra assmebly gibi düşük seviye makine dillerine,
ardından C gibi düşük seviye programlama dillerine ve sonunda LLVM gibi modern altyapılar kullanan Zig, Rust gibi programlama dillerine dönüşmüştür.

Bu gelişmelerde göze çarpan iyileştirmelerden birisi de standardalizasyon alanındadır.

Mesela C/C++ gibi programlama dillerinin standard bir paket yöneticileri yoktur. Şimdiye kadar vcpkg, canon gibi paket yöneticisi girişimleri olsa da
genel bir standard yakalanamamıştır. Daha sonraları çıkan Rust gibi programlama dilleri bu problemi çözmek adına Cargo gibi standart bir paket yöneticisi kullanmış ve kullancıların
karşılaşabileceği belirsizlikleri ortadan kaldırmıştır.

Aynı standardalizasyon sıkıntıları işletim sistemi paket yöneticilerinde de görülmektedir. Arch Linux İşletim Sistemi'nin `pacman -S <paket>` komutu bir paket indirirken
Windows temelli Scoop paket yöneticisinde aynı `scoop install <paket>` şeklindedir. Bu durum, bazı programların dökümanlarında yer almayan komutları kullanmanızı gerektirebildiğinden,
kimi programların kurulumunu zorlaştırmaktadır.

Bu probleme çözüm olarak geliştirdiğimiz `merge` paket yöneticisi emülatörü, işletim sisteminizde istediğiniz paket yöneticisinin komutlarını kullanmanıza olanak sağlar.
`merge` sayesinde Windows'ta `pacman -S <paket>`, `scoop install <paket>`, `emerge --install <paket>` gibi bütün paket yöneticilerinin işletim sisteminizde çalışmasını sağlar.

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

- Punch Kards: https://en.wikipedia.org/wiki/Punched_card_input/output

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
