import gleam/option.{type Option}
import lustre/attribute.{type Attribute} as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event as e

const main_menu_id = "main-menu"

pub fn view(
  home_page: Bool,
  on_set_language: fn(String) -> msg,
  on_fullscreen: msg,
  on_reset: msg,
) {
  h.div([a.class("main-menu")], [
    hamburger_icon(),
    h.div(
      [
        a.class("main-menu__items"),
        a.popover("auto"),
        a.id(main_menu_id),
      ],
      [
        set_english_menu_item(on_set_language),
        set_italian_menu_item(on_set_language),
        fullscreen_menu_item(on_fullscreen),
        reset_menu_item(home_page, on_reset),
      ],
    ),
  ])
}

const hamburger = "\u{2630}"

fn hamburger_icon() -> Element(msg) {
  button(
    hamburger,
    [a.class("main-menu__root"), a.popovertarget(main_menu_id)],
    option.None,
  )
}

const uk_flag = "\u{1F1EC}\u{1F1E7}"

fn set_english_menu_item(on_set_language: fn(String) -> msg) -> Element(msg) {
  button(uk_flag, [a.title("English")], option.Some(on_set_language("en")))
}

const it_flag = "\u{1F1EE}\u{1F1F9}"

fn set_italian_menu_item(on_set_language: fn(String) -> msg) -> Element(msg) {
  button(it_flag, [a.title("Italian")], option.Some(on_set_language("it")))
}

const fullscreen = "\u{26F6}"

fn fullscreen_menu_item(on_fullscreen: msg) -> Element(msg) {
  button(
    fullscreen,
    [a.title("Fullscreen"), a.class("emphasis")],
    option.Some(on_fullscreen),
  )
}

const power_on = "\u{23FC}"

fn reset_menu_item(home_page: Bool, on_reset: msg) -> Element(msg) {
  case home_page {
    True -> element.none()

    False ->
      element.fragment([
        h.hr([a.class("main-menu__items__spacer")]),
        button(
          power_on,
          [a.title("Reset"), a.class("emphasis")],
          option.Some(on_reset),
        ),
      ])
  }
}

fn button(
  text: String,
  attributes: List(Attribute(msg)),
  on_click: Option(msg),
) -> Element(msg) {
  let on_click =
    echo case on_click {
      option.Some(msg) -> e.on_click(msg)
      option.None -> a.none()
    }

  h.button([a.class("main-menu__button"), on_click, ..attributes], [
    h.text(text),
  ])
}
