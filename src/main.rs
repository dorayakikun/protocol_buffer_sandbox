extern crate protobuf;

mod addressbook;

use protobuf::{Message, RepeatedField};

use self::addressbook::{AddressBook, Person};

fn main() {
    // creates a message struct
    let mut person = Person::new();
    person.set_name("Dorayaki Koala-Ya's".to_string());
    person.set_id(101);
    person.set_email("dorayaki@tsubuan.com".to_string());
    let mut address_book = AddressBook::new();
    address_book.set_people(RepeatedField::from_vec(vec![person]));

    println!("{:?}", address_book);

    // to bytes
    let bytes = address_book.write_to_bytes().unwrap();
    
    // from bytes
    let mut new_address_book = AddressBook::new();
    new_address_book.merge_from_bytes(&bytes).unwrap();

    println!("{:?}", new_address_book);
}
