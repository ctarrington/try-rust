struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Self {
       Library {
           books: vec![],
       }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for index in 0..self.books.len() {
            let book = self.books.get(index).unwrap();
            println!("{} {}", book.title, book.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        if self.is_empty() {
            return None;
        }

        let mut oldest= &self.books[0];
        for index in 0..self.books.len() {
            if self.books[index].year < oldest.year {
                oldest = &self.books[index];
            }
        }

        Some(oldest)
    }
}

#[test]
fn test_is_empty() {
    let mut library = Library::new();
    assert!(library.is_empty());
    library.add_book(Book::new("The end", 2000));
    assert!(!library.is_empty());
}

#[test]
fn test_len() {
    let mut library = Library::new();
    assert_eq!(library.len(), 0);
    library.add_book(Book::new("The end", 2000));
    assert_eq!(library.len(), 1);
}

#[test]
fn test_oldest_book() {
    let mut library = Library::new();
    match library.oldest_book() {
       Some(_book) => assert!(false),
        None => assert!(true),
    }

    library.add_book(Book::new("The end", 2000));
    library.add_book(Book::new("The beginning", 1999));
    library.add_book(Book::new("The other", 2010));

    match library.oldest_book() {
        Some(book) => assert_eq!(book.title, "The beginning"),
        None => assert!(false),
    }
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn main() {
    let mut library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
}