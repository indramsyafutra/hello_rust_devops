use std::env;
use local_ip_address::local_ip;

fn main() {
    // Menampilkan pesan awal
    println!("Hello, DevOps Engineer! 🚀");

    // Menampilkan direktori kerja saat ini
    match env::current_dir() {
        Ok(path) => println!("📂 Current directory: {}", path.display()),
        Err(e) => eprintln!("❌ Gagal membaca direktori: {}", e),
    }

    // Menampilkan IP lokal
    match local_ip() {
        Ok(ip) => println!("🌐 Local IP Address: {}", ip),
        Err(e) => eprintln!("❌ Gagal mendapatkan IP {}", e),
    }
}

