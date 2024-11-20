// rule1; rule2; rule3;
#[derive(Debug)]
pub struct Grammar {
    pub declarations: Vec<Declaration>,
    pub rules: Vec<Rule>,
    pub programs: String,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub arguments: Vec<String>,
}

// foo: bar baz { ... } | qux { ...};
#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub alternatives: Vec<Alternative>,
}

#[derive(Debug)]
pub struct Alternative {
    pub elements: Vec<String>,
}
