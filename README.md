# CrowdFunding

## Project yapısı

CrowdFunding
Proje Yapısı
Bu proje, Rust ve Stellar blok zinciri kullanılarak geliştirilmiş bir fon toplama (crowdfunding) uygulamasıdır. Soroban akıllı sözleşme platformunu kullanarak fon toplama kampanyalarını yönetir ve yürütür. Uygulama, hem backend hem de frontend bileşenlerini içerir.

proje mimarisi: 

.
├── .gitignore
├── .vscode/
│   └── settings.json
├── Cargo.lock
├── Cargo.toml
├── package.json
├── README.md
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── project.rs
│   ├── stellar.rs
│   └── tests/
│       ├── mod.rs
│       ├── project_tests.rs
│       └── stellar_tests.rs
├── static/
│   ├── app.js
│   ├── index.html
│   └── style.css
├── target/
│   ├── .rustc_info.json
│   ├── CACHEDIR.TAG
│   ├── debug/
│   ├── release/
│   └── wasm32-unknown-unknown/


# Backend (Rust)

Proje Yönetimi: Project yapısı, projenin adı, sahibi, hedef miktarı, mevcut miktarı, fonlanma durumu ve destekçilerini yönetir.

Stellar Entegrasyonu: stellar.rs modülü, Stellar blok zinciri ile etkileşim kurmak için fonksiyonlar sağlar. Bu fonksiyonlar arasında işlem oluşturma ve hesap bilgilerini alma bulunur.

Web Sunucusu: Backend, HTTP isteklerini işlemek için Warp kullanır. Bu, main.rs dosyasında görülmektedir.
Frontend (HTML, CSS, JavaScript)

Proje Oluşturma: Kullanıcılar, index.html dosyasındaki bir form aracılığıyla yeni projeler oluşturabilir. Bu işlemin mantığı app.js dosyasında ele alınmıştır.

Fon Transferi: Kullanıcılar projelere fon transferi yapabilir. Bu işlemin mantığı da app.js dosyasında ele alınmıştır.
Bağımlılıklar
#Rust Bağımlılıkları

soroban-sdk: Soroban akıllı sözleşmeleri ile etkileşim kurmak için kullanılır.
warp: HTTP sunucusunu oluşturmak için kullanılan bir web framework'ü.
serde ve serde_json: JSON verilerini serileştirmek ve seriden çıkarmak için kullanılır.

## tokio:
 Eşzamanlı görevleri işlemek için kullanılan bir asenkron çalışma zamanı.
ureq ve reqwest: Stellar Horizon API'ye istek yapmak için kullanılan HTTP istemcileri.
getrandom: Rastgele sayı üretmek için kullanılır, JavaScript ortamlarını destekler.

## JavaScript Bağımlılıkları
soroban-client: Soroban akıllı sözleşmeleri ile etkileşim kurmak için kullanılan bir istemci kütüphanesi.
stellar-sdk: Stellar ağı ile etkileşim kurmak için kullanılan bir JavaScript kütüphanesi.
Kullanım
## Backend'i Çalıştırma
Backend sunucusunu çalıştırmak için aşağıdaki komutu kullanın:

## Frontend'i Çalıştırma
Frontend ile etkileşim kurmak için static/index.html dosyasını bir web tarayıcısında açın.

## Stellar ve Soroban Entegrasyonu
Stellar
Hesap Bilgileri: get_account_info fonksiyonu, Stellar hesabının bilgilerini alır.
İşlem Oluşturma: create_transaction fonksiyonu, Stellar ağı üzerinde bir işlem oluşturur ve gönderir.
Soroban
Akıllı Sözleşmeler: Soroban SDK'sı kullanılarak akıllı sözleşmeler oluşturulur ve yönetilir.
İstemci Kütüphanesi: Soroban istemci kütüphanesi, frontend tarafında akıllı sözleşmeler ile etkileşim kurmak için kullanılır.
Bu proje, Rust ve Stellar blok zinciri ile çalışan bir fon toplama uygulaması geliştirmek isteyenler için iyi bir örnek teşkil eder.