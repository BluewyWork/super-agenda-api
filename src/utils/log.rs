pub fn plog(message: String, scope: String, err: bool) {
   if err {
      println!("[ERR]         {} => {}", scope, message);
   } else {
      println!("[LOG]         {} => {}", scope, message);
   }
}
