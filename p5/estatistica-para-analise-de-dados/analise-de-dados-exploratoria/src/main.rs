mod regressao;

const DATA: [f32; 14] = [
    8., 12., 7.5, 7., 7.5, 8., 8.5, 9., 10., 10.5, 6.5, 10., 9., 8.,
];

fn main() {
    let mut data: Vec<f32> = DATA.to_vec();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Ordenado: {:?}", data);

    let (m, max_count) = moda(&data);
    println!("Moda = {m} (aparece {max_count} vezes)");

    println!("Mediana = {}", mediana(&data));

    let n = data.len();
    let lower_data = &data[..n / 2];
    let q1 = mediana(lower_data);
    println!("Q1 = {q1} (mediana de {:?})", lower_data);

    let higher_data = &data[n / 2..];
    let q3 = mediana(higher_data);
    println!("Q3 = {q3} (mediana de {:?})", higher_data);

    println!("AIQ = Q3 - Q1 = {}", q3 - q1);
    println!();

    regressao::run();
}

fn moda(data: &[f32]) -> (f32, usize) {
    let mut max_count = 0;
    let mut moda: f32 = data[0];
    for value in data {
        let count = data.iter().filter(|&x| x == value).count();
        if count > max_count {
            max_count = count;
            moda = *value;
        }
    }
    (moda, max_count)
}

fn mediana(data: &[f32]) -> f32 {
    let n = data.len();
    if n.is_multiple_of(2) {
        (data[n / 2 - 1] + data[n / 2]) / 2.
    } else {
        data[n / 2]
    }
}
