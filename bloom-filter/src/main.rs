use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct BloomFilter {
    storage: Vec<bool>,
    hash_count: usize
}

impl BloomFilter {
    // m -> storage
    // n -> number of hash functions
    fn new(m: usize, n: usize) -> Self {
        BloomFilter {
            storage: vec![false; m],
            hash_count: n
        }
    }

    fn hash_indices(&self, word: String) -> Vec<usize> {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        let hash = hasher.finish();

        (0..self.hash_count)
        .map(|i| ((hash as usize) >> (i * 8)) % self.storage.len())
        .collect()
    }

    fn insert(&mut self, word: String) {
      // calculate hash and set the hash!
      for i in self.hash_indices(word) {
        self.storage[i] = true;
      }
    }

    fn is_username_present(self, word: String) -> bool {
        self.hash_indices(word).iter().all(|i| self.storage[*i])
    }
}

#[get("/username")]
async fn user_exists(req: web::Query<HashMap<String, String>>) -> impl Responder {

    let mut bloom_filter = BloomFilter::new(100, 3);
    bloom_filter.insert("test".to_string());
    bloom_filter.insert("mystica".to_string());
    bloom_filter.insert("sarah".to_string());

    let username = match req.get("username") {
        Some(name) => name,
        None => "",
    };


    println!("{}",username);
    if bloom_filter.is_username_present(username.to_string()) {
        HttpResponse::Ok().body(format!("User '{}' exists ",username))
    } else {
        HttpResponse::NotFound().body(format!("User '{}' not exists", username))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .service(user_exists)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}


