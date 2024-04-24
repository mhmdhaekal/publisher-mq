## Muhammad Haekal Kalipaksi

## 2206817490

### Reflections

**How many data your publlsher program will send to the message broker in one run?**

```rs
let users: Vec<&str> = vec!["Amir", "Budi", "Cica", "Dira", "Emir"];

for (i, user) in users.iter().enumerate() {
    let user_id = (i + 1).to_string();
    let user_name = format!("2206817490-{}", user);
    let message: UserCreatedEventMesage = UserCreatedEventMesage { user_id, user_name };
    _ = publisher.send(String::from("user_created"), message)
}
```

Based on code above the publisher will send 5 user_created data with messages for user "Amir", "Budi", "Cica", "Dira", "Emir"

**The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?**

The publisher will send the messages into the same messages broker and the subcriber will receive the data.

**Running RabbitMq as message broker**
![init-raabit](images/init-rabbit.png?raw=true)
