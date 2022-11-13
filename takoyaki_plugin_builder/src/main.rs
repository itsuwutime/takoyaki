mod builder;
mod logger;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    builder::create_new_build(
        "https://github.com/worldhellosdj/sfdfdfd",
        "main",
        "/",
        "uuu--uuu--ui"
    );
}
