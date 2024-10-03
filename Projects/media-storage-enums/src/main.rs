mod content;

use content::catalog::Catalog;
use content::media::Media;

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
    content::media::print_media(Media::Placeholder);

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
