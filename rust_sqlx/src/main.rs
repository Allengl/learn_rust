use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use std::env;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    id: i32,
    user_id: i32,
    product_id: i32,
    quantity: i32,
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;

    // 创建订单
    let order = Order {
        id: 1,
        user_id: 1,
        product_id: 1,
        quantity: 2,
        status: "pending".to_string(),
    };
    sqlx::query!(
        "INSERT INTO orders (id, user_id, product_id, quantity, status) VALUES (?, ?, ?, ?, ?)",
        order.id,
        order.user_id,
        order.product_id,
        order.quantity,
        order.status
    )
    .execute(&pool)
    .await?;

    // 查询订单
    let orders = sqlx::query_as!(
        Order,
        "SELECT id, user_id, product_id, quantity, status FROM orders"
    )
    .fetch_all(&pool)
    .await?;
    println!("Orders: {:?}", orders);

    // 模拟微服务之间的数据一致性
    let pool = Arc::new(Mutex::new(pool));
    let pool1 = pool.clone();
    let handle1 = tokio::spawn(async move {
        let mut conn = pool1.lock().await;
        sqlx::query!("UPDATE orders SET status = 'processing' WHERE id = 1")
            .execute(&mut *conn)
            .await
            .unwrap();
    });

    let pool2 = pool.clone();
    let handle2 = tokio::spawn(async move {
        let mut conn = pool2.lock().await;
        sqlx::query!("UPDATE orders SET status = 'shipped' WHERE id = 1")
            .execute(&mut *conn)
            .await
            .unwrap();
    });

    tokio::join!(handle1, handle2);

    // 查询订单状态
    let orders = sqlx::query_as!(
        Order,
        "SELECT id, user_id, product_id, quantity, status FROM orders"
    )
    .fetch_all(&pool.lock().await)
    .await?;
    println!("Orders: {:?}", orders);

    // 读取数据库迁移脚本
    let migration_script =
        fs::read_to_string("migrations/001_init.sql").expect("Failed to read migration script");

    // 执行数据库迁移
    sqlx::query(&migration_script)
        .execute(&pool.lock().await)
        .await?;

    println!("Database migration completed");

    // 运行测试
    let orders = sqlx::query_as!(
        Order,
        "SELECT id, user_id, product_id, quantity, status FROM orders"
    )
    .fetch_all(&pool.lock().await)
    .await?;
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].status, "pending");

    println!("Tests passed");

    Ok(())
}
