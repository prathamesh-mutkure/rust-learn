#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book {} by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie {} by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast ID - {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }

    fn _description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book {} by {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie {} by {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook {}", title)
        } else {
            format!("Unknown media")
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let book = Media::Book {
        title: String::from("Rich Dad Poor Dad"),
        author: String::from("Prathamesh"),
    };

    let movie = Media::Movie {
        title: String::from("3 Idiots"),
        director: String::from("Prathamesh"),
    };

    let audiobook = Media::Audiobook {
        title: String::from("Kindle"),
    };

    let podcast = Media::Podcast(8);
    let placeholder = Media::Placeholder;

    // println!("{}", book.description());
    // println!("{}", movie.description());
    // println!("{}", audiobook.description());

    // print_media(book);
    // print_media(movie);
    // print_media(audiobook);

    let mut catalog = Catalog::new();

    catalog.add(book);
    catalog.add(movie);
    catalog.add(audiobook);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(0);

    match item {
        Option::Some(value) => {
            println!("Item {:#?}", value);
        }
        None => {
            println!("No item found");
        }
    }

    // println!("{:#?}", item.unwrap());
    // println!("{:#?}", item.expect("It should contain a Media value"));
    // println!("{:#?}", item.unwrap_or(&Media::Placeholder));
}
