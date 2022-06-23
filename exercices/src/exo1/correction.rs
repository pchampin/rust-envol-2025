#![allow(dead_code)]

pub fn add_c(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 + b.0, a.1 + b.1)
}

// Écrivez maintenant une fonction qui prend deux complexes et retourne leur produit.
pub fn mul_c(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

// Écrivez une fonction qui calcule le carré de la norme du nombre complexe a
pub fn abs_sq_c(a: (f64, f64)) -> f64 {
    a.0 * a.0 + a.1 * a.1
}

// Écrivez une fonction qui calcule successivement
// les 1000 premiers éléments de la suite de Mandelbrot (pour un z et un c donné)
// et retourne l'indice du premier dont la norme (au carré) est infinie,
// ou retourne 0 si la suite ne diverge pas.
pub fn diverge(c: (f64, f64), limit: usize) -> usize {
    let mut i = 0;
    let mut z = (0.0, 0.0);
    while i < limit {
        i += 1;
        z = super::add_c(super::mul_c(z, z), c);
        if super::abs_sq_c(z).is_infinite() {
            return i;
        }
    }
    0
}
