#![allow(dead_code, unused)]
use std::collections::HashMap;
use std::env;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    pub fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

pub fn main() {
    println!("Chapter 21: Project Solution");

    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order: [u32; 6] = [2, 1, 2, 3, 4, 1];

    // Task 1
    blender_orders(&orders);

    // Task 2
    microwave_quantity_sum(&orders);

    // Task 3
    command_line_quantity(&orders);

    // Task 4
    quantity_of_unshipped_orders(&orders);

    // Task 5
    ship_next_unshipped_order(&mut orders);

    // Task 6
    let customers = build_customers(&orders, &customer_ids_by_order);
    println!("Customers: {customers:#?}");
}

/// Task 1: Print Customer Orders for blenders
fn blender_orders(orders: &[CustomerOrder]) {
    let blender_orders: Vec<&CustomerOrder> = orders
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect();
    println!("Blender orders: {:#?}", blender_orders);
}

/// Task 2: Print Microwave Orders Quantity Sum using filter and map, and then filter_map
fn microwave_quantity_sum(orders: &[CustomerOrder]) {
    let microwave_quantity_filter_and_map: u32 = orders
        .iter()
        .filter(|order| order.product == Product::Microwave)
        .map(|order| order.quantity)
        .sum();
    println!("Microwave Orders Quantity (via filter + map): {microwave_quantity_filter_and_map}");

    let microwave_quantity_filter_map: u32 = orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum();
    println!("Microwave Orders Quantity (via filter_map): {microwave_quantity_filter_map}");
}

/// Task 3: Command-line quantity filter
fn command_line_quantity(orders: &[CustomerOrder]) {
    // Print vector of orders where the quantity is >= to the quantity passed via commandline args
    let quantity = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u32>().unwrap_or(2))
        .next()
        .unwrap_or(2);
    // let Some(quantity) = quantity else {
    //     eprintln!("Enter a quantity to filter by");
    //     std::process::exit(-1);
    // };

    let orders = orders
        .iter()
        .filter(|order| order.quantity >= quantity)
        .collect::<Vec<&CustomerOrder>>();

    println!("Orders >= to commandline quantity: {orders:#?}");
}

/// Task 4: Print out a HashMap showing the quantity of unshipped orders
/// { Fridge: 10, Toaster: 2, Blender: 4 }
fn quantity_of_unshipped_orders(orders: &[CustomerOrder]) {
    let unshipped_quantity =
        orders
            .iter()
            .filter(|order| !order.shipped)
            .fold(HashMap::new(), |mut acc, order| {
                acc.entry(&order.product)
                    .and_modify(|v| *v += order.quantity)
                    .or_insert(order.quantity);
                acc
            });

    println!("Quantity unshipped orders: {unshipped_quantity:#?}");
}

/// Task 5: Ship next unshipped order
fn ship_next_unshipped_order(orders: &mut Vec<CustomerOrder>) {
    if let Some(order) = orders.iter_mut().find(|order| !order.shipped) {
        order.shipped = !order.shipped;
    }
    println!("Orders: {orders:#?}");
}

/// Task 6: Build Customers, order by id
fn build_customers(orders: &[CustomerOrder], customer_ids: &[u32]) -> Vec<Customer> {
    let id_orders_map = orders.iter().zip(customer_ids.iter()).fold(
        HashMap::<u32, Vec<&CustomerOrder>>::new(),
        |mut acc, (order, &id)| {
            acc.entry(id)
                .and_modify(|v| v.push(order))
                .or_insert(vec![order]);

            acc
        },
    );

    let mut customers: Vec<Customer> = id_orders_map
        .into_iter()
        .map(|(id, orders)| Customer {
            id,
            orders: orders
                .iter()
                .map(|order| CustomerOrder::new(order.product, order.quantity, order.shipped))
                .collect(),
        })
        .collect();
    customers.sort_by_key(|customer| customer.id);

    customers
}
