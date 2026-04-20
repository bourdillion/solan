use anchor_lang::prelude::*;

declare_id!("8N1LC4H8fJVMP5BhXESbDrEJdWHDSijTHNhjbUVoDjVN");

#[program]
pub mod tryrust {
    use super::*;
    use std::collections::HashMap;

    pub fn matchAge(ctx: Context<Initialize>, age: u64) -> Result<()> {
        match age {
            1 => {
                msg!("Your age is 1");
            }
            2 | 3 => {
                msg!("Your age is 2 or 3");
            }
            4..=6 => {
                msg!("Your age is between 4 to 6");
            }
            _ => {
                msg!("Your age is something else")
            }
        }
        Ok(())
    }

    pub fn loopOver(ctx: Context<Initialize>) -> Result<()> {
        for i in (0..=10).step_by(2) {
            msg!("the number is {}", i);
        }

        Ok(())
    }

    pub fn arrayOver(ctx: Context<Initialize>) -> Result<()> {
        let my_array: [u32; 5] = [10, 20, 30, 40, 50];

        //Accessing elements out of te array
        let first_element = my_array[0];
        let third_element: u32 = my_array[2];

        //declare a mutable array with a fixed size of 3
        let mut mutArray: [u32; 3] = [100, 200, 300];

        //change the second element from 200 to 250
        mutArray[1] = 250;

        for i in 0..mutArray.len() {
            msg!("The array is {}", mutArray[i]);
        }

        Ok(())
    }

    pub fn dynamicArray(ctx: Context<Initialize>) -> Result<()> {
        let mut dynamic_array: Vec<u32> = Vec::new();
        //add element to the dynamic array
        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(30);

        //Access the first and third element
        let first_element: u32 = dynamic_array[0];
        let third_element: u32 = dynamic_array[2];

        msg!("third element = {}", third_element);

        Ok(())
    }

    pub fn mapping(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        //initialize the mapping
        let mut my_map = HashMap::new();

        //Add a key value pair to the mapping
        my_map.insert(key.to_string(), value.to_string());

        //log the value corresponding to a key from the mapping
        msg!("My name is {}", my_map[&key]);
        Ok(())
    }

    pub fn structs(ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        struct Person {
            my_name: String,
            my_age: u64,
        }

        //creating an instance of the struct
        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        //modifying struct string
        person1.my_name = "Bob".to_string();
        person1.my_age = 18;

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        Ok(())
    }

    pub fn checkEven(ctx: Context<Initialize>) -> Result<()> {
        let mut dynamic_array: Vec<u64> = Vec::new();
        let mut new_dynarray: Vec<u64> = Vec::new();
        for i in 0..=100 {
            dynamic_array.push(i);
        }

        for i in 0..dynamic_array.len() {
            if i == 0 || i % 2 == 0 {
                new_dynarray.push(i as u64);
            }
        }

        msg!("old array is {:?}", dynamic_array);
        msg!("new array is {:?}", new_dynarray);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
