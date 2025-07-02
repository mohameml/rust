pub fn div(a :f64 , b : f64) -> Option<f64> {

    if b == 0.0 {
        return None ;
    } else {
        return Some(a / b);
    }

}


// let result = diviser(10.0, 0.0);
// match result {
//     Some(res) => println!("Résultat = {}", res),
//     None => println!("Erreur : Division par zéro !"),
// }




