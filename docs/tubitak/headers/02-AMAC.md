# AMAÇ

> Bu bölümde doğrudan projenin amacına, somut hedeflerine ve Ar-Ge içeriğine odaklanılmalıdır. Önerilen proje konusunun
> çözülmesi gereken ya da önceden çalışılmış aydınlatılması gereken bir problem olup olmadığı, hangi eksikliği nasıl
> gidereceği veya hangi sorunlara çözüm getireceği açıklanmalıdır. Hazırlanan projenin ilgili olduğu alanlarda uzman
> kişilere sunulacağı dikkate alınarak değerlendirmeye hiçbir katkı sağlamayacak genel konu ve tarihçe anlatımlarından
> kaçınılmalıdır.

Bir yazılımı sisteminize indirirken öncelikle kullandığınız paket yöneticisinin kayıt sistemindeki adını bilmeniz
gerekmektedir. Bu isimler, kayıt sistemlerine bağlı olarak değişkenlik gösterdiğinden yazılım geliştiricileri
kimi paketlerin kurulumunu yaparken paket isimlerinin uyuşmazlığı sonucu sorunlarla karşılaşmaktadır.

GENE; paket yöneticilerindeki belirsizlikleri, yeni bir standart oluşturmadan tek boyuta indirgeyerek ortadan kaldırmayı
amaçlamaktadır.

Örneğin [cURL]() kütüphanesi; kullandığınız paket yöneticisine
göre `curl-devel`, `libcurl`, `curl-dev`, `libcurl-devel`, `curl` gibi pek çok isim almaktadır.

Aynı durum paket yöneticilerinin kullanımı için de geçerlidir. Mesela `Pacman` paket yöneticisinde paketlerinizi
güncellemek için `pacman -Syu` komutunu kullanırken `APT` paket yöneticisinde `apt upgrade` komutunu kullanırsınız.
(admin yetkileri dahil edilmemiştir)

GENE yazılımcıların günlük işlerinde sık karşılaştıkları bu tarz sıkınıntıların önüne geçmeyi hedeflemektedir.

Ancak paket yöneticilerinin komutlarını standartlaştırmak, yeni bir standart ortaya koyarak amacına ulaşamama riski
barındırdığından GENE yeni bir paketleme sistemi oluşturarak bilgi kargaşası oluşturmak yerine var olan standartları
anlamlandırarak geniş kitlelere hitap etmeye odaklıdır.

Figürde verildiği gibi GENE, bütün işletim sistemlerinin paket yöneticilerinin komutlarını anlayıp sizin sisteminize
uyarlar. Bu sayede yeni bir döküman okumanıza gerek kalmadan istediğiniz paket yöneticisini kullanabilirsiniz

```text
                                              .-------------.
                                              |    GENE     |
                                              '------.------'
                                                     |
       .---------------------+-----------------------+------------------------+------------------------.
.------^------.       .------^------.       .--------^---------.       .------^-------.        .-------^-------.
| apt upgrade |       | pacman -Syu |       | scoop update "*" |       | brew upgrade |        |      ...      |
'-------------'       '-------------'       '------------------'       '--------------'        .---------------.
|   UBUNTU    |       |  Archlinux  |       |      WINDOWS     |       |    MAC OS    |        |      ...      |
'-------------'       '-------------'       '------------------'       '--------------'        '---------------'
```

Aşağıdaki tabloda popüler paket yöneticilerinin basit eylemler için kullandıkları komutlar arasındaki farklılıklara örnekler verilmiştir.

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
| `Zypper`	        | `zypper install <paket>` | `zypper update <paket>`    | `zypper search <paket>`   | `zypper remove <paket>`    |
| `APK`            | `apk add <paket>`        | `apk upgrade <paket>`      | `apk search <paket>`      | `apk del <paket>`          |
| `Xbps`           | `xbps-install <paket>`   | `xbps-install -Su <paket>` | `xbps-query -Rs <paket>`  | `xbps-remove <paket>`      |
| `RPM`            | `rpm -i <paket>`         | `rpm -U <paket>`           | `rpm -qf <paket>`         | `rpm -e <paket>`           |
| `Portage`        | `emerge <paket>`         | `emerge --update <paket>`  | `emerge --search <paket>` | `emerge --unmerge <paket>` | [//]: # (Validate)

```text
  .----------.                        
  | curl-dev |------.
  '----------'      |       
  .-------------.   |     
  | libcurl-dev |---+                    
  '-------------'   |                                  .---------.  .-----------------.
  .---------------. |                               .--| WINDOWS | | scoop, chocolatey |
  | libcurl-devel |-+                               |  '---------'  '-----------------'
  '---------------' |          .-------------.      |  .---------.  .------------------------------------------.
  .------------.    +----------|    GENE     |------+--|  LINUX  | | pacman, apt, yum, portage, xbps, nix, ...  | 
  | curl-devel |----+          '-------------'      |  '---------'  '------------------------------------------'
  '------------'    |                               |  .---------.  .------------------.
  .------.          |                               '--|  MACOS  | | homebrew, macports |
  | curl |----------+                                  '---------'  '------------------'
  '------'          |   
  .---------.       | 
  | libcurl |-------'
  '---------'      
```                                     

## Kaynaklar

https://pkgs.org/search/?q=curl

https://github.com/ScoopInstaller/Scoop/issues/897
https://apple.stackexchange.com/questions/56419/how-can-i-update-everything-installed-through-homebrew-after-osx-upgrade

https://pkgs.org/search/?q=curl
https://forums.fedoraforum.org/showthread.php?191104-install-libcurl

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
