struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data {}", self.data);
  }
}

// Drop trait enable to execute some code in drop method in the moment of deallocation of the value from memory
fn main() {
  {
    let _x = CustomSmartPointer {
      data: String::from("smart pointer 1")
    };
    let _y = CustomSmartPointer {
      data: String::from("smart pointer 2")
    };
  }
  println!("End of first scope");

  {
    let _x = CustomSmartPointer {
      data: String::from("smart pointer 1")
    };
    drop(_x); // drop function deallocate the value from memory (std::mem::drop)
    let _y = CustomSmartPointer {
      data: String::from("smart pointer 2")
    };
  }
  println!("End of second scope");
}