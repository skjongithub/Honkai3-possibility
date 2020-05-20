fn main(){
    println!("{}", B());
}

fn A()-> f64 { //擴充

    let alpha = 0.916/100f64;
    let beta = 1.831/100f64;
    let average = (alpha*3f64+beta)/4f64;
    fn draw (n:i32, average: f64) -> f64{
        let mut a = 0f64;
        for k in 0..49{
            let _k = k as f64;
        a += (_k+1f64)*((1f64-4f64*average).powf(_k)) * average* (4 - n) as f64;

        }    a += (1f64-4f64*average).powf(49f64)*50f64;
        a
    }
    (0..4).map(|n|draw(n, average)).collect::<Vec<_>>().into_iter().sum()
    //draw(0, average)
}//57

fn B() -> f64{ //精準
    let mut a = 0f64;
    let alpha = 1.24/100f64;
    let beta = 2.97/100f64;
    let average = (alpha*3f64+beta)/4f64;
    //for n in 0..4{
    for n in 0..4{
    for k in 0..10000{
        let _k = k as f64 +1f64;
        a += _k*(1f64-(4f64-n as f64)*average).powf(_k-1f64)*(4f64-n as f64)*average;
        }}
//    a += (1f64-(3f64*alpha + beta)).powf(49f64)*50f64;
    a
}//124


