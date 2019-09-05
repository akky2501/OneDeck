use std::collections::HashMap;

pub type Id = String;

#[derive(Debug)]
pub struct Interpreter {
    objects: ObjectPool,
    roles: RoleTable,
    entry_actor: Id,
    entry_situation: Id,
}

impl Interpreter {
    pub fn new(pool: HashMap<Id, Object>, table: HashMap<Id, Role>, entry_actor: Id, entry_situation: Id) -> Self {
        Interpreter {
            objects: ObjectPool{pool},
            roles: RoleTable{table},
            entry_actor,
            entry_situation,
        }
    }

    pub fn run(&mut self) {
        println!("[[BEGIN]]\n{:?}", self);

        let mut action_stack: Vec<Operation> = vec![
            Operation::Action(Action::DoSituation(
                PredicateAtom::Obj(self.entry_actor.clone()), 
                PredicateAtom::Obj(self.entry_situation.clone())
            ))
        ];

        while let Some(op) = action_stack.pop() {
            match op {
                Operation::Action(action) => self.exec_action(action, &mut action_stack),
            }
        }

        println!("[[END]]\n{:?}", self);
    }

    fn exec_action(&mut self, action: Action, stack: &mut Vec<Operation>) {
        use Action::*;
        use PredicateAtom::*;

        match action {
            Skip => (),
            Finish => stack.clear(),
            Shuffle(Obj(d)) => unimplemented!(),
            Reverse(Obj(d)) => unimplemented!(),
            AppendACard(Obj(d1), Card(c), Obj(d2)) => unimplemented!(),
            ExchangeACard(Obj(d1), Card(c1), Obj(d2), Card(c2)) => unimplemented!(),
            AppendNCards(Obj(d), Number(n), Obj(d2)) => unimplemented!(),
            MoveCards(Obj(d1), Obj(d2)) => unimplemented!(),
            ExchangeCards(Obj(d1), Obj(d2)) => unimplemented!(),
            DoSituation(Obj(actor), Obj(situation)) => {
                // 新しいActor(Env), Roleの下で「選択」を行う
                // 選択されたアクション列を逆順にスタックにpush
                let mut actions = self.select_choice(actor, situation)
                                      .into_iter()
                                      .rev()
                                      .map(Operation::Action)
                                      .collect();
                stack.append(&mut actions);
            },
            BranchLastAction(Obj(a), label, then, else_) => {
                if let Some(ref last_label) = self.objects.get_last_action(&a) {
                    if last_label == &label {
                        stack.push(Operation::Action(*then));                        
                    }
                }

                if let Some(else_) = else_ {
                    stack.push(Operation::Action(*else_));
                }
            },
            _ => {
                println!("不正なアクションです。");
                unreachable!();
            },
        }
    }

    fn select_choice(&mut self, actor: Id, situation: Id) -> Vec<Action> {
        let env = self.objects.get_env(&actor);
        let role_name = self.objects.get_role(&actor);
        let role = &self.roles.table[role_name];
        let choices = &role.situations[&situation];
        
        let mut availables:Vec<(usize, HashMap<PredicateAtom, PredicateAtom>)> = vec![];
        // それぞれの選択肢について仮定生成、availablesにpush
        for (i, choice) in choices.iter().enumerate() {
            let assu = self.assume(env);
            if let Some(assu) = assu {
                for a in assu {
                    availables.push((i, a));
                }
            }
        }

        // ユーザーにchoicesの何番目を選択するか問い合わせ
        //    (表示の時に仮定とラベルを見せること、変数や仮定はデリファレンスすること)
        // 選ばれた選択肢のラベルをactorのlast_actionに設定、ただしラベルもデリファレンスすること
        // 選ばれた選択肢のアクション列をデリファレンスしてこれを返す
        let (label, actions) = self.user_select(&availables, choices);
        self.objects.set_last_action(&actor, label);
        actions
    }

