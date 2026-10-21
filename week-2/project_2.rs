fn main(){

	// sum
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
    let a:f64 = 250_000.00;
    let s:f64 = (t*2.00) + m + (h*3.00) + (d*3.00) + a;
    println!("the total sum of the sales is {}",s );

    // average
    let v = s/10.00;
    println!("the total average is {}", v);
}