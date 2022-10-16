use cursive::traits::*;
use cursive::views::{Dialog, EditView, SelectView, TextView};
use cursive::Cursive;

#[derive(Clone)]
struct Bookmark {
    title: String,
    url: String,
    tags: Vec<String>,
}

fn main() {
    let mut c = cursive::default();

    let book_a = Bookmark {
        title: "mail".to_string(),
        url: "mail.gooogle.com".to_string(),
        tags: ["mail".to_string(), "productivity".to_string()].to_vec(),
    };

    let book_b = Bookmark {
        title: "elden ring wiki".to_string(),
        url: "wiki.eldenring.com".to_string(),
        tags: ["lore".to_string()].to_vec(),
    };

    let mut select = SelectView::<String>::new().on_submit(on_submit);

    let bookmarks = [book_a, book_b].to_vec();
    for bookmark in bookmarks.iter() {
        select.add_item_str(bookmark.title.clone());
    }
    let select = select.with_name("bookmarks");

    c.add_layer(
        Dialog::around(select)
            .title("dog-ear")
            .button("add", on_add),
    );
    c.add_global_callback('q', Cursive::quit);

    c.run();
}

fn on_submit(c: &mut Cursive, title: &str) {
    c.add_layer(
        Dialog::text(format!("title: {}", title)).button("back", |c| {
            c.pop_layer();
        }),
    );
}

fn on_add(c: &mut Cursive) {
    fn add(c: &mut Cursive, name: &str) {
        c.call_on_name("bookmarks", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        c.pop_layer();
    }

    c.add_layer(
        Dialog::around(EditView::new().on_submit(add).with_name("bookmark_add"))
            .title("add bookmark")
            .button("add", |c| {
                let title = c
                    .call_on_name("bookmark_add", |view: &mut EditView| view.get_content())
                    .unwrap();
                add(c, &title);
            })
            .button("cancel", |c: &mut Cursive| {
                c.pop_layer();
            }),
    );
}
