use es2::{RingBuf,RingState};
use std::sync::{Arc};
use std::thread;
use std::time::Duration;

fn main() {
    /*il producer legge ad intervalli di 1s dei dati da dieci sensori e li inserisce sul buffer
    ● il consumer ogni 10 secondi legge i dati dal buffer e stampa la
    media, il minimo e il massimo dei valori di ciascun sensore*/
    let data : Vec<i32>= (0..9).collect();
    let cap = 15;
    let p_buf = Arc::new(RingBuf::new(cap-5));  //puntatore 1 al buffer
    let p2_buf = p_buf.clone();      //puntatore 2 al buffer

    let t_prod = thread::spawn(move || {
        let mut count = 0;
        while count < cap{
            for i in &data{
                if let Ok(_) = p_buf.write(*i){
                    count+=1;
                    if count >= cap {break;}
                    println!("Scrittura {} ovvero {} eseguita con successo",count, i);

                }
                thread::sleep(Duration::from_millis(500));
            }
        }

        return count;
    });

    let t_cons = thread::spawn(move || {
        let mut count = 0;
        let mut data = -1;
        while count < cap{
            if let Some(data) = p2_buf.read(){
                count+=1;
                println!("Lettura {} ovvero {} avvenuta con successo",count,data);
            }
            thread::sleep(Duration::from_millis(5000));
        }
        return count;
    });

    let th_prod = t_prod.join().unwrap();
    let th_cons = t_cons.join().unwrap();

    println!("Contatore read : {} , contatore write : {}",th_cons,th_prod);



}
