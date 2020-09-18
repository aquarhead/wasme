use seed::{prelude::*, *};
use std::cell::Cell;

struct Model {
  show_friend_code: bool,
}

impl Default for Model {
  fn default() -> Self {
    Self {
      show_friend_code: false,
    }
  }
}

#[derive(Clone)]
enum Msg {
  ShowFriendCode,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
  match msg {
    Msg::ShowFriendCode => model.show_friend_code = true,
  }
}

fn hidden(text: &str, href: &str) -> Node<Msg> {
  a![class!["hidden"], attrs![At::Href => href, At::Target => "_blank"], text]
}

fn codelink(text: &str, href: &str) -> Node<Msg> {
  a![
    class!["codelink"],
    attrs![At::Href => href, At::Target => "_blank"],
    text
  ]
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
  let li_count = Cell::new(0);

  let link = |text: &str, href: &str| {
    li_count.replace(li_count.get() + 1);
    li![a![attrs![At::Href => href, At::Target => "_blank"], text]]
  };

  let dead = |text: &str| {
    li_count.replace(li_count.get() + 1);
    li![span![class!["dead"], text]]
  };

  let switch = || {
    li_count.replace(li_count.get() + 1);
    if model.show_friend_code {
      li![span!["SW-2985-1992-7098"]]
    } else {
      li![a!["Nintendo Friend Code"], simple_ev(Ev::Click, Msg::ShowFriendCode)]
    }
  };

  div![
    div![
      class!["iam"],
      h1!["LOU ", small!["'aquarhead'"], " Xun",],
      h3!["a Rusty programmer"],
    ],
    div![
      class!["daily"],
      h2!["Daily"],
      code![
        "fun (",
        codelink("CCP_Games", "https://www.ccpgames.com/"),
        ", SE) -> ",
        hidden("true", "https://www.erlang.org/"),
        ".",
      ],
      ol![
        attrs![At::Start => li_count.get() + 1],
        link("EVE Online", "https://www.eveonline.com/"),
        dead("DUST 514"),
        dead("Gunjack"),
        link("ESI", "https://esi.evetech.net/"),
        link(
          "EVE Chat",
          "https://www.eveonline.com/article/pms1i4/chat-system-an-update-and-the-future"
        ),
      ]
    ],
    div![
      class!["indie"],
      h2!["Indie"],
      code![
        "(",
        hidden("&mut", "https://www.rust-lang.org/"),
        " Wizard) ",
        hidden("|>", "https://elixir-lang.org/"),
        " ",
        codelink("ElaWorkshop", "https://github.com/ElaWorkshop"),
      ],
      ol![
        attrs![At::Start => li_count.get() + 1],
        link("Expanse", "https://ela.build/expense"),
        link("HaloSir", "https://github.com/HaloWordApp/halosir"),
        link("One Clock", "https://ela.build/oneclock"),
        link("Shoka", "https://github.com/TeaWhen/Shoka"),
        link("FuncDiff", "https://github.com/spawnfest/func_diff"),
      ]
    ],
    div![
      class!["links"],
      h2!["Links"],
      ol![
        attrs![At::Start => li_count.get() + 1],
        link("GitHub", "https://github.com/aquarhead"),
        link("Twitter", "https://twitter.com/aquarhead"),
        link("Blog", "https://blog.aquarhead.me/"),
        link("Speaker Deck", "https://speakerdeck.com/aquarhead"),
        link("Bangumi", "https://bgm.tv/user/aquarhead"),
        link("Goodreads", "https://www.goodreads.com/aquarhead"),
        link("Letterboxd", "https://letterboxd.com/aquarhead/"),
        link("Bandcamp", "https://bandcamp.com/aquarhead"),
        link("Flickr", "https://www.flickr.com/photos/aquarhead"),
        link("Instagram", "https://www.instagram.com/aquarhead/"),
        link("Steam", "https://steamcommunity.com/id/aquarhead"),
        switch(),
        link("Telegram", "https://t.me/aquarhead"),
        link("PGP Public Key", "assets/publickey.txt"),
        link("Currently Inked", "https://airtable.com/shrpMEu09HJ8o3Bkl"),
        link("CV", "https://stackoverflow.com/cv/aquarhead"),
      ]
    ],
    div![class!["heart"], h2!["Ást"], code!["Ég elska @zinnialulu"]],
  ]
}

#[wasm_bindgen(start)]
pub fn render() {
  App::builder(update, view).build_and_start();
}
