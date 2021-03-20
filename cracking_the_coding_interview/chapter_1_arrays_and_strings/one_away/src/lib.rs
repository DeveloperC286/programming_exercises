#[derive(Debug, PartialEq)]
pub enum Change {
    INSERT { to_insert: char },
    REMOVE { to_remove: char },
    MOVE { to_move: char },
    REPLACE { to_replace: char },
    NONE,
    IMPOSSIBLE,
}

pub fn one_away(current: String, desired: String) -> Change {
    if current == desired {
        return Change::NONE;
    }

    if (current.chars().count() + 1) == desired.chars().count() {
        if let Some(to_insert) = get_different_character(current, desired) {
            return Change::INSERT { to_insert };
        }
    } else if current.chars().count() == (desired.chars().count() + 1) {
        if let Some(to_remove) = get_different_character(desired, current) {
            return Change::REMOVE { to_remove };
        }
    } else if current.chars().count() == desired.chars().count() {
        let mut current_chars: Vec<char> = current.chars().collect();
        let mut desired_chars: Vec<char> = desired.chars().collect();
        let mut current_different_chars: Vec<char> = vec![];
        let mut desired_different_chars: Vec<char> = vec![];

        while !current_chars.is_empty() && !desired_chars.is_empty() {
            if current_chars.get(0) == desired_chars.get(0) {
                current_chars.remove(0);
                desired_chars.remove(0);
            } else {
                current_different_chars.push(current_chars.remove(0));
                desired_different_chars.push(desired_chars.remove(0));
            }
        }

        if current_different_chars.len() == 2 {
            desired_different_chars.reverse();

            if current_different_chars == desired_different_chars {
                return Change::MOVE {
                    to_move: *current_different_chars.get(0).unwrap(),
                };
            }
        }

        if current_different_chars.len() == 1 {
            return Change::REPLACE {
                to_replace: *current_different_chars.get(0).unwrap(),
            };
        }
    }

    Change::IMPOSSIBLE
}

fn get_different_character(shorter: String, longer: String) -> Option<char> {
    let mut shorter_chars: Vec<char> = shorter.chars().collect();
    let mut longer_chars: Vec<char> = longer.chars().collect();
    let mut different_chars = vec![];

    while !shorter_chars.is_empty() && !longer_chars.is_empty() {
        if shorter_chars.get(0) == longer_chars.get(0) {
            shorter_chars.remove(0);
            longer_chars.remove(0);
        } else {
            different_chars.push(longer_chars.remove(0));
        }
    }

    different_chars.extend(longer_chars);

    match different_chars.len() {
        1 => Some(*different_chars.get(0).unwrap()),
        _ => None,
    }
}

#[cfg(test)]
#[macro_use]
extern crate proptest;

#[cfg(test)]
mod tests;
