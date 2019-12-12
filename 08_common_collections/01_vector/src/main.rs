fn main() {
    let mut vec1 = Vec::new();

    vec1.push(1);
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(5);

    let vec2 = vec!["a", "b", "c"];

    let item = &vec1[1];

    println!("{}", item);

    for i in vec2 {
        println!("{}", i)
    }

    match vec1.get(1000) {
        Some(valor) => print!("Achou {}", valor),
        None => println!("Não tem 1000")
    }
}
