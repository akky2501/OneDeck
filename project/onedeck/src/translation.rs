use std::collections::HashMap;
use crate::obj;
use crate::syntax;


pub fn setup(tl: Vec<syntax::Toplevel>) -> Result<obj::Interpreter, &'static str> {
    use syntax::Toplevel::*;
    use obj::*;

    let mut pool = HashMap::new();
    let mut table = HashMap::new();
    let mut entry_actor = None;
    let mut entry_situation = None;

    let mut actors = vec![];

    for t in tl {
        match t {
            DeclCards(name, deck) => {
                pool.insert(name, deck);
            },
            DeclActor(name, role, actual) => {
                actors.push((name, role, actual))
            },
            DeclEntry(Action::DoSituation(PredicateAtom::Const(actor), PredicateAtom::Const(situation))) => {
                if entry_actor.is_some() {
                    return Err("'Entry'は1つだけ記述できます");
                }
                entry_actor = Some(actor);
                entry_situation = Some(situation);
            },
            DeclRole(name, role) => {
                // _assum_varsは後で解析して埋める必要があるかも
                table.insert(name, role);
            },
            _ => return Err("不正な入力を読み込みました"),
        }
    }

    if entry_actor.is_none() {
        return Err("'Entry'がありません");
    }

    for (name, role, actual) in actors {
        let args = &table[&role].args;
        let mut env = args.iter()
                      .cloned()
                      .zip(actual.into_iter())
                      .collect::<HashMap<_, _>>();
        env.insert("自分".to_string(), name.clone());
        pool.insert(name,
            Object::Actor{
                role,
                env,
                last_action: None,
            }
        );
    }

    Ok(Interpreter::new(pool, table, entry_actor.unwrap(), entry_situation.unwrap()))
}