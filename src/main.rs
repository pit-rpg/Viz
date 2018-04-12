// extern crate math;
mod macros;
mod math;
mod core;
mod render;
mod helpers;
use core::BufferGeometry;

// use math::Vector_3::Vector3;
use math::vector3::*;
// use math::vector::Vector;
// // use math::Vector3;
//
// fn foo(x: &Vector3, y: &Vector3) {
//     println!("{:?}", x.x + y.x);
// }


fn main() {
    let mut origin = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    origin.x = 0.2;

    // let geometry = Geometry::<f32>::new();

    // println!("{:?}", geometry);
    {
        let mut geom = BufferGeometry::new();

        geom.on_drop(|buff| {
            println!("LOLOLO");
        });

    }

    render::render_gl::init();
    render::render_gl::create_window();


    // let mut buffer = [0u8; 10];

    // let po = Vector3 {
    //     x: 1.0,
    //     y: 1.0,
    //     z: 1.0,
    // };
    // let po2 = Vector3 {
    //     x: 2.0,
    //     y: 2.0,
    //     z: 2.0,
    // };
    //
    // println!("Начало координат находится в ({}, {}, {})",
    //          &origin.x,
    //          &origin.y,
    //          &origin.z);
    //
    //
    // foo(&origin, &po);
    // origin.add_vectors(&po, &po2);
    // origin.add(&po);
    //
    // println!("Начало координат находится в ({}, {}, {})",
    //          &origin.x,
    //          &origin.y,
    //          &origin.z);
    //
    //         let a: i32 = 1;
    //         let mut b: f64 = 3.3;
    //         let c: f64 = (a / b as i32)as f64;
    //
    //         b = b.floor();
    //
    // println!("Начало координат находится в ({}, {}, {})",a,b,c);
    // // println!("Угадайте число!");
    // //
    // // println!("Пожалуйста, введите предположение.");
    // //
    // // let mut guess = String::new();
    // //
    // // io::stdin().read_line(&mut guess)
    // //     .expect("Не удалось прочитать строку");
    // //
    // // println!("Ваша попытка: {}", guess);
}

// struct Point {
//     x: i32,
//     y: i32,
// }
//
// fn main() {
//     let origin = Point { x: 0, y: 0 }; // origin: Point
//
//     println!("Начало координат находится в ({}, {})", origin.x, origin.y);
// }