    fn assume(&self, env: &HashMap<Id, Id>) -> Option<Vec<HashMap<PredicateAtom, PredicateAtom>>> {
        // 深さ優先に探索しつつ仮定を埋めていく
        // 充足できる仮定を見つけて返す、Noneの時は充足できなかったということ
        // 仮定がない場合はSome(vec![HashMap::new()])になるはず
        unimplemented!()
    }

    fn user_select<T, U>(&self, _: &T, _: &U) -> (ChoiceLabel, Vec<Action>) {
        // 選ばれた選択肢のラベルをデリファレンスしてこれを返す
        // 選ばれた選択肢のアクション列をデリファレンスしてこれを返す
        unimplemented!()
    }

}

enum Operation {
    Action(Action),
}

#[derive(Debug)]
struct ObjectPool {
    pool: HashMap<Id, Object>,
}

impl ObjectPool {
    fn get_env(&self, actor: &Id) -> &HashMap<Id, Id> {
        match &self.pool[actor] {
            Object::Actor{role: _, env, ..} => env,
            _ => unreachable!(),
        }
    }

    fn get_role(&self, actor: &Id) -> &String {
        match &self.pool[actor] {
            Object::Actor{role,..} => role,
            _ => unreachable!(),
        }
    }

    fn get_last_action(&self, actor: &Id) -> &Option<ChoiceLabel> {
        match &self.pool[actor] {
            Object::Actor{role: _, env: _, last_action} => last_action,
            _ => unreachable!(),
        }
    }

    fn set_last_action(&mut self, actor: &Id, label: ChoiceLabel) {
        match self.pool.get_mut(actor).unwrap() {
            &mut Object::Actor{role: _, env: _, ref mut last_action} =>
                *last_action = Some(label),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct RoleTable {
    table: HashMap<Id, Role>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Suit {
    Spade,
    Hart,
    Club,
    Diamond,
    Joker,
}

pub type Number = u8;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Card(pub Suit, pub Number);

#[derive(Debug, Clone)]
pub enum Object {
    Actor {
        role: Id,
        env: HashMap<Id, Id>,
        last_action: Option<ChoiceLabel>,
    },
    Cards(Vec<Card>),
}

#[derive(Debug)]
pub struct Role {
    pub args: Vec<Id>,
    pub situations: HashMap<Id, Vec<Choice>>,
}


#[derive(Debug, Clone)]
pub struct Choice {
    pub _assum_vars: Vec<Id>,
    pub when: PredicateExpr,
    pub _must: bool,
    pub label: ChoiceLabel,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChoiceLabel {
    pub name: Id,

    // パラメタにはカード、スート、数字が登場可能(述語でアトムとしてあらわされるもの)
    // アクターのLastに保存するときは変数(仮定)はdereferenceすること
    pub params: Vec<PredicateAtom>, 
}

#[derive(Debug, Clone)]
pub enum Action {
    Skip,
    Finish,
    Shuffle(PredicateAtom),
    Reverse(PredicateAtom),
    AppendACard(PredicateAtom, PredicateAtom, PredicateAtom),
    ExchangeACard(PredicateAtom, PredicateAtom, PredicateAtom, PredicateAtom),
    AppendNCards(PredicateAtom, PredicateAtom, PredicateAtom),
    MoveCards(PredicateAtom, PredicateAtom),
    ExchangeCards(PredicateAtom, PredicateAtom),
    DoSituation(PredicateAtom, PredicateAtom),
    BranchLastAction(PredicateAtom, ChoiceLabel, Box<Action>, Option<Box<Action>>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PredicateExpr {
    True,
    False,

    And(Box<PredicateExpr>, Box<PredicateExpr>),
    Or(Box<PredicateExpr>, Box<PredicateExpr>),
    Not(Box<PredicateExpr>),
    
    In(PredicateAtom, PredicateAtom),
    Equal(PredicateAtom, PredicateAtom),
    EqSuit(PredicateAtom, PredicateAtom),
    EqNumber(PredicateAtom, PredicateAtom),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PredicateAtom {
    Number(Number),
    Suit(Suit),
    Card(Card),
    Const(Id),
    Var(Id),
    Obj(Id), // 変数や仮定をデリファレンスしたあとのオブジェクトを表す名前
}
