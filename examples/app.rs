use anyhow::Result;
use wasmtime::{Engine, Instance, Memory, Module, Store, TypedFunc};

pub fn main() -> Result<()> {
    let mut wasm = WebAssembly::new("target/wasm32-unknown-unknown/debug/wasm-memory.wasm")?;

    let input = vec![1u8, 2, 3];
    let result = wasm.sum_array(input)?;
    println!("{}", result);

    let input = String::from("convert to uppercase");
    let result = wasm.upper_string(input).unwrap();
    println!("{}", result);

    Ok(())
}

struct WebAssembly {
    store: Store<()>,
    memory: Memory,
    alloc: TypedFunc<i32, i32>,
    upper: TypedFunc<(i32, i32), i32>,
    sum: TypedFunc<(i32, i32), i32>,
    dealloc: TypedFunc<(i32, i32), ()>,
}

impl WebAssembly {
    pub fn new(filename: &str) -> Result<WebAssembly> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, filename)?;

        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;

        let wasm = WebAssembly {
            alloc: instance.get_typed_func(&mut store, "alloc")?,
            dealloc: instance.get_typed_func(&mut store, "dealloc")?,
            upper: instance.get_typed_func(&mut store, "upper")?,
            sum: instance.get_typed_func(&mut store, "sum")?,
            memory: instance.get_memory(&mut store, "memory").unwrap(),
            store,
        };

        Ok(wasm)
    }

    pub fn sum_array(&mut self, input: Vec<u8>) -> Result<i32, anyhow::Error> {
        let ptr = self.alloc.call(&mut self.store, input.len() as i32)?;

        // write the input array to the module's linear memory
        unsafe {
            let buf = self.memory.data_ptr(&mut self.store).offset(ptr as isize);
            buf.copy_from(input.as_ptr(), input.len());
        }

        // sum the array
        let params = (ptr as i32, input.len() as i32);
        let result = self.sum.call(&mut self.store, params)?;

        Ok(result)
    }

    pub fn upper_string(&mut self, input: String) -> Result<String, anyhow::Error> {
        // write the input string to the module's linear memory
        let bytes = input.as_bytes();
        let ptr = self.alloc.call(&mut self.store, bytes.len() as i32)?;
        unsafe {
            let buf = self.memory.data_ptr(&mut self.store).offset(ptr as isize);
            buf.copy_from(bytes.as_ptr(), bytes.len());
        }

        // upper the input string
        let ptr = self
            .upper
            .call(&mut self.store, (ptr, bytes.len() as i32))?;

        // read the output string from the module's linear memory
        // let slice = &self.memory.data(&mut self.store)[ptr as usize..ptr as usize + bytes.len()];
        let slice = self
            .memory
            .data(&mut self.store)
            .get(ptr as usize..)
            .and_then(|s| s.get(..bytes.len()))
            .unwrap();
        let str = std::str::from_utf8(slice)?;
        /*
        let mut buf = vec![0u8; bytes.len()];
        self.memory.read(&mut self.store, ptr as usize, buf.as_mut_slice())?;
        let str = std::str::from_utf8(buf.as_slice())?;
        */

        let result = String::from(str);

        self.dealloc
            .call(&mut self.store, (ptr as i32, bytes.len() as i32))?;

        Ok(result)
    }
}
