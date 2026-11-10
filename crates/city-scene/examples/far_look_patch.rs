//! Adds `farLook` to every building of a render fixture's `result.modernCities`,
//! as the desktop shell does when it builds the world payload.
//!
//!     cargo run --release -p city-scene --example far_look_patch -- IN.json OUT.json

use urban::ModernBuilding;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (input, output) = (&args[1], &args[2]);
    let mut fixture: serde_json::Value =
        serde_json::from_slice(&std::fs::read(input).expect("read fixture")).expect("parse fixture");
    let cities = fixture["result"]["modernCities"]
        .as_array_mut()
        .expect("result.modernCities");
    let (mut done, mut skipped) = (0, 0);
    for city in cities {
        let Some(buildings) = city["buildings"].as_array_mut() else {
            continue;
        };
        let parsed: Vec<Option<ModernBuilding>> = buildings
            .iter()
            .map(|value| serde_json::from_value::<ModernBuilding>(value.clone()).ok())
            .collect();
        // Buildings that do not parse stand outside the podium search.
        let all: Vec<ModernBuilding> = parsed.iter().flatten().cloned().collect();
        let podiums = city_scene::buildings::far::podiums_of(&all);
        let mut next = 0;
        for (value, building) in buildings.iter_mut().zip(parsed) {
            let Some(building) = building else {
                skipped += 1;
                continue;
            };
            let podium = podiums[next];
            next += 1;
            if let Some(look) = city_scene::buildings::far::far_look(&building, podium) {
                value["farLook"] = serde_json::to_value(look).unwrap();
                done += 1;
            } else {
                skipped += 1;
            }
        }
    }
    std::fs::write(output, serde_json::to_vec(&fixture).unwrap()).expect("write");
    println!("{done} buildings given a far look, {skipped} left to the plan");
}
