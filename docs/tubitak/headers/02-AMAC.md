# AMAÇ

> Bu bölümde doğrudan projenin amacına, somut hedeflerine ve Ar-Ge içeriğine odaklanılmalıdır. Önerilen proje konusunun
> çözülmesi gereken ya da önceden çalışılmış aydınlatılması gereken bir problem olup olmadığı, hangi eksikliği nasıl
> gidereceği veya hangi sorunlara çözüm getireceği açıklanmalıdır. Hazırlanan projenin ilgili olduğu alanlarda uzman
> kişilere sunulacağı dikkate alınarak değerlendirmeye hiçbir katkı sağlamayacak genel konu ve tarihçe anlatımlarından
> kaçınılmalıdır.

GENE Paket Yöneticilerini standart bir zemine oturtmayı amaçlamaktadır.
Farklı Paket Yöneticilerinin paket isimlendirme sistemleri bir standarda uymadığından dolayı
İşletim sistemleri arasında gerekli kütüphane (dependency) kurulumlarında
karşılaşılan sıkıntıları çözmek ve standartlaştırılmamış paket eylemleri komutlarını
standartlaştırmak için geliştirilmiştir.

Örneğin [cURL]() kütüphanesi [RPM]() temelli paket yöneticilerinde `curl-devel`,
[DPKG]() temelli paket yöneticilerinde `libcurl` adını almaktadır.

Aynı şekilde sisteminizde bulunan paketleri güncellemek için `Pacman` paket yöneticisine `pacman -Syu`
komutunu verirken `APT` paket yöneticisine `apt upgrade` komutunu girmelisiniz. (admin yetkileri dahil edilmemiştir)

GENE, yazılımcıların günlük işlerinde sık karşılaştıkları bu tarz sıkınıntıları yeni bir standart oluşturmadan önüne
geçmeyi hedeflemektedir.
