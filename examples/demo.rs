use denshi::App;
use denshi::component::button::Button;
use denshi::component::text::TextField;
use denshi::component::layout::Form;

struct DemoState {
    text: String,
}

fn main() {

    let state = DemoState {
        text: "".to_string(),
    };

    let mut form = Form::new();

    let mut button = Button::new("Test Button".to_owned());
    button.set_click_event(|| {
       dbg!("Clicked");
    });
    let text = TextField::new("Enter Name: ".to_owned());

    form.add_line(text);
    form.add_line(button);

    let app = App::new("Demo".to_owned(), form);
    app.run();
}