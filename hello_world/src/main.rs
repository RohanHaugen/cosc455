fn main() {
    let numbers=[1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let mut max:i32;
    let mut min:i32;
    let mut mean:f64;
    max=numbers[0];
    min=numbers[0];
    mean=0.0;

    for &number in numbers.iter(){
        if number>max{max=number}else if number<min{min=number}
        mean+=number as f64;
    }
    mean=mean/numbers.len() as f64;
    assert_eq!(max,56);
    assert_eq!(min,-18);
    assert_eq!(mean,12.5);
    println!("Tests passed");
    /*let message=['h','e','l','l','o'];
    for(index,&item) in message.iter().enumerate(){
        println!("item {} is {}",index,item);
        if item =='e'{
            break;
        }
    }*/
    //let make_x_odd=true;
    //let x= if make_x_odd{1}else{2};
    /*let celsius_temp=23.0;
    let fahrenheit_temp=celsius_to_fahrenheit(celsius_temp);
    assert_eq!(fahrenheit_temp,73.4);
    println!("Test Passed");*/
    /*say_hello();
    say_hello();
    let x=1;
    let y=2;
    say_the_sum(x, y);*/
    /*let stuff = (10, 3.14, 'x');
    println!("stuff: {}",stuff.0);*/

    //let garage: [[[i32; 100]; 20];5];
    /*let numbers: [i32; 5];
    numbers = [0;5];
    let index: usize = numbers.len();
    println!("last number is {}",numbers[index]);*/

    /*let a =13;
    let b =2.3;
    let c :f32 = 120.0;
    let average = (a as f64+b +c as f64)/3.0;
    assert_eq!(average, 45.1);
    println!("Test Passed");*/

    /*let letter = 'a';
    let number = '1';
    let finger ='\u{261D}';*/
    /*let a =true;
    let b =false;
    println!("a is {} and b is {}", a, b);
    println!("a is EQUAL TO b is {}", a==b);
    println!("a NOT EQUAL TO b is {}", a!=b);
    println!("a GREATER THAN b is {}", a>b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a>=b);
    println!("a LESS THAN OR EQUAL TO b is {}", a<=b);
    println!("a LESS THAN b is {}", a<b);*/

    /*let a = true;
    let b = false;
    println!("a is {} and b is {}", a,b);
    println!("NOT a is {}",!a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^b);
    let c = (a ^b)&&panic!();
    println!("c is {}",c);*/
    /*let mut value =0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}",value);
    value = !value;
    println!("value is {:08b}",value);
    value = value & 0b1111_0111;
    println!("value is {:08b}",value);
    println!("bit 6 is {}", value & 0b0100_0000);
    value = value | 0b0100_0000;
    println!("value is {:08b}",value);
    value = value ^ 0b0101_0101;
    println!("value is {:08b}",value);
    value = value << 4;
    println!("value is {:08b}",value);
    value = value >> 2;
    println!("value is {:08b}",value);*/
    //let mut x: u8 =255;
    //let y:f32=10.123456789;
    //println!("x is {}", x);
    //x = x+1;
    //println!("x is {}",x);
    //let a =10;
    //let b=3.0;
    //let c = a as f64 /(b+1.0);
    //println!("c is {0:08.3}\na is {1}\nonce again, c is {0}",c,a);

}
fn say_hello(){
    println!("Hello");
    say_a_number(13);
}
fn say_a_number(number:i32){
    println!("Number is {}", number);
}
fn say_the_sum(a:u8,b:u8){
    let sum=a+b;
    println!("Sum is {}",sum);
}
fn celsius_to_fahrenheit(celsius:f64) -> f64{
    let fahrenheit=(celsius*1.8)+32.0;
    return fahrenheit;
}