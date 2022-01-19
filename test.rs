fn
main (){
    let pi = "3.14159";
    let stellen = pi.len();
    println!("{}",stellen);
    let mut f = pi.parse::<f32>().unwrap();
    let mut n = 1;
    let mut z = 1;
    let mut i = 1;
    while true {
        n = n+i;
        z = n*3;
        while true {
            let i: f32 = 1.0;
            f =f + i;
        }
    }
}
