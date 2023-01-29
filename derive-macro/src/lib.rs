extern crate proc_macro;


use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AsWorldObject)]
pub fn derive_as_worldobject_fn(_item: TokenStream) -> TokenStream {
    let DeriveInput {ident,..} = parse_macro_input!(_item);
    let struct_name = ident.to_owned();
    let mut imp_string = "impl WorldObject for ".to_owned();
    imp_string.push_str( struct_name.to_string().as_str());
    imp_string.push_str(
        "
        {
            fn global_id(&self) -> usize {
                self.id
            }
            fn name(&self) -> &str {
                self.name.as_str()
            }
            fn tag(&self) -> &str {
                self.tag.as_str()
            }
            fn get_transform(&self) -> &Transform
            {
                &self.transform
            }
            fn get_mut_transform(&mut self) -> &mut Transform
            {
                &mut self.transform
            }

            fn is_active(&self) -> bool {
                self.is_active
            }
            fn set_active(&mut self, active: bool) {
                self.is_active = active;
                self.sprite.is_enabled = active;
                self.collider.is_enabled = active;
                //self.set_world(world);
            }
            fn reset(&mut self) {
                self.reset();
            }
            fn update_world(&self, world: &mut World) {
                self.set_world(world);
            }

            fn get_collider(&self) -> Collider {
                self.collider
            }
            fn get_mut_collider(&mut self) -> &mut Collider {
                &mut self.collider
            }

            fn draw_DEBUG_Collider(&mut self) {
                self.draw_DEBUG_Collider();
            }

            fn receive_damage(&mut self, dmg: f32) {
                self.params.receive_damage(dmg);
            }
            fn get_damage(&self) -> f32 {
                self.params.damage
            }

            fn get_health(&self) -> f32 {
                self.params.health
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self as &dyn std::any::Any
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self as &mut dyn std::any::Any
            }
            fn clone_dyn(&self) -> Box<dyn WorldObject>
            {
                Box::new(self.clone())
            }

            fn is_marked(&self) -> bool {
                self.mark_to_destroy
            }

        }
    "
    );
    imp_string.parse().unwrap()
}