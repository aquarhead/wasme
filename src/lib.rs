use seed::{prelude::*, *};

struct Model {}

impl Default for Model {
  fn default() -> Self {
    Self {}
  }
}

#[derive(Clone)]
enum Msg {}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

fn link(text: &str, href: &str) -> Node<Msg> {
  li![a![attrs![At::Href => href, At::Target => "_blank"], text]]
}

fn view(_: &Model) -> impl View<Msg> {
  div![
    div![
      class!["iam"],
      h1!["Lou ", small!["'aquarhead'"], " Xun",],
      h3!["a Rusty developer"],
    ],
    div![
      class!["daily"],
      h2!["Daily"],
      code!["fun (CCP_Games, SE) -> true."],
      ol![
        attrs![At::Start => "1"],
        link("EVE Online", "https://www.eveonline.com/"),
        li!["DUST 514"],
        li!["Gunjack"],
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
      code!["(&mut Wizard) |> ElaWorkshop"],
      ol![
        attrs![At::Start => "6"],
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
        attrs![At::Start => "11"],
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
        link("Keybase", "https://keybase.io/aquarhead"),
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
