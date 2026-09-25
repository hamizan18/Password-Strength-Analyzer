# Password-Strength-Analyzer

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Status](https://img.shields.io/badge/Status-Active-brightgreen.svg)](#)

Aplikasi sederhana namun memiliki manfaat yang powerful untuk dapat menguji password yang ingin kita isikan apakah powerful atau tidak nya. Dan agar mengetahui vulnerability nya

### Problem

> _"Bukannya yang penting tu kita ingat password nya aja yak?"_

Banyak orang diluar sana tidak kepikiran "apakah Password memang sepenting itu?" "Bukannya yang penting kita bisa ingat aja yaa?". Sebenarnya itu juga sebelumnya termasuk diriku sendiri, namun setelah melewati beberapa semester saya dalam mempelajari mengenai teknologi seperti Cybersecurity, dan bahkan pada ketika pertama kali pembuatan database yang sering kudengar kalo database itu isinya banyak hal-hal krusial yang sangat vital jika diketahui oleh orang lain.

Ketika saya masih memiliki pikiran yang sama seperti yang sebelumnya saya sebutkan diatas, saat itulah terjadi masalah di negara ini seperti data yang dicuri oleh seorang hacker karena memiliki password yang sangat simpel seperti "Admin123", i-itu kaya anak tk yang baru aja masuk ke dunia pemrograman pls..

Nah mendengar hal itu saya kepikiran untuk membuat sebuah sistem yang dapat melakukan pengecekan dalam pembuatan password untuk suatu akun.

---

### Goal

- Membantu user dalam pembuatan Password yang sulit ditebak atau diretas
- Membantu dan menyarankan user dalam memberikan saran untuk pembuatan suatu Password untuk akun mereka.

---

### Input

User memasukkan data Password yang ingin dimasukkan

---

### Output

- Jika user tidak memasukkan password sesuai dengan yang disarankan maka akan menampilkan suggest dalam mengisi password
- Jika user memasukkan password yang sesuai dengan yang sudah disarankan, maka akan tampil valid.

---

### Rules
Password akan di evaluasi berdasarkan kriteria berikut:
- [x] **Panjang Minimal**: Minimal 8-12 karakter.
- [x] **Huruf Kapital & Kecil**: Mengandung kombinasi huruf besar (`A-Z`) dan kecil.
- [x] **Angka**: Mengandung setidaknya satu buah angka (`0-9`).
- [x] **Karakter Spesial**: Mengandung simbol khusus (misal: `@`, `#`, `$`, `%`, `!`)

---

### ⚠️ Edge Case
Beberapa skenario yang dapat ditangani oleh sistem:
* **String Kosong / Whitespaces**: Mencegah verifikasi jika input hanya berisi spasi.
* **Common Passwords**: Memeriksa apakah password yang diinput tersebut apakah ada pada salah satu password populer dan mudah diretas (misal: `password`, `123456789`, `qwertyuiop`).
* **Karakter Berulang**: Mengidentifikasi pola berulang seperti `aaaaaa` atau `111111`.

---

### Tech Stack
- Rust (Programming Language)

---

### Version release
- Version 1

---

### Dokumentasi

---

## Implementasi dan Cara Penggunaan

### Prerequisites

- Rust 

### Installation & Run

1. **Clone Repo ini**
   ```bash
   git clone https://github.com/hamizan18/Password-Strength-Analyzer.git
   ```
2. Masuk ke dalam folder:
   ```bash
   cd Password-Strength-Analyzer
   ```
3.
