#![allow(dead_code)]
#[derive(Debug)]
struct Page {
    pub content: String,
    pub number: u32
}

impl Page {
    fn new(content: String, number: u32) -> Page {
        Page {
            content,
            number
        }
    }
}


#[derive(Debug)]
struct Chapter {
    pub title: String,
    pub pages: Vec<Page>,
}

impl Chapter {
    fn new(title: String, pages: Vec<Page>) -> Chapter {
        Chapter {
            title,
            pages
        }
    }
}


#[derive(Debug)]
struct Book {
    pub introduction: String,
    pub title: String,
    pub chapters: Vec<Chapter>,
}

impl Book {
    fn new(introduction: String, title: String, chapters: Vec<Chapter>) -> Book {
        Book {
            introduction,
            title,
            chapters
        }
    }
}



fn main() {
    let page1 = Page::new(
        String::from("here's content of the first page"),
        1
    );
    let page2 = Page::new(
        String::from("here's the content of a second page"),
        2
    );

    let mut pages: Vec<Page> = Vec::new();
    pages.push(page1);
    pages.push(page2);

    let chapter1 = Chapter::new(
        String::from("Chapter 1"),
        pages
    );

    let mut chapters: Vec<Chapter> = Vec::new();
    chapters.push(chapter1);

    let book = Book::new(
        String::from("Here's the introduction"),
        String::from("Book Title"),
        chapters
    );
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_page() {
        let page = Page::new(
            String::from("here's content of the first page"),
            1
        );

        assert_eq!(page.number, 1);
        assert_eq!(page.content, "here's content of the first page");
    }

    #[test]
    fn test_new_chapter() {
        let page = Page::new(
            String::from("here's content of the first page"),
            1
        );

        let chapter = Chapter::new(
            String::from("Chapter 1"),
            vec![page]
        );

        assert_eq!(chapter.title, "Chapter 1");
        assert_eq!(chapter.pages.len(), 1);
        assert_eq!(chapter.pages[0].number, 1);
    }

    #[test]
    fn test_fails() {
        assert!(false);
    }
}