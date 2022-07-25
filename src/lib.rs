// built with the help of
// * <https://github.com/wildarch/pest-calculator/blob/main/docs/Tutorial.md>
// * <https://pest.rs/book/>


extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate lazy_static;

use std::collections::HashMap;
use pest::Parser;
use pest::iterators::Pairs;
use lazy_static::lazy_static;
use pest::prec_climber::*;
#[derive(Parser)]
#[grammar = "./calculator.pest"]
pub struct CalculatorParser;


lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use pest::prec_climber::Assoc::*;
        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right)
        ])
    };
}

fn eval(expression: Pairs<Rule>, identifiers: &HashMap<String, f64>) -> f64 {
    use pest::iterators::Pair;
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::id => {
                let identifier = pair.as_str();
                if !identifiers.contains_key(identifier) {
                    panic!("{} is not a known identifier", identifier);
                }
                (identifiers.get(identifier).unwrap()).clone()
            }
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval(pair.into_inner(), identifiers),
            _ => unreachable!(),
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add      => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide   => lhs / rhs,
            Rule::power    => lhs.powf(rhs),
            _ => unreachable!(),
        },
    )
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test() {
        let mut ids: HashMap<String, f64> = HashMap::new();
        ids.insert(String::from("a"), 1.0);
        ids.insert(String::from("b"), 2.0);
        ids.insert(String::from("c"), 3.0);
        ids.insert(String::from("e"), 4.0);
        ids.insert(String::from("jeff"), 5.0);
        match CalculatorParser::parse(Rule::calculation, "a+b+c+e+jeff")  {
            Ok(pairs) => {
                let res = eval(pairs, &ids);
                println!("{}", res);
                assert_eq!(15.0, res);
            },
            Err(e) => {
                eprintln!("Parse failed {:?}",e);
            }
        }
    }

}
