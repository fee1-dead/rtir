fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let r = j * 255 / (image_height - 1);
            let g = i * 255 / (image_width - 1);
            let b = 0;

            println!("{r} {g} {b}");
        }
    }
}
