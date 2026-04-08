pub fn run() {
    let (xs, ys) = load_colunms(
        "sampleData.csv",
        "Text_Messages_Sent_Yesterday",
        "Text_Messages_Received_Yesterday",
    );

    let n = xs.len() as f64;

    // Médias
    let mean_x = xs.iter().sum::<f64>() / n;
    let mean_y = ys.iter().sum::<f64>() / n;

    // Covariância: Σ(xi - x̄)(yi - ȳ)
    let cov_xy: f64 = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();

    // Variância de x: Σ(xi - x̄)²
    let var_x: f64 = xs.iter().map(|x| (x - mean_x).powi(2)).sum();

    // Coeficiente angular: b = cov(x,y) / var(x)
    let b = cov_xy / var_x;

    // Intercepto: a = ȳ - b·x̄
    let a = mean_y - b * mean_x;

    println!("n               = {}", xs.len());
    println!("média x         = {mean_x:.4}");
    println!("média y         = {mean_y:.4}");
    println!("b (inclinação)  = {b:.4}");
    println!("a (intercepto)  = {a:.4}");
    println!("Equação: ŷ = {b:.2}x + {a:.2}");
}

fn load_colunms(path: &str, col_x: &str, col_y: &str) -> (Vec<f64>, Vec<f64>) {
    let mut reader = csv::Reader::from_path(path).expect("CSV não encontrado");

    // Pega os índices pelo cabeçalho
    let headers = reader.headers().unwrap().clone();
    let idx_x = headers.iter().position(|h| h == col_x).unwrap();
    let idx_y = headers.iter().position(|h| h == col_y).unwrap();

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for record in reader.records() {
        let record = record.unwrap();
        xs.push(clean(record.get(idx_x).unwrap_or("")));
        ys.push(clean(record.get(idx_y).unwrap_or("")));
    }
    (xs, ys)
}

// Limpeza dos valores conforme o enunciado:
// - vazio ou "idk" -> 0
// - "80+" -> 80 (ignorar o '+', e o que vier depois)
fn clean(val: &str) -> f64 {
    let v = val.trim();
    if v.is_empty() || v.to_lowercase().contains("idk") {
        return 0.;
    }

    let v = v.split('+').next().unwrap_or("0"); // "80+" -> "80"
    v.parse().unwrap_or(0.)
}
