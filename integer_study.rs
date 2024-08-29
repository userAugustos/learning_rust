fn calc_price(unit_price: u32, quantity: u16, discount: i8, shipping_tax: u16) -> i64 {
    // So here we adjust the discount, so if the discount is a positive number, like: 10%, we
    // need to make it negative, so it becomes -10%, and if it's a negative number, like: -10%,
    // we need to make it positive, so it becomes 10%.
    // This way, a 10% discount will be -10% in the total price, and a -10% tax will be 10% in the total pice
    let real_discount: i32 = if discount >= 0 {
        -(discount as i32)
    } else {
        discount.abs() as i32
    };

    // calc total price before discount or tax
    let subtotal: i64 = (unit_price as i64) * (quantity as i64);

    // apply discount or tax (* because discount or tax is a percentage)
    let price: i64 = subtotal + ((subtotal * real_discount as i64) / 100);

    let total = price + (shipping_tax as i64);

    total
}

fn main() {
    let unit_price: u32 = 1999; // R$ 19,99
    let quantity: u16 = 3;
    let discount: i8 = 10; // 10%
    let shipping_tax: u16 = 799; // R$ 7,99

    let total = calc_price(unit_price, quantity, discount, shipping_tax);

    println!("Order total: R$ {:.2}", total as f64 / 100.0);
}
