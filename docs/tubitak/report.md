# GENE Paket 2xYöneticisi

> PROJE ANA ALANI: Yazılım
>
> PROJE TEMATİK ALANI: Dijital Dönüşüm
>
> PROJE ADI: GENE

## PROJE ÖZETİ

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
