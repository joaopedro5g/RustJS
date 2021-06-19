use js_sandbox::{self, AnyError, Script};
use std::{fs::File, io::Read, thread, time::Duration};

fn thread(js: String) {
  let mut script = Script::from_string(&js)
    .expect("Erro ao tentar criar a função em uma nova thread");
  let threads: Vec<String> = script.call("main", &())
    .expect("Erro ao tentar executar a função main");
  
  let handle = thread::spawn(move || {
    for thread in threads {
      let mut script_thread = Script::from_string(&thread)
        .expect("Erro ao tentar buscar as funções retornadas da thread");
      let _: bool = script_thread.call(&thread, &())
        .expect("Error ao tentar executar os scripts da thread");
      thread::sleep(Duration::from_millis(1));
    }
  });
  handle.join().unwrap();
}

fn main() -> Result<(), AnyError> {
  let mut file = File::open("./js/main.js")
    .expect("Erro ao tentar ler o arquivo javascript");
  let mut js = String::new();
  file.read_to_string(&mut js)
    .expect("Erro ao tentar converter para String");
  thread(js);
  Ok(())
}
