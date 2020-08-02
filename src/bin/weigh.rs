use uom::si::{
    f64::Force,
    force::{kilogram_force, ounce_force, pound_force},
};

use weighty;

fn pretty_print_force(f: Force) -> String {
    let kg = f.get::<kilogram_force>();
    let lb = f.get::<pound_force>();
    let oz = f.get::<ounce_force>();

    if f <= Force::new::<kilogram_force>(1.0) {
        format!("{}g ({}oz)", kg * 1000.0, oz)
    } else {
        let lb = lb as i64;
        let oz = oz - (lb as f64 * 16.0);
        format!("{}kg ({}lbs {}oz)", kg, lb, oz)
    }
}

fn main() {
    let all_scales = weighty::get_all_scales();
    if all_scales.len() > 0 {
        for scale in all_scales {
            println!("{}", pretty_print_force(scale.read().unwrap()));
        }
    } else {
        println!("No scales found.  Are you sure it's plugged in, on, and something you have access to?");
    }
}

// TODO: add some command line arguments here
