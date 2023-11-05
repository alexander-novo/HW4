use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::Write,
};

use nalgebra::Matrix4;

fn main() -> Result<(), Box<dyn Error>> {
    // fill $\(\mat{A}\)$ with given values, column major order
    let a = Matrix4::from_iterator([
        1.0, -4.0, -1.0, 0.0, 0.0, 5.0, 3.0, 2.0, 2.0, 3.0, 1.0, 0.0, 1.0, -1.0, 1.0, 1.0,
    ]);

    // compute UL decompositon
    let ul = ul_decomp(a);

    // extract the upper triangular part
    let mut upper = ul.upper_triangle();
    // enforce upper is unit triangular
    upper.fill_diagonal(1.0);
    // extract lower triangular part
    let lower = ul.lower_triangle();

    create_dir_all("./out/")?;
    let f = File::create("out/output5.txt")?;
    writeln!(&f, "U={upper}\nL={lower}\nUL={}", upper * lower)?;

    Ok(()) // :)
}

/// Computes UL decomposition in place, note input matrix is overridden
fn ul_decomp(mut ul: Matrix4<f64>) -> Matrix4<f64> {
    // rust is 0 indexed
    for i in (1..4).rev() {
        let alphaii = ul[(i, i)];
        let mut a_vec = ul.view_mut((0, i), (i, 1));
        a_vec /= alphaii;

        let lct = ul.view((0, i), (i, 1)) * ul.view((i, 0), (1, i));
        // start (row, col), size (nrows, ncols)
        let mut a_hat = ul.view_mut((0, 0), (i, i));
        a_hat -= lct;
    }
    ul
}
