
pub mod functions{
    pub mod create_new{
        pub fn add(){
            println!("New function is added successfully");
        }
    }
}



fn main(){
  crate::functions::create_new::add();
}