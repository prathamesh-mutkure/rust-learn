/* Returns last language */
/* No need of type annotation when function takes one ref and returns one */
/* Rust assumes we are returning the singular input ref */
fn last_language(languages: &[String]) -> &str {
    return languages.last().unwrap();
}

/* Return next language in sequence, defaults to last */
/* In case function takes multiple ref as input and returns one */
/* We need to specify the type annotations to tell rust which ref is returned */
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true
        }
    }

    return languages.last().unwrap();
}

/* Returns longer of two */
/* In case of multiple ref as input and single ref being returned */
/* And the ref to return is dynamic */
/* We can as assign same defination to multiple args */
fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() > lang_b.len() {
        return lang_a;
    } else {
        return lang_b;
    }
}

/* EXCEPTIONS */
/* 1. No return ref */
/* 2. Single ref, single return ref */
/* 3. &self ref, rust assumes we will return ref to self and no need of annotations */
/* 4. To return ref other than &self, we need annotations */

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    println!("Last language: {}", last_language(&languages));
    println!("Next of go: {}", next_language(&languages, "rust"));
    println!(
        "Longest language: {}",
        longest_language(&languages.get(1).unwrap(), &languages.get(2).unwrap())
    );
}
