use neon::prelude::*;
mod obj;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("difference", obj::difference)?;
    cx.export_function("intersection", obj::intersection)?;
    cx.export_function("statements", obj::statements)?;
    cx.export_function("merge", obj::merge)?;
    Ok(())
}
