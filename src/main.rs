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

    // Generate non-normalized basis polynomials
    let mut bases: Vec<Polynomial<i32>> = {
        // Generate left polynomial expansion
        let mut ll = vec![Polynomial::new_degree1(1i32); points.len()];
        for i in 1..points.len() {
            ll[i] = ll[i - 1].clone() * Polynomial::new_degree2(1, -points[i - 1].0);
        }

        // Generate right polynomial expansion
        let mut lr = vec![Polynomial::new_degree1(1); points.len()];
        for i in (0..points.len() - 1).rev() {
            lr[i] = lr[i + 1].clone() * Polynomial::new_degree2(1, -points[i + 1].0);
        }

        // Combine
        std::iter::zip(ll, lr).map(|(l, r)| l * r).collect()
    };

    // Apply factors to "normalize" bases
    for (i, poly) in bases.iter_mut().enumerate() {
        let y = points[i].1;
        *poly *= nf[i] * y;
    }

    // Add all bases to generate final polynomial in 'mod p'
    let coeffs = (0..points.len())
        .map(|i| mod_norm(bases.iter().map(|ln| ln.0[i]).sum(), p) % p)
        .collect();
    Polynomial(coeffs)
}

fn main() {
    let poly = rs_lagrange(vec![(0, 2), (1, 4), (2, 3), (3, 1)], 5);
    println!("final: {poly:?}");
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
