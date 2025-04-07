use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use histo::Histogram;
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
use rasciigraph::{plot, Config};
use std::io::Error;
use std::num::ParseFloatError;

fn main() -> Result<(), ParseFloatError> {
    let mut craw: ClipboardContext = ClipboardProvider::new().unwrap();
    let raw = craw.get_contents().unwrap();
    craw.set_contents(raw.clone()).unwrap(); //this is to "reset" clipboard contents
                                             //I wonder if there is a better way to do this without clone
                                             //add error handling and not sure if that actually works.
    let weight: Vec<&str> = raw
        .split(&['\n', '\t', '\r'][..])
        .filter(|part| !part.is_empty()) //thanks to Claude for this part of the method chain
        .collect();
    //println!("{:?}", weight);
    let mut weights: Vec<f64> = Vec::new();
    for w in weight {
        let x: f64 = w.trim().parse()?; //add better error handling or just skip it
        weights.push(x);
    }

    let mut mode = std::env::args();
    match mode.nth(1).unwrap().as_str() {
        //I sense this is a very messy unwrap
        "p" => printdata(weights),
        "l" => linedata(weights),
        "h" => histodata(weights),
        "r" => simplereg(weights),
        _ => println!(
            "Usage: 'p' to print data,
            'l' for line,
            'h' for histogram,
            'r' to regress columns, y/dependent on left column, x on right"
        ),
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

fn simplereg(weights: Vec<f64>) {
    //I got lazy and asked claude to solve the problem of disentangling the data for now
    // pull out the even‑index items
    let x: Vec<_> = weights
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if i % 2 == 0 { Some(v) } else { None })
        .collect();

    // pull out the odd‑index items
    let y: Vec<_> = weights
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if i % 2 == 1 { Some(v) } else { None })
        .collect();
    let pearson = pearson_correlation(&x, &y);
    if x.len() != y.len() {
        panic!("you tried to regress with unequal columns")
    }
    let data = vec![("Y", y), ("X", x)];
    let data = RegressionDataBuilder::new().build_from(data).unwrap(); //should use ?
    let formula = "Y ~ X";
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit()
        .unwrap(); //should use ?
    let parameters: Vec<_> = model.iter_parameter_pairs().collect();
    let pvalues: Vec<_> = model.iter_p_value_pairs().collect();
    let standard_errors: Vec<_> = model.iter_se_pairs().collect();
    let rsq = model.rsquared();
    println!(
        "pearson r: {:?}, parameters: {:?}, pvalues: {:?}, standard errors: {:?}, r squared: {:?}",
        pearson, parameters, pvalues, standard_errors, rsq
    );
    ()
}

//below this line are simple functions
//again, using claude for the basic math parts
fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

fn std_dev(data: &[f64]) -> f64 {
    let m = mean(data);
    let var = data.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (data.len() as f64 - 1.0);
    var.sqrt()
}

fn covariance(x: &[f64], y: &[f64]) -> f64 {
    let mx = mean(x);
    let my = mean(y);
    x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - mx) * (yi - my))
        .sum::<f64>()
        / (x.len() as f64 - 1.0)
}

fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    covariance(x, y) / (std_dev(x) * std_dev(y))
}
