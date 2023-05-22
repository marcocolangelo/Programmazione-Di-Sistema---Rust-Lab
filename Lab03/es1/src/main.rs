mod my_lib_mod;
mod my_thread_mod;
use crate::my_lib_mod::lib::*;
use crate::my_thread_mod::my_thread::*;


use std::thread;
use std::sync::Arc;
use std::sync::Mutex;




fn main() {
    let cifre = [2,7,2,2,1].to_vec();
    let elements = vec!['+', '-', '/', '*'];
    let sol : Vec<(Vec<i32>,Vec<Vec<char>>)> = Vec::new();
    let n = 3;
    

    //la funzione calcola le permutazioni senza ripetizioni (se due permutazioni risultano uguali allora le cancella)
    let perm = permutazioni(&cifre);
    let chunk_size = (perm.len() + n - 1) / n;  
    

    //ora devo fare in modo di usare +,-,/ e * in tutte le combinazioni possibili
    let dispositions = dispositions_with_repetition(&elements, 4);


    //questa funzione trova tutte le possibili soluzioni per arrivare a 10 dato un vettore di permutazioni ed un vettore di permutazioni
    //find_sol(&mut sol,&perm,&dispositions);

    //divido le permutazioni (dunque il Vec<Vec<i32>>) in un vettore di vettori di permutazioni (cioè in un vettore Vec di chunck, percio Vec<Vec> di permutazioni e quindi Vec<Vec<Vec>>)
    let handles : Vec<Vec<Vec<i32>>>=  perm.chunks(chunk_size).map(|c| c.to_vec()).collect() ;
    

    //qui sotto comincia l'implementazione con i thread
    let shared_sol_as_string = Arc::new(Mutex::new(Vec::new()));
    let shared_sol = Arc::new(Mutex::new(sol));
    let shared_disp: Arc<Mutex<Vec<Vec<char>>>> = Arc::new(Mutex::new(dispositions.clone()));

    let mut threads = vec![];

    for i in 1..n{
        let shared_handles = Arc::new(Mutex::new(handles[i].clone()));
        
        let arc_sol: Arc<Mutex<Vec<(Vec<i32>, Vec<Vec<char>>)>>> = shared_sol.clone(); 
        let arc_sol_2: Arc<Mutex<Vec<(Vec<i32>, Vec<Vec<char>>)>>> = shared_sol.clone(); 
        let arc_hand: Arc<Mutex<Vec<Vec<i32>>>> = shared_handles.clone();
        let arc_disp: Arc<Mutex<Vec<Vec<char>>>> = shared_disp.clone();

        let arc_sol_as_string = shared_sol_as_string.clone();

        threads.push(thread::spawn(move ||{
            thread_find_sol(arc_sol, arc_hand, arc_disp);
            thread_sol_into_string(arc_sol_2,arc_sol_as_string);
        }));
    }
  
    
    for t in threads { t.join().unwrap(); }


    for x in &(*shared_sol_as_string.lock().unwrap()){
        println!("{:?}",x);
    }

    
}
