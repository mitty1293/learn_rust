// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    // 関数の最後のコード行を返す値と同じにすることで関数の終了時に値を返す
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0
    }
}

fn main() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileaga: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileaga: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileaga: {}", car.color, car.transmission, car.convertible, car.mileage);
}