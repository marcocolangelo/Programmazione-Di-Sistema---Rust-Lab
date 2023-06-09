use std::env::args;

const SUBS_I : &str =
"àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůű
ųẃẍÿýžźż";

const SUBS_O: &str =
"aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuu
uwxyyzzz";


fn conv(c: char) -> char {
    let char_i = SUBS_I.chars();
    let char_o = SUBS_O.chars();
    let mut i = 0;
    let mut j = 0;


    for x in char_i{
        
        if x == c{
            break;
        }
        i=i+1;
    }

    for x in char_o {
        if j == i{
            return x;
        }
        j=j+1;
    }

    return c;

    
    
}

fn slugify(s: &str) -> String {
    
    
    let mut s2:String = String::new();
    let mut counter = 0;
    let mut i = 0;
    
    

    for mut x in s.chars(){

        x = conv(x.to_ascii_lowercase());

        if x.is_alphanumeric() == false {

            counter = counter + 1;

            if counter < 2{

                if i != (s.chars().count()-1) || s.chars().count() == 1 {
                    s2.push('-');
                }
                
            }else{
                continue;            
            }

        }else{
            counter = 0;
            s2.push(x);
        }

        i=i+1;
    }

    

    return s2;
}




fn main(){
    let args: Vec<String> = args().skip(1).collect();

    if args.len() > 0{
        let st = slugify(args[0].as_str());
        println!("{},{}",st.chars().count(), st);
    }

    }