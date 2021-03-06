use seed::{prelude::*, *};
use std::cell::Cell;

#[derive(Default)]
struct Model {
  show_friend_code: bool,
  show_discord_tag: bool,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
  Model::default()
}

#[derive(Clone)]
enum Msg {
  ShowFriendCode,
  ShowDiscordTag,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
  match msg {
    Msg::ShowFriendCode => model.show_friend_code = true,
    Msg::ShowDiscordTag => model.show_discord_tag = true,
  }
}

fn hidden(text: &str, href: &str) -> Node<Msg> {
  a![C!["hidden"], attrs![At::Href => href, At::Target => "_blank"], text]
}

fn codelink(text: &str, href: &str) -> Node<Msg> {
  a![C!["codelink"], attrs![At::Href => href, At::Target => "_blank"], text]
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
  let li_count = Cell::new(0);

  let link = |text: &str, href: &str| {
    li_count.replace(li_count.get() + 1);
    li![a![attrs![At::Href => href, At::Target => "_blank"], text]]
  };

  let dead = |text: &str| {
    li_count.replace(li_count.get() + 1);
    li![span![C!["dead"], text]]
  };

  let switch = || {
    li_count.replace(li_count.get() + 1);
    if model.show_friend_code {
      li![span!["SW-2985-1992-7098"]]
    } else {
      li![a!["Nintendo Friend Code"], ev(Ev::Click, |_| Msg::ShowFriendCode)]
    }
  };

  let discord = || {
    li_count.replace(li_count.get() + 1);
    if model.show_discord_tag {
      li![span!["aquarhead#3502"]]
    } else {
      li![a!["Discord Tag"], ev(Ev::Click, |_| Msg::ShowDiscordTag)]
    }
  };

  div![
    div![
      C!["iam"],
      h1!["LOU ", small!["'aquarhead'"], " Xun",],
      h3!["a Rusty programmer"],
    ],
    div![
      C!["daily"],
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
        link("ESI", "https://esi.evetech.net/"),
        link(
          "EVE Chat",
          "https://www.eveonline.com/article/pms1i4/chat-system-an-update-and-the-future"
        ),
      ]
    ],
    div![
      C!["indie"],
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
        link("Expense", "https://ela.build/expense"),
        link("HaloSir", "https://github.com/HaloWordApp/halosir"),
        link("One Clock", "https://ela.build/oneclock"),
        link("Shoka", "https://github.com/TeaWhen/Shoka"),
        link("FuncDiff", "https://github.com/spawnfest/func_diff"),
      ]
    ],
    div![
      C!["links"],
      h2!["Links"],
      ol![
        attrs![At::Start => li_count.get() + 1],
        link("GitHub", "https://github.com/aquarhead"),
        link("Twitter", "https://twitter.com/aquarhead"),
        link("Blog", "https://blog.aqd.is/"),
        link("Speaker Deck", "https://speakerdeck.com/aquarhead"),
        link("Bangumi", "https://bgm.tv/user/aquarhead"),
        link("Goodreads", "https://www.goodreads.com/aquarhead"),
        link("Letterboxd", "https://letterboxd.com/aquarhead/"),
        link("Bandcamp", "https://bandcamp.com/aquarhead"),
        link("Flickr", "https://www.flickr.com/photos/aquarhead"),
        link("Instagram", "https://www.instagram.com/aquarhead/"),
        link("Steam", "https://steamcommunity.com/id/aquarhead"),
        switch(),
        discord(),
        link("Telegram", "https://t.me/aquarhead"),
        link("PGP Public Key", "assets/publickey.txt"),
        link("CV", "https://stackoverflow.com/cv/aquarhead"),
      ]
    ],
    div![C!["heart"], h2!["Ást"], code!["Ég elska @zinnialulu"]],
  ]
}

#[wasm_bindgen(start)]
pub fn render() {
  App::start("app", init, update, view);
}
