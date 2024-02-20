// Enum tanımı
enum Publication {
    Book(BookInfo),
    Magazine(MagazineInfo),
}

// Kitap bilgileri
struct BookInfo {
    title: String,
    author: String,
    page_count: u32,
}

// Dergi bilgileri
struct MagazineInfo {
    title: String,
    issue: u32,
    topic: String,
}

// Yayınları yazdıran fonksiyon
fn print_publication(publication: &Publication) {
    match publication {
        Publication::Book(book_info) => {
            println!("Kitap: {} - Yazar: {}, Sayfa Sayısı: {}", book_info.title, book_info.author, book_info.page_count);
        }
        Publication::Magazine(magazine_info) => {
            println!("Dergi: {} - Sayı: {}, Konu: {}", magazine_info.title, magazine_info.issue, magazine_info.topic);
        }
    }
}

// main func
fn main() {
    // Kitap örneği
    let book_example = Publication::Book(BookInfo {
        title: String::from("Rust Programlama Dili"),
        author: String::from("Steve Klabnik, Carol Nichols"),
        page_count: 588,
    });

// Dergi örneği
let magazine_example = Publication::Magazine(MagazineInfo {
title: String::from("National Geographic"),
issue: 2024,
topic: String::from("Yaban Hayatta Yasam"),
});

// Vec dizisi oluşturma
let publications: Vec<Publication> = vec![book_example, magazine_example];

// Vec içeriğini yazdırma
for publication in &publications {
    print_publication(publication);
}
}