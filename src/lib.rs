pub struct Reaction {
    reactants: ReactionGroup,
    products: ReactionGroup,
    energy_change: f64,
}

pub struct ReactionGroup {
    primary: Vec<ReactionItem>,
    other: Vec<ReactionItem>,
}

enum ReactionItem {
    Molecule(Molecule),
}

impl Reaction {
    pub fn is_exothermic(&self) -> bool { self.energy_change < 0.0 }
    pub fn is_endothermic(&self) -> bool { self.energy_change > 0.0}
    pub fn is_isothermic(&self) -> bool { self.energy_change == 0.0 }
}
