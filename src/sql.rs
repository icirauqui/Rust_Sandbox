use opendp::core::*;
//use postgres::{Connection, TlsMode};
use postgres::{Client, Error, NoTls};





/*
fn create_table() {
    let mut client = Client::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres",
        NoTls,
    );

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ",
    );
}
*/

/*
fn insert_user(username: &str, password: &str, email: &str) {
    client.execute(
        "
        INSERT INTO users (username, password, email)
        VALUES ($1, $2, $3)
    ",
        &[&username, &password, &email],
    )?;
}
*/




fn main() {



    //let url = "postgresql://dboperator:operatorpass123@localhost:5243/postgres";
    let url = "postgresql://postgres:postgres@localhost:5432/postgres";
    let mut conn = Client::connect(url, NoTls).unwrap();


    let mut query = conn.query("
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ", &[]).unwrap();



    query = conn.query(
        "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)",
        &[&"user1", &"mypass", &"user@test.com"],
    ).unwrap();

}   



/*
fn main() -> Result<(), Error> {
    let client = Client::connect("postgres://postgres:postgres@localhost:5432", NoTls)?;
    let mut conn = client.get_txn()?;
    let mut query = conn.query("SELECT * FROM users", &[])?;
    let mut users: Vec<User> = Vec::new();
    while let Some(row) = query.next()? {
        let user: User = row.get(0);
        users.push(user);
    }
    println!("{:?}", users);
    Ok(())
}



fn main() {
    println!("Hello, world!");
}


*/















fn main() -> Result<(), Error> {

    let url = "postgresql://postgres:postgres@localhost:5432/postgres";
    let mut conn = Client::connect(url, NoTls).unwrap();

    let mut sn : VecVec64 = vec![];


    for row in conn.query("SELECT * from sn", &[])? {

        // let var00 : Option<f64> = row.get(0);
        // let var01 : Option<f64> = row.get(1);
        // let var02 : Option<f64> = row.get(2);
        // let var03 : Option<f64> = row.get(3);
        // let var04 : Option<f64> = row.get(4);
        // let var05 : Option<f64> = row.get(5);
        // let var06 : Option<f64> = row.get(6);
        // let var07 : Option<f64> = row.get(7);
        // let var08 : Option<f64> = row.get(8);
        // let var09 : Option<f64> = row.get(9);
        // let var10 : Option<f64> = row.get(10);
        // let var11 : Option<f64> = row.get(11);
        // let var12 : Option<f64> = row.get(12);
        // let var13 : Option<f64> = row.get(13);
        // let var14 : Option<f64> = row.get(14);
        // let var15 : Option<f64> = row.get(15);
        // let var16 : Option<f64> = row.get(16);
        // let var17 : Option<f64> = row.get(17);
        // let var18 : Option<f64> = row.get(18);
        // let var19 : Option<f64> = row.get(19);
        // let var20 : Option<f64> = row.get(20);


        sn.push(vec![row.get(0),
                     row.get(1),
                     row.get(2),
                     row.get(3),
                     row.get(4),
                     row.get(5),
                     row.get(6),
                     row.get(7),
                     row.get(8),
                     row.get(9),
                     row.get(10),
                     row.get(11),
                     row.get(12),
                     row.get(13),
                     row.get(14),
                     row.get(15),
                     row.get(16),
                     row.get(17),
                     row.get(18),
                     row.get(19),
                     row.get(20)]);

        //println!(
        //    "row i : {}) {}",
        //    var00.unwrap(), var01.unwrap()
        //);
    }

    println!("{}", sn[0][0].unwrap());



    // Split dataset into train and test
    let mut x_train_1 : VecVec64 = vec![];
    let mut y_train_1 : Vec<Option<f64>> = vec![];
    let mut x_test_1 : VecVec64 = vec![];
    let mut y_test_1 : Vec<Option<f64>> = vec![];

    let mut i : i64 = 0;
    let split : i64 = (sn.len() as f64 * 0.9) as i64;
    for n in sn {
        if i < split {
            x_train_1.push(n[0..20].to_vec());
            y_train_1.push(n[20]);
        } else {
            x_test_1.push(n[0..20].to_vec());
            y_test_1.push(n[20]);
        }
        i = i + 1;
    }


