pub struct DataBuffer{
    data: Vec<u8>
}


impl DataBuffer{
   pub fn new() -> Self{
       DataBuffer{data: Vec::new()}
   } 

   pub fn create(data: &[u8]) -> Self{
       DataBuffer {data: Vec::from(data)}
   }

   pub fn append(&mut self, byte_data: u8){
       self.data.push(byte_data);
   }

   pub fn append_all(&mut self, bytes: &[u8]){
        self.data.extend_from_slice(bytes);
   }

   pub fn clear(&mut self){
       self.data.clear();
   }

   pub fn empty_the_buffer(&mut self) -> Vec<u8>{
       let data_copy = self.data.clone();
       self.clear();
       data_copy
   }
   pub fn get_data(&self) -> &Vec<u8>{
       &self.data
   }

   pub fn len(&self)-> usize{
       self.data.len()
   }

}
