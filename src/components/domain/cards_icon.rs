use dioxus::prelude::*;

#[component]
pub fn CardsIcon(cids: String) -> Element {
    let cids = cids.chars()
        .collect::<Vec<_>>()
        .chunks(2)          
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();
    rsx! { 
        for cid in cids {
            Card { cid: cid }
        }
    }
}

#[component]
pub fn Card(
    cid: String,
    #[props(extends = img)]
    attributes: Vec<Attribute>
) -> Element {
    let cid = cid.as_str();
    rsx! {
        span {
            class: "cards-icon",
            playing-card {
                cid: cid,
                suitcolor: {colour(cid)},
                rankcolor: {colour(cid)},
                ..attributes,
            }
        }
    }
}
fn colour(cid: &str) -> String {
    let Some(c) = cid.chars().nth(1) else { panic!("Invalid cards cid.") };   
    match c {
        'h' | 'H' => "red",
        'c' | 'C' => "green",
        'd' | 'D' => "darkorange",
        's' | 'S' => "dodgerblue",
        _ => panic!("Invalid cards cid."),
    }.into()
}