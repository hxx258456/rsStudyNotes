fn main() {
    greet_world();

    let pengiun_data = "\
    common name, length(cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid,data
    ";
    let records = pengiun_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fileds: Vec<_> = record.split(",").map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fileds)
        }

        let name = fileds[0];

        if let Ok(length) = fileds[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn greet_world() {
    let southren_germany = "Grüß Gott!";
    let chinese = "世界,你好!";
    let english = "World,hello!";
    let regions = [southren_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region)
    }
}
