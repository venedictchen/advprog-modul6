# Reflection
- [module 6](#module-6)
# Module 6

##  Commit 1 Reflection notes
- What is inside the `handle_connection` method?
    - Fungsi `handle_connection` memiliki parameter `stream` yang didapatkan dari ownership `TcpStream` dan bersifat mutable sehingga dapat dimodifikasi di dalam fungsi.
    - Lalu, `buf_reader` adalah variabel yang berisi hasil bacaan menggunakan `BufReader` dari buffer `stream` yang di pass.
    - Setelah itu, kita akan membuat variabel baru yaitu `http_request`. Variabel ini akan berisi vektor yang berisi request dari HTTP. 
    - Dari `buf_reader` kita akan membaca line yang ada dengan menggunakan `.line()`. Fungsi ini return berupa `Lines<BufReader<&mut TcpStream>>` yang dapat diiterasi.
    - Selanjutnya, kita akan iterasi menggunakan `.map()` dan jika terjadi error, `unwrap()` akan menyebabkan panic dan menghentikan program.
    - `.take_while(!line || !line.is_empty())` berfungsi untuk mengambil line-line yang ada sampai bertemu baris kosong dan berhenti.
    - Terakhir `.collect()` akan mengambil semua line yang dihasilkan dan menyimpan elemen dalam bentuk Vector. Selanjutnya, hasil akan di print.


## Commit 2 Reflection notes
- What does the additional lines of code in `handle_connection` do?
    - Kita menambahkan response untuk fungsi `handle_connection` dari yang awalnya print di terminal, sekarang akan memberikan response.
    - Response terdiri atas `status_line`, yaitu `"HTTP/1.1 200 OK"` yang menandakan proses berhasil.
    - Selanjutnya, kita akan membaca isi dari `hello.html` menjadi string dan menyimpannya di `contents`. Jika terjadi error, `unwrap()` akan menyebabkan panic dan menghentikan program.
    - `length` akan mendapatkan panjang dari content yang ada.
    - Lalu, response akan dibentuk dengan format yang sesuai dengan `status_line`, `Content-length`, `contents` yang sudah kita buat sebelumnya dalam bentuk HTTP response.
    - Terakhir, menggunakan `stream.write_all()`, kita akan mengembalikan ke TCP dengan konversi menjadi bentuk bytes. Jika terjadi error, `unwrap()` akan menyebabkan panic dan menghentikan program.

    ![](/public/images/commit2.png)  