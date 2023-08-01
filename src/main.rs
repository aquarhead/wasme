use std::ops::AddAssign;
use sycamore::prelude::*;

#[component(inline_props)]
fn Item<'a, G: Html>(cx: Scope<'a>, count: &'a Signal<i32>, children: Children<'a, G>) -> View<G> {
  count.modify().add_assign(1);

  let inner = children.call(cx);
  view! {cx,
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
fn Link<'a, G: Html>(cx: Scope<'a>, props: LinkProps) -> View<G> {
  view! {cx,
      a(class=props.class, target="_blank", href=props.href) {
        (props.text)
      }
  }
}

fn main() {
  sycamore::render(|cx| {
    let daily_count = create_signal(cx, 0);
    let indie_count = create_signal(cx, 0);
    let links_count = create_signal(cx, 0);

    let show_switch = create_signal(cx, false);
    let show_discord = create_signal(cx, false);

    view! { cx,
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

        div(class="daily") {
          h2 { "Daily" }
          code {
            "&mut "
            Link(class="hidden", text="Controlant", href="https://controlant.com/")
          }

          ol {
            Item(count = daily_count) {
              Link(text="ctrl-cidr", href="https://github.com/controlant-org/ctrl-cidr")
            }
            Item(count = daily_count) {
              Link(text="r5d3", href="https://github.com/controlant-org/r5d3")
            }
            Item(count = daily_count) {
              Link(text="aws-auth-operator", href="https://github.com/controlant-org/aws-auth-operator")
            }
          }
        }

        div(class="indie") {
          h2 { "Indie" }
          code {
            "&'static "
            Link(class="hidden", text="ElaWorkshop", href="https://github.com/ElaWorkshop")
          }

          ol(start=(*daily_count.get() + 1)) {
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
          ol(start=(*daily_count.get() + *indie_count.get() + 1)) {
            Item(count = links_count) {
              Link(text="GitHub", href="https://github.com/aquarhead")
            }
            Item(count = links_count) {
              Link(text="Blog", href="https://blog.aqd.is/")
            }
            Item(count = links_count) {
              Link(text="Strava", href="https://www.strava.com/athletes/108132560")
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
              (if *show_switch.get() {
                view! { cx, span { "SW-2985-1992-7098" } }
              } else {
                view! { cx, a(on:click=|_| { show_switch.set(true) }) { "Nintendo" } }
              })
            }
            Item(count = links_count) {
              (if *show_discord.get() {
                view! { cx, span { "aquarhead" } }
              } else {
                view! { cx, a(on:click=|_| { show_discord.set(true) }) { "Discord" } }
              })
            }
            Item(count = links_count) {
              Link(text="Telegram", href="https://t.me/aquarhead")
            }
            Item(count = links_count) {
              Link(text="PGP Public Key", href="assets/publickey.txt")
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
