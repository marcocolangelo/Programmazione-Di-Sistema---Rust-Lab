use std::sync::{Arc,Mutex, Condvar};

use es1::{CBState, CBclassic, CBchannel};

//implementazione classica con stato reso come Mutex e wait_while tramite Condition variable
fn classic_barrier() {
    let cbarrrier = Arc::new(CBclassic::new(3));  
    let mut vt = Vec::new();
    for i in 0..3 {
        let ccb = cbarrrier.clone();
        vt.push(std::thread::spawn(move || {
            
            for j in 0..10 {
                

                ccb.wait();
               
                println!("after barrier {} {}", i, j);
            }

        }));
    }

    for t in vt {
        t.join().unwrap();
    }
}

fn channel_barrier(){
    let cbc = Arc::new(CBchannel::new(3));
    let mut vt = Vec::new();
    for i in 0..3{
        let barrier = cbc.clone();      //alla fine dello scope in automatico drop
        vt.push(std::thread::spawn(move || {
            for k in 0..10{
                barrier.wait(i);
                println!("thread {} after barrier {}", i, k);
            }
        }))
    }

//NB:  se fosse stato un unico canale MPSC con più tx (tutti clonati dal primo con il primo non usato) avremmo dovuto fare il drop
//esplicito del primo tx. Qui invece lavoriamo con un numero pari di canali e dunque un numero pari di tx (dunque cloniamo la coppia non il solo tx)
//il che significa che alla fine dello scoop viene fatto il drop automatico dell'intera coppia 
//questo permette alla comunicazione di arrestarsi in automatico
//nel caso MPSC bisogna fare il drop(tx) per chiudere il canale (dove altrimenti la recv non saprà mai quando bloccarsi)

    for t in vt {
        t.join().unwrap();
    }
}
    

fn main(){
    //classic_barrier();
    channel_barrier();
}
