use std::io;

fn main() {
    
    loop{ 
        println!("Which operation you want to perform:");
        println!("1.Addition  2.Subtraction   3.Multiplication   4.Division");

        let option = input("Choose the option:");

        if option==1{
            addition()
        }
        else if option==2{
            subtraction()
        }
        else if option==3{
            multiplication()
        }
        else if option==4{
            division()
        }
    }
}

fn input(string:&str) -> i32{
    println!("{}",string);
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read input");
    let input:i32 = input_str.trim().parse().unwrap();
    input
}

fn addition(){
    let num1=input("Enter 1st number:");
    let num2=input("Enter 2nd number:");

    println!("The sum of {}+{}={}",num1,num2,num1+num2);
}

fn subtraction(){
    let num1=input("Enter 1st number:");
    let num2=input("Enter 2nd number:");

    println!("The subtraction of {}-{}={}",num1,num2,num1-num2);
}

fn multiplication(){
    let num1=input("Enter 1st number:");
    let num2=input("Enter 2nd number:");

    println!("The multiplication of {}x{}={}",num1,num2,num1*num2);
}

fn division(){
    let num1=input("Enter 1st number:");
    let num2=input("Enter 2nd number:");

    println!("The division of {}/{}={}",num1,num2,num1/num2);
}



// fn input(string:&str) -> i32{
//     println!("{}",string);
//     let mut input_str = String::new();
//     io::stdin().read_line(&mut input_str).expect("Failed to read input");
//     let input:i32 = input_str.trim().parse().unwrap();
//     input
// }

// fn addition(){
//     let num1=input("Enter 1st number:");
//     let num2=input("Enter 2nd number:");

//     println!("The sum of {} and {} is {}",num1,num2,num1+num2);
// }

// fn subtraction(){
//     let num1=input("Enter 1st number:");
//     let num2=input("Enter 2nd number:");

//     println!("The subtraction of {} and {} is {}",num1,num2,num1-num2);
// }

// fn multiplication(){
//     let num1=input("Enter 1st number:");
//     let num2=input("Enter 2nd number:");

//     println!("The multiplication of {} and {} is {}",num1,num2,num1*num2);
// }

// fn division(){
//     let num1=input("Enter 1st number:");
//     let num2=input("Enter 2nd number:");

//     println!("The division of {} and {} is {}",num1,num2,num1/num2);
// }