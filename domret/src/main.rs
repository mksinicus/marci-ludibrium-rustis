use rand::{Rng, thread_rng};
use chrono::{Local, Locale};
use std::collections::HashMap;
use colour::{red};
use std::process;

fn main() {
    let die_tempore = carpe_die(); // misso de salutatione
    let hora: Vec<&str> = die_tempore.2.split(':').collect();
    // println!("{:?}", hora);
    let hora: u32 = hora[0].parse().unwrap();
    assert!(hora < 24);
    // println!("{}", hora);
    let limine_omne = vec![
        ("anno", 360, 8),
        ("satione", 90, 5),
        ("mensi", 30, 3),
        ("septimana", 7, 2),
    ];
    let mut rng = thread_rng();
    let mut totale: u32 = 0;

    // limine_omne.reverse();
    println!(
        "Salve! Hodie es {} {}, et nunc es {} (UTC{}).  ", 
        die_tempore.0, die_tempore.1, die_tempore.2, die_tempore.3
    );

    match hora {
        n if n <  7 => print!("Bono gallicinio! Quamquam nimis matutino es nunc..."),
        n if n < 12 => print!("Bono mane! Sis die mirabili ad te."),
        n if n < 14 => print!("Bono meridie!"),
        n if n < 18 => print!("Bono postmeridiano! Pauco tardo, ne?"),
        n if n < 20 => print!("Bono crepusculo!"),
        _ => {
            red!("Nimis tardo. Sed bono nocti; sis te redordina labore ad cras.");
            process::exit(0);
        }
    }

    for distantia in limine_omne {
        let fortuito = rng.gen_range(0..distantia.1);
        // println!("Numero fortuito pro {} es {}.", distantia.0, fortuito);
        if fortuito < distantia.2 {
            print!(" Ordina et projecta pro {}. ", distantia.0);
            // println!("Obiter fortuito es {} ab {}.", fortuito, distantia.1);
            totale += 1;
            break
        }
    }

    match totale {
        // n if n > 2 => red_ln!("ATTENDE! Forsit es nimis labore hodie, sis te itero opera."),
        0 => print!(" Sis te bene faci commentario de hodie."),
        _ => ()
        // _ => println!("Quoque bene faci commentario de hodie.")
    }
}

fn carpe_die() -> (String, String, String, String) {
    let die_sys = Local::now();
    let die_dato = die_sys.format("%Y-%m-%d").to_string();
    let die_in_sep = die_sys.format_localized("%A", Locale::en_US).to_string();
    let tempore = die_sys.format("%H:%M:%S").to_string();
    let zona = die_sys.format("%Z").to_string();
    
    let die_in_sep = redde_septimana(&die_in_sep);
    return (die_dato, die_in_sep, tempore, zona)
    // return format!("Salve! Hodie es {} {}, et nunc es {}.", die_dato, die_in_sep, tempore);
}

fn redde_septimana(sep_anglico: &String) -> String {
    let septimana = HashMap::from([
        ("Monday", "lunidie"),
        ("Tuesday", "martidie"),
        ("Wednesday", "mercuridie"),
        ("Thursday", "jovidie"),
        ("Friday", "veneridie"),
        ("Saturday", "saturnidie"),
        ("Sunday", "solidie")
    ]);
    let latino_sep = septimana.get(sep_anglico.as_str());
    match latino_sep {
        Some(x) => return String::from(*x),
        None => panic!("Invalid day name, is your time locale in English?")
    }
}
