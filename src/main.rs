mod characters;
mod rules;

fn main() {
    fight()
}

fn fight() {
    let fighter1 = characters::BaseCharacter::new();
}

/*
fn arena() {
    //report start
    run_simulation()
    //report end
}

fn run_simulation() {
    // database insert_simulation
    let number_of_years = 100;
    let fights_per_year = 12;
    for _year in 0..number_of_years {
        for _fight in 0..fights_per_year {
            run_one_year()
        }
        end_year()
    }
    // database insert_many_fighters
}

fn run_one_year() {
    size = self._fighters.get_size()
    for i in range(size, self._model.fights_per_year):
        fighter:Fighter = Fighter.create(self._model.uuid, "John"+str(Arena.NUM_SEQ), 1, ArmorList.LEATHER)
        Arena.NUM_SEQ += 1
        fighter.append_action(MeleeWeaponAttack.create_greataxe(fighter))
        self._fighters.add_member(fighter)
    self._fighters.shuffle()
    for i in range(0, self._fighters.get_size(), 2):
        party_1 = ClassParty()
        party_1.add_member(self._fighters.get_member(i))
        party_2 = ClassParty()
        party_2.add_member(self._fighters.get_member(i+1))
        winner = self.fight(party_1, party_2)
        if winner:
            party_1.add_experience(party_2.get_xp_from_fallen())
        else:
            party_2.add_experience(party_1.get_xp_from_fallen())
    self._fighters.remove_dead()
    self._fighters.long_rest()
}

fn end_year() {

}
*/
