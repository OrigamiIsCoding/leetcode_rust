pub mod assert_macro;
pub mod container_macro;

pub mod i32_binary_tree;
pub mod i32_singly_linked_list;

#[macro_export]
macro_rules! input {
    ( $( $name : item = $value:item ), *) => {
            $(
                println!("name = {:?} value = {:?}", name, value);
            )*
        todo!()
    };
}
