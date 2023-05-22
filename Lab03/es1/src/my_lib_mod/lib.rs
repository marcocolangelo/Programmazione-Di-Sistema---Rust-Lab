    use std::collections::HashSet;

    //wrapper di permute
    pub fn permutazioni(v: &Vec<i32>) -> Vec<Vec<i32>> {
        let mut perm = HashSet::new();
        permute(v, 0, &mut perm);
        perm.into_iter().collect()
    }

    //genera le permutazioni dei numeri del vettore iniziale
    fn permute(v: &Vec<i32>, start: usize, perm: &mut HashSet<Vec<i32>>) {
        if start == v.len() {
            perm.insert(v.clone());
        } else {
            for i in start..v.len() {
                let mut v = v.clone();
                v.swap(start, i);
                permute(&v, start + 1, perm);
            }
        }
    }

    //wrapper di generate_dispositions
    pub fn dispositions_with_repetition(elements: &Vec<char>, k: usize) -> Vec<Vec<char>> {
        let n = elements.len();
        let mut dispositions = Vec::new();
        generate_dispositions(elements, n, k, &mut dispositions, &mut Vec::new());
        dispositions
    }

    //genera tutte le disposizioni con ripetizione in gruppi da k=4 (poichè sono 5 le cifre da trattare) delle quattro operazioni 
    pub fn generate_dispositions(
        elements: &Vec<char>,
        n: usize,
        k: usize,
        dispositions: &mut Vec<Vec<char>>,
        current: &mut Vec<char>,
    ) {
        if current.len() == k {
            dispositions.push(current.clone());
        } else {
            for i in 0..n {
                current.push(elements[i]);
                generate_dispositions(elements, n, k, dispositions, current);
                current.pop();
            }
        }
    }

    pub fn operations(elem1 : f32, elem2 :f32,op : char) -> Option<f32>{
        //gestisce tutti i possibili casi di op (considerando anche i casi proibiti per ciascuna operazione)

        if op == '+'{
            return Some(elem1+elem2);
        }else if op == '-'{
            return Some(elem1-elem2);
        }else if op == '/'{
            //devi ovviamente gestire i casi proibiti dell'aritmetica
            if elem2 == 0.0 || (elem1 == 0.0 && elem2 == 0.0){
                return None;
            }
            return Some(elem1/elem2);
        }else if op == '*'{
            if elem1 == 0.0 && elem2 == 0.0{
                return None;
            }
            return Some(elem1*elem2);
        }else{
            return None;
        }
    }

    pub fn is_ten (numbers :  &Vec<i32>, op :  &Vec<char>) -> bool{
        let mut sol = numbers[0] as f32;
        let mut result : Option<f32>;
        
        for (x,op) in numbers[1..].iter().zip(op.iter()){
            
            //calcola l'operazione tra i due elementi (soluzione fino ad ora e l'elemento corrente della permutazione) in base a op
            result = operations(sol, *x as f32, *op);

            if result.is_none(){
                return false;
            }else{
                sol = result.unwrap();
            }
        }

        if sol == 10.0{
            return true;
        }

        return false;
    }

    pub fn find_sol <'a> (sol : &mut Vec<(Vec<i32>,Vec<Vec<char>>)>, perm : &'a Vec<Vec<i32>>, dispositions: &'a Vec<Vec<char>>){
        for x in perm{
            let mut found = false;
            let mut ys = Vec::new();
            for y in dispositions{
                //verifica se l'operazione ritorna 10 come risultato, in caso affermativo salva la soluzione nella tupla
                if is_ten(&x,&y) {
                    ys.push(y.to_vec());
                    found = true;
                }
            } 
            //questa gestione di sol è dovuta solo al fatto che la variabile è una tupla
            if found {
                sol.push((x.to_vec(), ys));
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


