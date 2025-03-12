use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use rasciigraph::{plot, Config};

fn main() {
    let sraw = "230
    230
    228
    228
    227
    226
    228
    229
    227
    225
    223
    230
    231
    233
    223
    212
    204
    199
    194
    191
    190
    188
    195
    197.9
    197.8
    197.6
    197.5";

    let mut traw: ClipboardContext = ClipboardProvider::new().unwrap();
    let raw = traw.get_contents().unwrap();

    let weight: Vec<&str> = raw.split("\n").collect();
    //println!("{:?}", weight);
    let mut weights: Vec<f64> = Vec::new();
    for w in weight {
        let x: f64 = w.trim().parse().unwrap(); //add error handling
        weights.push(x);
    }
    //println!("{:?}", weights);
    println!(
        "{}",
        plot(weights, Config::default()) //.with_height(10).with_offset(10))
    );
}
