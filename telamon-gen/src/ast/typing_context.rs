use super::*;
use std::ops::Deref;

/// CheckContext is a type system.
#[derive(Debug, Default)]
pub struct CheckerContext {
    /// Map Name of unique identifiant.
    hash_set: HashMap<String, Spanned<Hint>>,
    hash_choice: HashMap<String, Spanned<Hint>>,
}

impl CheckerContext {

    /// This checks the redefinition of SetDef.
    pub fn declare_set(
        &mut self,
        object_name: Spanned<String>,
    ) -> Result<(), TypeError> {
        if let Some(pre) = self.hash_set.insert(
            object_name.data.to_owned(), object_name.with_data(Hint::Set)
        ) {
            Err(TypeError::Redefinition { object_kind: pre, object_name })
        } else {
            Ok(())
        }
    }

    /// This checks the redefinition of ChoiceDef (EnumDef and IntegerDef).
    pub fn declare_choice(
        &mut self,
        object_name: Spanned<String>,
        object_type: Hint,
    ) -> Result<(), TypeError> {
        if let Some(pre) = self.hash_choice.insert(
            object_name.data.to_owned(), object_name.with_data(object_type)
        ) {
            Err(TypeError::Redefinition { object_kind: pre, object_name })
        } else {
            Ok(())
        }
    }

    /// TODO: https://github.com/ulysseB/telamon/blob/41fbbf33a091106988b623ab4f76f1a33bb3ee0f/src/search_space/choices.exh#L83
    pub fn check_require_define(
        &self,
        variables: &Vec<VarDef>,
        conditions: &Vec<Vec<Condition>>,
    ) -> Result<(), TypeError> {
        for var in variables.iter() {
            let name: String = var.set.name.data.to_owned().to_string();
            if !self.hash_set.contains_key(&name) {
                Err(TypeError::Undefined {
                    object_name: var.set.name.with_data(name.to_owned())
                })?;
            }
        }
        for conds in conditions.iter() {
            for cond in conds {
                match cond {
                    Condition::Is {
                        lhs: ChoiceInstance { name: enumeration, .. }, rhs, ..
                    } => {
                        let name: String = enumeration.data.to_owned().to_string();
                        if !self.hash_choice.contains_key(&name) {
                            Err(TypeError::Undefined {
                                object_name: enumeration.with_data(name.to_owned())
                            })?;
                        }
                    },
                    _ => {},
                }
            }
        }
        Ok(())
    }

    /// This checks the undefined of SetDef superset and arg.
    pub fn check_set_define(
        &self,
        object_name: &Spanned<String>,
        field_arg: &Option<VarDef>,
        field_superset: &Option<SetRef>,
    ) -> Result<(), TypeError> {
        if let Some(VarDef { name: _, set: SetRef { name, .. } }) = field_arg {
            let name: &String = name.data.deref();
            if !self.hash_set.contains_key(name) {
                Err(TypeError::Undefined {
                    object_name: object_name.with_data(name.to_owned()),
                })?;
            }
        }
        if let Some(SetRef { name: supername, .. }) = field_superset {
            let name: &String = supername.data.deref();
            if !self.hash_set.contains_key(name) {
                Err(TypeError::Undefined {
                    object_name: object_name.with_data(name.to_owned()),
                })?;
            }
        }
        Ok(())
    }

    /// This checks the undefined of EnumDef or IntegerDef.
    pub fn check_choice_define(
        &self,
        object_name: &Spanned<String>,
        field_variables: &Vec<VarDef>,
    ) -> Result<(), TypeError> {
        for VarDef { name: _, set: SetRef { name, .. } } in field_variables {
            let name: &String = name.data.deref();
            if !self.hash_set.contains_key(name) {
                Err(TypeError::Undefined {
                    object_name: object_name.with_data(name.to_owned()),
                })?;
            }
        }
        Ok(())
    }
}
