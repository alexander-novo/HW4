use nalgebra::Matrix4;
fn main() {
    let a = Matrix4::from_iterator([1.0,-4.0,-1.0,0.0,0.0,5.0,3.0,2.0,2.0,3.0,1.0,0.0,1.0,-1.0,1.0,1.0]);
    let mut ul = a;
    // rust is 0 indexed
    for i in (1..4).rev() {
        let alphaii = ul[(i,i)];
        let mut view = ul.view_mut((0,i),(i,1));
        view /= alphaii;
        
        let lct= ul.view((0,i),(i,1))*ul.view((i,0),(1,i));
        // start (row, col), size (nrows, ncols)
        let mut view = ul.view_mut((0,0),(i,i));
        view -= lct;

        println!("{}",ul);
    }
    println!("Hello, world!{a}{ul}");
}
