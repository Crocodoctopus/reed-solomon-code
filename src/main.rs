mod polynomial;
use polynomial::Polynomial;

fn mod_norm(x: i32, p: u32) -> u32 {
    (x.abs() * p as i32 + x) as u32
}

fn mod_inv(x: i32, p: u32) -> i32 {
    i32::pow(x, p - 2)
}

fn rs_lagrange(points: Vec<(i32, i32)>, p: u32) -> Polynomial<u32> {
    // Collect normalizing factors in 'mod p'
    let nf: Vec<i32> = points
        .iter()
        .map(|(x, _)| {
            points
                .iter()
                .filter(|(ix, _)| *ix != *x)
                .map(|(ix, _)| x - ix)
                .product()
        })
        .map(|x| mod_inv(x, p))
        .collect();

    // Roots
    let roots: Vec<Polynomial<i32>> = points
        .iter()
        .map(|(x, _)| Polynomial::new_degree2(1, -x))
        .collect();

    // Generate non-normalized basis polynomials
    let mut bases: Vec<Polynomial<i32>> = {
        // Generate left polynomial expansion
        let mut ll = vec![Polynomial::new_degree1(1i32); points.len()];
        for i in 1..points.len() {
            ll[i] = ll[i - 1].clone() * roots[i - 1].clone();
        }
        // Generate right polynomial expansion
        let mut lr = vec![Polynomial::new_degree1(1); points.len()];
        for i in (0..points.len() - 1).rev() {
            lr[i] = lr[i + 1].clone() * roots[i + 1].clone();
        }
        // Combine
        std::iter::zip(ll, lr).map(|(l, r)| l * r).collect()
    };

    // Normalize
    for (i, poly) in bases.iter_mut().enumerate() {
        let factor = nf[i];
        let y = points[i].1;
        for coeff in poly.0.iter_mut() {
            *coeff *= factor * y;
        }
    }

    // Generate final lagrange polynomial
    let mut f: Vec<u32> = vec![0; points.len()];
    for i in 0..points.len() {
        let x: i32 = bases.iter().map(|ln| ln.0[i]).sum();
        f[i] = mod_norm(x, p) % p;
    }

    println!("final: {f:?}");

    Polynomial(f)
}

fn main() {
    rs_lagrange(vec![(0, 2), (1, 4), (2, 3), (3, 1)], 5);
}

/*
(0,2)
(1,4)
(2,3)
(3,1)

l0: -1/6 [3]: x^3 -6x^2 +11x -6
l1: 1/2 [2]: x^3 -5x^2 +6x
l2: -1/2 [2]: x^3 -4x^2 +3x
l3: 1/6 [1]: x^3 - 3x^2 + 2x
*/
