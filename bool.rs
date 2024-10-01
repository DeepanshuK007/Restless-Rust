fn bool() {
    let is_male = false;
    let _is_above_18 = true;

    if is_male {
        println!("You are a male"); 
        //as false this would not get printed
    } else {
        println!("You are not a male");
        //as is_male is false thus will get printed
    }

    if _is_above_18 && is_male {
        print!("You are a legal male");
        //this will not get printed
    }
}