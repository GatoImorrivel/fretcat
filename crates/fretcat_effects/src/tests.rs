use crate::{Chain, ChainCommand};
use crate::effects::Overdrive;

#[test]
fn chain_query() {
    let chain = Chain::default();

    let effect = chain.effects.get(0).unwrap();

    chain.query(effect).unwrap();
}

#[test]
fn chain_query_mut() {
    let mut chain = Chain::default();

    let effect = chain.effects.get(0).unwrap().clone();

    chain.query_mut(&effect).unwrap();
}

#[test]
fn chain_query_mut_cast() {
    let mut chain = Chain::default();

    let effect = chain.effects.get(0).unwrap().clone();

    let overdrive = {
        let overdrive = chain.query_cast_mut::<Overdrive>(&effect).unwrap();

        overdrive.blend = 10.0;
        overdrive.clone()
    };

    assert_eq!(overdrive.blend, chain.query_cast::<Overdrive>(&effect).unwrap().clone().blend);
}

#[test]
fn chain_insert() {
    let mut chain = Chain::default();

    let (_index, effect) = chain.insert(Box::new(Overdrive::default()));

    chain.query(&effect).unwrap();
}

#[test]
fn chain_insert_at() {
    let mut chain = Chain::default();

    let effect1 = chain.insert_at(1, Box::new(Overdrive::default()));
    let effect2 = chain.insert_at(2, Box::new(Overdrive::default()));

    assert_eq!(1usize, chain.get_position(&effect1).unwrap());
    assert_eq!(2usize, chain.get_position(&effect2).unwrap());
}

#[test]
fn chain_remove() {
    let mut chain = Chain::default();

    let (_index, effect) = chain.insert(Box::new(Overdrive::default()));
    assert_eq!(2, chain.effects.len());

    chain.remove(&effect);

    assert_eq!(1, chain.effects.len());
}

#[test]
fn chain_queue() {
    let mut chain = Chain::default();

    chain.add_to_queue(ChainCommand::Insert(Box::new(Overdrive::default())));

    assert_eq!(1, chain.update_queue.len());

    let command = chain.update_queue.pop().unwrap();

    chain.handle_command(command);

    assert_eq!(0, chain.update_queue.len());
    assert_eq!(2, chain.effects.len());
}
