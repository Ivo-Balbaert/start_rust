mod library {
  pub struct Interface {
    m_impl: Impl
  }
  
  impl Interface {
    pub fn new() -> Interface {
      Interface{ m_impl: Impl }
    }
    
    pub fn f(&self){
      self.m_impl.f();
    }
  }
  
  struct Impl;
  
  impl Impl {
    pub fn f(&self){
      println!("f");
    }
  }
}

fn main() {
  let o = library::Interface::new();
  o.f();
}