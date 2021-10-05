fn rgb(r: i32, g: i32, b: i32) -> String {
    let  mut xs =[r,g,b];
    for i in xs.iter_mut() {
        if *i < 0 {
            *i = 0;
        } else if *i > 255 {
            *i = 255;
        }
    }
    format!("{:0width$X}{:0width$X}{:0width$X}", xs[0], xs[1], xs[2], width=2)
}


