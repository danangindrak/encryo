use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit}; // Untuk AES-GCM
use aes_gcm::aead::{Aead}; // Untuk operasi enkripsi dan dekripsi
use base64::{engine::general_purpose, Engine}; // Untuk encoding Base64
use warp::Filter; // Untuk API
use serde::{Deserialize, Serialize}; // Untuk serialisasi/deserialisasi JSON
use rand::Rng; // Untuk nonce
use std::fmt;
use warp::reject::Reject;

// Struktur request dan response
#[derive(Deserialize)]
struct EncryptRequest {
    data: String,
}

#[derive(Serialize)]
struct EncryptResponse {
    encrypted: String,
}

#[derive(Deserialize)]
struct DecryptRequest {
    encrypted: String,
}

#[derive(Serialize)]
struct DecryptResponse {
    data: String,
}

// Struktur custom error
#[derive(Debug)]
struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Reject for CustomError {}

// Fungsi untuk mengenkripsi data
fn encrypt(data: &str, key: &[u8; 32]) -> Result<String, CustomError> {
    let key = Key::<Aes256Gcm>::from_slice(key); // Spesifikasikan tipe key
    let cipher = Aes256Gcm::new(key);

    let nonce: [u8; 12] = rand::thread_rng().gen(); // Nonce acak

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), data.as_bytes())
        .map_err(|_| CustomError {
            message: "Gagal mengenkripsi".into(),
        })?;

    let encrypted = general_purpose::STANDARD.encode([nonce.as_ref(), ciphertext.as_ref()].concat());
    Ok(encrypted)
}

fn decrypt(encrypted: &str, key: &[u8; 32]) -> Result<String, CustomError> {
    let key = Key::<Aes256Gcm>::from_slice(key); // Spesifikasikan tipe key
    let cipher = Aes256Gcm::new(key);

    let decoded = general_purpose::STANDARD
        .decode(encrypted)
        .map_err(|_| CustomError {
            message: "Data terenkripsi tidak valid".into(),
        })?;

    if decoded.len() < 12 {
        return Err(CustomError {
            message: "Nonce terlalu pendek".into(),
        });
    }

    let (nonce, ciphertext) = decoded.split_at(12);
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| CustomError {
            message: "Gagal mendekripsi".into(),
        })?;

    String::from_utf8(plaintext).map_err(|_| CustomError {
        message: "Hasil dekripsi tidak valid".into(),
    })
}


#[tokio::main]
async fn main() {
    // Kunci enkripsi (tetap rahasia dan aman)
    let key: [u8; 32] = rand::thread_rng().gen();

    // Endpoint untuk enkripsi
    let encrypt_route = warp::post()
        .and(warp::path("encrypt"))
        .and(warp::body::json())
        .and(warp::any().map(move || key))
        .and_then(|req: EncryptRequest, key| async move {
            match encrypt(&req.data, &key) {
                Ok(encrypted) => Ok(warp::reply::json(&EncryptResponse { encrypted })),
                Err(e) => Err(warp::reject::custom(e)),
            }
        });

    // Endpoint untuk dekripsi
    let decrypt_route = warp::post()
        .and(warp::path("decrypt"))
        .and(warp::body::json())
        .and(warp::any().map(move || key))
        .and_then(|req: DecryptRequest, key| async move {
            match decrypt(&req.encrypted, &key) {
                Ok(data) => Ok(warp::reply::json(&DecryptResponse { data })),
                Err(e) => Err(warp::reject::custom(e)),
            }
        });

    // Gabungkan semua route
    let routes = encrypt_route.or(decrypt_route);

    println!("Layanan Encryo berjalan di http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
