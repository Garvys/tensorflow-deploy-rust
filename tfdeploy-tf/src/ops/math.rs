use tfdeploy::ops as tfdops;

use tfpb::node_def::NodeDef;
use ops::OpRegister;
use tfdeploy::TfdResult;

pub fn register_all_ops(reg: &mut OpRegister) {
    reg.insert("Abs", with_T!(tfdops::math::Abs));
    reg.insert("Add", with_T!(tfdops::math::Add));
    reg.insert("AddN", add_n);
    reg.insert("BiasAdd", with_T!(tfdops::math::Add));
    reg.insert("Div", with_T!(tfdops::math::Div));
    reg.insert("FloorMod", with_T!(tfdops::math::Rem));
    reg.insert("Mul", with_T!(tfdops::math::Mul));
    reg.insert("Neg", with_T!(tfdops::math::Neg));
    reg.insert("Rsqrt", with_T!(tfdops::math::Rsqrt));
    reg.insert("Sub", with_T!(tfdops::math::Sub));
    reg.insert("Tanh", with_T!(tfdops::math::Tanh));
}

pub fn add_n(pb: &NodeDef) -> TfdResult<Box<tfdops::Op>> {
    let dtype = pb.get_attr_datum_type("T")?;
    let n = pb.get_attr_int("N")?;
    Ok(Box::new(tfdops::math::AddN::new(dtype.into(), Some(n))))
}

