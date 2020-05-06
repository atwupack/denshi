use denshi::App;
use denshi::component::button::{Button, Checkbox};
use denshi::component::text::TextField;
use denshi::component::layout::{Form, Page};
use denshi::component::menu::MenuBar;

struct DemoState {
    text: String,
}

fn main() {

    let state = DemoState {
        text: "".to_string(),
    };

    let mut form = Form::new();

    let mut menu = MenuBar::new();
    menu.add_entry("File".to_string());
    menu.add_entry("Edit".to_string());
    menu.add_entry("Help".to_string());

    let mut button = Button::new("Test Button".to_owned());
    button.set_click_event(|| {
       dbg!("Clicked");
    });
    let text = TextField::new("Enter Name: ".to_owned());

    let checkbox = Checkbox::new("Checkbox".to_owned());

    //form.add_line(menu);
    form.add_line(text);
    form.add_line(button);
    form.add_line(checkbox);

    let mut page = Page::new();
    page.add_component(menu);
    page.add_component(form);

    let app = App::new("Demo".to_owned(), page);
    app.run();
}
