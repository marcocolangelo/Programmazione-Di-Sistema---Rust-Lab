
    
    use es1::libreria_ten::*;
    use std::sync::Arc;
    use std::sync::Mutex;

    pub fn thread_find_sol<'a>(
    sol: Arc<Mutex<Vec<(Vec<i32>, Vec< Vec<char>>)>>>,
    perm: Arc<Mutex<Vec<Vec<i32>>>>,
    dispositions: Arc<Mutex<Vec<Vec<char>>>>,
) {

    let p =    perm.lock().unwrap();
         
    
    for x in &(*p){
        let mut found = false;
        let mut ys = Vec::new();


        let d = dispositions.lock().unwrap();


        for y in &(*d){
            //verifica se l'operazione ritorna 10 come risultato, in caso affermativo salva la soluzione nella tupla
            if is_ten(x,y) {
                ys.push(y.clone());
                found = true;
            }
        } 
        //questa gestione di sol è dovuta solo al fatto che la variabile è una tupla

        if found {


            let mut s = sol.lock().unwrap();


            s.push((x.clone(), ys));
        }
    }
}


    pub fn thread_sol_into_string(p_sol : Arc<Mutex<Vec<(Vec<i32>, Vec<Vec<char>>)>>>, p_stringa: Arc<Mutex<Vec<String>>>) {
        
        //let mut sol_as_string : Vec<String> = Vec::new();
        
        let sol = p_sol.lock().unwrap(); 
        let mut stringa = p_stringa.lock().unwrap();
    
        //per ogni distribuzione di operazioni prendi la corrispettiva permutazione di numeri, converti in stringa e aggiungi a tutte le altre soluzioni convertite
        for (num,op) in &(*sol){
            for distr in op {
                    stringa.push(convert_into_string(distr,num));
                
            }
        }
    
    }