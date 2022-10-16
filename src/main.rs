use std::fs;

use cursive::Cursive;
use serde::{Deserialize, Serialize};

use cursive::traits::*;
use cursive::views::{Dialog, EditView, SelectView};

#[derive(Clone)]
struct Bookmark {
    title: String,
    url: String,
}

fn main() {
    let items = read_bookmarks();
    let bookmarks = items.iter().map(|i| {
        let title = i.title.clone();
        let url = i.href.clone().unwrap_or("unknown".to_string());
        let b = Bookmark {
            title: if title.is_empty() { url.clone() } else { title },
            url: url,
        };
        b
    });

    let mut c = cursive::default();
    let mut select = SelectView::<Bookmark>::new().on_submit(on_submit);

    for bookmark in bookmarks {
        let b = bookmark.clone();
        select.add_item(b.title.clone(), b);
    }
    let select = select.with_name("bookmarks").full_screen().scrollable();

    c.add_layer(
        Dialog::around(select)
            .title("dog-ear")
            .button("add", on_add),
    );
    c.add_global_callback('q', Cursive::quit);

    c.run();
}

fn on_submit(c: &mut Cursive, bookmark: &Bookmark) {
    let bk = bookmark.clone();
    c.add_layer(
        Dialog::text(format!(
            "title: {}\n url: {}\n",
            bookmark.title, bookmark.url
        ))
        .button("open", move |c| {
            open::that(bk.url.clone()).unwrap();
            c.pop_layer();
        })
        .button("back", |c| {
            c.pop_layer();
        }),
    );
}

fn on_add(c: &mut Cursive) {
    fn add(c: &mut Cursive, title: &str) {
        c.call_on_name("bookmarks", |view: &mut SelectView<Bookmark>| {
            let bookmark = Bookmark {
                title: title.to_string(),
                url: "".to_string(),
            };
            view.add_item(title, bookmark);
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub folders: Vec<Item>,
}
// }
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Folder {
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub items: Vec<Item>,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub add_date: Option<String>,
    pub last_modified: Option<String>,
    #[serde(default)]
    pub items: Vec<Item>,
    pub href: Option<String>,
    pub icon: Option<String>,
}

fn read_bookmarks() -> Vec<Item> {
    let json = fs::read_to_string("./bookmarks.json").unwrap();
    let root: Root = serde_json::from_str(&json).unwrap();
    // println!("{}", root.folders[0].type_field);
    let items = extract_items(&root.folders[0]);
    // let count = items.clone().count();
    let count = items.len();
    println!("count: {}", count);
    items
}

fn extract_items(item: &Item) -> Vec<Item> {
    let mut items = Vec::<Item>::new();
    let mut count = 0;
    for x in item.items.iter() {
        match x.type_field.as_str() {
            "link" => {
                // println!("adding {}", x.title);
                count += 1;
                items.push(x.clone());
            }
            "folder" => {
                // println!("extracting {}", x.title);
                let folder_items = extract_items(x);
                items.append(&mut folder_items.clone());
            }
            _ => {
                println!("{}", x.type_field);
            }
        }
    }
    items
}
