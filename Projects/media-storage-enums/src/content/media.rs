#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
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

    pub fn _description(&self) -> String {
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

pub fn print_media(media: Media) {
    println!("{:#?}", media)
}
