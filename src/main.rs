use std::ops::AddAssign;
use sycamore::prelude::*;

#[component(inline_props)]
fn Item<G: Html>(count: Signal<i32>, children: Children<G>) -> View<G> {
  count.update(|c| c.add_assign(1));

  let inner = children.call();
  view! {
    li {
      (inner)
    }
  }
}

#[derive(Props)]
struct LinkProps {
  #[prop(default)]
  class: &'static str,
  text: &'static str,
  href: &'static str,
}

#[component]
fn Link<G: Html>(props: LinkProps) -> View<G> {
  view! {
      a(class=props.class, target="_blank", href=props.href) {
        (props.text)
      }
  }
}

fn main() {
  sycamore::render(|| {
    let indie_count = create_signal(0);
    let links_count = create_signal(0);

    let show_switch = create_signal(false);
    let show_discord = create_signal(false);

    view! {
      div {
        div(class="iam") {
          h1 {
            "LOU "
            small { "'aquarhead'" }
            " Xun"
          }
          h3 {
            "a "
            Link(class="hidden", text="Rusty", href="https://www.rust-lang.org/")
            " programmer" }
        }

        div(class="indie") {
          h2 { "Indie" }
          code {
            "&'static "
            Link(class="hidden", text="ElaWorkshop", href="https://github.com/ElaWorkshop")
          }

          ol {
            Item(count = indie_count) {
              Link(text="Expense", href="https://ela.build/expense")
            }
            Item(count = indie_count) {
              Link(text="HaloSir", href="https://github.com/HaloWordApp/halosir")
            }
            Item(count = indie_count) {
              Link(text="One Clock", href="https://ela.build/oneclock")
            }
          }
        }

        div(class="links") {
          h2 { "Links" }
          ol(start=(indie_count.get() + 1)) {
            Item(count = links_count) {
              Link(text="GitHub", href="https://github.com/aquarhead")
            }
            Item(count = links_count) {
              Link(text="Blog", href="https://blog.aqd.is/")
            }
            Item(count = links_count) {
              Link(text="Strava", href="https://www.strava.com/athletes/aquarhead")
            }
            Item(count = links_count) {
              Link(text="Speaker Deck", href="https://speakerdeck.com/aquarhead")
            }
            Item(count = links_count) {
              Link(text="Bangumi", href="https://bgm.tv/user/aquarhead")
            }
            Item(count = links_count) {
              Link(text="Goodreads", href="https://www.goodreads.com/aquarhead")
            }
            Item(count = links_count) {
              Link(text="Letterboxd", href="https://letterboxd.com/aquarhead/")
            }
            Item(count = links_count) {
              Link(text="Steam", href="https://steamcommunity.com/id/aquarhead")
            }
            Item(count = links_count) {
              (if show_switch.get() {
                view! { span { "SW-2985-1992-7098" } }
              } else {
                view! { a(on:click= move |_| { show_switch.set(true) }) { "Nintendo" } }
              })
            }
            Item(count = links_count) {
              (if show_discord.get() {
                view! { span { "aquarhead" } }
              } else {
                view! { a(on:click= move |_| { show_discord.set(true) }) { "Discord" } }
              })
            }
            Item(count = links_count) {
              Link(text="Telegram", href="https://t.me/aquarhead")
            }
            Item(count = links_count) {
              Link(text="Signal", href="https://signal.me/#eu/4J4a2A28b5VzF2sIqLeIARupgyFGFQq_6I7_HDeTGGLhen3IWv9CiqH1bY_Y2Zxe")
            }
            Item(count = links_count) {
              Link(text="PGP Public Key", href="publickey.txt")
            }
          }
        }

        div(class="heart") {
          h2 { "Ást" }
          code { "Ég elska @zinnialulu" }
        }
      }
    }
  });
}
