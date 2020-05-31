#![cfg(feature = "serialize")]

extern crate quick_xml;
extern crate serde;

use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Link {
    rel: String,
    href: String,
    sizes: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Fr,
    De,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Head {
    title: String,
    #[serde(rename = "link", default)]
    links: Vec<Link>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Script {
    src: String,
    integrity: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Body {
    #[serde(rename = "script", default)]
    scripts: Vec<Script>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Html {
    lang: Option<String>,
    head: Head,
    body: Body,
}

#[test]
fn crates_io() {
    let xml = r#"<!DOCTYPE html>
        <html lang="en">
          <head>
            <title>crates.io: Rust Package Registry</title>

            <link rel="manifest" href="/manifest.webmanifest"/>
            <link rel="apple-touch-icon" href="/cargo-835dd6a18132048a52ac569f2615b59d.png" sizes="227x227"/>
          </head>
          <body>
            <noscript>
                <div id="main">
                    <div class='noscript'>
                        This site requires JavaScript to be enabled.
                    </div>
                </div>
            </noscript>

            <script src="/assets/vendor-bfe89101b20262535de5a5ccdc276965.js" integrity="sha256-U12Xuwhz1bhJXWyFW/hRr+Wa8B6FFDheTowik5VLkbw= sha512-J/cUUuUN55TrdG8P6Zk3/slI0nTgzYb8pOQlrXfaLgzr9aEumr9D1EzmFyLy1nrhaDGpRN1T8EQrU21Jl81pJQ==" ></script>
            <script src="/assets/cargo-4023b68501b7b3e17b2bb31f50f5eeea.js" integrity="sha256-9atimKc1KC6HMJF/B07lP3Cjtgr2tmET8Vau0Re5mVI= sha512-XJyBDQU4wtA1aPyPXaFzTE5Wh/mYJwkKHqZ/Fn4p/ezgdKzSCFu6FYn81raBCnCBNsihfhrkb88uF6H5VraHMA==" ></script>

          </body>
        </html>
}"#;
    let html: Html = from_str(xml).unwrap();
    dbg!(&html);
    assert_eq!(&html.head.title, "crates.io: Rust Package Registry");
}
