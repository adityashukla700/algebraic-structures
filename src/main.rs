use std::ops;

#[derive(Debug)]
struct Field{
    value:u32,
    prime:u32
}
impl Field{
    fn is_prime(n:u32)->bool{
        for i in 2 .. n-1{
            if n%i == 0{
                return false;
            } 
        }
        return true;
    }
    fn new(value:u32,prime:u32)->Self{
        if value>=prime {
            panic!("Value {} is not in range 0 to {}",value,prime-1);
        }
        if !Self::is_prime(prime){
            panic!("Second argument must be prime. {} is not prime",prime);
        }
        Field {value:value,prime:prime}
    }
}

impl ops::Add<Field> for Field{
    type Output=Field;
    fn add(self,f2:Field)->Field{
        if self.prime!=f2.prime {
            panic!("F1 and F2 should be field of same prime");
        }
        let output:u32=(self.value+f2.value)%self.prime;
        return Field::new(output,self.prime);
    }
}

impl ops::Mul<Field> for Field{
    type Output=Field;
    fn mul(self,f2:Field)->Field{
        if self.prime!=f2.prime{
            panic!("F1 and F2 should be field of same prime");
        }
        let output:u32=(self.value*f2.value)%self.prime;
        return Field::new(output,self.prime);
    }
}

fn main(){
    let field1:Field=Field::new(3,7);
    let field2:Field=Field::new(5,7);
    println!("common prime: {}",field1.prime);
    println!("value1: {}",field1.value);
    println!("value2: {}",field2.value);

    println!("Now doing their addition in Field");
    let resultant:Field=field1+field2;
    println!("Resultant: {:?}",resultant);

    let field3:Field=Field::new(3,7);
    let field4:Field=Field::new(6,7);
    let field5:Field=Field::new(5,7);

    println!("Field multiplication");
    let mul:Field=field3*(field4+field5);
    println!("Resultant: {:?}",mul);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_creation() {
        assert_eq!(Field::new(3, 7).value, Field { value: 3, prime: 7 }.value);
    }

    #[test]
    #[should_panic(expected = "Value 7 is not in range 0 to 6")]
    fn test_field_creation_out_of_range() {
        Field::new(7, 7);
    }

    #[test]
    #[should_panic(expected = "Second argument must be prime. 8 is not prime")]
    fn test_field_creation_non_prime() {
        Field::new(3, 8);
    }

    #[test]
    fn test_field_addition() {
        let field1 = Field::new(3, 7);
        let field2 = Field::new(5, 7);
        let expected_result = Field::new(1, 7);
        assert_eq!((field1 + field2).value, expected_result.value);
    }

    #[test]
    fn test_field_multiplication() {
        let field3 = Field::new(3, 7);
        let field4 = Field::new(6, 7);
        let field5 = Field::new(5, 7);
        let result = field3*(field4+field5);
        let expected_result = Field::new(5, 7);
        assert_eq!(result.value, expected_result.value);
    }
}
