fn main() {
    greet_word();
}

fn greet_word() {
    let spanish = "Hola Mundo!";
    let germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [spanish, germany, japan];
    for region in regions.iter() {
        println!("{}", &region)
    }
}