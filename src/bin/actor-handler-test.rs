use actix::prelude::*;

// this is our Message
// we have to define the response type (rtype)
#[derive(Message)]
#[rtype(result = "usize")]
struct Data(usize, usize);

#[derive(Message)]
#[rtype(result = "usize")]
struct Mul(usize, usize);

trait GetData<T> {
    fn calc(&self) -> T;
}

impl GetData<usize> for Mul {
    fn calc(&self) -> usize {
        self.0 * self.1
    }
}

impl GetData<usize> for Data {
    fn calc(&self) -> usize {
        self.0 + self.1
    }
}

// Actor definition
struct Calculator;

impl Actor for Calculator {
    type Context = Context<Self>;
}

trait CalcMessage: GetData<usize> + Message<Result = usize> {}

impl GetData<Box<dyn CalcMessage>> for Calculator {
    fn calc(&self) -> Box<dyn CalcMessage> {
        todo!()
    }
}

// now we need to implement `Handler` on `Calculator` for the `Sum` message.
impl Handler<Data> for Calculator {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Data, _ctx: &mut Context<Self>) -> Self::Result {
        msg.calc()
    }
}

impl Handler<Mul> for Calculator {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Mul, _ctx: &mut Context<Self>) -> Self::Result {
        msg.calc()
    }
}

#[actix::main] // <- starts the system and block until future resolves
async fn main() {
    // let data_to_send: Vec<Box<dyn GetData<usize> + Message<Result = usize>>> = vec![
    //     Box::new(Data(1, 2)),
    //     Box::new(Mul(3, 4)),
    // ];
    // data_to_send.iter().for_each(|x| {
    //     match Calculator.start().send(x).await {
    //         Ok(result) => println!("add result is: {result}"),
    //         _ => println!("ERR")
    //     }
    // });

    match Calculator.start().send(Data(114, 514)).await {
        Ok(result) => println!("add result is: {result}"),
        _ => println!("ERR")
    }
    match Calculator.start().send(Mul(114, 514)).await {
        Ok(result) => println!("add result is: {result}"),
        _ => println!("ERR")
    }
}