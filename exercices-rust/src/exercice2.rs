/// Implémente le crible d'Eratosthène.
/// La fonction retourne un vecteur contenant les nombres premiers inférieurs ou égaux à `n`.
///-> Vec<u32>
/// # Exemple
/// ```
/// let r = td_exercices_choisis::exercice2::sieve(7);
/// assert_eq!(r, vec![2, 3, 5, 7])
/// ```
pub fn sieve(n: u32) -> Vec<u32> {

    /*panic!("Not implemented!");*/

    let mut vec: Vec<u32> = Vec::new();
    for i in 2..=n {
        vec.push(i);
    }
    let mut premier: Vec<u32> = Vec::new();
    let mut a = 1;

    for i in 2..n  {

        if !vec.is_empty() && a != vec[0] { //si le vecteur n'est pas vide 
            a = vec[0];                    //et faire en sorte que a ne soit qu'une fois
            premier.push(a);
            println!("a vaut {}", a);
        }

        vec.retain(|&j| j % i != 0);

        println!("vec vaut {:?}", vec);
        println!("premier vaut {:?}", premier);
    }

    return premier;

}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn n_zero() {
        let m = sieve(0);
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn n_thirty() {
        let m = sieve(30);
        assert_eq!(m, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
}
