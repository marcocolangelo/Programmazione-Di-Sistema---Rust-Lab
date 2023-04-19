//consumer
use fslock::{LockFile, Error};

#[repr(C)]
struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}


fn consume(file:&mut LockFile) -> Result<(), fslock::Error>{
   let sensordata: SensorData;



    while !(*file).lock().is_ok(){

    }

        

    (*file).unlock()?;

    Ok(())
}
fn main()  -> Result<(),fslock::Error>{
    let mut file = LockFile::open("D:\\Desktop\\I ANNO LM\\II SEMESTRE\\Programmazione di sistema\\PS-Rust\\Lab\\Lab02\\lock_file")?;
    let mut  result;
    loop {
        result = consume(&mut file); 

        result?     
    }



    
}

