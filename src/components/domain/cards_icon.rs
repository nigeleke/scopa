use dioxus::prelude::*;

#[component]
pub fn CardsIcon(
    cids: String,
    disabled: bool,
    checked: bool,
) -> Element {
    let cids = cids.chars()
        .collect::<Vec<_>>()
        .chunks(2)          
        .map(|chunk| chunk.iter().collect::<String>())
        .enumerate()
        .collect::<Vec<_>>();

    let len = cids.len();

    rsx! {
        div {
            class: "cards-icon",
            class: if disabled { "disabled" },
            class: if checked { "checked" }, 
            for (i, cid) in cids {
                Card {
                    cid: cid,
                    n: i,
                    of: len,
                }
            }
        } 
    }
}

#[component]
pub fn Card(
    cid: String,
    n: usize,
    of: usize,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>
) -> Element {
    let cid = cid.as_str();
    let class = format!("card-{}-of-{}", n+1, of);

    rsx! {
        playing-card {
            class: class,
            cid: cid,
            suitcolor: colour(cid),
            rankcolor: colour(cid),
            ..attributes,
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