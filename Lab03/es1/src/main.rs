use std::ops::Deref;
use std::thread;
use es1::libreria_ten::*;
use std::sync::Arc;
use std::sync::Mutex;
//use lib::my_threads::*;


//devi gestire il lifetime qui

//nella versione senza thread si usava pub fn find_sol <'a> (sol : &mut Vec<(&'a Vec<i32>,Vec<&'a Vec<char>>)>, perm : &'a Vec<Vec<i32>>, dispositions: &'a Vec<Vec<char>>)
pub fn thread_find_sol<'a>(
    sol: Arc<Mutex<Vec<(&'a Vec<i32>, Vec<&'a Vec<char>>)>>>,
    perm: Arc<Mutex<Vec<&'a Vec<i32>>>>,
    dispositions: Arc<Mutex<&'a Vec<Vec<char>>>>,
) {


        
            
         let mut p =    perm.lock().unwrap();
         
    
    for x in p.deref(){
        let mut found = false;
        let mut ys = Vec::new();


        let mut d = dispositions.lock().unwrap();


        for y in *d.deref(){
            //verifica se l'operazione ritorna 10 come risultato, in caso affermativo salva la soluzione nella tupla
            if is_ten(x,y) {
                ys.push(y);
                found = true;
            }
        } 
        //questa gestione di sol è dovuta solo al fatto che la variabile è una tupla

        if found {


            let mut s = sol.lock().unwrap();


            s.push((x, ys));
        }
    }
}



fn main() {
    let cifre = [2,7,2,2,1].to_vec();
    let elements = vec!['+', '-', '/', '*'];
    let mut sol : Vec<(&Vec<i32>,Vec<&Vec<char>>)> = Vec::new();
    let n = 3;
    

    //la funzione calcola le permutazioni senza ripetizioni (se due permutazioni risultano uguali allora le cancella)
    let perm = permutazioni(&cifre);
    let chunk_size = (perm.len() + n - 1) / n;  
    

    //ora devo fare in modo di usare +,-,/ e * in tutte le combinazioni possibili
    let dispositions = dispositions_with_repetition(&elements, 4);

    //questa funzione trova tutte le possibili soluzioni per arrivare a 10 dato un vettore di permutazioni ed un vettore di permutazioni
    //find_sol(&mut sol,&perm,&dispositions);

    //divido le permutazioni (dunque il Vec<Vec<i32>>) in un vettore di vettori di permutazioni (cioè in un vettore Vec di chunck, percio Vec<Vec> di permutazioni e quindi Vec<Vec<Vec>>)
    let mut handles : Vec<Vec<Vec<i32>>>=  perm.chunks(chunk_size).map(|c| c.to_vec()).collect() ;


//qui sotto comincia l'implementazione con i thread
thread::scope(|s|{
    let shared_sol = Arc::new(Mutex::new(sol));
    let shared_disp = Arc::new(Mutex::new(&dispositions));

    let mut threads = vec![];

    //mettiamo scope per assicurarci che il lifetime delle variabili non siano più lunghe di quelle dei thread 
   
        //verifica come sistemare la cosa di handles[i]
        for i in 1..n{
            let shared_handles = Arc::new(Mutex::new(handles[i].iter().collect()));

            let mut arc_sol: Arc<Mutex<Vec<(&Vec<i32>, Vec<&Vec<char>>)>>> = shared_sol.clone(); 
            let mut arc_hand = shared_handles.clone();
            let mut arc_disp: Arc<Mutex<&Vec<Vec<char>>>> = shared_disp.clone();
            threads.push(thread::spawn(move ||{
                thread_find_sol(arc_sol, arc_hand, arc_disp)
            }));
        }
    });
    

//non puoi usare la soluzione sotto perchè richiederebbe di rendere mutuamente esclusivo più volte sol, e senza Arc questo non si può fare


    // let t1 = thread::scope(|s|{
    //                                                 s.spawn(|| 
    //                                                     {
    //                                                         find_sol(&mut sol,&handles[0], &dispositions)
    //                                                     });
    //                                                 s.spawn(|| 
    //                                                     {
    //                                                     find_sol(&mut sol,&handles[1], &dispositions)
    //                                                     });
    //                                                 s.spawn(|| 
    //                                                     {find_sol(&mut sol,&handles[2], &dispositions)
    //                                                     });    
    //                                                 });
    
    
   

    //converte le soluzioni in stringhe
   
   
   /*  let sol_as_string = sol_into_string(&sol);

    for x in sol_as_string{
        println!("{:?}",x);
    }
    */
    
}
