use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use histo::Histogram;
use rasciigraph::{plot, Config};
use std::num::ParseFloatError;

fn main() -> Result<(), ParseFloatError> {
    let mut craw: ClipboardContext = ClipboardProvider::new().unwrap();
    let raw = craw.get_contents().unwrap();
    craw.set_contents(raw.clone()).unwrap(); //this is to "reset" clipboard contents
                                             //I wonder if there is a better way to do this without clone
                                             //add error handling and not sure if that actually works.
    let weight: Vec<&str> = raw.split("\n").collect();
    //println!("{:?}", weight);
    let mut weights: Vec<f64> = Vec::new();
    for w in weight {
        let x: f64 = w.trim().parse()?; //add better error handling
        weights.push(x);
    }

    let mut mode = std::env::args();
    match mode.nth(1).unwrap().as_str() {
        //I sense this is a very messy unwrap
        "p" => printdata(weights),
        "l" => linedata(weights),
        "h" => histodata(weights),
        _ => println!("Usage: 'p' to print data, 'l' for line, 'h' for histogram"),
    };

    Ok(())
}

fn printdata(weights: Vec<f64>) {
    println!("{:?}", weights);
}

fn linedata(weights: Vec<f64>) {
    println!(
        "{}",
        plot(weights, Config::default().with_height(10).with_offset(10))
    );
}

fn histodata(weights: Vec<f64>) {
    let mut histogram = Histogram::with_buckets(10);
    let mut rounded: Vec<u64> = Vec::new();
    for x in weights {
        if x < 0.0 {
            panic!("histogram currently doesn't support negatives")
        }
        let y = x as u64;
        histogram.add(y); //this is obviously bad and will fail
    }
    println!("{}", histogram);
}
