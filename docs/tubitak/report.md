# GENE Paket 2xYöneticisi

> DİKKAT: Proje raporunuza kesinlikle “kapak sayfası” yapmayınız ve kişisel bilgilerinizi
(adınız-soyadınız, okul adı, danışman adı, okul logosu ve filigranı dâhil) yerleştirmeyiniz.
Oluşturacağınız proje raporu, Times New Roman karakterleriyle, 12 punto, sayfanın her
yanından 2,5 cm boşluk bırakarak, tek satır aralığı ile iki yana yaslı olarak yazılmalıdır. Proje
Raporu en fazla 20 sayfa olmalıdır. Aşağıda yer alan bölümlerin yazılı kısımlarını çıkararak
kendi metinlerinizi ekleyebilirsiniz (Raporunuzu yazarken bu “Dikkat” paragrafını silmeyi
unutmayınız).

> PROJE ANA ALANI: Yazılım
>
> PROJE TEMATİK ALANI: Dijital Dönüşüm
>
> PROJE ADI: GENE

## PROJE ÖZETİ

### OKULDA

Son yıllarda yazılım geliştirmeyi kolaylaştırmak adına geliştirilen teknolojiler arasında
günlük kullanımda en sık kullanılan yazılımlardan birisi de şüphesiz paket yöneticileridir.

Paket Yöneticilerini 3 temel başlık altında inceleyebiliriz:
#### SPM (System Package Manager)

