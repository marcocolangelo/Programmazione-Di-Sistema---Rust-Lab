use es1::libreria_ten::*;

fn main() {
    let cifre = [2,7,2,2,1].to_vec();
    let elements = vec!['+', '-', '/', '*'];
    let mut sol : Vec<(&Vec<i32>,Vec<&Vec<char>>)> = Vec::new();

    //la funzione calcola le permutazioni senza ripetizioni (se due permutazioni risultano uguali allora le cancella)
    let perm = permutazioni(&cifre);

    //ora devo fare in modo di usare +,-,/ e * in tutte le combinazioni possibili
    let dispositions = dispositions_with_repetition(&elements, 4);

    //questa funzione trova tutte le possibili soluzioni per arrivare a 10 dato un vettore di permutazioni ed un vettore di permutazioni
    find_sol(&mut sol,&perm,&dispositions);

    //converte le soluzioni in stringhe
    let sol_as_string = sol_into_string(&sol);

    for x in sol_as_string{
        println!("{:?}",x);
    }
    
}
