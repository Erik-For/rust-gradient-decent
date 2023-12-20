use thousands::Separable;
struct DataPoint {
    x: f64,
    y: f64,
}

fn function(x: f64, k: f64, m: f64) -> f64 {
    k * x + m
}

fn cost(data: &Vec<DataPoint>, k: f64, m: f64) -> f64 {
    let mut sum: f64 = 0.0;
    for datapoint in data {
        sum += (datapoint.y - function(datapoint.x, k, m)).powi(2);
    }
    sum / (2*data.len()) as f64
}

fn derivative_k(data: &Vec<DataPoint>, k: f64, m: f64) -> f64 {
    let mut sum: f64 = 0.0;
    for datapoint in data {
        sum += -2.0 * datapoint.x * (datapoint.y - (k * datapoint.x + m));
    }
    sum / (2*data.len()) as f64
}

fn derivative_m(data: &Vec<DataPoint>, k: f64, m: f64) -> f64 {
    let mut sum: f64 = 0.0;
    for datapoint in data {
        sum += -2.0 * (datapoint.y - (k * datapoint.x + m));
    }
    sum / (2*data.len()) as f64
}

fn main() {
    let data = vec![DataPoint {x: 1.0, y: 2.0}, DataPoint {x: 3.0, y: 4.0}, DataPoint {x: 5.0, y: 6.0}, DataPoint {x: 7.0, y: 8.0}];
    let mut k = 2.0;
    let mut m = 1.0;
    let alpha = 0.00001;
    let mut i = 0;

    while i < 3_000_000 {    
        let slope_k = derivative_k(&data, k, m);
        let slope_m = derivative_m(&data, k, m);

        k = k - alpha * slope_k;
        m = m - alpha * slope_m;

        let _loss = cost(&data, k, m);
        //println!("k: {}, m: {}, loss: {}", k, m, loss);

        i = i + 1;
    }
    println!("y = {}x + {}, iterations: {}", k.round(), m.round(), i.separate_with_commas());
}
