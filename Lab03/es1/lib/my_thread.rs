pub mod my_threads{

    pub fn find_sol <'a> (sol : &mut Vec<(&'a Vec<i32>,Vec<&'a Vec<char>>)>, perm : &'a Vec<Vec<i32>>, dispositions: &'a Vec<Vec<char>>){
        for x in perm{
            let mut found = false;
            let mut ys = Vec::new();
            for y in dispositions{
                //verifica se l'operazione ritorna 10 come risultato, in caso affermativo salva la soluzione nella tupla
                if is_ten(x,y) {
                    ys.push(y);
                    found = true;
                }
            } 
            //questa gestione di sol è dovuta solo al fatto che la variabile è una tupla
            if found {
                sol.push((x, ys));
            }
        }
    }

    pub fn convert_into_string(distr : &Vec<char>,num:&Vec<i32>) -> String {
        //gestisci il primo elemento da solo
        let mut s :String = num[0].to_string();

        //converti ogni elemento dei due vettori in char e aggiungi alla stringa per costruire l'espressione in String
        for (x,y) in distr.iter().zip(num[1..].iter()){
            s.push(*x);
            s.push_str(&y.to_string());
        }

        return s;

    }

    pub fn sol_into_string(sol : &Vec<(&Vec<i32>,Vec<&Vec<char>>)>) -> Vec<String>{
        
        let mut sol_as_string : Vec<String> = Vec::new();
        
        //per ogni distribuzione di operazioni prendi la corrispettiva permutazione di numeri, converti in stringa e aggiungi a tutte le altre soluzioni convertite
        for (num,op) in sol{
            for distr in op {
                    sol_as_string.push(convert_into_string(distr,num));
                
            }
        }

        return sol_as_string;

    }
}