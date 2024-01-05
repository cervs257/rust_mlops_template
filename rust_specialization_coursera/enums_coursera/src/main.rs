// enums are very useful for modeling data that can be classified into a set of variants
// example, avoid typos when specifying data (like wine region or price range)

#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
}

#[derive(Debug)]
enum PriceRange {
    Below10,
    Below20,
    Below50,
    Below100,
}

#[derive(Debug)]
enum WineType {
    Red,
    White,
    Rose,
    Sparkling,
}

#[derive(Debug)]
struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
    price_range: PriceRange,
    wine_type: WineType,
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
        price_range: PriceRange::Below100,
        wine_type: WineType::Red,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
        price_range: PriceRange::Below20,
        wine_type: WineType::Sparkling,
    };

    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);
    println!("Wine 1: {:?}", wine1);
    println!("Wine 2: {:?}", wine2);
}