Sistem Paket Yöneticileri çoğu Linux dağıtımında varsayılan olarak yer alan, yazılım geliştirme adına
faydalarından dolayı Windows ve MacOS gibi işletim sistemlerine de replikaları yazılmış bir programdır. [//] # (sureify)

SPM'lere 

#### LPM (Language Package Manager)

Programlama dillerine özgü paket yöneticilerinin genel kullanım alanı bir projeyi geliştirirken
kullanılan kütüphaneleri indirme amacıyla kullanılır. Örnek olarak:
Python programlama dilinin ünlü paket yönetici olan "Preffered Installer Program" (PIP)
ya da Rust programlama dilinin `cargo` paket yöneticisi örnek verilebilir.

#### PDM (Project Dependency Manager)

Kullanımı LPM ve SPM'lere kıyasla daha az yaygın olan bir paket yöneticisi tipidir.
Genellikle büyük çaplı frameworklerin kullanması gereken kütüphanelerin indirilmesi ve
düzenlenmesi için kullanılır.

PDM'lere Conda paket yöneticisi, Neovim kod editörü için geliştirilmiş Lazy.nvim paket yöneticisi,
JetBrains IDE'lerinde kullanılan eklenti sistemi veya .NET framework için kullanılan NuGet örnek verilebilir. [//] # (search furthermore for NuGet Package manager)


### OKULDA END

Profesyonel yazılımcıların 40% ila 50%'u linux temelli işletim sistemi kullanıyor.
Kullanımının bu kadar yaygın olmasının pek çok sebebi olsa da Linux tabanlı işletim
sistemlerinin sunduğu en önemli avantajlardan biri paket yöneticileridir. Her şeyin
sanal ortama taşındığı günümüzde paket yöneticilerinin önemi özellikle yazılım geliştiren
kişiler için daha da artmıştır.

Paket yöneticileri, kullanıcıların yazılım kurulumu, güncelleme ve kaldırma gibi
işlemleri kolayca yapabilmelerini sağlamaktadır. Bu işlemlerin öneminden dolayı
linux dağıtımlarının çoğu varsayılan olarak kendi paket yöneticisi ve paket kayıt
sistemleri (package registry) ile gelmektedir.
Bu durum dağıtımların birbirleri ile uyumsuz olmasına ve kullanıcıların farklı dağıtımlar
arasında geçiş yaparken sorun yaşamasına sebep olmasının yanında, yazılım geliştiricilerin
bir paketin kimi dağıtımlarda bulunup kimi dağıtımlarda bulunması, bulunuyorsa paketin adının
ne olduğu gibi sorunlarla karşılaşmasına sebep olmaktadır.

Neredeyse bütün Linux kullanıcılarının karşılaştığı bu sorunları çözmek için geliştirdiğimiz
GENE, var olan bütün paket yöneticilerinin standartlaştırılmış bir arayüz ile kullanılmasını
sağlayarak, kullanıcıların farklı dağıtımlar arasında geçiş yaparken karşılaştıkları sorunları
soyutlayarak (abstraction) ortadan kaldırmayı hedeflemektedir.

> Anahtar Kelimeler: Linux, Paket Yöneticisi, Standartlaştırma, Soyutlama

## PROJE AMACI



## Giriş

> Amaç, (alt) problemler, (alt) hipotezler, varsayımlar, sınırlılıklar, tanımlar ve alt başlıklara yer verilmeli.
> 
> Benzer Çalışmalar, farklılıklar, neyi nasıl farklı yapıyor?, 

GENE projemizin temel amacı, paket yöneticileri arasındaki temel farklılıkları ve bu farklılıklar
sonucu ortaya çıkan sıkıntıları ortadan kaldırmaktır. Neredeyse bütün yazılım geliştiricilerinin
kullandığı paket yöneticileri, sistemden sisteme farklılık gösterdiğinden dolayı ortaya çıkan kafa
karışıklığına sebep olmaktadır. Örneğin Debian tabanlı Linux dağıtımlarında yaygın olan `apt` paket 
yöneticisinde en ünlü yapılandırma sistemlerinden biri olan make paketinin adı `make` iken, son
zamanlarda Linux dağıtımları arasında popülerlik kazanan NixOS'in kullandığı `nix` paket yöneticisinde
`gnumake` ismine sahiptir. Bu tarz bir problemi düzeltmenin yolu internet üzerinden kısa bir araştırma
yapmak ve paketin doğru adını bulmaktır. Bu durum her paket için geçerli olmasa da gereksiz zaman kaybına
sebep olmaktadır.

GENE paket yöneticisi yalnızca paket isimlerini standartlaştırmakla yetinmez. Kullanılan paket yöneticisinin
komutlarını, dökümantasyonunu da belirli bir standarda oturtarak sistemler arası birliktelik ve standartlaşma
sağlar.

GENE paket yöneticisi geliştirilirken kolay modülarize edilebilir şekilde tasarlanmış ve üzerine yeni eklentiler
eklenmesi kolay hale getirilmiştir. Bu sayede GENE'nin desteklemediği bir paket yöneticisi için
destek sağlamak oldukça kolaydır.

### GPacistry

GENE'nin en özgün özelliklerinden birisi kendi paket kayıt sistemini tutarak paketler arası standartlaşmayı sağlamasıdır.

Algoritmamız şu şekilde çalışır:

- Desteklenmek üzere tasarlanmış paket yöneticilerinden paket bilgilerini karşılaştır.
- Paket verileri örtüşen paketleri eşleştir.
- Eşleşen paketlere en yaygın kullanılan ismi ata.
- Verileri Web tabanlı bir veri tabanına gönder (package registry)
- Geri kalan paketler arasında eşleşebilecek bütün paketleri eşleştirene kadar devam et.

Bu işlemlerden sonra eşleştirdiğimiz paketler, standartlaşma işlemini başarıyla tamamladıklarından dolayı amacımıza ulaşmış olduk.
Geriye kalan eşleşmeyen paketler için ise GENE, bir gruplama işlemi uygulayarak sisteme bağlı paketleri kendi veri havuzuna aktarır.
Bunu yapmak için aşağıdaki algoritmayı kullanır:

> [!NOTE]
> Bu aşamadaki bütün paketlerin tekil (eşi olmayan) olduğu bilindiğinden
> ayrı bir eleme aşamasından geçmemektedir.

- Elemeden geçecek olan paketin hangi paket yöneticilerininde çalışacağı bilgisini veri havuzuna alır.
- Eğer paketi indirmeye çalıştığınızda sisteminizde pakete uygun bir paket yönetcisi yoksa sizi uyarır ve paketi indirmez.

### Çevirimler

GENE Paket yöneticisi, bütün sistemlerde aynı şekilde çalışabilmek için geliştirilmiş bir komut şablonu kullanır.
aşağıda bu problemin örneklerini popüler paket yöneticileri ile gösterimi bulunmaktadır.


| Paket Yönetcisi | Paket İndirme | Paket Güncelleme | Paket Sorgulama |

| Paket Yöneticisi | İndirme Komutu          | Güncelleme Komutu                         | Sorgulama Komutu       | Silme Komutu                                   |
|:-----------------|:------------------------|:------------------------------------------|:-----------------------|:-----------------------------------------------|
| `APT`            | `apt install <paket>`   | `apt upgrade <paket>`                     | `apt search <paket>`   | `apt remove <paket>`                           |
| `Pacman`         | `pacman -S <paket>`     | `pacman -S <paket>`                       | `pacman -Ss <paket>`   | `pacman -Rsc <paket>`                          | 
| `Nix`            | `nix-env -i <paket>`    | `nix search <paket>`                      | `nix-env -u <paket>`   | `nix -e <paket>`                               |
| `Homebrew`       | `brew install <paket>`  | `brew upgrade <paket>`                    | `brew serach <paket>`  | `brew uninstall <paket>`                       |
| `Chocolatey`     | `choco install <paket>` | `choco upgrade <paket>`                   | `choco search <paket>` | `choco uninstall <paket>`                      |
| `Scoop`          | `scoop install <paket>` | `scoop update <paket>`                    | `scoop search <paket>` | `scoop uninstall <paket>`                      |
| `Yum`            | `yum install <paket>`   | `yum update <paket>`                      | `yum search <paket>`   | `yum erase <paket>` ya da `yum remove <paket>` |
| `Dnf`            | `dnf install <paket>`   | TODO: validate `dnf upgrade <paket>`      | `dnf search <paket>`   | `dnf remove <paket>`                           |
| `RPM`            | `rpm -i <paket>`        | TODO: betterify `rpm -U <rpm dosya ismi>` | TODO: `rpm  <paket>`   | `rpm -e <paket>`                               |

<!-- Projemizin çalışma süreci (runtime) içerisinde kimi algoritmalar tarafından çevirimler uygulanıp -->
<!-- GENE komutları kullandığınız sistemin paket yöneticisinin komutlarına çevrilmekte ve bu sayede   -->
<!-- tamamen soyutlaştırılmış bir şekilde sistemler arası standartlaşma sağlanmaktadır.               -->


## Yöntem

> Bu bölümde projenin nasıl geliştirildiği anlatılacaktır.

GENE Projesini geliştirmeden önce belirli bir süreyi planlamaya ayırdık. Bu süreçte
proje için gerekli olan teknolojileri araştırdık ve bu teknolojilerin kullanımı ile
ilgili örnekler geliştirdik. Genel proje tasarım şemasını çıkardıktan sonra projenin
geliştirilmesine başladık.

### Rust

GENE Rust programlama dili ile geliştirilmiştir. Rust programlama dilini seçmemizin
sebebi yüksek performanslı, güvenli ve sistem programlama için tasarlanmış olmasıdır.
Bunun yanında Cebirsel Veri Türleri (Algebraic Data Types) ve Pattern Matching gibi
fonksiyonel programlama dillerinde bulunan özelliklerin Rust'ta da bulunması ve Rust'ın
bu özellikleri kullanımı kolay bir şekilde sunması, Rust'ın paket yöneticisi olan 
Cargo'nun da Rust programlama dili ile geliştirilmiş programları geliştirmeyi inanılmaz 
derecede kolaylaştırması Rust'ı seçmemizde etkili olan etkenler arasındadır.


### Veri Otomasyonu

Paket yöneticilerinin kayıt sistemlerindeki paketlerin platformlar arası uyumluluğunu
test edecek bir otomasyon sistemi geliştirdik. Bu sistemde paket yöneticilerinin
kullanıcıya sunduğu opsiyonlar sonucu elde ettiğimiz verileri diğer paket yöneticilerinin
verileri ile karşılaştırarak uyumluluk testi yaparak bunu kendi kayıt sistemimiz olan GPacistry'ye
yedekledik. Böylece GPacistry'de önbellekte (cache) tutulan paketler arasından paket araması 
perfomansını [TODO: yüzdelik ver] %10000 daha hızlı bir şekilde sunabiliyoruz.

### GPacistry

> Generic PACkage regISTRY

Rust programlama dili ile geliştirilmiş olan `sqlx` crate'i ile web tabanlı bir paket
kayıt sistemi geliştirdik. Bu kayıt sistemi GPacistry olarak adlandırılmaktadır. GPacistry

## Bulgular

## Sonuç ve Tartışma

## Öneriler

## Kaynaklar

- Nix Paket Yöneticisi komutları: https://www.mankier.com/1/nix-env https://github.com/brainrake/nixos-tutorial/blob/master/cheatsheet.md
- Pacman Paket Yöneticisi komutları: https://devhints.io/pacman
- HomeBrew Paket Yöneticisi komutları: https://devhints.io/homebrew https://stackoverflow.com/questions/8833230/how-do-i-find-a-list-of-homebrews-installable-packages
- Chocolatey Paket Yöneticisi komutları: https://gist.github.com/yunga/ https://docs.chocolatey.org/en-us/choco/commands/upgrade https://docs.chocolatey.org/en-us/choco/commands/uninstall
- Scoop Paket Yöneticisi komutları: https://github.com/ScoopInstaller/Scoop/wiki/Commands
- Yum Paket Yöneticisi komutları: https://access.redhat.com/sites/default/files/attachments/rh_yum_cheatsheet_1214_jcs_print-1.pdf
- Dnf Paket Yöneticisi komutları: https://docs.fedoraproject.org/en-US/quick-docs/dnf/
- RPM Pkaet Yöneticisi komutları: https://www.golinuxcloud.com/rpm-command-in-linux/

- Paket Yöneticisi tasarımı genel bilgi: https://medium.com/@sdboyer/so-you-want-to-write-a-package-manager-4ae9c17d9527


## Ekler

### NOTES

Geniş çapta yapılan literatür taramaları sonucunda geliştirdiğimiz projemize benzer
bir proje bulunmamaktadır. Projemiz GENE'ye en çok benzeyen proje ise bedrock linux
dağıtımının `pmm` paket yöneticisidir. Bedrock linux, hem çoğu linux kullanıcısı
tarafından bilinmeyen hem de geliştirici kitlesi oldukça küçük ve aktif olmayan bir
linux dağıtımıdır.

### Footnotes

| Operating System  | Professional use |
|:------------------|-----------------:|
| Windows           |           46.91% |
| MacOS             |              33% |
| Ubuntu            |           26.69% |
| Android           |            8.23% |
| WSL               |           15.68% |
| iOS               |            7.37% |
| Debian            |            8.09% |
| Other Linux-based |             7.7% |
| Arch              |            4.37% |
| iPadOS            |            2.77% |
| Red Hat           |            4.64% |
| Fedora            |            3.05% |
| ChromeOS          |            1.06% |
| Cygwin            |            0.92% |
| BSD               |            0.59% |
| AIX               |            0.41% |
| Solaris           |            0.36% |
| Haiku             |            0.08% |

- https://www.enterpriseappstoday.com/stats/linux-statistics.html#:~:text=Linux%20Usage%20Statistics%201%20Professional%20developers%20love%20Linux,1%25%20use%20another%20operating%20system%29%20prefer%20Windows.%20%28Statista%29
- https://survey.stackoverflow.co/2023/#section-most-popular-technologies-operating-system
- https://gs.statcounter.com/os-market-share/desktop/worldwide
- https://gs.statcounter.com/os-market-share
- https://en.wikipedia.org/wiki/Usage_share_of_operating_systems

Linux paket yöneticileri

- https://theboreddev.com/8-reasons-why-linux-is-good-for-coding/
