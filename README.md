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

## Commit 3 Reflection notes
- Pada commit ini, saya menambahkan error page, yaitu `404.html`. 
```HTML
<!DOCTYPE html>
<html lang="en">
 <head>
 <meta charset="utf-8"> <title>Oops!</title>
 </head> 
 <body> 
    <h1>Oops!</h1> 
    <p>Sorry, I don't know what you're asking for.</p>
    <p>Hi from Rust, running from venedictchen's machine.</p>
 </body>
</html>
```
- Lalu, saya mengubah bagian `handle_connection` agar jika user memberikan page yang tidak dikenali, maka akan memunculkan `404.html`. 
```rust
fn handle_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request:Vec<_> = buf_reader
    .lines()
    .map(|result|result.unwrap())
    .take_while(|line|!line.is_empty()) 
    .collect();


    if http_request.is_empty(){
        return;
    }
    
    let request_line = http_request.get(0).unwrap();
    let response = generate_response(request_line);

    stream.write_all(response.as_bytes()).unwrap();
    
}

fn generate_response(request_line: &str) -> String {
    let get = "GET / HTTP/1.1"; 
    let (status_line, filename) = if request_line == get {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",
    )
}

```

- Di sini, saya juga melakukan refactor, menjadi fungsi `generate_response()`. Hal ini bertujuan agar setiap fungsi memiliki tugasnya masing-masing.

- Bagian `let request_line = http_request.get(0).unwrap();` akan membaca request yang diminta. Setelah itu akan dilakukan pengecekan dan memberikan `status_line` dan `filename` yang akan diberikan ke `contents` dan format String yang akan kita berikan.
- Setelah berhasil `generate_response()` maka `stream.write_all()` akan mengembalikan ke TCP dengan konversi menjadi bentuk bytes. Jika terjadi error, `unwrap()` akan menyebabkan panic dan menghentikan program.

![](/public/images/commit3.png)


## Commit 4 Reflection notes
- Saya melakukan refactor kembali pada bagian `handle_connection`
```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line.is_empty() {
        return;
    }

    let (status_line, filename) = generate_response(&request_line);

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",);
    stream.write_all(response.as_bytes()).unwrap();
}
```

- Lalu, saya mengganti fungsi `generate_response` dengan tambahan 
```rust
return match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
``` 

- Penambahan tersebut bertujuan untuk menambahkan durasi 10 detik untuk program dapat bekerja kembali. Hal ini tidak dapat menangani banyaknay request yang concurrent karena program single threaded.