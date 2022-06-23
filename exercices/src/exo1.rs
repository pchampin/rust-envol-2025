mod correction;

// Dans la suite, on va représenter les nombres complexes
// par des tuples (f64, f64) de la forme (partie_réelle, partie_imaginaire).

// Commencez par écrire une fonction qui prend deux complexes et retourne leur somme.
fn add_c(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    correction::add_c(a, b) // REMPLACEZ CETTE LIGNE PAR VOTRE CODE
}

// Écrivez maintenant une fonction qui prend deux complexes et retourne leur produit.
fn mul_c(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    correction::mul_c(a, b) // REMPLACEZ CETTE LIGNE PAR VOTRE CODE
}

// Écrivez une fonction qui calcule le carré de la norme du nombre complexe a
fn abs_sq_c(a: (f64, f64)) -> f64 {
    correction::abs_sq_c(a) // REMPLACEZ CETTE LIGNE PAR VOTRE CODE
}

// Écrivez une fonction qui calcule successivement
// les 'limit' premiers éléments de la suite de Mandelbrot (pour un 'c' donné)
// et retourne l'indice du premier dont la norme (au carré) est infinie,
// ou retourne 0 si la suite n'a pas divergé.
fn diverge(c: (f64, f64), limit: usize) -> usize {
    correction::diverge(c, limit) // REMPLACEZ CETTE LIGNE PAR VOTRE CODE
}

pub fn main() {
    const STEP_X: f64 = 0.025;
    const STEP_Y: f64 = STEP_X * 2.0;
    const LIMIT: usize = 2000;

    for y in (-20..=20).map(|y| (y as f64) * STEP_Y) {
        for x in (-60..=20).map(|x| (x as f64) * STEP_X) {
            print!(
                "{}",
                if diverge((x, y), LIMIT) == 0 {
                    "*"
                } else {
                    " "
                }
            );
        }
        println!();
    }
}
