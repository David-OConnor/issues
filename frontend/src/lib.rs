use seed::{*, prelude::*};


#[derive(Clone, Copy, Debug)]
enum Page {
    Home,
    FAQ,
    About,
    Business,
}

struct Model {
    pub search_text: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            search_text: String::new(),
        }
    }
}


#[derive(Clone)]
enum Msg {
    ChgSearch(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChgSearch(text) => model.search_text = text,
    }
}

fn menu() -> Node<Msg> {
    div![]
}

/// A search bar to find a business by name, or city
fn find(text: &str) -> Node<Msg> {
    div![
        input![
            style!{St::Width => px(400)},
            attrs!{At::Value => text},
            input_ev(Ev::Input, Msg::ChgSearch),
        ],
    ]
}


fn view(model: &Model) -> impl View<Msg> {
    div![ style!{St::Display => "flex", St::FlexDirection => "column"},
        menu(),

        h1!["Issues"],  // todo: Name is a placeholder
        h2!["Where customers and businesses share ideas"],

        find(&model.search_text),

        h2!["About"],
        h4!["Bringing business and customers together."],
        p!["How it works: If you've visited a local business, and have constructive\
        feedback, find the business's page using the search bar above. Create an issue\
        that identifies what you think could be improved, and a suggestion for how it\
        that could be accomplished. Example: \"The coffee's great, but it would be nice\
        if there were a water dispensor with glasses in the shop. That way we wouldn't\
        have to ask the barista for water directly. Adding a dispensor would save the baristas\
        time, and make it easier for customers to get water.\""],

        h3!["Reviews and Issues"],
        p!["Reviews are great places to share info among customers about experiences\
        at restaurants, bars, clothing stores etc, but aren't always an effective way to share\
        feedback with these businesses about how they can improve. At __, our goal\
         is to improve businesses by opening a channel of constructive feedback between\
         businesses and customers."]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}