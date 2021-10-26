mod hitron;
use hitron::HitronSummary;

fn main() {
    let htsz = hitron::Suzhou {
        boss: String::from("Kevin"),
        location: String::from("SIP Jinhe 88"),
        pplnumber: 80,
        sw_kind: String::from("FW and Cloud"),
    };
    
    println!("Hitron Suzhou: {}", htsz.summarize());
    println!("Hitron Suzhou: {}", htsz.happy());

    let hthc = hitron::HC {
        boss: String::from("Kevin"),
        location: String::from("Lixin Rode"),
        depart_kind: String::from("Viaste"),
        depart_pplnum: 8,
    };

    println!("Hitron HC: {}", hthc.happy());

    // trait as args.
    hitron::notify(hthc);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

}

/* Trait Bound */
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}