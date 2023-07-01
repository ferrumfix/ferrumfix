#[allow(dead_code)]
#[rustfmt::skip]
mod generated_fix42;

use core::fmt;
use fefix::definitions::fix42;
use fefix::FieldType;
use generated_fix42 as strum_fix42;
use strum::IntoEnumIterator;

fn main() -> anyhow::Result<()> {
    loop {
        // TODO & FIXME: this example is still incomplete.

        let user_action = prompt_user_action()?;
        match user_action {
            UserAction::EnterOrder => {
                let symbol = prompt_symbol()?;
                let side = prompt_side()?;
                let ord_type = prompt_order_type()?;
                let time_in_force = prompt_time_in_force()?;
                let price = prompt_price()?;
                let sender_comp_id = inquire::Text::new("SenderCompID").prompt()?;
                let target_comp_id = inquire::Text::new("TargetCompID").prompt()?;
                println!("symbol: {}", symbol);
                println!("side: {:?}", side);
                println!("ord_type: {:?}", ord_type);
                println!("time_in_force: {:?}", time_in_force);
                println!("price: {}", price);
                println!("sender_comp_id: {}", sender_comp_id);
                println!("target_comp_id: {}", target_comp_id);
            }
            UserAction::Quit => break,
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter, strum::Display)]
enum UserAction {
    EnterOrder,
    Quit,
}

fn prompt_user_action() -> anyhow::Result<UserAction> {
    let s = inquire::Select::new("Select an action", UserAction::iter().collect()).prompt()?;
    Ok(s)
}

fn prompt_symbol() -> anyhow::Result<String> {
    Ok(inquire::Text::new("Symbol").prompt()?)
}

fn prompt_side() -> anyhow::Result<fix42::Side> {
    prompt_fix_enum::<strum_fix42::Side, _>("Side")
}

fn prompt_order_type() -> anyhow::Result<fix42::OrdType> {
    prompt_fix_enum::<strum_fix42::OrdType, _>("Order type")
}

fn prompt_time_in_force() -> anyhow::Result<fix42::TimeInForce> {
    prompt_fix_enum::<strum_fix42::TimeInForce, _>("Time in force")
}

fn prompt_price() -> anyhow::Result<f32> {
    Ok(inquire::CustomType::<f32>::new("Price").prompt()?)
}

fn prompt_fix_enum<Strum, T>(prompt: &str) -> anyhow::Result<T>
where
    T: for<'a> FieldType<'a> + Clone,
    Strum: FieldType<'static> + strum::IntoEnumIterator + Copy,
    &'static str: From<Strum>,
{
    let choices: Vec<FixEnum<Strum>> = Strum::iter().map(FixEnum).collect();
    let selection = inquire::Select::new(prompt, choices).prompt()?;
    let result = T::deserialize(&selection.0.to_bytes()).ok().unwrap();

    Ok(result)
}

pub struct FixEnum<Strum>(pub Strum);

impl<Strum> fmt::Display for FixEnum<Strum>
where
    Strum: FieldType<'static> + strum::IntoEnumIterator + Copy,
    &'static str: From<Strum>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            FieldType::to_string(&self.0),
            <&'static str>::from(self.0)
        )
    }
}
