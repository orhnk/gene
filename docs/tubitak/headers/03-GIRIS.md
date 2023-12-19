# GİRİŞ

> Giriş, araştırma konusu hakkında yapılmış araştırmaların sonuçlarının ve bu alanda cevapsız olan soruların bilimsel
> makalelere dayandırılarak anlatıldığı (kaynak taraması) bölümdür.

[//]: # (GENE, projesi yapılan derin literatür araştırması sonucunda benzersiz bir proje olarak ortaya çıkmıştır.)

## Pkgs.org

> [pkgs.org](https://pkgs.org/)

Genel Linux ve BSD paket kayıt sistemlerini tek bir yerde sorgulamak için geliştirilmiş bir web sitesidir.

## WhoHas

WhoHas, Sistemler arası paket sorgulama aracıdır. Kendisi bir paket yöneticisi değildir. Sadece paket yöneticilerinin
kayıt sistemlerine erişerek sizin aradığınız paket isminin hangi paket yöneticisinde hangi isimle kayıtlı olduğunu
gösterir.

WhoHas ile GENE arasındaki temel fark, WhoHas aradığınız paketin hangi işletim sistemine ait olduğunu söylerken
bulunan paketin sisteminize indirilmesi hakkında fayda sağlamaz. GENE ise aradığınız paketin sizin sisteminizdeki
kaydını bulup indirir.

WhoHas, kullanıcıdan aldığı girdiyi sorguladığı için kimi durumlarda GENE'nin deterministik altyapısının indirdiği
paketleri bulamayabilir. Bu problemin temel sebebi, `whohas` yazılımının paket isimlerini standartlaştırma gibi bir amaç
gütmemesidir.

## Pacaptr

MEW ve Pacaptr, paket yöneticilerinin komutlarını standartlaştırmak için geliştirilmiş projelerdir.
Bu projelerin gözden kaçırdığı nokta, paket yöneticilerinin paket kayıt sistemlerinin standartlaştırılmamış olmasıdır.
Ayrıca bir paket ekosistemi standardize etmek için o ekosiste yeni komutlar eklemek, standartlaştırmanın tam tersine,
yeni bir standart oluşturmak ve karmaşayı daha da artırmak anlamına gelmektedir. Bu problemin üstesinden gelmek için
GENE Paket Yöneticisi, yeni bir komut sistemi icad etmeden var olan paket yöneticilerinin komutları ile çalışır.

## Mew

Mew paket yönetici komutlarını standartlaştırma konusunda kullanıcılara yardımcı olmayı hedefleyen ufak çaplı bir
projedir. Çalışma mantığı olarak .PO ([GNU gettext
utilities](https://www.gnu.org/software/gettext/manual/html_node/PO-Files.html))  dosyaları gibi çalışır.
5 yıldır geliştirilmeyen ve basit bir python kodu ile elle yazılmış JSON dosyalarından oluştuğu için GENE, MEW yerine
tercih edilmelidir.

## Bedrock Linux

Pek bilinmeyen bir Linux dağıtımı olan Bedrock Linux, farklı işletim sistemlerinin programlarını, aynı bilgisayar
üzerinde çalıştırır. Bu sayede bedrock linux, bir çok paket yöneticisini beraber kullanmanıza olanak tanır. ancak
bedrock linux, başlı başına bir işletim sistemi olduğundan, standardalizasyon amacı gütmediğinden ve stabilite, bakım
(maintenance) gibi konularda sıkıntılar yaşadığından dolayı paket yöneticisi standardalizasyonu için uygun değildir.

## Cevaplanmamış Sorular

- Paket Yöneticilerinin paket kayıt sistemleri araralarındaki tek fark paket isimlendirmeleri değildir.
  Kimi paketler tamamiyle aynı olurken bazı paketler parçalara ayrılıp kaydedilir. Bu farklar, paket isimlerini
  standartlaştırmanın kesin bir çözüm olmadığını göstermektedir.

- Paket yönetici kayıt sistemlerinin boyutu; sistemden sisteme değiştiğinden dolayı indirmek istediğiniz paket,
  paket yöneticinizin kayıt sistemine kayıtlı olmayabilir. Böyle bir durumda GENE sizi uyaracaktır.

## Kaynaklar

- yumitude: https://github.com/timols/yumitude Project with the same idea but not implemented.
- MEW: https://github.com/fossasia/mew Project similar to pacaptr.
- whohas: https://github.com/whohas/whohas - A system utility to search from general package registries.
- pacaptr: https://github.com/icy/pacapt - pacman-like syntax for all package managers.
- bedrock linux: https://bedrocklinux.org/0.7/pmm-beta.html package manager manager (a.k.a pmm)
