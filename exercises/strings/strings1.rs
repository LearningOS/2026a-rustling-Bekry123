// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
let mut s = String::from("lo");
s.push('l');
println!("{}",s);

}

fn current_favorite_color() -> String {
    String::from("blue")
}
//字符串以UTF-8编码，各语言文字均可编码，甚至还有emosji