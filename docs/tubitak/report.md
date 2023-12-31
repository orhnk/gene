# GENE Paket 2xYöneticisi

> DİKKAT: Proje raporunuza kesinlikle “kapak sayfası” yapmayınız ve kişisel bilgilerinizi
(adınız-soyadınız, okul adı, danışman adı, okul logosu ve filigranı dâhil) yerleştirmeyiniz.
> Oluşturacağınız proje raporu, Times New Roman karakterleriyle, 12 punto, sayfanın her
> yanından 2,5 cm boşluk bırakarak, tek satır aralığı ile iki yana yaslı olarak yazılmalıdır. Proje
> Raporu en fazla 20 sayfa olmalıdır. Aşağıda yer alan bölümlerin yazılı kısımlarını çıkararak
> kendi metinlerinizi ekleyebilirsiniz (Raporunuzu yazarken bu “Dikkat” paragrafını silmeyi
> unutmayınız).

> PROJE ANA ALANI: Yazılım
>
> PROJE TEMATİK ALANI: Dijital Dönüşüm
>
> PROJE ADI: GENE Paket Yöneticisi

## PROJE ÖZETİ

Son yıllarda yazılım dijital dünya adına geliştirilen teknolojiler sayesinde program geliştirmek
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

## Proje Özeti son

Son yıllarda yazılım geliştirmeyi kolaylaştırmak adına geliştirilen araçlar arasında arasında
bilgisayar bilimcilerinin en sık kullanıdıklarından birisi şüphesiz paket yöneticileridir.

Paket Yöneticileri; Projenizi geliştirme, bakım faaliyetlerini sürdürme (maintainance), kullanım kolaylığı sağlama gibi
pek çok açıdan
işinizi kolaylaştırdıklarından günümüz yazılım sistemleri için bir standart halini almıştır.

Bu standardtlaşmanın yansımaları olarak Linux kernel'ini kullanan işletim sistemlerinin varsayılan paket yöneticileri,
Windows ve MacOS için geliştirilmiş özel paket yöneticileri örnek verilebilir.

