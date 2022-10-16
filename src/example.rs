use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;

fn main() {
    let mut c = cursive::default();
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size::<(u32, u32)>((10, 5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete ", delete_name))
        .child(DummyView)
        .child(Button::new("quit", Cursive::quit));

    c.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("select a profile"),
    );

    c.run();
}

fn on_submit(c: &mut Cursive, name: &str) {
    c.add_layer(
        Dialog::text(format!("Name: {}\n Awesome: yes", name))
            .title(format!("{}'s info", name))
            .button("quit", |c| {
                c.pop_layer();
            }),
    );
}

fn add_name(c: &mut Cursive) {
    fn ok(c: &mut Cursive, name: &str) {
        c.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        c.pop_layer();
    }

    c.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("name")
                .fixed_width(11),
        )
        .title("enter a new name")
        .button("ok", |c| {
            let name = c
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(c, &name);
        })
        .button("cancel", |c| {
            c.pop_layer();
        }),
    );
}

fn delete_name(c: &mut Cursive) {
    let mut select = c.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => c.add_layer(Dialog::info("no name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

// TWO
// use cursive::views::Dialog;
// use cursive::Cursive;
// fn main() {
//     let mut c = cursive::default();
//     c.add_layer(
//         Dialog::text("press next when you're ready")
//             .title("important thing")
//             .button("next", show_next),
//     );

//     c.run();
// }

// fn show_next(c: &mut Cursive) {
//     c.pop_layer();
//     c.add_layer(
//         Dialog::text("did you do the thing")
//             .title("question 1")
//             .button("yes", |c| show_answer(c, "great job"))
//             .button("no", |c| show_answer(c, "you had one chance, ya blew it"))
//             .button("uh", |c| c.add_layer(Dialog::info("try again"))),
//     );
// }

// fn show_answer(c: &mut Cursive, msg: &str) {
//     c.pop_layer();
//     c.add_layer(
//         Dialog::text(msg)
//             .title("results")
//             .button("finish", |c| c.quit()),
//     );
// }

// ONE
// use cursive::views::TextView;
// fn main() {
//     let mut c = cursive::default();
//     c.add_global_callback('q', |c| c.quit());
//     c.add_layer(TextView::new("Hello cursive, Press <q> to quit!"));
//     c.run();
// }
