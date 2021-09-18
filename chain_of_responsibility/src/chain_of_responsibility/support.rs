use crate::chain_of_responsibility::trouble::Trouble;

pub trait SupportTrait {
    fn resolve(&self, trouble: Trouble) -> bool;
    fn set_next(&mut self, next: Box<dyn SupportTrait>);
    fn handle(&self, trouble: Trouble);
}

pub struct Support {
    name: String,
    next: Option<Box<dyn SupportTrait>>,
}

impl Support {
    fn new(name: String) -> Support {
        Support {
            name: name,
            next: None,
        }
    }

    fn print(&self) -> String {
        format!("[{}]", self.name)
    }

    fn done(&self, trouble: Trouble) {
        println!("{} is resolved by {}.", trouble.print(), self.print());
    }

    fn fail(&self, trouble: Trouble) {
        println!("{} cannot be resolved.", trouble.print());
    }
}

pub struct NoSupport {
    support: Support,
}

impl NoSupport {
    pub fn new(name: String) -> NoSupport {
        NoSupport {
            support: Support::new(name),
        }
    }
}

impl SupportTrait for NoSupport {
    fn resolve(&self, _trouble: Trouble) -> bool {
        false
    }

    fn set_next(&mut self, next: Box<dyn SupportTrait>) {
        self.support.next = Some(next);
    }

    fn handle(&self, trouble: Trouble) {
        if self.resolve(trouble.clone()) {
            self.support.done(trouble);
        } else {
            match &self.support.next {
                Some(n) => n.handle(trouble),
                None => self.support.fail(trouble),
            }
        }
    }
}

pub struct LimitSupport {
    support: Support,
    limit: u32,
}

impl LimitSupport {
    pub fn new(name: String, limit: u32) -> LimitSupport {
        LimitSupport {
            support: Support::new(name),
            limit: limit,
        }
    }
}

impl SupportTrait for LimitSupport {
    fn resolve(&self, trouble: Trouble) -> bool {
        if trouble.get_number() < self.limit {
            true
        } else {
            false
        }
    }

    fn set_next(&mut self, next: Box<dyn SupportTrait>) {
        self.support.next = Some(next);
    }

    fn handle(&self, trouble: Trouble) {
        if self.resolve(trouble.clone()) {
            self.support.done(trouble);
        } else {
            match &self.support.next {
                Some(n) => n.handle(trouble),
                None => self.support.fail(trouble),
            }
        }
    }
}

pub struct OddSupport {
    support: Support,
}

impl OddSupport {
    pub fn new(name: String) -> OddSupport {
        OddSupport {
            support: Support::new(name),
        }
    }
}

impl SupportTrait for OddSupport {
    fn resolve(&self, trouble: Trouble) -> bool {
        if trouble.get_number() % 2 == 1 {
            true
        } else {
            false
        }
    }

    fn set_next(&mut self, next: Box<dyn SupportTrait>) {
        self.support.next = Some(next);
    }

    fn handle(&self, trouble: Trouble) {
        if self.resolve(trouble.clone()) {
            self.support.done(trouble);
        } else {
            match &self.support.next {
                Some(n) => n.handle(trouble),
                None => self.support.fail(trouble),
            }
        }
    }
}

pub struct SpecialSupport {
    support: Support,
    number: u32,
}

impl SpecialSupport {
    pub fn new(name: String, number: u32) -> SpecialSupport {
        SpecialSupport {
            support: Support::new(name),
            number: number,
        }
    }
}

impl SupportTrait for SpecialSupport {
    fn resolve(&self, trouble: Trouble) -> bool {
        if trouble.get_number() == self.number {
            true
        } else {
            false
        }
    }

    fn set_next(&mut self, next: Box<dyn SupportTrait>) {
        self.support.next = Some(next);
    }

    fn handle(&self, trouble: Trouble) {
        if self.resolve(trouble.clone()) {
            self.support.done(trouble);
        } else {
            match &self.support.next {
                Some(n) => n.handle(trouble),
                None => self.support.fail(trouble),
            }
        }
    }
}