Kullandığınız paket yöneticisini, işletim sisteminizin çalışma mantığı ile aynı doğrultuda olduğundan dolayı,
( bkz. [NixOS](https://nixos.org/)) paket yöneticinizi sisteminizi geliştirilen organizasyon tarafından
geliştirilmiştir.

Büyük işletim sistemi projeleri (örn. Arch, Debian, RedHat, NixOS) kendi paket yöneticilerini temelden
tasarladıklarından dolayı
paket yöneticileri, kullandığınız sisteme özgüdür. Kullandığınız komutlar, dökümanlar, hatta paket kayıt sisteminiz dahi
diğer genellikle sistemlerle uyumlu değildir.

Bu durum, programlama
prensiplerinden ["Generalizasyon Prensibi"](http://principles-wiki.net/principles:generalization_principle)ne aykırı
olup program tasarımının en zor kısmı olan koruma (maintainance) başlığı altındaki standartalizasyon (standardization)
konusunda
ciddi sıkıntılar içermekte ve kafa karışıklığına sebep olmaktadır.

Probleme örnek olarak:

- Ubuntu kullanan bir kullanıcının Archlinux işletim sistemine geçmesi sonucu `pacman` paket yöneticisini kullanmakta
  zorlanması
- SuSe işletim sisteminde ünlü standartlardan biri olan `make` C yapılandırma sisteminin nix paket
  yöneticisinde `gnumake` olması
- Github üzerinden kurulumu yapılmaya çalışılan bir yazılımın bağlı olduğu kütüphaneleri bulmakta zorlanılması

Yazılım geliştiriciliği ile uğraşmış pek çok kişinin karşılaşmış olduğu bu problemi çözme amacıyla geliştirdiğimiz
projemiz GENE,
platformlar arası değişiklikleri, soyutlayarak (
bkz. [abstraction](https://en.wikipedia.org/wiki/Abstraction_(computer_science))) ortadan kaldırmayı hedeflemektedir.
Hedef kitlesi paket yöneticileri aktif kullanan yazılım geliştiricileri olduğundan bu alandaki diğer projelerin güçlü
yanlarından ilham almıştır.

GENE Paket Yöneticisi kendisine üç temel prensip biçer:

- Standart
- Bağımsız
- Dekleratif

## Standart

Bir çok paket ismi sistemden sisteme göre değişiyor. Örneğin Windows Scoop paket yöneticisi `rider` paketi
Nix `jetbrains.rider`

## Bağımsız

## Dekleratif

> Deklerativite açısından Nix Paket Yöneticisinden etkilenmiş ve NixOS işletim sisteminin temel çalışma şemasına bağlı
> kalmıştır.

Paket Yönetimi gerektiren projelerin insan faktöründen dolayı kolaylıkla karmaşık hale gelebilmektedir.
Bir problemi çözerken başka bir probleme yol açmamak için GENE Paket Yöneticisi'ni dekleratif biçimde tasarladık.

Konfigürasyonu basit, okunabilir ve hataya kapalı bir şekilde yazabilmek için GENE, Yalnızca TOML (Tom's Obvious Markup
Language)
İşaretleme Dili'ni tercih etmiştir.

GENE Paket Yöneticisi'ni önerilen şekilde kullanmak için en az iki adet konfigürasyon dosyası gerekmektedir.
> NOT: Aşağıdaki dosya konumları XDG standardına ve APT paket yöneticisine uygun şekilde yazılmıştır.

1. `$XDG_CONFIG_HOME/gene/gene.toml` - Sistem ana konfigurasyonu
   ```toml
   [system]
   backends = [
		"apt",
   ]
   ```

> `<PROJE_KÖK_KLASÖRÜ>` yerine indireceğiniz paketerin ilgili projesinin ana klasörünü yazınız.

2. `<PROJE_KÖK_KLASÖRÜ>/gene.toml` - Paket indirme amaçlı Lokal konfigürasyon
   ```toml
   [system]
   deps = [
		"make",
   ]
   ```

Kullandığınız projeyi dekleratif biçimde konfigüre ettiğinizden dolayı sizin bilgisayarınızda çalışan bir paket
başkasının bilgisayarında da çalışacaktır.

<!-- COMMENTSTR
Paket Yöneticileri sağladıkları faydalardan dolayı İşletim Sistemi bağlamından başka sistemlerle de kullanılmaktadır
Bu Paket Yöneticilerini 3 temel başlık altında inceleyebiliriz:

#### SPM (System Package Manager)

"Sistem Paket Yöneticisi" ingilizce kısaltılmasıdır.

Sistem Paket Yöneticileri; çoğu Linux dağıtımında varsayılan olarak yer alan, yazılım geliştirme adına
faydalarından dolayı Windows ve MacOS gibi işletim sistemlerine de replikaları yazılmış bir program türüdür. [//] # (sureify)

Örnek olarak;
- MacOS X'i hedef alan `homebrew`, `MacPorts`
- Windows'u hedef alan `scoop` ve `chocolatey`
- Linux temelli işletim sistemleri için pek çok paket yöneticisi olsa da en çok bilinenler arasında `APT`, `Pacman`, `XBPS`, `RPM` ve RPM'i temel alan paket yöneticileri örnek verilebilir.

#### LPM (Language Package Manager)

"Programlama Dili Paket yöneticisi" ingilizce kısaltılmasıdır.

Programlama dillerine özgü paket yöneticilerinin genel kullanım alanı bir projeyi geliştirirken
kullanılan kütüphaneleri indirme amacıyla kullanılır.

Örnek olarak;
- Python programlama dilinin ünlü paket yönetici olan "Preffered Installer Program" (PIP)
- Rust programlama dilinin `cargo` paket yöneticisi.
- C/C++ programlama dilleri için (standartlaştırılmamış olmalarına rağmen) `vcpkg`, `conda`
verilebilir.

#### PDM (Project Dependency Manager)

"Proje Paket Yöneticisi" ingilizce kısaltamsıdır.

Kullanımı LPM ve SPM'lere kıyasla daha az yaygın olan bir paket yöneticisi tipidir.
Genellikle büyük çaplı frameworklerin kullanması gereken kütüphanelerin indirilmesi ve
düzenlenmesi için kullanılır.

PDM'lere `Conda` paket yöneticisi, Neovim kod editörü için geliştirilmiş `Lazy.nvim` paket yöneticisi,
JetBrains IDE'lerinde kullanılan eklenti sistemi veya `.NET` framework için kullanılan `NuGet` örnek verilebilir. [//] # (search furthermore for NuGet Package manager)


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
-->

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

GENE'nin en özgün özelliklerinden birisi kendi paket kayıt sistemini tutarak paketler arası standartlaşmayı
sağlamasıdır.

Algoritmamız şu şekilde çalışır:

- Desteklenmek üzere tasarlanmış paket yöneticilerinden paket bilgilerini karşılaştır.
- Paket verileri örtüşen paketleri eşleştir.
- Eşleşen paketlere en yaygın kullanılan ismi ata.
- Verileri Web tabanlı bir veri tabanına gönder (package registry)
- Geri kalan paketler arasında eşleşebilecek bütün paketleri eşleştirene kadar devam et.

Bu işlemlerden sonra eşleştirdiğimiz paketler, standartlaşma işlemini başarıyla tamamladıklarından dolayı amacımıza
ulaşmış olduk.
Geriye kalan eşleşmeyen paketler için ise GENE, bir gruplama işlemi uygulayarak sisteme bağlı paketleri kendi veri
havuzuna aktarır.
Bunu yapmak için aşağıdaki algoritmayı kullanır:

> [!NOTE]
> Bu aşamadaki bütün paketlerin tekil (eşi olmayan) olduğu bilindiğinden
> ayrı bir eleme aşamasından geçmemektedir.

- Elemeden geçecek olan paketin hangi paket yöneticilerininde çalışacağı bilgisini veri havuzuna alır.
- Eğer paketi indirmeye çalıştığınızda sisteminizde pakete uygun bir paket yönetcisi yoksa sizi uyarır ve paketi
  indirmez.

### Çevirimler

GENE Paket yöneticisi, bütün sistemlerde aynı şekilde çalışabilmek için geliştirilmiş bir komut şablonu kullanır.
aşağıda bu problemin örneklerini popüler paket yöneticileri ile gösterimi bulunmaktadır.


> NOT: Aşağıda Nix Paket Yöneticisi'nin komut hali verilmiştir ancak Nix Paket Yöneticisi'nin manuel olarak
> çalıştırılması, tavsiye edilmez. Bunun yerine `configuration.nix` dosyası üzerinden yapılandırma yapılmalıdır.

| Paket Yöneticisi | İndirme Komutu           | Güncelleme Komutu          | Sorgulama Komutu          | Silme Komutu               |
|:-----------------|:-------------------------|:---------------------------|:--------------------------|:---------------------------|
| `APT`            | `apt install <paket>`    | `apt upgrade <paket>`      | `apt search <paket>`      | `apt remove <paket>`       |
| `Pacman`         | `pacman -S <paket>`      | `pacman -S <paket>`        | `pacman -Ss <paket>`      | `pacman -Rsc <paket>`      | 
| `Nix`            | `nix-env -i <paket>`     | `nix search <paket>`       | `nix-env -u <paket>`      | `nix -e <paket>`           |
| `Homebrew`       | `brew install <paket>`   | `brew upgrade <paket>`     | `brew serach <paket>`     | `brew uninstall <paket>`   |
| `Chocolatey`     | `choco install <paket>`  | `choco upgrade <paket>`    | `choco search <paket>`    | `choco uninstall <paket>`  |
| `Scoop`          | `scoop install <paket>`  | `scoop update <paket>`     | `scoop search <paket>`    | `scoop uninstall <paket>`  |
| `Yum`            | `yum install <paket>`    | `yum update <paket>`       | `yum search <paket>`      | `yum remove <paket>`       |
| `Dnf`            | `dnf install <paket>`    | `dnf update <paket>`       | `dnf search <paket>`      | `dnf remove <paket>`       |
| `Zypper`		       | `zypper install <paket>` | `zypper update <paket>`    | `zypper search <paket>`   | `zypper remove <paket>`    |
| `APK`            | `apk add <paket>`        | `apk upgrade <paket>`      | `apk search <paket>`      | `apk del <paket>`          |
| `Xbps`           | `xbps-install <paket>`   | `xbps-install -Su <paket>` | `xbps-query -Rs <paket>`  | `xbps-remove <paket>`      |
| `RPM`            | `rpm -i <paket>`         | `rpm -U <paket>`           | `rpm -qf <paket>`         | `rpm -e <paket>`           |
| `Portage`        | `emerge <paket>`         | `emerge --update <paket>`  | `emerge --search <paket>` | `emerge --unmerge <paket>` | [//]: # (Validate)

[//]: # (| `RPM-ostree`     | `rpm-ostree install <paket>` | `rpm-ostree upgrade <paket>` | `rpm-ostree search <paket>` | `rpm-ostree uninstall <paket>` |)

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

- Nix Paket Yöneticisi
  komutları: https://www.mankier.com/1/nix-env https://github.com/brainrake/nixos-tutorial/blob/master/cheatsheet.md
- Pacman Paket Yöneticisi komutları: https://devhints.io/pacman
- HomeBrew Paket Yöneticisi
  komutları: https://devhints.io/homebrew https://stackoverflow.com/questions/8833230/how-do-i-find-a-list-of-homebrews-installable-packages
- Chocolatey Paket Yöneticisi
  komutları: https://gist.github.com/yunga/ https://docs.chocolatey.org/en-us/choco/commands/upgrade https://docs.chocolatey.org/en-us/choco/commands/uninstall
- Scoop Paket Yöneticisi komutları: https://github.com/ScoopInstaller/Scoop/wiki/Commands
- Yum Paket Yöneticisi
  komutları: https://access.redhat.com/sites/default/files/attachments/rh_yum_cheatsheet_1214_jcs_print-1.pdf
- Dnf Paket Yöneticisi komutları: https://docs.fedoraproject.org/en-US/quick-docs/dnf/
  [//] # (- RPM Paket Yöneticisi
  komutları: https://www.golinuxcloud.com/rpm-command-in-linux/ https://access.redhat.com/solutions/1189)
- Zypper Paket Yöneticisi komutları: https://www.maketecheasier.com/cheatsheet/zypper-package-manager/
- APK Paket Yöneticisi komutları: https://wiki.alpinelinux.org/wiki/Alpine_Linux_package_management
- Xbps Paket Yöneticisi komutları: https://docs.voidlinux.org/xbps/index.html

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
