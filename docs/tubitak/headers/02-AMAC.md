# AMAÇ

> Bu bölümde doğrudan projenin amacına, somut hedeflerine ve Ar-Ge içeriğine odaklanılmalıdır. Önerilen proje konusunun
> çözülmesi gereken ya da önceden çalışılmış aydınlatılması gereken bir problem olup olmadığı, hangi eksikliği nasıl
> gidereceği veya hangi sorunlara çözüm getireceği açıklanmalıdır. Hazırlanan projenin ilgili olduğu alanlarda uzman
> kişilere sunulacağı dikkate alınarak değerlendirmeye hiçbir katkı sağlamayacak genel konu ve tarihçe anlatımlarından
> kaçınılmalıdır.

GENE Paket Yöneticilerini standart bir zemine oturtmayı amaçlamaktadır.
Farklı Paket Yöneticilerinin paket isimlendirme sistemleri bir standarda uymadığından dolayı
İşletim sistemleri arasında gerekli kütüphane (dependency) kurulumlarında
karşılaşılan sıkıntıları çözmek için paket yöneticilerinin değişken özelliklerini
standartlaştırarak fikir birliği sağlamaktır.

Örneğin [cURL]() kütüphanesi [RPM]() temelli paket yöneticilerinde `curl-devel`,
[DPKG]() temelli paket yöneticilerinde `libcurl` adını almaktadır.

Aynı şekilde sisteminizde bulunan paketleri güncellemek için `Pacman` paket yöneticisine `pacman -Syu`
komutunu verirken `APT` paket yöneticisine `apt upgrade` komutunu girmelisiniz. (admin yetkileri dahil edilmemiştir)

GENE yazılımcıların günlük işlerinde sık karşılaştıkları bu tarz sıkınıntıların önüne geçmeyi hedeflemektedir.

Ancak paket yöneticilerinin komutlarını standartlaştırmak, yeni bir standart ortaya koyarak mümkün olmayacağından
GENE yeni bir standart oluşturup bilgi kargaşası oluşturmak yerine var olan standartları anlamaya odaklıdır.

figürde verildiği gibi GENE, bütün işletim sistemlerinin paket yöneticilerinin komutlarını anlayıp sizin sisteminize uyarlar. 
Bu sayede yeni bir döküman okumanıza gerek kalmadan istediğiniz paket yöneticisini kullanabilirsiniz

```text
                                              .-------------.
                                              |     ***     |
                                              '------.------'
                                                     |
                                              .------^------.
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

Aşağıdaki tabloda popüler paket yöneticilerinin basit eylemler için kullandıkları komutlar bulunmaktadır.

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

```text
  .-----------------.-------.                        
  | simdjson v3.6.0 | NixOS |------------------------.                        
  '-----------------'-------'                        |    
  .-----------------.--------------.               |            
  | simdjson v3.6.0 | Alpine Linux |------------+ |                       
  '-----------------'--------------'             | |                           .---------.---------.
  .-----------------------------------------.----------------.    | |                        .--| WINDOWS |         |  
  | libsimdjson-devel-3.6.2-alt1.x86_64.rpm | Classic x86_64 |--+ | |                        |  '---------'---------'
  '-----------------------------------------'----------------'  | | |   .-------------.      |  .---------.---------.
                                                                +-+-+---|    GENE     |------+--|  LINUX  |         |           
  .-----------------------------------.-------------------.     | | |   '-------------'      |  '---------'---------'
  | libsimdjson-dev_3.0.1-1_arm64.deb | Debian Main arm64 |-----+ | |                        |  .---------.---------.
  '-----------------------------------'-------------------'       | |                        '--|  MACOS  |         |                                                   
  .----------------------------------------.---------------.      | |                           '---------'---------'
  | simdjson v[1] | Fedora x86_64 |------+ |
  '----------------------------------------'---------------'        |
  .------------------------.-----------------------.       |
  | libsimdjson-dev v3.0.1 | Debian |-------' 
  '------------------------'-----------------------'
  .-----------------.----------.       |
  | simdjson v3.6.3 | homebrew |-------' 
  '-----------------'----------'
  .--------------------------.---------.       |
  | dev-libs/simdjson v3.6.3 | Portage |-------' 
  '--------------------------'---------'
  .---------------------------------.-----------------------.       |     [1]: Fedora Rawhide	3.6.3-1.fc40	-	-
  | libsimdjson16_3.2.1-1_amd64.deb | Ubuntu Universe amd64 |-------'          Fedora 39     	3.1.0-2.fc39
  '---------------------------------'-----------------------'                  Fedora 38	    3.1.0-1.fc38

```                                     

## Kaynaklar

- simdjson:
	- https://pkgs.org/search/?q=simdjson
	- https://search.nixos.org/packages?channel=unstable&show=simdjson&from=0&size=50&sort=relevance&type=packages&query=simdjson
	- https://pkgs.org/search/?q=simdjson
 	- https://pkgs.alpinelinux.org/packages?name=simdjson&branch=edge&repo=&arch=&maintainer=
  	- https://formulae.brew.sh/formula/simdjson#default
  	- https://scoop.sh/#/apps?q=simdjson
  	- https://packages.debian.org/search?suite=bookworm&section=all&arch=any&searchon=names&keywords=simdjson

https://pkgs.org/search/?q=curl

https://github.com/ScoopInstaller/Scoop/issues/897
https://apple.stackexchange.com/questions/56419/how-can-i-update-everything-installed-through-homebrew-after-osx-upgrade

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
